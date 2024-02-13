use domain::{AdminUseCase, Password, ReaderUseCase, Repository};
use mimir::application::Application;
use mimir::configuration::Settings;
use mimir::telemetry::init_subscriber;
use once_cell::sync::Lazy;
use reqwest::Client;
use secrecy::Secret;

const ADMIN_PASSWORD: &'static str = "123456578";

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = "test";
    let default_filter = "info";

    if std::env::var("TEST_LOG").is_ok() {
        init_subscriber(subscriber_name, default_filter, std::io::stdout)
    } else {
        init_subscriber(subscriber_name, default_filter, std::io::sink)
    }
    .expect("Failed to init subscriber");
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub reader_use_case: web::Data<ReaderUseCase>,
    pub admin_use_case: web::Data<AdminUseCase>,
    pub client: Client,
}

impl TestApp {
    pub async fn login(&self) -> reqwest::Response {
        self.client
            .post(&format!("{}/login", self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("password={}", ADMIN_PASSWORD))
            .send()
            .await
            .expect("Failed to login as admin.")
    }

    pub async fn post(&self, endpoint: &str, body: &str) -> reqwest::Response {
        self.client
            .post(&format!("{}/{}", self.address, endpoint))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get(&self, endpoint: &str) -> reqwest::Response {
        self.client
            .get(&format!("{}/{}", self.address, endpoint))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let configuration = {
        let mut c = Settings::read_from_file().expect("Failed to read configuration.");
        c.database.url = "sqlite::memory:".to_string(); // Use in-memory DB
        c.application.port = 0; // Use random OS port
        c
    };

    let application = Application::start(configuration)
        .await
        .expect("Failed to start application");
    let application_port = application.port();

    application
        .repository
        .update_admin_password(
            Password::parse(Secret::new(ADMIN_PASSWORD.to_string()))
                .unwrap()
                .hash_password()
                .unwrap()
                .as_str(),
        )
        .await
        .expect("Failed to insert admin password");

    let address = format!("http://127.0.0.1:{}", application_port);

    let reader_use_case = application.reader_use_case.clone();
    let admin_use_case = application.admin_use_case.clone();
    let _handle = tokio::spawn(application.run_until_stopped());

    let client = Client::builder().cookie_store(true).build().unwrap();

    TestApp {
        address,
        port: application_port,
        reader_use_case,
        admin_use_case,
        client,
    }
}
