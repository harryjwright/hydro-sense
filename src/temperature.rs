/*
 * Always keep my coding and comment style.
 */
use std::f32;

/// Convert ADC voltage reading from 10k NTC thermistor voltage divider (thermistor on high side).
pub fn voltage_to_temperature(
    supply_voltage: f32,   // e.g. 5.0 volts
    pga_voltage: f32,      // e.g. 6.144 volts (ADS1115 PGA full scale)
    measured_voltage: f32, // voltage measured at ADC (volts)
) -> f32 {
    // Constants for thermistor and fixed resistor
    let r_fixed = 10000.0_f32; // Fixed resistor in ohms (10k)
    let r0 = 10000.0_f32; // Thermistor resistance at T0 (10k)
    let b = 3950.0_f32; // Beta coefficient
    let t0_kelvin = 25.0_f32 + 273.15_f32; // Reference temp in Kelvin (25°C)

    // Validate input voltages
    if measured_voltage <= 0.0
        || measured_voltage >= supply_voltage
        || measured_voltage > pga_voltage
    {
        return f32::NAN; // invalid input signals
    }

    // Calculate thermistor resistance from voltage divider formula:
    // Vout = Vsupply * (R_thermistor) / (R_fixed + R_thermistor)
    // => R_thermistor = R_fixed * Vout / (Vsupply - Vout)
    let r_thermistor = r_fixed * measured_voltage / (supply_voltage - measured_voltage);
    // let r_thermistor = r_fixed * (supply_voltage / measured_voltage - 1.0);

    if r_thermistor <= 0.0 {
        return f32::NAN; // invalid resistance
    }

    // Calculate temperature using Beta parameter equation:
    // 1/T = 1/T0 + 1/B * ln(R/R0)
    let ln_ratio = (r_thermistor / r0).ln();
    let inv_t = (1.0 / t0_kelvin) + (1.0 / b) * ln_ratio;

    let temperature_kelvin = 1.0 / inv_t;
    let temperature_celsius = temperature_kelvin - 273.15_f32;

    temperature_celsius
}
