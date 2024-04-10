pub mod callback;
pub mod config;
pub mod middleware;
pub mod response;
pub mod router;
pub mod server;
pub mod state;
pub mod swagger;

use lazy_static::lazy_static;
use snowflake_rs::SnowFlakeId;
use std::sync::Mutex;

lazy_static! {
    pub static ref COMMON_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(1, snowflake_rs::STANDARD_EPOCH));
    pub static ref ATTACK_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(2, snowflake_rs::STANDARD_EPOCH));
    pub static ref PROCESS_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(3, snowflake_rs::STANDARD_EPOCH));
    pub static ref SYSTEM_LOG_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(4, snowflake_rs::STANDARD_EPOCH));
    pub static ref HOST_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(5, snowflake_rs::STANDARD_EPOCH));
    pub static ref VULSTATE_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(6, snowflake_rs::STANDARD_EPOCH));
    pub static ref ALERT_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(7, snowflake_rs::STANDARD_EPOCH));
    pub static ref WEAKPASSWORD_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(8, snowflake_rs::STANDARD_EPOCH));
    pub static ref FILE_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(9, snowflake_rs::STANDARD_EPOCH));
    pub static ref WEB_THREAT_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(10, snowflake_rs::STANDARD_EPOCH));
    pub static ref HONEYPOT_THREAT_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(11, snowflake_rs::STANDARD_EPOCH));
    pub static ref ASSET_SNOW_FLAKE_ID: Mutex<SnowFlakeId> =
        Mutex::new(SnowFlakeId::new(12, snowflake_rs::STANDARD_EPOCH));
}
