use std::time::Duration;

use anyhow::{anyhow, Result};
use poller::poller_task;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet};
use sensor::{add_sensors, SensorBoxes, NET_HANDLER, TEMP_HANDLER};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use vars::Vars;

mod poller;
mod sensor;
mod vars;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let vars = Vars::from_env()?;

    let mqttoptions = MqttOptions::parse_url(vars.broker_url.clone())?;

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let mut sensors: SensorBoxes = Vec::new();

    add_sensors(
        vars.sensors_temp,
        Duration::from_millis(30000),
        "temp",
        &TEMP_HANDLER,
        &mut sensors,
    )?;

    add_sensors(
        vars.sensors_net,
        Duration::from_millis(5000),
        "net",
        &NET_HANDLER,
        &mut sensors,
    )?;

    if sensors.is_empty() {
        return Err(anyhow!("No sensors defined"));
    }

    info!("Starting with {} sensors", sensors.len());

    tokio::task::spawn(poller_task(sensors, client, vars.base_topic));

    loop {
        match eventloop.poll().await {
            Ok(ev) => match ev {
                Event::Incoming(Packet::ConnAck(ack)) => {
                    info!(code = format!("{:?}", ack.code), "Connected to broker")
                }
                _ => (),
            },
            Err(err) => {
                error!(error = format!("{:?}", err), "Connection error");
            }
        }
    }
}
