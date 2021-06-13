
/// Enables additional sampling of the sensor data to reduce the noise 
/// effect (or to increase resolution)
pub enum ConvAvg {
    /// 1x - 13.33Kbps (3-axes) or 40Kpbs (1 axis)
    Avg1x = 0x00,

    /// 2x - 6.65Kbps (3-axes) or 20Kpbs (1 axis)
    Avg2x = 0x01,

    /// 4x - 3.33Kbps (3-axes) or 10Kpbs (1 axis)
    Avg4x = 0x02,

    /// 8x - 1.66Kbps (3-axes) or 5Kpbs (1 axis)
    Avg8x = 0x03,

    /// 16x - 0.833Kbps (3-axes) or 2.5Kpbs (1 axis)
    Avg16x = 0x04,

    /// 32x - 0.417Kbps (3-axes) or 1.25Kpbs (1 axis)
    Avg32x = 0x05,
}

/// Temperature Coefficient of Sense Magnet
pub enum MagTempco {
    /// 0% (Current sensor applications)
    CurrentSensor = 0x00,

    /// 0.12%/°C (NdBFe)
    NdBFe = 0x01,

    /// 0.2%/°C (Ceramic)
    Ceramic = 0x03,
}


/// Selects Operating Mode
pub enum OperatingMode {
    /// Configuration mode, DEFAULT (TRIGGER_MODE Active)
    Configuration = 0x00,

    /// Stand-by mode (TRIGGER_MODE Active)
    Standby = 0x01,

    /// Active Measure mode (Continuous conversion)
    Active = 0x02,

    /// Active Trigger Mode (TRIGGER_MODE Active)
    ActiveTrigger = 0x03,

    /// Wake-up and Sleep mode (duty-cycled mode)
    WakeupAndSleep = 0x04,

    /// Sleep mode
    Sleep = 0x05,

    /// Deep sleep mode (wakes up at CS signal from Master)
    DeepSleep = 0x06,
}

/// Temperature Conversion Rate. It is linked to the CONV_AVG field
pub enum TRate {
    /// Same as other sensors per CONV_AVG, DEFAULT
    SameRate = 0x00,

    /// Once per conversion set
    OncePerConvSet = 0x01,
}

/// Configure Device Operation Modes - DEVICE_CONFIG
pub struct DeviceConfig {
    config: u16,
}

impl DeviceConfig {
    /// Creates default config
    pub fn new() -> Self {
        let config = 0x00;
        let conf = DeviceConfig { config };

        conf
    }

    /// Creates config from u16 value
    pub fn form_u16(config: u16) -> Self {
        let conf = DeviceConfig { config };

        conf
    }

    /// Convert config to u16 value
    pub fn to_u16(&self) -> u16 {
        self.config
    }

    /// Set ConvAvg field
    pub fn set_conv_avg(mut self, conv_avg : ConvAvg) -> Self {
        self.config = self.config & !(0b111 << 12) | ((conv_avg as u16) << 12);
        self
    }

    /// Set MagTempco field
    pub fn set_mag_tempco(mut self, mag_tempco : MagTempco) -> Self {
        self.config = self.config & !(0b11 << 8) | ((mag_tempco as u16) << 8);
        self
    }

    /// Set OperatingMode field
    pub fn set_operating_mode(mut self, operating_mode : OperatingMode) -> Self {
        self.config = self.config & !(0b111 << 4) | ((operating_mode as u16) << 4);
        self
    }

    /// Enables data acquisition of the temperature channel
    pub fn set_t_en(mut self, t_en : bool) -> Self {
        self.config = self.config & !(0b1 << 3) | ((t_en as u16) << 3);
        self
    }

    /// Temperature Conversion Rate. It is linked to the CONV_AVG field
    pub fn set_t_rate(mut self, t_rate : TRate) -> Self {
        self.config = self.config & !(0b1 << 2) | ((t_rate as u16) << 2);
        self
    }

    /// Enables temperature limit check
    pub fn set_t_limit_check_en(mut self, t_limit_check_en : bool) -> Self {
        self.config = self.config & !(0b1 << 1) | ((t_limit_check_en as u16) << 1);
        self
    }

    /// Enables device on-chip temp sensor to improve linearization of magnetic sensor output
    pub fn set_t_comp_en(mut self, comp_en : bool) -> Self {
        self.config = self.config & !(0b1 << 0) | ((comp_en as u16) << 0);
        self
    }
}