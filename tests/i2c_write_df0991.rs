mod common;

use hydro_sense::{df0991::*, i2c::find_adapter};
use linux_embedded_hal::I2cdev;
use std::thread;
use std::time::Duration;

#[test]
fn test_btn_flash() -> Result<(), Box<dyn std::error::Error>> {
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
    // │           Attempt to initialize the RGB button driver        │
    // │                                                              │
    // │ - Creates a new instance of the DFRobotRGBButton struct.     │
    // │ - Uses the provided I2C bus and the default I2C address.     │
    // │ - Returns early with error if instantiation fails.           │
    // └──────────────────────────────────────────────────────────────┘
    let mut button = match DFRobotRGBButton::new(i2c, RGBBUTTON_DEFAULT_I2C_ADDR) {
        Ok(device) => device,
        Err(e) => {
            log::error!("Failed to create RGB button instance: {:?}", e);
            return Err(e.into());
        }
    };

    // ┌──────────────────────────────────────────────────────────────┐
    // │                Initialize and Test RGB Button                │
    // │                                                              │
    // │ If the button is detected:                                   │
    // │ - Set initial LED color to blue.                             │
    // │ - Loop 10 times:                                             │
    // │     • Log whether the button is pressed.                     │
    // │     • Cycle LED color through red, green, blue, and white.  │
    // │     • Wait 1 second per iteration.                           │
    // │ - Turn off the LED at the end.                               │
    // │                                                              │
    // │ If detection fails, log a warning.                           │
    // └──────────────────────────────────────────────────────────────┘
    if button.begin()? {
        log::info!("✅ RGB button detected!");
        button.set_rgb_color_enum(GeneralRGBColor::Blue)?;

        for i in 0..10 {
            let pressed = button.get_button_status()?;
            log::info!("[{i}] Button pressed? {}", pressed);

            // Change color to visually confirm loop iteration
            let color = match i % 4 {
                0 => GeneralRGBColor::Red,
                1 => GeneralRGBColor::Green,
                2 => GeneralRGBColor::Blue,
                _ => GeneralRGBColor::White,
            };
            button.set_rgb_color_enum(color)?;

            thread::sleep(Duration::from_secs(1));
        }

        button.set_rgb_color_enum(GeneralRGBColor::Black)?; // Turn off LED
    } else {
        log::warn!("❌ Failed to detect RGB button.");
    }

    Ok(())
}
