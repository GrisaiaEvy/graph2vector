use config::{Config, File};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONF: Config = Config::builder().add_source(File::with_name("config.toml")).build().expect("initial config failed");
    pub static ref RUST_LOG: String = CONF.get_string("RUST_LOG").unwrap_or_default();
}