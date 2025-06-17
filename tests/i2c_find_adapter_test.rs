mod common;

use std::io::Result;

// Import the function from your library crate; adjust the path as needed.
use hydro_sensor::i2c::find_adapter;

#[test]
fn test_find_mcp2221_adapter() -> Result<()> {
    common::init_logger();

    let device_name = "MCP2221";

    let result = find_adapter(device_name);

    assert!(
        result.is_ok(),
        "Expected Ok(_) for device '{}', got Err: {:?}",
        device_name,
        result.err()
    );

    // Optional: check the returned string contains "/dev/"
    let path = result.unwrap();
    assert!(
        path.starts_with("/dev/"),
        "Returned device path does not start with /dev/: {}",
        path
    );

    log::info!("{}", path);
    Ok(())
}
