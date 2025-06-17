mod common;

use embedded_hal::i2c::I2c;
use hydro_sense::{
    df0991::{DFRobotRGBButton, RGBBUTTON_DEFAULT_I2C_ADDR},
    i2c::find_adapter,
};
use linux_embedded_hal::I2cdev;
use log::info;
use std::{thread::sleep, time::Duration};

fn log_button_state_loop<I2C, E>(
    button: &mut DFRobotRGBButton<I2C>,
    iterations: usize, // max number of loops to run
) -> Result<(), E>
where
    I2C: I2c<Error = E>,
{
    for _ in 0..iterations {
        let pressed = button.get_button_status()?;
        if pressed {
            info!("Button is pressed");
        } else {
            info!("Button is NOT pressed");
        }
        sleep(Duration::from_millis(500));
    }
    Ok(())
}

#[test]
fn test_state() -> anyhow::Result<()> {
    common::init_logger();

    // Setup I2C and button for testing
    let device_name = "MCP2221";
    let dev = find_adapter(device_name)?;
    let i2c = I2cdev::new(dev)?;

    let mut button = DFRobotRGBButton::new(i2c, RGBBUTTON_DEFAULT_I2C_ADDR)?;
    assert!(button.begin()?);

    // Run logging loop 5 times only, so test finishes
    log_button_state_loop(&mut button, 5)?;

    Ok(())
}
