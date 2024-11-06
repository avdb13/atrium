use atrium_xrpc::HttpClient;

pub trait SessionManager<T>
where
    T: HttpClient,
{
}
