#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub domain_root_url: Option<String>,
}

impl Config {
    #[cfg(debug_assertions)]
    pub fn read() -> Config {
        Config {
            google_client_id: None,
            google_client_secret: None,
            domain_root_url: None,
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn read() -> Config {
        Config::read_from_env()
    }

    #[allow(dead_code)]
    pub fn read_from_env() -> Config {
        Config {
            google_client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
            domain_root_url: std::env::var("DOMAIN_ROOT_URL").ok(),
        }
    }
}
