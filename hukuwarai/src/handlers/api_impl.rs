use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct ApiImpl {
    pub pool: Pool<Postgres>
}

impl AsRef<ApiImpl> for ApiImpl {
    fn as_ref(&self) -> &ApiImpl {
        self
    }
}
