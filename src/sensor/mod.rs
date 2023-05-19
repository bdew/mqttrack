use std::{path::PathBuf, str::FromStr, time::Duration};

use anyhow::{Context, Result};
use tokio::time::Instant;

mod net;
mod read;
mod temp;

pub use net::NET_HANDLER;
pub use temp::TEMP_HANDLER;
use tracing::info;

pub struct Sensor<T: 'static> {
    pub name: String,
    pub path: PathBuf,
    pub interval: Duration,
    pub last: Option<SensorSample<T>>,
    pub handler: &'static SensorHandler<T>,
}

pub struct PollResultData {
    pub suffix: &'static str,
    pub data: String,
}

pub enum PollResult {
    Data(Vec<PollResultData>, Duration),
    Delay(Duration),
}

pub trait PollableSensor<'a> {
    fn poll(&mut self) -> Result<PollResult>;
    fn name(&self) -> String;
}

pub struct SensorSample<T> {
    pub val: T,
    pub ts: Instant,
}

type SensorHandlerReader<T> = fn(&Sensor<T>) -> Result<(T, Vec<PollResultData>)>;

pub struct SensorHandler<T: 'static> {
    pub read: SensorHandlerReader<T>,
}

impl<T> Sensor<T> {
    pub fn new(
        file_path: String,
        interval: Duration,
        prefix: &str,
        handler: &'static SensorHandler<T>,
    ) -> Result<Self> {
        let path = PathBuf::from_str(&file_path)?;
        let name = path
            .file_name()
            .and_then(|v| v.to_str())
            .context("Invalid path")?;

        Ok(Sensor {
            name: format!("{}/{}", prefix, name),
            path,
            interval,
            handler,
            last: None,
        })
    }
}

pub type SensorBoxes<'a> = Vec<Box<dyn PollableSensor<'a> + Send>>;

pub fn add_sensors<T: Send>(
    paths: Vec<String>,
    interval: Duration,
    prefix: &str,
    handler: &'static SensorHandler<T>,
    into: &mut SensorBoxes,
) -> Result<()> {
    for path in paths {
        let sensor = Box::new(
            Sensor::new(path.clone(), interval, prefix, handler)
                .context(format!("Failed to parse sensor '{}'", path))?,
        );
        info!(sensor = sensor.name, "Added sensor");
        into.push(sensor);
    }

    Ok(())
}

impl<'a, T> PollableSensor<'a> for Sensor<T> {
    fn poll(&mut self) -> Result<PollResult> {
        let now: Instant = Instant::now();
        let (needs_update, next) = match &self.last {
            Some(x) => {
                if x.ts + self.interval < now {
                    (true, self.interval)
                } else {
                    (false, self.interval - (now - x.ts))
                }
            }
            None => (true, self.interval),
        };

        if needs_update {
            let (res, pkt) = (self.handler.read)(self)?;
            self.last = Some(SensorSample { ts: now, val: res });
            Ok(PollResult::Data(pkt, next))
        } else {
            Ok(PollResult::Delay(next))
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
