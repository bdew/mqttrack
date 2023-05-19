use anyhow::Result;

use super::{read::read_subpath, PollResultData, Sensor, SensorHandler};

fn handle_sensor_temp(sensor: &Sensor<()>) -> Result<((), Vec<PollResultData>)> {
    let v = read_subpath::<u64>(&sensor.path, "temp")? as f32 / 1000f32;

    Ok((
        (),
        vec![PollResultData {
            suffix: "",
            data: format!("{:.2}", v),
        }],
    ))
}

pub static TEMP_HANDLER: SensorHandler<()> = SensorHandler {
    read: handle_sensor_temp,
};
