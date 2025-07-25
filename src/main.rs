#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use embedded_hal::i2c::I2c;
use hydro_sense::df0991::*;
use std::time::{Duration, Instant};

// ┌──────────────────────────────────────────────────────────────┐
// │                      Initialize App State                    │
// │                                                              │
// │ Create the main application state struct, starting with      │
// │ button_pressed set to false. This struct tracks user input   │
// │ and signals whether the display needs to be redrawn.         │
// └──────────────────────────────────────────────────────────────┘
struct AppState {
    btn_press: bool,
    state_changed: bool,
    last_change: Instant,
}

const DEBOUNCE_TIME: Duration = Duration::from_millis(50);

// ┌──────────────────────────────────────────────────────────────┐
// │                     Check Button Press                       │
// │                                                              │
// │ Poll the hardware button state. If the value has changed     │
// │ since last check, update the app state and mark display as   │
// │ needing redraw.                                              │
// └──────────────────────────────────────────────────────────────┘
fn check_press<I2C, E>(
    state: &mut AppState,
    button: &mut DFRobotRGBButton<I2C>,
) -> core::result::Result<(), E>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    let current_pressed = button.get_button_status()?;

    if current_pressed != state.btn_press {
        // Check how long since last change
        let now = Instant::now();
        if now.duration_since(state.last_change) >= DEBOUNCE_TIME {
            state.btn_press = current_pressed;
            state.state_changed = true;
            state.last_change = now;
        }
        // else ignore change as it is bouncing
    }

    Ok(())
}

// ┌──────────────────────────────────────────────────────────────┐
// │                     Update Display Logic                     │
// │                                                              │
// │ Redraw the display based on the current app state.           │
// │ This function assumes a redraw is required and does not      │
// │ perform change detection itself.                             │
// └──────────────────────────────────────────────────────────────┘
fn update_display(state: &AppState) -> anyhow::Result<()> {
    if state.btn_press {
        println!("Button is pressed - updating display.");
    } else {
        println!("Button is not pressed - updating display.");
    }
    // Your OLED drawing code here
    Ok(())
}

// ┌──────────────────────────────────────────────────────────────────┐
// │                           Main Loop                              │
// │                                                                  │
// │ Continuously run the event loop:                                 │
// │                                                                  │
// │ 1. Check button press and update app state accordingly.          │
// │ 2. If the state has changed, redraw the display.                 │
// │ 3. Reset the state_changed flag to prevent unnecessary redraws. │
// │ 4. Sleep briefly to prevent excessive CPU usage.                 │
// └──────────────────────────────────────────────────────────────────┘
fn main() -> anyhow::Result<()> {
    // Set default RUST_LOG to info if not set by user
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // ┌──────────────────────────────────────────────────────────────┐
    // │                    Open I2C Device Adapter                   │
    // │                                                              │
    // │ Attempt to find the I2C adapter named "MCP2221" and open it. │
    // │ If either step fails, the program will panic with an error.  │
    // └──────────────────────────────────────────────────────────────┘
    let device_name = "MCP2221";
    let dev = hydro_sense::i2c::find_adapter(device_name)?;
    let i2c = linux_embedded_hal::I2cdev::new(dev)?;

    // ┌──────────────────────────────────────────────────────────────┐
    // │                  Initialize RGB Button Device                │
    // │                                                              │
    // │ - Create new instance of DFRobotRGBButton using I2C bus      │
    // │ - Use default I2C address (0x2A unless changed by switch)    │
    // │ - Call `begin()` to verify presence and read part ID         │
    // │ - If detection fails, handle gracefully or exit early        │
    // └──────────────────────────────────────────────────────────────┘
    let mut ph_cal_btn = hydro_sense::df0991::DFRobotRGBButton::new(
        i2c,
        hydro_sense::df0991::RGBBUTTON_DEFAULT_I2C_ADDR,
    )?;

    if !ph_cal_btn.begin()? {
        eprintln!("RGB button not detected.");
        return Ok(());
    }

    // ┌────────────────────────────────────────────────────────────┐
    // │                  Initialize Application State              │
    // │                                                            │
    // │ - Tracks dynamic runtime state like button press           │
    // │ - Includes a flag to indicate if a display redraw is needed│
    // └────────────────────────────────────────────────────────────┘
    let initial_press = ph_cal_btn.get_button_status()?;
    let now = Instant::now();

    let mut app_state = AppState {
        btn_press: initial_press,
        state_changed: false,
        last_change: now,
    };

    // ┌──────────────────────────────────────────────────────────────┐
    // │                           Main Loop                          │
    // │                                                              │
    // │ Continuously run the event loop:                             │
    // │                                                              │
    // │ 1. Check the button press and update the application state.  │
    // │ 2. If the state changed, perform any actions such as         │
    // │    updating the display or logging the event.                │
    // │ 3. Reset the state_changed flag to avoid redundant updates.  │
    // │ 4. Sleep briefly to reduce CPU usage and debounce input.     │
    // └──────────────────────────────────────────────────────────────┘
    loop {
        check_press(&mut app_state, &mut ph_cal_btn)?;

        if app_state.state_changed {
            // update_display(&app_state)?;
            log::info!("Button pressed");
            app_state.state_changed = false;
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}
