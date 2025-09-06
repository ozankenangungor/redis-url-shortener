use deadpool_redis::Pool;

#[derive(Clone)]
pub struct AppState {
    pub redis_pool: Pool,
}
