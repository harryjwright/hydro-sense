use embedded_hal::i2c::I2c;
use std::{thread, time::Duration};

/// I2C addresses
pub const ADS1115_ADDR_A: u8 = 0x48;
pub const ADS1115_ADDR_B: u8 = 0x49;

/// MUX input selection bits (bits 14-12 shifted to bits 6-4 in MSB)
#[derive(Clone, Copy)]
pub enum Mux {
    Ain0Gnd = 0b100 << 4,
    Ain1Gnd = 0b101 << 4,
    Ain2Gnd = 0b110 << 4,
    Ain3Gnd = 0b111 << 4,
}

/// PGA gain bits (bits 11-9 shifted to bits 3-1 in MSB)
#[derive(Clone, Copy)]
pub enum Pga {
    Gain6_144V = 0b000 << 1,
    Gain4_096V = 0b001 << 1,
    Gain2_048V = 0b010 << 1,
    Gain1_024V = 0b011 << 1,
    Gain0_512V = 0b100 << 1,
    Gain0_256V = 0b101 << 1,
}

/// Operating mode bit (bit 8 in MSB)
#[derive(Clone, Copy)]
pub enum Mode {
    Continuous = 0b0,
    SingleShot = 0b1,
}

/// Data rate bits (bits 7-5 in LSB)
#[derive(Clone, Copy)]
pub enum DataRate {
    Sps8 = 0b000 << 5,
    Sps16 = 0b001 << 5,
    Sps32 = 0b010 << 5,
    Sps64 = 0b011 << 5,
    Sps128 = 0b100 << 5, // fixed default
    Sps250 = 0b101 << 5,
    Sps475 = 0b110 << 5,
    Sps860 = 0b111 << 5,
}

/// Disable comparator (bits 1-0 = 11)
pub const COMP_QUE_DISABLE: u8 = 0b11;

/// ADS1115 registers
pub const CONFIG_REG: u8 = 0x01;
pub const CONVERSION_REG: u8 = 0x00;

/// Converts PGA enum to corresponding full-scale voltage range in volts
pub fn pga_to_voltage(pga: Pga) -> f32 {
    match pga {
        Pga::Gain6_144V => 6.144,
        Pga::Gain4_096V => 4.096,
        Pga::Gain2_048V => 2.048,
        Pga::Gain1_024V => 1.024,
        Pga::Gain0_512V => 0.512,
        Pga::Gain0_256V => 0.256,
    }
}

/// Converts raw ADC value to voltage using the given gain voltage range
pub fn adc_to_voltage(raw: i16, gain_volts: f32) -> f32 {
    (raw as f32) * gain_volts / 32768.0
}

/// ADS1115 driver using embedded-hal I2C
pub struct AdsSensor<I2C> {
    i2c: I2C,
    addr: u8,
    mux: Mux,
    pga: Pga,
    mode: Mode,
    dr: DataRate,
    pub name: &'static str,  // sensor friendly name
    pub units: &'static str, // units of measurement, e.g. "Celsius"
}

impl<I2C, E> AdsSensor<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Create new AdsSensor instance with fixed data rate 128 SPS
    /// Create new AdsSensor instance with fixed data rate 128 SPS,
    /// including sensor name and units
    pub fn new(
        i2c: I2C,
        addr: u8,
        mux: Mux,
        pga: Pga,
        name: &'static str,
        units: &'static str,
    ) -> Result<Self, E> {
        Ok(Self {
            i2c,
            addr,
            mux,
            pga,
            mode: Mode::SingleShot,
            dr: DataRate::Sps128,
            name,
            units,
        })
    }

    /// Build configuration bytes to write to ADS1115 config register
    fn build_config_bytes(&self) -> [u8; 3] {
        const OS_SINGLE_CONVERSION: u8 = 0b1000_0000; // bit 15 (MSB bit 7)
        let msb = OS_SINGLE_CONVERSION | (self.mux as u8) | (self.pga as u8) | (self.mode as u8);
        let lsb = (self.dr as u8) | COMP_QUE_DISABLE;
        [CONFIG_REG, msb, lsb]
    }

    /// Perform a single-shot conversion and return voltage reading in volts
    pub fn get_voltage(&mut self) -> Result<f32, E> {
        let config = self.build_config_bytes();
        self.i2c.write(self.addr, &config)?;

        // Wait ~8ms for conversion at 128 SPS
        thread::sleep(Duration::from_millis(10));

        // Set pointer to conversion register before read
        self.i2c.write(self.addr, &[CONVERSION_REG])?;

        // Read 2 bytes conversion result
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.addr, &[CONVERSION_REG], &mut buf)?;

        let raw = i16::from_be_bytes(buf);
        Ok(adc_to_voltage(raw, pga_to_voltage(self.pga)))
    }

    /// Release underlying I2C interface
    pub fn release(self) -> I2C {
        self.i2c
    }
}
