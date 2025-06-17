#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::time::{Duration, Instant};

// ┌──────────────────────────────────────────────────────────────┐
// │                      Initialize App State                    │
// │                                                              │
// │ Create the main application state struct, starting with      │
// │ button_pressed set to false. This struct tracks user input   │
// │ and other runtime state necessary for UI updates.            │
// └──────────────────────────────────────────────────────────────┘
struct AppState {
    button_pressed: bool,
    // add more fields as needed
}

// ┌──────────────────────────────────────────────────────────────┐
// │                    Initialize Display Cache                  │
// │                                                              │
// │ Create a DisplayCache instance to track the last displayed   │
// │ state. This cache enables efficient redraws by only          │
// │ updating the screen when the application state changes.      │
// └──────────────────────────────────────────────────────────────┘
struct DisplayCache {
    last_button_pressed: bool,
}

impl DisplayCache {
    fn new() -> Self {
        Self {
            last_button_pressed: false,
        }
    }

    // Returns true if the state differs and display should be updated
    fn needs_redraw(&self, state: &AppState) -> bool {
        self.last_button_pressed != state.button_pressed
    }

    fn update(&mut self, state: &AppState) {
        self.last_button_pressed = state.button_pressed;
    }
}

// ┌──────────────────────────────────────────────────────────────┐
// │                           Main Loop                          │
// │                                                              │
// │ Continuously run the event loop:                             │
// │                                                              │
// │ 1. Check button press and update app state accordingly.      │
// │ 2. If the app state differs from the last displayed state,   │
// │    redraw the display to reflect changes.                    │
// │ 3. Cache the new displayed state for future comparisons.     │
// │ 4. Sleep briefly to prevent excessive CPU usage.             │
// └──────────────────────────────────────────────────────────────┘
fn main() -> anyhow::Result<()> {
    let mut app_state = AppState {
        button_pressed: false,
    };

    let mut display_cache = DisplayCache::new();

    loop {
        check_press(&mut app_state)?;

        if display_cache.needs_redraw(&app_state) {
            update_display(&app_state)?;
            display_cache.update(&app_state);
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}

// Stub: Check button input, update app_state
fn check_press(state: &mut AppState) -> anyhow::Result<()> {
    // Your code to poll buttons and update state.button_pressed
    Ok(())
}

// Stub: Update display based on app_state
fn update_display(state: &AppState) -> anyhow::Result<()> {
    if state.button_pressed {
        println!("Button is pressed - updating display.");
    } else {
        println!("Button is not pressed - updating display.");
    }
    // Your OLED drawing code here
    Ok(())
}
