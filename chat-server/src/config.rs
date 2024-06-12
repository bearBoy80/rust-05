use std::{
    env::{self, current_dir},
    fs::File,
};

use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppCfg {
    pub server: ServerCfg,
    pub auth: AuthCfg,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCfg {
    pub port: u16,
    pub db_url: String,
    pub base_dir: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCfg {
    pub sk: String,
    pub pk: String,
}
impl AppCfg {
    pub fn load() -> anyhow::Result<Self> {
        println!("{:?}", current_dir());
        let ret = match (
            File::open("./chat.yml"),
            File::open("/etc/config/chat.yml"),
            env::var("CHAT_CFG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("config file not found"),
        };
        Ok(ret?)
    }
}
#[cfg(test)]
mod tests {
    use super::AppCfg;

    #[test]
    pub fn test_file_load() {
        let ret = AppCfg::load().unwrap();
        assert!(!ret.auth.pk.is_empty());
        assert!(!ret.auth.sk.is_empty());
        assert!(!ret.server.db_url.is_empty());
        println!("{:?}", ret);
    }
}
