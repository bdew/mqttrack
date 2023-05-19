use std::env;

use anyhow::{Context, Result};

#[derive(Clone, Debug)]
pub struct Vars {
    pub broker_url: String,
    pub base_topic: String,
    pub sensors_temp: Vec<String>,
    pub sensors_net: Vec<String>,
}

fn split_var_strs(key: &str) -> Vec<String> {
    env::var(key)
        .map(|v| v.split(';').map(|e| e.trim().to_string()).collect())
        .unwrap_or(vec![])
}

impl Vars {
    pub fn from_env() -> Result<Vars> {
        Ok(Vars {
            broker_url: env::var("MQTT_BROKER").context("MQTT_BROKER not set")?,
            base_topic: env::var("MQTT_TOPIC").context("MQTT_TOPIC not set")?,
            sensors_temp: split_var_strs("S_TEMP"),
            sensors_net: split_var_strs("S_NET"),
        })
    }
}
