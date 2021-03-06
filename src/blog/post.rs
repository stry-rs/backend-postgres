use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{models::Post, backend::{BackendEntry, Id, New, Existing}},
};

#[async_trait::async_trait]
impl BackendEntry<Post, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Post>, PostgresBackendError> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Post>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Post>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Post>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
