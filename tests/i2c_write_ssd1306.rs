mod common;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::BinaryColor;
use hydro_sense::i2c::find_adapter;
use linux_embedded_hal::I2cdev;
use ssd1306::prelude::*;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::I2CDisplayInterface;
use ssd1306::Ssd1306;
use std::thread;
use std::time::Duration;

#[test]
fn test_ssd1306() -> anyhow::Result<()> {
    common::init_logger();

    // ┌──────────────────────────────────────────────────────────────┐
    // │                    Open I2C Device Adapter                   │
    // │                                                              │
    // │ Attempt to find the I2C adapter named "MCP2221" and open it. │
    // │ If either step fails, the program will panic with an error.  │
    // └──────────────────────────────────────────────────────────────┘
    let device_name = "MCP2221";
    let dev = find_adapter(device_name)
        .expect("I2C adapter not found. Ensure that the device is connected.");
    let i2c = I2cdev::new(dev).expect("Failed to open I2C device");

    // ┌──────────────────────────────────────────────────────────────┐
    // │                  Create Display I²C Interface                │
    // │                                                              │
    // │ Wrap the I²C device in an interface compatible with the      │
    // │ SSD1306 driver. This prepares it for communication with the  │
    // │ OLED display over the I²C bus using embedded-hal traits.     │
    // └──────────────────────────────────────────────────────────────┘
    let interface = I2CDisplayInterface::new(i2c);

    // // Use concrete display size type Display128x64 here
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    thread::sleep(Duration::from_millis(10)); // tiny delay
                                              // --- Init -------------------------------------------
    let init = display.init();

    if let Err(e) = init {
        log::error!("Unexpected display init error: {:#?}", e);
    } else {
        log::info!("Display initialized successfully");
    }
    thread::sleep(Duration::from_millis(10)); // tiny delay
    display.clear(BinaryColor::On).expect("");
    thread::sleep(Duration::from_millis(10)); // tiny delay
    let flush = display.flush();
    log::info!("Flush result: {:?}", flush);

    Ok(())
}
