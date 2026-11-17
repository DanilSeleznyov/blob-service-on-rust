use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::internal::config::DatabaseConfig;

pub async fn connect(cfg: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(
            PgConnectOptions::new()
                .host(&cfg.host)
                .port(cfg.port)
                .username(&cfg.user)
                .password(&cfg.password)
                .database(&cfg.name),
        )
        .await
}
