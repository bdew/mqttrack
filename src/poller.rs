use std::time::Duration;

use anyhow::Result;
use rumqttc::{AsyncClient, QoS};
use tokio::time;
use tracing::{error, info};

use crate::sensor::{PollResult, PollableSensor};

pub async fn poller_task(
    mut sensors: Vec<Box<dyn PollableSensor<'_> + Send>>,
    client: AsyncClient,
    base_topic: String,
) -> Result<()> {
    loop {
        let mut next = Duration::from_secs(60);

        for sensor in sensors.iter_mut() {
            let name = sensor.name();
            match sensor.poll() {
                Ok(PollResult::Data(data, delay)) => {
                    for item in data {
                        let topic = format!("{}/{}{}", base_topic, name, item.suffix);

                        info!(sensor = name, topic, data = &item.data, "Sensor send");

                        client
                            .publish(topic, QoS::AtLeastOnce, true, item.data)
                            .await?;
                    }

                    if next > delay {
                        next = delay;
                    };
                }

                Ok(PollResult::Delay(delay)) => {
                    info!(sensor = name, delay = format!("{:?}", delay), "Sensor skip");
                    if next > delay {
                        next = delay;
                    }
                }

                Err(err) => error!(sensor = name, error = format!("{:?}", err), "Sensor error"),
            }
        }

        time::sleep(next).await;
    }
}
