# Hydro-Sense 💦

A simple and reliable system to monitor temperature, pH, and electrical conductivity (EC) using I2C sensors on Linux.

## Overview

Hydro-Sense reads analog sensor data through an ADS1115 ADC and converts it into meaningful values for hydroponics and water quality monitoring. It supports common sensors like the **PH4502C pH sensor**, a **generic EC meter for Arduino**, and the **LM35DZ temperature probe**.

## Features

- Reads temperature (LM35DZ), pH (PH4502C), and EC (generic Arduino EC meter) via ADS1115 ADC
- Configurable gain and sample rate settings
- Uses Linux I2C interface for hardware communication
- Modular Rust codebase for easy extension and customization
- Basic logging for debugging and monitoring

## Why linux-embedded-hal?

We use **linux-embedded-hal** to simplify I2C communication by leveraging Linux’s native I2C device support with an idiomatic Rust API. This keeps the code clean, portable, and easy to maintain.

## Usage

1. Connect your sensors (PH4502C, generic EC meter, LM35DZ) via ADS1115 to your Linux system’s I2C bus.
2. Run the Hydro-Monitor application.
3. View logged sensor readings in the console.

## Dependencies

- **rust** — The Rust programming language and compiler (stable version recommended, e.g. 1.72 or later)
- **linux-embedded-hal = "0.4"** — Provides Linux-specific implementations of embedded-hal traits for I2C, SPI, GPIO, etc. Latest stable version as of mid-2025, fully compatible with embedded-hal 1.0 alpha.
- **embedded-hal = "1.0.0-alpha.8"** — Defines hardware abstraction traits for embedded systems used by linux-embedded-hal and Hydro-Monitor. The 1.0 alpha series is currently the latest trait version to ensure modern API consistency.
- **anyhow = "1.0"** — For flexible and ergonomic error handling.
- **log = "0.4"** and **env_logger = "0.11"** — For structured logging output, helping with debugging and monitoring.
- **byteorder = "1.5"** — To handle endian conversions when reading raw ADC data.

These dependencies are carefully selected and pinned to versions that are stable and widely supported at the time of development (June 2025). The linux-embedded-hal and embedded-hal crates are kept up to date to leverage improvements in embedded hardware abstraction on Linux platforms, while the others provide robust error handling and logging.

## License

Hydro-Sense is released under the MIT License, a permissive open-source license widely used in academic and commercial projects.

This license allows anyone to freely use, modify, distribute, and even incorporate the software into proprietary products, as long as the original copyright notice and license text are included in all copies or substantial portions of the software. The MIT License also provides the software “as is,” without any warranty, protecting both authors and users.

This makes Hydro-Sense an accessible and flexible tool for research, education, and practical applications in hydroponics and environmental monitoring.
