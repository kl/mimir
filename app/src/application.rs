use crate::configuration::{DatabaseSettings, Settings};
use data::sqlite_repository::SqliteRepository;
use domain::{AdminUseCase, ReaderUseCase};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::TcpListener;
use std::time;
use web::startup::ServerArguments;

pub struct Application {
    port: u16,
    server: web::Server,
    pub repository: SqliteRepository,
    pub reader_use_case: web::Data<ReaderUseCase>,
    pub admin_use_case: web::Data<AdminUseCase>,
}

impl Application {
    pub async fn start(config: Settings) -> anyhow::Result<Self> {
        let connection_pool = get_connection_pool(&config.database, false).await?;
        // migrate! includes the migrations in the binary at compile time
        sqlx::migrate!("../migrations")
            .run(&connection_pool)
            .await?;

        let repository = SqliteRepository::new(connection_pool.clone());
        let reader_use_case = web::Data::new(ReaderUseCase::new(repository.clone()));
        let admin_use_case = web::Data::new(AdminUseCase::new(repository.clone()));

        let address = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = web::startup::run_server(ServerArguments {
            listener,
            reader_use_case: reader_use_case.clone(),
            admin_use_case: admin_use_case.clone(),
            hmac_secret: config.application.hmac_secret,
        })?;

        Ok(Self {
            port,
            server,
            repository,
            reader_use_case,
            admin_use_case,
        })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub async fn get_connection_pool(
    config: &DatabaseSettings,
    create_if_missing: bool,
) -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .acquire_timeout(time::Duration::from_secs(2))
        .connect_with(
            config
                .connect_options()?
                .create_if_missing(create_if_missing),
        )
        .await
}
