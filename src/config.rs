const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080; 

pub struct Config {
  pub host: String,
  pub port: u16,
}

impl Config {
    pub fn load() -> Self {
      Self {
        host: HOST.to_string(),
        port: PORT
      }
    }
}