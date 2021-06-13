/// Latched ALERT Mode Select
pub enum AlertLatch {
    /// ALERT sources are not latched. ALERT is asserted only
    /// while the source of the ALERT response is present
    NotLatched = 0x00,

    /// ALERT sources are latched. ALERT response is latched
    /// when the source of the ALERT is asserted until cleared on
    /// Read of the corresponding status register (AFE_STATUS,
    /// SYS_STATUS, or result registers)
    Latched = 0x01,
}

/// Latched ALERT Mode Select
pub enum AlertMode {
    /// Interrupt Mode
    Interrupt = 0x00,

    /// Comparator Mode. This mode overrides any interrupt
    /// function (ALERT trigger is also disabled), and implements Hall
    /// switch function based off the *_THRX_ALRT settings
    Comparator = 0x01,
}

/// Number of conversions above the HIGH Threshold or below the
/// LOW Threshold before the ALERT Response is initiated
pub enum ThrxCount {
    /// 1-Conversion Result
    ConvResult1 = 0x00,

    /// 2-Conversion Results
    ConvResult2 = 0x01,

    /// 3-Conversion Results
    ConvResult3 = 0x02,

    /// 4-Conversion Results
    ConvResult4 = 0x03,
}

/// Configure Device Operation Modes - ALERT_CONFIG
pub struct AlertConfig {
    config: u16,
}

impl AlertConfig {
    /// Creates default config
    pub fn new() -> Self {
        let config = 0x00;
        let conf = AlertConfig { config };

        conf
    }

    /// Creates config from u16 value
    pub fn form_u16(config: u16) -> Self {
        let conf = AlertConfig { config };

        conf
    }

    /// Convert config to u16 value
    pub fn to_u16(&self) -> u16 {
        self.config
    }

    /// Set AlertLatch field
    pub fn set_alert_latch(mut self, alert_latch: AlertLatch) -> Self {
        self.config = self.config & !(0b1 << 13) | ((alert_latch as u16) << 13);
        self
    }

    /// Set AlertMode field
    pub fn set_alert_mode(mut self, alert_mode: AlertMode) -> Self {
        self.config = self.config & !(0b1 << 12) | ((alert_mode as u16) << 12);
        self
    }

    /// Set AlertMode field
    pub fn set_status_alrt_enable(mut self, status_alrt_enable: bool) -> Self {
        self.config = self.config & !(0b1 << 11) | ((status_alrt_enable as u16) << 11);
        self
    }

    /// Set STATUS_ALRT field
    pub fn set_rslt_alrt_enable(mut self, rslt_alrt_enable: bool) -> Self {
        self.config = self.config & !(0b1 << 8) | ((rslt_alrt_enable as u16) << 8);
        self
    }

    /// Set ThrxCount field
    pub fn set_thrx_count(mut self, thrx_count: ThrxCount) -> Self {
        self.config = self.config & !(0b11 << 4) | ((thrx_count as u16) << 4);
        self
    }

    /// Set T_THRX_ALRT field
    pub fn set_t_thrx_alrt_enable(mut self, t_thrx_alrt_enable: bool) -> Self {
        self.config = self.config & !(0b1 << 3) | ((t_thrx_alrt_enable as u16) << 3);
        self
    }

    /// Set Z_THRX_ALRT field
    pub fn set_z_thrx_alrt_enable(mut self, z_thrx_alrt_enable: bool) -> Self {
        self.config = self.config & !(0b1 << 2) | ((z_thrx_alrt_enable as u16) << 2);
        self
    }

    /// Set Y_THRX_ALRT field
    pub fn set_y_thrx_alrt_enable(mut self, y_thrx_alrt_enable: bool) -> Self {
        self.config = self.config & !(0b1 << 1) | ((y_thrx_alrt_enable as u16) << 1);
        self
    }

    /// Set X_THRX_ALRT field
    pub fn set_x_thrx_alrt_enable(mut self, x_thrx_alrt_enable: bool) -> Self {
        self.config = self.config & !(0b1 << 0) | ((x_thrx_alrt_enable as u16) << 0);
        self
    }
}
