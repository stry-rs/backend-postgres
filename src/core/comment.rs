use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{models::Comment, backend::{BackendEntry, Id, New, Existing}},
};

#[async_trait::async_trait]
impl BackendEntry<Comment, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Comment>, PostgresBackendError> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Comment>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Comment>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Comment>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
