
/// Enable Angle calculation using two axis data
pub enum AngleEn {
    /// No angle calculation (default)
    NoAngle = 0x00,

    /// X-Y-angle calculation enabled
    XY = 0x01,

    /// Y-Z-angle calculation enabled
    YZ = 0x02,

    /// Z-X-angle calculation enabled
    ZX = 0x03,
}

/// Selects the time spent in low power mode between conversions
/// when OPERATING_MODE =010b (OperatingMode::Active)
pub enum SleepTime {
    /// 1ms 
    Ms1 = 0x00,

    /// 5ms 
    Ms5 = 0x01,

    /// 10ms 
    Ms10 = 0x02,

    /// 15ms 
    Ms15 = 0x03,

    /// 20ms 
    Ms20 = 0x04,

    /// 30ms 
    Ms30 = 0x05,

    /// 50ms 
    Ms50 = 0x06,

    /// 100ms 
    Ms100 = 0x07,

    /// 500ms 
    Ms500 = 0x08,

    /// 1000ms 
    Ms1000 = 0x09,
}

/// Enables data acquisition of the magnetic axis channel(s)
pub enum MagChEn {
    /// All magnetic channels of OFF 
    Off = 0x00,
    
    /// X channel enabled
    X = 0x01,
    
    /// Y channel enabled
    Y = 0x02,
    
    /// XY channel enabled
    XY = 0x03,
    
    /// Z channel enabled
    Z = 0x04,
    
    /// ZX channel enabled
    ZX = 0x05,
    
    /// YZ channel enabled
    YZ = 0x06,
    
    /// XYZ channel enabled
    XYZ = 0x07,
    
    /// XYX channel enabled
    XYX = 0x08,
    
    /// YXY channel enabled
    YXY = 0x09,
    
    /// YZY channel enabled
    YZY = 0x0a,
    
    /// ZYZ channel enabled
    ZYZ = 0x0b,
    
    /// ZXZ channel enabled
    ZXZ = 0x0c,
    
    /// XZX channel enabled
    XZX = 0x0d,
    
    /// XYZYX channel enabled
    XYZYX = 0x0e,
    
    /// XYZZYX channel enabled
    XYZZYX = 0x0f,
}

/// Enables different magnetic ranges
#[allow(non_camel_case_types)]
pub enum Range {
    /// ±50mT (TMAG5170A1) / ±200mT(TMAG5170A2)
    A1_50mT_A2_200mT = 0x00,

    /// ±25mT (TMAG5170A1) / ±133mT(TMAG5170A2)
    A1_25mT_A2_133mT = 0x01,

    /// ±100mT (TMAG5170A1) / ±300mT(TMAG5170A2)
    A1_100mT_A2_300mT = 0x02,
}

/// Configure Device Operation Modes - SENSOR_CONFIG
pub struct SensorConfig {
    config: u16,
}

impl SensorConfig {
    /// Creates default config
    pub fn new() -> Self {
        let config = 0x00;
        let conf = SensorConfig { config };

        conf
    }

    /// Creates config from u16 value
    pub fn form_u16(config: u16) -> Self {
        let conf = SensorConfig { config };

        conf
    }

    /// Convert config to u16 value
    pub fn to_u16(&self) -> u16 {
        self.config
    }

    /// Set AngleEn field
    pub fn set_angle_en(mut self, angle_en : AngleEn) -> Self {
        self.config = self.config & !(0b11 << 14) | ((angle_en as u16) << 14);
        self
    }
    
    /// Set SleepTime field
    pub fn set_sleep_time(mut self, sleep_time : SleepTime) -> Self {
        self.config = self.config & !(0b1111 << 10) | ((sleep_time as u16) << 10);
        self
    }   

    /// Set AngleEn field
    pub fn set_mag_ch_en(mut self, mag_ch_en : MagChEn) -> Self {
        self.config = self.config & !(0b1111 << 6) | ((mag_ch_en as u16) << 6);
        self
    }

    /// Set Z Range field
    pub fn set_z_range(mut self, z_range : Range) -> Self {
        self.config = self.config & !(0b11 << 4) | ((z_range as u16) << 4);
        self
    }

    /// Set Y Range field
    pub fn set_y_range(mut self, y_range : Range) -> Self {
        self.config = self.config & !(0b11 << 2) | ((y_range as u16) << 2);
        self
    }

    /// Set X Range field
    pub fn set_x_range(mut self, x_range : Range) -> Self {
        self.config = self.config & !(0b11 << 0) | ((x_range as u16) << 0);
        self
    }
}
