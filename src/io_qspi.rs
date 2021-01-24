#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO status"]
    pub gpio_qspi_sclk_status: crate::Reg<gpio_qspi_sclk_status::GPIO_QSPI_SCLK_STATUS_SPEC>,
    #[doc = "0x04 - GPIO control including function select and overrides."]
    pub gpio_qspi_sclk_ctrl: crate::Reg<gpio_qspi_sclk_ctrl::GPIO_QSPI_SCLK_CTRL_SPEC>,
    #[doc = "0x08 - GPIO status"]
    pub gpio_qspi_ss_status: crate::Reg<gpio_qspi_ss_status::GPIO_QSPI_SS_STATUS_SPEC>,
    #[doc = "0x0c - GPIO control including function select and overrides."]
    pub gpio_qspi_ss_ctrl: crate::Reg<gpio_qspi_ss_ctrl::GPIO_QSPI_SS_CTRL_SPEC>,
    #[doc = "0x10 - GPIO status"]
    pub gpio_qspi_sd0_status: crate::Reg<gpio_qspi_sd0_status::GPIO_QSPI_SD0_STATUS_SPEC>,
    #[doc = "0x14 - GPIO control including function select and overrides."]
    pub gpio_qspi_sd0_ctrl: crate::Reg<gpio_qspi_sd0_ctrl::GPIO_QSPI_SD0_CTRL_SPEC>,
    #[doc = "0x18 - GPIO status"]
    pub gpio_qspi_sd1_status: crate::Reg<gpio_qspi_sd1_status::GPIO_QSPI_SD1_STATUS_SPEC>,
    #[doc = "0x1c - GPIO control including function select and overrides."]
    pub gpio_qspi_sd1_ctrl: crate::Reg<gpio_qspi_sd1_ctrl::GPIO_QSPI_SD1_CTRL_SPEC>,
    #[doc = "0x20 - GPIO status"]
    pub gpio_qspi_sd2_status: crate::Reg<gpio_qspi_sd2_status::GPIO_QSPI_SD2_STATUS_SPEC>,
    #[doc = "0x24 - GPIO control including function select and overrides."]
    pub gpio_qspi_sd2_ctrl: crate::Reg<gpio_qspi_sd2_ctrl::GPIO_QSPI_SD2_CTRL_SPEC>,
    #[doc = "0x28 - GPIO status"]
    pub gpio_qspi_sd3_status: crate::Reg<gpio_qspi_sd3_status::GPIO_QSPI_SD3_STATUS_SPEC>,
    #[doc = "0x2c - GPIO control including function select and overrides."]
    pub gpio_qspi_sd3_ctrl: crate::Reg<gpio_qspi_sd3_ctrl::GPIO_QSPI_SD3_CTRL_SPEC>,
    #[doc = "0x30 - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x34 - Interrupt Enable for proc0"]
    pub proc0_inte: crate::Reg<proc0_inte::PROC0_INTE_SPEC>,
    #[doc = "0x38 - Interrupt Force for proc0"]
    pub proc0_intf: crate::Reg<proc0_intf::PROC0_INTF_SPEC>,
    #[doc = "0x3c - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints: crate::Reg<proc0_ints::PROC0_INTS_SPEC>,
    #[doc = "0x40 - Interrupt Enable for proc1"]
    pub proc1_inte: crate::Reg<proc1_inte::PROC1_INTE_SPEC>,
    #[doc = "0x44 - Interrupt Force for proc1"]
    pub proc1_intf: crate::Reg<proc1_intf::PROC1_INTF_SPEC>,
    #[doc = "0x48 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints: crate::Reg<proc1_ints::PROC1_INTS_SPEC>,
    #[doc = "0x4c - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte: crate::Reg<dormant_wake_inte::DORMANT_WAKE_INTE_SPEC>,
    #[doc = "0x50 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf: crate::Reg<dormant_wake_intf::DORMANT_WAKE_INTF_SPEC>,
    #[doc = "0x54 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints: crate::Reg<dormant_wake_ints::DORMANT_WAKE_INTS_SPEC>,
}
#[doc = "GPIO_QSPI_SCLK_STATUS register accessor: an alias for `Reg<GPIO_QSPI_SCLK_STATUS_SPEC>`"]
pub type GPIO_QSPI_SCLK_STATUS = crate::Reg<gpio_qspi_sclk_status::GPIO_QSPI_SCLK_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio_qspi_sclk_status;
#[doc = "GPIO_QSPI_SCLK_CTRL register accessor: an alias for `Reg<GPIO_QSPI_SCLK_CTRL_SPEC>`"]
pub type GPIO_QSPI_SCLK_CTRL = crate::Reg<gpio_qspi_sclk_ctrl::GPIO_QSPI_SCLK_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sclk_ctrl;
#[doc = "GPIO_QSPI_SS_STATUS register accessor: an alias for `Reg<GPIO_QSPI_SS_STATUS_SPEC>`"]
pub type GPIO_QSPI_SS_STATUS = crate::Reg<gpio_qspi_ss_status::GPIO_QSPI_SS_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio_qspi_ss_status;
#[doc = "GPIO_QSPI_SS_CTRL register accessor: an alias for `Reg<GPIO_QSPI_SS_CTRL_SPEC>`"]
pub type GPIO_QSPI_SS_CTRL = crate::Reg<gpio_qspi_ss_ctrl::GPIO_QSPI_SS_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_ss_ctrl;
#[doc = "GPIO_QSPI_SD0_STATUS register accessor: an alias for `Reg<GPIO_QSPI_SD0_STATUS_SPEC>`"]
pub type GPIO_QSPI_SD0_STATUS = crate::Reg<gpio_qspi_sd0_status::GPIO_QSPI_SD0_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio_qspi_sd0_status;
#[doc = "GPIO_QSPI_SD0_CTRL register accessor: an alias for `Reg<GPIO_QSPI_SD0_CTRL_SPEC>`"]
pub type GPIO_QSPI_SD0_CTRL = crate::Reg<gpio_qspi_sd0_ctrl::GPIO_QSPI_SD0_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sd0_ctrl;
#[doc = "GPIO_QSPI_SD1_STATUS register accessor: an alias for `Reg<GPIO_QSPI_SD1_STATUS_SPEC>`"]
pub type GPIO_QSPI_SD1_STATUS = crate::Reg<gpio_qspi_sd1_status::GPIO_QSPI_SD1_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio_qspi_sd1_status;
#[doc = "GPIO_QSPI_SD1_CTRL register accessor: an alias for `Reg<GPIO_QSPI_SD1_CTRL_SPEC>`"]
pub type GPIO_QSPI_SD1_CTRL = crate::Reg<gpio_qspi_sd1_ctrl::GPIO_QSPI_SD1_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sd1_ctrl;
#[doc = "GPIO_QSPI_SD2_STATUS register accessor: an alias for `Reg<GPIO_QSPI_SD2_STATUS_SPEC>`"]
pub type GPIO_QSPI_SD2_STATUS = crate::Reg<gpio_qspi_sd2_status::GPIO_QSPI_SD2_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio_qspi_sd2_status;
#[doc = "GPIO_QSPI_SD2_CTRL register accessor: an alias for `Reg<GPIO_QSPI_SD2_CTRL_SPEC>`"]
pub type GPIO_QSPI_SD2_CTRL = crate::Reg<gpio_qspi_sd2_ctrl::GPIO_QSPI_SD2_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sd2_ctrl;
#[doc = "GPIO_QSPI_SD3_STATUS register accessor: an alias for `Reg<GPIO_QSPI_SD3_STATUS_SPEC>`"]
pub type GPIO_QSPI_SD3_STATUS = crate::Reg<gpio_qspi_sd3_status::GPIO_QSPI_SD3_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio_qspi_sd3_status;
#[doc = "GPIO_QSPI_SD3_CTRL register accessor: an alias for `Reg<GPIO_QSPI_SD3_CTRL_SPEC>`"]
pub type GPIO_QSPI_SD3_CTRL = crate::Reg<gpio_qspi_sd3_ctrl::GPIO_QSPI_SD3_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sd3_ctrl;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "PROC0_INTE register accessor: an alias for `Reg<PROC0_INTE_SPEC>`"]
pub type PROC0_INTE = crate::Reg<proc0_inte::PROC0_INTE_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte;
#[doc = "PROC0_INTF register accessor: an alias for `Reg<PROC0_INTF_SPEC>`"]
pub type PROC0_INTF = crate::Reg<proc0_intf::PROC0_INTF_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf;
#[doc = "PROC0_INTS register accessor: an alias for `Reg<PROC0_INTS_SPEC>`"]
pub type PROC0_INTS = crate::Reg<proc0_ints::PROC0_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints;
#[doc = "PROC1_INTE register accessor: an alias for `Reg<PROC1_INTE_SPEC>`"]
pub type PROC1_INTE = crate::Reg<proc1_inte::PROC1_INTE_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte;
#[doc = "PROC1_INTF register accessor: an alias for `Reg<PROC1_INTF_SPEC>`"]
pub type PROC1_INTF = crate::Reg<proc1_intf::PROC1_INTF_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf;
#[doc = "PROC1_INTS register accessor: an alias for `Reg<PROC1_INTS_SPEC>`"]
pub type PROC1_INTS = crate::Reg<proc1_ints::PROC1_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints;
#[doc = "DORMANT_WAKE_INTE register accessor: an alias for `Reg<DORMANT_WAKE_INTE_SPEC>`"]
pub type DORMANT_WAKE_INTE = crate::Reg<dormant_wake_inte::DORMANT_WAKE_INTE_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte;
#[doc = "DORMANT_WAKE_INTF register accessor: an alias for `Reg<DORMANT_WAKE_INTF_SPEC>`"]
pub type DORMANT_WAKE_INTF = crate::Reg<dormant_wake_intf::DORMANT_WAKE_INTF_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf;
#[doc = "DORMANT_WAKE_INTS register accessor: an alias for `Reg<DORMANT_WAKE_INTS_SPEC>`"]
pub type DORMANT_WAKE_INTS = crate::Reg<dormant_wake_ints::DORMANT_WAKE_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints;
