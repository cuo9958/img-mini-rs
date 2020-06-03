use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        //添加默认配置
        s.merge(File::with_name("config/default"))?;
        //获取环境变量
        let env = env::var("MODE_ENV").unwrap_or("development".into());
        //获取环境配置
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        //获取本地配置
        s.merge(File::with_name("config/local").required(false))?;
        // 从手动设置的变量中获取配置 (需要有一个固定的前缀)
        // 例如：`APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;
        s.try_into()
    }
}
