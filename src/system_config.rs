
/// Selects a safety diagnostic mode run
pub enum DiagSel {
    /// Run all data path diagnostics all together (default)
    AllData = 0x00,

    /// Run only enabled data path diagnostics all together
    EnabledData = 0x01,

    /// Run all data path diagnostics in sequence
    AllDataInSeq = 0x02,

    /// Run only enabled data path diagnostics in sequence
    EnabledDataInSeq = 0x03,
}

/// Selects a condition which initiates a single conversion
pub enum TriggerMode {
    /// Conversion Start at SPI Command Bits (default)
    SPI = 0x00,

    /// nCS Sync Pulse
    CS = 0x01,

    /// ALERT Sync Pulse
    ALERT = 0x02,
}

/// Data Type to be accessed from results registers via SPI
pub enum DataType {
    /// Default 32-bit Register Access
    Default = 0x00,
    
    /// 12-Bit XY Data Access
    XY = 0x01,
    
    /// 12-Bit XZ Data Access
    XZ = 0x02,
    
    /// 12-Bit ZY Data Access
    ZY = 0x03,
    
    /// 12-Bit XT Data Access
    XT = 0x04,
    
    /// 12-Bit YT Data Access
    YT = 0x05,
    
    /// 12-Bit ZT Data Access
    ZT = 0x06,
    
    /// 12-Bit AM Data Access
    AM = 0x07,
}

/// Configure Device Operation Modes - SYSTEM_CONFIG
pub struct SystemConfig {
    config: u16,
}

impl SystemConfig {
    /// Creates default config
    pub fn new() -> Self {
        let config = 0x00;
        let conf = SystemConfig { config };

        conf
    }

    /// Creates config from u16 value
    pub fn form_u16(config: u16) -> Self {
        let conf = SystemConfig { config };

        conf
    }

    /// Convert config to u16 value
    pub fn to_u16(&self) -> u16 {
        self.config
    }

    /// Set AngleEn field
    pub fn set_diag_sel(mut self, diag_sel : DiagSel) -> Self {
        self.config = self.config & !(0b11 << 12) | ((diag_sel as u16) << 12);
        self
    }
    
    /// Set SleepTime field
    pub fn set_trigger_mode(mut self, trigger_mode : TriggerMode) -> Self {
        self.config = self.config & !(0b11 << 9) | ((trigger_mode as u16) << 9);
        self
    }   

    /// Set AngleEn field
    pub fn set_data_type(mut self, data_type : DataType) -> Self {
        self.config = self.config & !(0b111 << 6) | ((data_type as u16) << 6);
        self
    }

    /// Enables AFE Diagnostic Tests to be executed 
    pub fn set_diag_en(mut self, diag_en : bool) -> Self {
        self.config = self.config & !(0b1 << 5) | ((diag_en as u16) << 5);
        self
    }

    /// Enables magnetic field limit check on Z axis
    pub fn set_t_z_limit_check(mut self, z_limit_check : bool) -> Self {
        self.config = self.config & !(0b1 << 2) | ((z_limit_check as u16) << 2);
        self
    }
    
    /// Enables magnetic field limit check on Y axis
    pub fn set_t_y_limit_check(mut self, y_limit_check : bool) -> Self {
        self.config = self.config & !(0b1 << 1) | ((y_limit_check as u16) << 1);
        self
    }

    /// Enables magnetic field limit check on X axis
    pub fn set_t_x_limit_check(mut self, x_limit_check : bool) -> Self {
        self.config = self.config & !(0b1 << 0) | ((x_limit_check as u16) << 0);
        self
    }

}
