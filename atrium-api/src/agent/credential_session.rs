use crate::client::{com, Service};
use crate::com::atproto::server::{create_account, create_session, delete_session, get_session};
use crate::did_doc::DidDocument;
use crate::types::string::Did;
use crate::types::TryFromUnknown;
use atrium_common::store::SimpleStore;
use atrium_xrpc::error::Error;
use atrium_xrpc::XrpcClient;
use std::sync::{Arc, RwLock};

use super::{inner, Session};

pub struct CredentialSession<S, T>
where
    S: SimpleStore<(), Session> + Send + Sync,
    T: XrpcClient + Send + Sync,
{
    pub pds_endpoint: RwLock<String>,
    pub store: Arc<inner::Store<S>>,
    server: com::Service<inner::Client<S, T>>,
}

impl<S, T> CredentialSession<S, T>
where
    S: SimpleStore<(), Session> + Send + Sync,
    T: XrpcClient + Send + Sync,
{
    /// Create a new agent.
    pub fn new(pds_endpoint: String, xrpc: T, store: S) -> Self {
        let store = Arc::new(inner::Store(store));

        Self {
            pds_endpoint: RwLock::new(pds_endpoint),
            store: Arc::clone(&store),
            server: com::Service::new(Arc::new(inner::Client::new(store, xrpc))),
        }
    }

    pub async fn get_did(&self) -> Option<Did> {
        self.store.get_session().await.map(|session| session.did)
    }

    // get dispatchUrl() {
    //   return this.pdsUrl || this.serviceUrl
    // }

    pub async fn create_account(
        &mut self,
        input: create_account::InputData,
    ) -> Result<(), Error<create_account::Error>> {
        let result = self.server.atproto.server.create_account(input.clone().into()).await?;
        // TODO
        // this.session = undefined
        // this.persistSession?.('create-failed', undefined)

        let create_account::OutputData { access_jwt, did, did_doc, handle, refresh_jwt } = *result;

        self.store
            .set_session(
                create_session::OutputData {
                    access_jwt,
                    did,
                    did_doc,
                    handle,
                    refresh_jwt,
                    active: Some(true),
                    email: input.email.clone(),
                    // TODO
                    // emailConfirmed: false,
                    // emailAuthFactor: false,
                    email_auth_factor: None,
                    email_confirmed: None,
                    status: None,
                }
                .into(),
            )
            .await;

        if let Ok(Some(did_doc)) = result.did_doc.map(DidDocument::try_from_unknown).transpose() {
            self.update_endpoint(&did_doc);
        }
        Ok(())
    }

    /// Start a new session with this agent.
    pub async fn login(
        &self,
        identifier: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<Session, Error<create_session::Error>> {
        let result = self
            .server
            .atproto
            .server
            .create_session(
                create_session::InputData {
                    auth_factor_token: None,
                    identifier: identifier.as_ref().into(),
                    password: password.as_ref().into(),
                }
                .into(),
            )
            .await?;

        self.store.set_session(result.clone()).await;

        if let Some(did_doc) = result
            .did_doc
            .as_ref()
            .and_then(|value| DidDocument::try_from_unknown(value.clone()).ok())
        {
            self.update_endpoint(&did_doc);
        }

        Ok(result)
    }

    pub async fn logout(&self) -> Result<(), Error<delete_session::Error>> {
        self.store.clear_session().await;

        self.server.atproto.server.delete_session().await
    }

    /// Resume a pre-existing session with this agent.
    pub async fn resume_session(
        &self,
        session: Session,
    ) -> Result<(), Error<crate::com::atproto::server::get_session::Error>> {
        self.store.set_session(session.clone()).await;

        match self.server.atproto.server.get_session().await {
            Ok(output) => {
                // TODO
                assert_eq!(output.data.did, session.data.did);

                if let Some(session) = self.store.get_session().await.as_deref().cloned() {
                    let session = create_session::OutputData {
                        did_doc: output.data.did_doc.clone(),
                        email: output.data.email,
                        email_confirmed: output.data.email_confirmed,
                        handle: output.data.handle,
                        ..session
                    };
                    self.store.set_session(session.into()).await;
                }
                if let Some(did_doc) = output
                    .data
                    .did_doc
                    .as_ref()
                    .and_then(|value| DidDocument::try_from_unknown(value.clone()).ok())
                {
                    self.update_endpoint(&did_doc);
                }
                Ok(())
            }
            Err(err) => {
                self.store.clear_session().await;
                Err(err)
            }
        }
    }

    // Internal helper to refresh sessions
    // - Wraps the actual implementation to ensure only one refresh is attempted at a time.
    async fn refresh_session(&self) {
        {
            let mut is_refreshing = self.is_refreshing.lock().await;
            if *is_refreshing {
                drop(is_refreshing);
                return self.notify.notified().await;
            }
            *is_refreshing = true;
        }
        // TODO: Ensure `is_refreshing` is reliably set to false even in the event of unexpected errors within `refresh_session_inner()`.
        self.refresh_session_inner().await;
        *self.is_refreshing.lock().await = false;
        self.notify.notify_waiters();
    }
    async fn refresh_session_inner(&self) {
        if let Ok(output) = self.call_refresh_session().await {
            if let Some(mut session) = self.store.get_session().await {
                session.access_jwt = output.data.access_jwt;
                session.did = output.data.did;
                session.did_doc = output.data.did_doc.clone();
                session.handle = output.data.handle;
                session.refresh_jwt = output.data.refresh_jwt;
                self.store.set_session(session).await;
            }
            if let Some(did_doc) = output
                .data
                .did_doc
                .as_ref()
                .and_then(|value| DidDocument::try_from_unknown(value.clone()).ok())
            {
                self.store.update_endpoint(&did_doc);
            }
        } else {
            self.store.clear_session().await;
        }
    }
    // same as `crate::client::com::atproto::server::Service::refresh_session()`
    async fn call_refresh_session(
        &self,
    ) -> Result<
        crate::com::atproto::server::refresh_session::Output,
        crate::com::atproto::server::refresh_session::Error,
    > {
        let response = self
            .inner
            .send_xrpc::<(), (), _, _>(&XrpcRequest {
                method: Method::POST,
                nsid: crate::com::atproto::server::refresh_session::NSID.into(),
                parameters: None,
                input: None,
                encoding: None,
            })
            .await?;
        match response {
            OutputDataOrBytes::Data(data) => Ok(data),
            _ => Err(Error::UnexpectedResponseType),
        }
    }
    fn is_expired<O, E>(result: &Result<OutputDataOrBytes<O>, E>) -> bool
    where
        O: DeserializeOwned + Send + Sync,
        E: DeserializeOwned + Send + Sync + Debug,
    {
        if let Err(Error::XrpcResponse(response)) = &result {
            if let Some(XrpcErrorKind::Undefined(body)) = &response.error {
                if let Some("ExpiredToken") = &body.error.as_deref() {
                    return true;
                }
            }
        }
        false
    }
    pub fn get_endpoint(&self) -> String {
        self.pds_endpoint.read().expect("failed to read endpoint").clone()
    }
    pub fn update_endpoint(&self, did_doc: &DidDocument) {
        if let Some(endpoint) = did_doc.get_pds_endpoint() {
            *self.pds_endpoint.write().expect("failed to write endpoint") = endpoint;
        }
    }
}

// public pdsUrl?: URL // The PDS URL, driven by the did doc
// public session?: AtpSessionData
// public refreshSessionPromise: Promise<void> | undefined

// /**
//  * Private {@link ComAtprotoServerNS} used to perform session management API
//  * calls on the service endpoint. Calls performed by this agent will not be
//  * authenticated using the user's session to allow proper manual configuration
//  * of the headers when performing session management operations.
//  */
// protected server = new ComAtprotoServerNS(
//   // Note that the use of the codegen "schemas" (to instantiate `this.api`),
//   // as well as the use of `ComAtprotoServerNS` will cause this class to
//   // reference (way) more code than it actually needs. It is not possible,
//   // with the current state of the codegen, to generate a client that only
//   // includes the methods that are actually used by this class. This is a
//   // known limitation that should be addressed in a future version of the
//   // codegen.
//   new XrpcClient((url, init) => {
//     return (0, this.fetch)(new URL(url, this.serviceUrl), init)
//   }, schemas),
// )

// constructor(
// ) {}

// get hasSession() {
//   return !!this.session
// }

// /**
//  * Create a new account and hydrate its session in this agent.
//  */
// async createAccount(
// }

// /**
//  * Start a new session with this agent.
//  */
// async login(
// }

// async logout(): Promise<void> {
// }

// /**
//  * Resume a pre-existing session with this agent.
//  */
// async resumeSession(
// }

// /**
//  * Internal helper to refresh sessions
//  * - Wraps the actual implementation in a promise-guard to ensure only
//  *   one refresh is attempted at a time.
//  */
// async refreshSession(): Promise<void> {
// }

// /**
//  * Internal helper to refresh sessions (actual behavior)
//  */
// private async _refreshSessionInner() {
// }

// /**
//  * Helper to update the pds endpoint dynamically.
//  *
//  * The session methods (create, resume, refresh) may respond with the user's
//  * did document which contains the user's canonical PDS endpoint. That endpoint
//  * may differ from the endpoint used to contact the server. We capture that
//  * PDS endpoint and update the client to use that given endpoint for future
//  * requests. (This helps ensure smooth migrations between PDSes, especially
//  * when the PDSes are operated by a single org.)
//  */
// private _updateApiEndpoint(didDoc: unknown) {
// }
