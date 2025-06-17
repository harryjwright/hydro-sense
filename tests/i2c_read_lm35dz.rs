/*
 * Always keep my coding and comment style.
 */

mod common;

use hydro_sensor::ads1115::{AdsSensor, Mux, Pga};
use hydro_sensor::i2c::find_adapter;
use linux_embedded_hal::I2cdev;

#[test]
fn test_read_lm35_voltage() {
    common::init_logger();

    // ┌──────────────────────────────────────────────────────────────┐
    // │                    Open I2C Device Adapter                   │
    // │                                                              │
    // │ Attempt to find the I2C adapter named "MCP2221" and open it. │
    // │ If either step fails, the program will panic with an error.  │
    // └──────────────────────────────────────────────────────────────┘
    let device_name = "MCP2221";
    let dev = find_adapter(device_name).expect("I2C adapter not found");
    let i2c = I2cdev::new(dev).expect("Failed to open I2C device");

    // ┌──────────────────────────────────────────────────────────────┐
    // │                   Initialize ADS1115 Sensors                 │
    // │                                                              │
    // │ Setup ADS1115 channels for four sensors with PGA and MUX:    │
    // │                                                              │
    // │  - 10k NTC Thermistor: AIN0, PGA 6.144V                      │
    // │  - pH Sensor:          AIN1, PGA 6.144V                      │
    // │  - LM35DZ Temp:        AIN2, PGA 0.512V                      │
    // │  - EC Sensor:          AIN3, PGA 2.048V                      │
    // └──────────────────────────────────────────────────────────────┘
    let mut lm35_sensor = AdsSensor::new(
        i2c,
        0x48,
        Mux::Ain0Gnd,
        Pga::Gain0_512V,
        "LM35DZ Temp",
        "Celsius",
    )
    .expect("Could not define sensor");

    // ┌──────────────────────────────────────────────────────────────┐
    // │                       LM35DZ Voltage Read                    │
    // │                                                              │
    // │ Attempt to read the analog voltage output from the LM35DZ    │
    // │ temperature sensor. If reading fails, the program will panic │
    // │ with an error message.                                       │
    // └──────────────────────────────────────────────────────────────┘
    let voltage: f32 = lm35_sensor
        .get_voltage()
        .expect("Failed to get LM35DZ voltage");

    log::info!("LM35DZ voltage: {}", voltage);
}
