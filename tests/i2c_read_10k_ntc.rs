/*
 * Always keep my coding and comment style.
 */

mod common;

use hydro_sense::ads1115::{AdsSensor, Mux, Pga};
use hydro_sense::i2c::find_adapter;
use hydro_sense::temperature::voltage_to_temperature;
use linux_embedded_hal::I2cdev;

#[test]
fn test_read_ntc_voltage() {
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
    // │ Setup ADS1115 channel for 10k NTC thermistor with PGA 6.144V │
    // │ connected to AIN0 (single-ended).                            │
    // └──────────────────────────────────────────────────────────────┘
    let mut ntc_sensor = AdsSensor::new(
        i2c,
        0x48,
        Mux::Ain0Gnd,
        Pga::Gain6_144V,
        "10k NTC Thermistor",
        "Volts",
    )
    .expect("Could not define sensor");

    // ┌──────────────────────────────────────────────────────────────┐
    // │                    Read NTC Thermistor Voltage               │
    // │                                                              │
    // │ Attempt to read the analog voltage output from the 10k NTC   │
    // │ thermistor. If reading fails, the program will panic with    │
    // │ an error message.                                            │
    // └──────────────────────────────────────────────────────────────┘
    let voltage: f32 = ntc_sensor
        .get_voltage()
        .expect("Failed to get 10k NTC thermistor voltage");
    log::info!("10k NTC thermistor voltage: {}", voltage);

    // ┌──────────────────────────────────────────────────────────────┐
    // │                Convert Voltage to Temperature                │
    // │                                                              │
    // │ Using the Beta parameter equation for NTC thermistors:       │
    // │                                                              │
    // │ T = 1 / (1/T₀ + (1/β) * ln(R / R₀))                          │
    // │                                                              │
    // │ Where:                                                       │
    // │  - T is temperature in Kelvin                                │
    // │  - T₀ is reference temperature (usually 25°C = 298.15 K)     │
    // │  - β (Beta) is thermistor constant (typical ~3950)           │
    // │  - R is resistance at temperature T                          │
    // │  - R₀ is resistance at T₀ (usually 10kΩ at 25°C)             │
    // │                                                              │
    // │ The voltage reading is converted to resistance first, then   │
    // │ temperature in Celsius.                                      │
    // └──────────────────────────────────────────────────────────────┘
    let temperature = voltage_to_temperature(5.0, 6.144, voltage);
    log::info!("NTC thermistor temperature: {}", temperature)
}
