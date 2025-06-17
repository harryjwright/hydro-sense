use core::convert::Infallible;
use embedded_hal::i2c::I2c;

/// Default I2C address for the RGB button.
pub const RGBBUTTON_DEFAULT_I2C_ADDR: u8 = 0x2A;
pub const RGBBUTTON_PART_ID: u16 = 0x43DF;

/// Register addresses
const RGBBUTTON_I2C_ADDR_REG: u8 = 0x00;
const RGBBUTTON_RED_REG: u8 = 0x01;
const RGBBUTTON_GREEN_REG: u8 = 0x02;
const RGBBUTTON_BLUE_REG: u8 = 0x03;
const RGBBUTTON_BUTTON_SIGNAL_REG: u8 = 0x04;
const RGBBUTTON_PID_MSB_REG: u8 = 0x09;
const RGBBUTTON_PID_LSB_REG: u8 = 0x0A;

/// Predefined RGB colors
#[derive(Copy, Clone)]
pub enum GeneralRGBColor {
    Red = 0xFF0000,
    Orange = 0xFF7F00,
    Yellow = 0xFFFF00,
    Green = 0x00FF00,
    Cyan = 0x00FFFF,
    Blue = 0x0000FF,
    Purple = 0x8B00FF,
    White = 0xFFFFFF,
    Black = 0x000000,
}

/// Struct for the RGB button driver
pub struct DFRobotRGBButton<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C, E> DFRobotRGBButton<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C, addr: u8) -> Result<Self, E> {
        Ok(Self { i2c, addr })
    }

    pub fn into_inner(self) -> I2C {
        self.i2c
    }

    pub fn begin(&mut self) -> Result<bool, E> {
        let pid = self.read_u16(RGBBUTTON_PID_MSB_REG)?;
        Ok(pid == RGBBUTTON_PART_ID)
    }

    pub fn set_rgb_color_enum(&mut self, color: GeneralRGBColor) -> Result<(), E> {
        let rgb = color as u32;
        self.set_rgb_color(
            ((rgb >> 16) & 0xFF) as u8,
            ((rgb >> 8) & 0xFF) as u8,
            (rgb & 0xFF) as u8,
        )
    }

    pub fn set_rgb_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), E> {
        self.write_bytes(RGBBUTTON_RED_REG, &[r, g, b])
    }

    pub fn get_button_status(&mut self) -> Result<bool, E> {
        let val = self.read_u8(RGBBUTTON_BUTTON_SIGNAL_REG)?;
        Ok(val != 0)
    }

    pub fn get_i2c_addr(&mut self) -> Result<u8, E> {
        self.read_u8(RGBBUTTON_I2C_ADDR_REG)
    }

    pub fn get_pid(&mut self) -> Result<u16, E> {
        self.read_u16(RGBBUTTON_PID_MSB_REG)
    }

    fn write_bytes(&mut self, reg: u8, data: &[u8]) -> Result<(), E> {
        let mut buf = [0u8; 4];
        buf[0] = reg;
        buf[1..(1 + data.len())].copy_from_slice(data);
        self.i2c.write(self.addr, &buf[..(1 + data.len())])
    }

    fn read_u8(&mut self, reg: u8) -> Result<u8, E> {
        let mut buf = [0u8];
        self.i2c.write_read(self.addr, &[reg], &mut buf)?;
        Ok(buf[0])
    }

    fn read_u16(&mut self, reg: u8) -> Result<u16, E> {
        let mut buf = [0u8; 2];
        self.i2c.write_read(self.addr, &[reg], &mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
}
