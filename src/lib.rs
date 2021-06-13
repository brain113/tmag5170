//! A platform agnostic driver to interface with the TMAG5170-Q1 (3D hall sensor)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/0.2


#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use embedded_hal::blocking::spi::{Transfer};
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::{Mode};

use crc_all::Crc;

/// Implements DEVICE_CONFIG Register
pub mod device_config;

/// Implements SENSOR_CONFIG Register
pub mod sensor_config;

/// Implements SYSTEM_CONFIG Register
pub mod system_config;

/// Implements ALERT_CONFIG Register
pub mod alert_config;

/// SPI mode
pub const MODE: Mode = embedded_hal::spi::MODE_0;

/// TMAG5170-Q1 driver
pub struct Tmag5170<SPI, CS> {
    spi: SPI,
    cs: CS,
    crc : Crc<u8>, // TODO: to move crc table outside of tmag instance
}

/// TMAG5170-Q1 error type
pub enum ExtError<E> {
    /// Generic SPI error
    E(E),

    /// TMAG5170-Q1 CRC Error
    CrcError,
}

impl<SPI, CS, E> Tmag5170<SPI, CS>
where
    SPI: Transfer<u8, Error = E>,
    CS: OutputPin,
{
    /// Creates a new driver from a SPI peripheral and a NCS pin
    pub fn new(spi: SPI, cs: CS) -> Self {
        let crc = Crc::<u8>::new(0x03, 4, 0x0f, 0x00, false);
        let tmag5170 = Tmag5170 { spi, cs, crc };

        tmag5170
    }

    fn write_register(&mut self, reg: Register, value: u16, cmd: u8) -> Result<(), ExtError<E>> {
        self.crc.init();
        
        let mut buffer: [u8; 4] = [0; 4];
        let value_bytes = value.to_be_bytes();

        buffer[0] = reg.addr();
        buffer[1] = value_bytes[0];
        buffer[2] = value_bytes[1];
        buffer[3] = cmd << 4;

        self.crc.update(&buffer);
        let crc4 = self.crc.finish();
        buffer[3] |= crc4;

        let _ = self.cs.set_low();
        let res = self.spi.transfer(&mut buffer);
        let _ = self.cs.set_high();
        match res {
            Ok(_buf) => (),
            Err(e) => return Err(ExtError::E(e)),
        };

        let packet_crc4 = buffer[3] & 0x0f; // save crc
        buffer[3] &= 0xf0; // erase crc in buffer
        self.crc.init();
        self.crc.update(&buffer);
        let calc_crc4 = self.crc.finish();

        if packet_crc4 == calc_crc4 {
            Ok(())
        } else {
            Err(ExtError::CrcError)
        }
    }

    fn read_register(&mut self, reg: Register, cmd: u8) -> Result<u16, ExtError<E>> {
        self.crc.init();
        
        let mut buffer: [u8; 4] = [0; 4];

        buffer[0] = reg.addr() | 0x80; // Set MSB to indicate read operation
        buffer[1] = 0;
        buffer[2] = 0;
        buffer[3] = cmd << 4;

        self.crc.update(&buffer);
        let crc4 = self.crc.finish();
        buffer[3] |= crc4;

        let _ = self.cs.set_low();
        let res = self.spi.transfer(&mut buffer);
        let _ = self.cs.set_high();
        match res {
            Ok(_buf) => (),
            Err(e) => return Err(ExtError::E(e)),
        };

        let packet_crc4 = buffer[3] & 0x0f; // save crc
        buffer[3] &= 0xf0; // erase crc in buffer
        self.crc.init();
        self.crc.update(&buffer);
        let calc_crc4 = self.crc.finish();

        if packet_crc4 == calc_crc4 {
            let value= ((buffer[1] as u16) << 8) + buffer[2] as u16;
            Ok(value)
        } else {
            Err(ExtError::CrcError)
        }
    }

    fn read_special(&mut self, cmd: u8) -> Result<(u16, u16), ExtError<E>> {
        self.crc.init();
        
        let mut buffer: [u8; 4] = [0; 4];

        buffer[0] = 0x80; // Set MSB to indicate read operation
        buffer[1] = 0;
        buffer[2] = 0;
        buffer[3] = cmd << 4;

        self.crc.update(&buffer);
        let crc4 = self.crc.finish();
        buffer[3] |= crc4;

        let _ = self.cs.set_low();
        let res = self.spi.transfer(&mut buffer);
        let _ = self.cs.set_high();
        match res {
            Ok(_buf) => (),
            Err(e) => return Err(ExtError::E(e)),
        };

        let packet_crc4 = buffer[3] & 0x0f; // save crc
        buffer[3] &= 0xf0; // erase crc in buffer
        self.crc.init();
        self.crc.update(&buffer);
        let calc_crc4 = self.crc.finish();

        if packet_crc4 == calc_crc4 {
            let ch1= ((buffer[1] as u16) << 4) + (buffer[2] & 0x0f) as u16;
            let ch2= ((buffer[0] as u16) << 4) + (buffer[2] >> 4) as u16;
            Ok((ch1, ch2))
        } else {
            Err(ExtError::CrcError)
        }
    }

    /// Reads TMAG5170-Q1 magnetic registers in raw format. Returns (x,y,z)
    pub fn read_mag_registers(&mut self) -> Result<(i16, i16, i16), ExtError<E>> {
        let x = self.read_register(Register::X_CH_RESULT, 0x00)?;
        let y = self.read_register(Register::Y_CH_RESULT, 0x00)?;
        let z = self.read_register(Register::Z_CH_RESULT, 0x00)?;

        Ok((x as i16, y as i16, z as i16))
    }

    /// Reads TMAG5170-Q1 angle and magnitude registers in raw format. Returns (angle,magnitude)
    pub fn read_angle_registers(&mut self) -> Result<(i16, i16), ExtError<E>> {
        let a = self.read_register(Register::ANGLE_RESULT, 0x00)?;
        let m = self.read_register(Register::MAGNITUDE_RESULT, 0x00)?;

        Ok((a as i16, m as i16))
    }

    /// Reads TMAG5170-Q1 temp registers in raw format.
    pub fn read_temp_register(&mut self) -> Result<i16, ExtError<E>> {
        let t = self.read_register(Register::TEMP_RESULT, 0x00)?;

        Ok(t as i16)
    }

    /// Reads TMAG5170-Q1 angle and magnitude registers in raw format as special read. Returns (angle,magnitude)
    pub fn read_am(&mut self) -> Result<(u16, u16), ExtError<E>> {
        let (a, m) = self.read_special(0x00)?;

        Ok((a, m))
    } 

    /// Reads TMAG5170-Q1 CONV_STATUS register in raw format.
    pub fn read_conv_status_register(&mut self) -> Result<i16, ExtError<E>> {
        let s = self.read_register(Register::CONV_STATUS, 0x00)?;

        Ok(s as i16)
    }

   /// Reads TMAG5170-Q1 AFE_STATUS register in raw format.
   pub fn read_afe_status_register(&mut self) -> Result<i16, ExtError<E>> {
        let s = self.read_register(Register::AFE_STATUS, 0x00)?;

        Ok(s as i16)
    }

   /// Reads TMAG5170-Q1 SYS_STATUS register in raw format.
   pub fn read_sys_status_register(&mut self) -> Result<i16, ExtError<E>> {
        let s = self.read_register(Register::SYS_STATUS, 0x00)?;

        Ok(s as i16)
    }

    /// Reads TMAG5170-Q1 TEST_CONFIG register in raw format.
    pub fn read_test_config_register(&mut self) -> Result<i16, ExtError<E>> {
        let tc = self.read_register(Register::TEST_CONFIG, 0x00)?;

        Ok(tc as i16)
    }



    /// Apply  TMAG5170-Q1 AlertConfig 
    pub fn conv_start(&mut self) -> Result<(), ExtError<E>> {
        self.read_register(Register::DEVICE_CONFIG, 0x01)?;

        Ok(())
    }

    /// Apply  TMAG5170-Q1 DeviceConfig
    pub fn apply_device_config(&mut self, config : device_config::DeviceConfig) -> Result<(), ExtError<E>> {
        self.write_register(Register::DEVICE_CONFIG, config.to_u16(), 0x00)?;

        Ok(())
    }
    /// Apply  TMAG5170-Q1 SensorConfig
    pub fn apply_sensor_config(&mut self, config : sensor_config::SensorConfig) -> Result<(), ExtError<E>> {
        self.write_register(Register::SENSOR_CONFIG, config.to_u16(), 0x00)?;

        Ok(())
    }

    /// Apply  TMAG5170-Q1 SystemConfig 
    pub fn apply_system_config(&mut self, config : system_config::SystemConfig) -> Result<(), ExtError<E>> {
        self.write_register(Register::SYSTEM_CONFIG, config.to_u16(), 0x00)?;

        Ok(())
    }

    /// Apply  TMAG5170-Q1 AlertConfig 
    pub fn apply_alert_config(&mut self, config : alert_config::AlertConfig) -> Result<(), ExtError<E>> {
        self.write_register(Register::ALERT_CONFIG, config.to_u16(), 0x00)?;

        Ok(())
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum Register {
    // TODO: Made registers real
    DEVICE_CONFIG = 0x00,
    SENSOR_CONFIG = 0x01,
    SYSTEM_CONFIG = 0x02,
    ALERT_CONFIG = 0x03,
    X_THRX_CONFIG = 0x04,
    Y_THRX_CONFIG = 0x05,
    Z_THRX_CONFIG = 0x06,
    T_THRX_CONFIG = 0x07,
    CONV_STATUS = 0x08,
    X_CH_RESULT = 0x09,
    Y_CH_RESULT = 0x0A,
    Z_CH_RESULT = 0x0B,
    TEMP_RESULT = 0x0C,
    AFE_STATUS = 0x0D,
    SYS_STATUS = 0x0E,
    TEST_CONFIG = 0x0F,
    OSC_MONITOR = 0x10,
    MAG_GAIN_CONFIG = 0x11,
    ANGLE_RESULT = 0x13,
    MAGNITUDE_RESULT = 0x14,
}

impl Register {
    fn addr(self) -> u8 {
        self as u8
    }
}
