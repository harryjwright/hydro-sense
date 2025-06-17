use std::{fs, io::Read};

/// Finds the Linux I2C adapter device path by matching a substring in its
/// "name" file.
///
/// This function scans the directory `/sys/class/i2c-adapter`, which
/// contains subdirectories named like `i2c-0`, `i2c-1`, etc., each
/// representing an I2C bus adapter on the system. For each adapter
/// directory:
/// 1. Reads the `name` file, which contains a descriptive string
///    identifying the adapter.
/// 2. Checks if the adapter's name contains the provided `device_name`
///    substring.
/// 3. If a match is found, returns the corresponding device file path
///    `/dev/i2c-x`, where `x` matches the adapter number from the
///    directory name.
///
/// # Arguments
///
/// * `device_name` - A substring to match against the adapter's descriptive
///                   name.
///
/// # Returns
///
/// * `Ok(String)` with the device path string like `/dev/i2c-1` if found.
/// * `Err(std::io::Error)` if any IO error occurs or if no matching adapter
///   is found.
///
pub fn find_adapter(device_name: &str) -> std::io::Result<String> {
    let adapters = fs::read_dir("/sys/class/i2c-adapter")?;

    for entry in adapters {
        let entry = entry?;
        let path = entry.path();
        let name_path = path.join("name");

        let mut file = fs::File::open(&name_path)?;
        let mut name = String::new();
        file.read_to_string(&mut name)?;

        if name.trim().contains(device_name) {
            let devname = path.file_name().unwrap().to_string_lossy();
            return Ok(format!("/dev/{}", devname));
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("I2C adapter '{}' not found", device_name),
    ))
}
