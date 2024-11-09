use std::future::Future;

use atrium_common::store::{
    memory::{MemoryCellStore, MemoryMapStore},
    CellStore, MapStore,
};

use crate::types::stage::Stage;

#[cfg_attr(not(target_arch = "wasm32"), trait_variant::make(Send))]
pub trait AtpStageMapStore: MapStore<String, Stage> + Clone {
    fn get_stage(&self, key: &String) -> impl Future<Output = Option<Stage>>;
    fn set_stage(&self, key: String, stage: Stage) -> impl Future<Output = ()>;
    fn del_stage(&self, key: &String) -> impl Future<Output = ()>;
}

impl<T> AtpStageMapStore for T
where
    T: MapStore<String, Stage> + Clone + Send + Sync,
{
    async fn get_stage(&self, key: &String) -> Option<Stage> {
        self.get(key).await.expect("Infallible")
    }
    async fn set_stage(&self, key: String, stage: Stage) {
        self.set(key, stage).await.expect("Infallible")
    }
    async fn del_stage(&self, key: &String) {
        self.del(key).await.expect("Infallible")
    }
}

pub type MemoryStageMapStore = MemoryMapStore<String, Stage>;

#[cfg_attr(not(target_arch = "wasm32"), trait_variant::make(Send))]
pub trait AtpStageCellStore: CellStore<Stage> + Clone {
    fn get_stage(&self) -> impl Future<Output = Option<Stage>>;
    fn set_stage(&self, stage: Stage) -> impl Future<Output = ()>;
    fn del_stage(&self) -> impl Future<Output = ()>;
}

impl<T> AtpStageCellStore for T
where
    T: CellStore<Stage> + Clone + Send + Sync,
{
    async fn get_stage(&self) -> Option<Stage> {
        self.get().await.expect("Infallible")
    }
    async fn set_stage(&self, stage: Stage) {
        self.set(stage).await.expect("Infallible")
    }
    async fn del_stage(&self) {
        self.clear().await.expect("Infallible")
    }
}

pub type MemoryStageCellStore = MemoryCellStore<Stage>;
