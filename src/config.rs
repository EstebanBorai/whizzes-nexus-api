use rocket::config::LogLevel;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;

pub struct Config {
    pub host: IpAddr,
    pub jwt_secret: String,
    pub port: u16,
    pub database_url: String,
    pub server_config: rocket::Config,
}

impl Config {
    pub fn new() -> Self {
        let port = Config::env_var::<u16>("PORT");
        let host = Config::env_var::<IpAddr>("HOST");
        let jwt_secret = Config::env_var::<String>("JWT_SECRET");
        let database_url = Config::env_var::<String>("DATABASE_URL");
        let log_level = if cfg!(debug_assertions) {
            LogLevel::Debug
        } else {
            LogLevel::Critical
        };
        let server_config = rocket::Config {
            address: host,
            port,
            log_level,
            ..rocket::Config::default()
        };

        Config {
            host,
            jwt_secret,
            port,
            database_url,
            server_config,
        }
    }

    fn env_var<T: FromStr>(key: &str) -> T {
        let value =
            env::var(key).unwrap_or_else(|_| panic!("Missing environment variable: {}", key));

        if let Ok(parsed) = str::parse::<T>(&value) {
            return parsed;
        }

        panic!(
            "Failed to parse environment variable value from key: {}",
            key
        );
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::Config;

    #[test]
    #[should_panic(expected = "Missing environment variable: PORT")]
    fn panics_if_env_variable_not_present() {
        env::remove_var("PORT");

        Config::new();
    }

    #[test]
    #[should_panic(expected = "Failed to parse environment variable value from key: PORT")]
    fn panics_if_env_variable_couldnt_be_parsed() {
        env::set_var("PORT", "NOT_A_NUMBER");

        Config::new();
    }
}
