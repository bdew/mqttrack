use anyhow::{Ok, Result};
use tokio::time::Instant;

use super::{read::read_subpath, PollResultData, Sensor, SensorHandler};

pub struct NetData {
    tx: u64,
    rx: u64,
}

const TX_PATH: &str = "statistics/tx_bytes";
const RX_PATH: &str = "statistics/rx_bytes";

fn handle_sensor_net(sensor: &Sensor<NetData>) -> Result<(NetData, Vec<PollResultData>)> {
    let next = NetData {
        tx: read_subpath(&sensor.path, TX_PATH)?,
        rx: read_subpath(&sensor.path, RX_PATH)?,
    };

    match &sensor.last {
        Some(last) => {
            let td = (Instant::now() - last.ts).as_secs_f32();
            let txd = (next.tx - last.val.tx) as f32 / td;
            let rxd = (next.rx - last.val.rx) as f32 / td;
            Ok((
                next,
                vec![
                    PollResultData {
                        suffix: "/tx",
                        data: format!("{:.0}", txd),
                    },
                    PollResultData {
                        suffix: "/rx",
                        data: format!("{:.0}", rxd),
                    },
                ],
            ))
        }
        None => Ok((next, vec![])),
    }
}

pub static NET_HANDLER: SensorHandler<NetData> = SensorHandler {
    read: handle_sensor_net,
};
