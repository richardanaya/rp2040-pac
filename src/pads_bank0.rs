#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage select. Per bank control"]
    pub voltage_select: crate::Reg<voltage_select::VOLTAGE_SELECT_SPEC>,
    #[doc = "0x04 - Pad control register"]
    pub gpio0: crate::Reg<gpio0::GPIO0_SPEC>,
    #[doc = "0x08 - Pad control register"]
    pub gpio1: crate::Reg<gpio1::GPIO1_SPEC>,
    #[doc = "0x0c - Pad control register"]
    pub gpio2: crate::Reg<gpio2::GPIO2_SPEC>,
    #[doc = "0x10 - Pad control register"]
    pub gpio3: crate::Reg<gpio3::GPIO3_SPEC>,
    #[doc = "0x14 - Pad control register"]
    pub gpio4: crate::Reg<gpio4::GPIO4_SPEC>,
    #[doc = "0x18 - Pad control register"]
    pub gpio5: crate::Reg<gpio5::GPIO5_SPEC>,
    #[doc = "0x1c - Pad control register"]
    pub gpio6: crate::Reg<gpio6::GPIO6_SPEC>,
    #[doc = "0x20 - Pad control register"]
    pub gpio7: crate::Reg<gpio7::GPIO7_SPEC>,
    #[doc = "0x24 - Pad control register"]
    pub gpio8: crate::Reg<gpio8::GPIO8_SPEC>,
    #[doc = "0x28 - Pad control register"]
    pub gpio9: crate::Reg<gpio9::GPIO9_SPEC>,
    #[doc = "0x2c - Pad control register"]
    pub gpio10: crate::Reg<gpio10::GPIO10_SPEC>,
    #[doc = "0x30 - Pad control register"]
    pub gpio11: crate::Reg<gpio11::GPIO11_SPEC>,
    #[doc = "0x34 - Pad control register"]
    pub gpio12: crate::Reg<gpio12::GPIO12_SPEC>,
    #[doc = "0x38 - Pad control register"]
    pub gpio13: crate::Reg<gpio13::GPIO13_SPEC>,
    #[doc = "0x3c - Pad control register"]
    pub gpio14: crate::Reg<gpio14::GPIO14_SPEC>,
    #[doc = "0x40 - Pad control register"]
    pub gpio15: crate::Reg<gpio15::GPIO15_SPEC>,
    #[doc = "0x44 - Pad control register"]
    pub gpio16: crate::Reg<gpio16::GPIO16_SPEC>,
    #[doc = "0x48 - Pad control register"]
    pub gpio17: crate::Reg<gpio17::GPIO17_SPEC>,
    #[doc = "0x4c - Pad control register"]
    pub gpio18: crate::Reg<gpio18::GPIO18_SPEC>,
    #[doc = "0x50 - Pad control register"]
    pub gpio19: crate::Reg<gpio19::GPIO19_SPEC>,
    #[doc = "0x54 - Pad control register"]
    pub gpio20: crate::Reg<gpio20::GPIO20_SPEC>,
    #[doc = "0x58 - Pad control register"]
    pub gpio21: crate::Reg<gpio21::GPIO21_SPEC>,
    #[doc = "0x5c - Pad control register"]
    pub gpio22: crate::Reg<gpio22::GPIO22_SPEC>,
    #[doc = "0x60 - Pad control register"]
    pub gpio23: crate::Reg<gpio23::GPIO23_SPEC>,
    #[doc = "0x64 - Pad control register"]
    pub gpio24: crate::Reg<gpio24::GPIO24_SPEC>,
    #[doc = "0x68 - Pad control register"]
    pub gpio25: crate::Reg<gpio25::GPIO25_SPEC>,
    #[doc = "0x6c - Pad control register"]
    pub gpio26: crate::Reg<gpio26::GPIO26_SPEC>,
    #[doc = "0x70 - Pad control register"]
    pub gpio27: crate::Reg<gpio27::GPIO27_SPEC>,
    #[doc = "0x74 - Pad control register"]
    pub gpio28: crate::Reg<gpio28::GPIO28_SPEC>,
    #[doc = "0x78 - Pad control register"]
    pub gpio29: crate::Reg<gpio29::GPIO29_SPEC>,
    #[doc = "0x7c - Pad control register"]
    pub swclk: crate::Reg<swclk::SWCLK_SPEC>,
    #[doc = "0x80 - Pad control register"]
    pub swd: crate::Reg<swd::SWD_SPEC>,
}
#[doc = "VOLTAGE_SELECT register accessor: an alias for `Reg<VOLTAGE_SELECT_SPEC>`"]
pub type VOLTAGE_SELECT = crate::Reg<voltage_select::VOLTAGE_SELECT_SPEC>;
#[doc = "Voltage select. Per bank control"]
pub mod voltage_select;
#[doc = "GPIO0 register accessor: an alias for `Reg<GPIO0_SPEC>`"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = "Pad control register"]
pub mod gpio0;
#[doc = "GPIO1 register accessor: an alias for `Reg<GPIO1_SPEC>`"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = "Pad control register"]
pub mod gpio1;
#[doc = "GPIO2 register accessor: an alias for `Reg<GPIO2_SPEC>`"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = "Pad control register"]
pub mod gpio2;
#[doc = "GPIO3 register accessor: an alias for `Reg<GPIO3_SPEC>`"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = "Pad control register"]
pub mod gpio3;
#[doc = "GPIO4 register accessor: an alias for `Reg<GPIO4_SPEC>`"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = "Pad control register"]
pub mod gpio4;
#[doc = "GPIO5 register accessor: an alias for `Reg<GPIO5_SPEC>`"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = "Pad control register"]
pub mod gpio5;
#[doc = "GPIO6 register accessor: an alias for `Reg<GPIO6_SPEC>`"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = "Pad control register"]
pub mod gpio6;
#[doc = "GPIO7 register accessor: an alias for `Reg<GPIO7_SPEC>`"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = "Pad control register"]
pub mod gpio7;
#[doc = "GPIO8 register accessor: an alias for `Reg<GPIO8_SPEC>`"]
pub type GPIO8 = crate::Reg<gpio8::GPIO8_SPEC>;
#[doc = "Pad control register"]
pub mod gpio8;
#[doc = "GPIO9 register accessor: an alias for `Reg<GPIO9_SPEC>`"]
pub type GPIO9 = crate::Reg<gpio9::GPIO9_SPEC>;
#[doc = "Pad control register"]
pub mod gpio9;
#[doc = "GPIO10 register accessor: an alias for `Reg<GPIO10_SPEC>`"]
pub type GPIO10 = crate::Reg<gpio10::GPIO10_SPEC>;
#[doc = "Pad control register"]
pub mod gpio10;
#[doc = "GPIO11 register accessor: an alias for `Reg<GPIO11_SPEC>`"]
pub type GPIO11 = crate::Reg<gpio11::GPIO11_SPEC>;
#[doc = "Pad control register"]
pub mod gpio11;
#[doc = "GPIO12 register accessor: an alias for `Reg<GPIO12_SPEC>`"]
pub type GPIO12 = crate::Reg<gpio12::GPIO12_SPEC>;
#[doc = "Pad control register"]
pub mod gpio12;
#[doc = "GPIO13 register accessor: an alias for `Reg<GPIO13_SPEC>`"]
pub type GPIO13 = crate::Reg<gpio13::GPIO13_SPEC>;
#[doc = "Pad control register"]
pub mod gpio13;
#[doc = "GPIO14 register accessor: an alias for `Reg<GPIO14_SPEC>`"]
pub type GPIO14 = crate::Reg<gpio14::GPIO14_SPEC>;
#[doc = "Pad control register"]
pub mod gpio14;
#[doc = "GPIO15 register accessor: an alias for `Reg<GPIO15_SPEC>`"]
pub type GPIO15 = crate::Reg<gpio15::GPIO15_SPEC>;
#[doc = "Pad control register"]
pub mod gpio15;
#[doc = "GPIO16 register accessor: an alias for `Reg<GPIO16_SPEC>`"]
pub type GPIO16 = crate::Reg<gpio16::GPIO16_SPEC>;
#[doc = "Pad control register"]
pub mod gpio16;
#[doc = "GPIO17 register accessor: an alias for `Reg<GPIO17_SPEC>`"]
pub type GPIO17 = crate::Reg<gpio17::GPIO17_SPEC>;
#[doc = "Pad control register"]
pub mod gpio17;
#[doc = "GPIO18 register accessor: an alias for `Reg<GPIO18_SPEC>`"]
pub type GPIO18 = crate::Reg<gpio18::GPIO18_SPEC>;
#[doc = "Pad control register"]
pub mod gpio18;
#[doc = "GPIO19 register accessor: an alias for `Reg<GPIO19_SPEC>`"]
pub type GPIO19 = crate::Reg<gpio19::GPIO19_SPEC>;
#[doc = "Pad control register"]
pub mod gpio19;
#[doc = "GPIO20 register accessor: an alias for `Reg<GPIO20_SPEC>`"]
pub type GPIO20 = crate::Reg<gpio20::GPIO20_SPEC>;
#[doc = "Pad control register"]
pub mod gpio20;
#[doc = "GPIO21 register accessor: an alias for `Reg<GPIO21_SPEC>`"]
pub type GPIO21 = crate::Reg<gpio21::GPIO21_SPEC>;
#[doc = "Pad control register"]
pub mod gpio21;
#[doc = "GPIO22 register accessor: an alias for `Reg<GPIO22_SPEC>`"]
pub type GPIO22 = crate::Reg<gpio22::GPIO22_SPEC>;
#[doc = "Pad control register"]
pub mod gpio22;
#[doc = "GPIO23 register accessor: an alias for `Reg<GPIO23_SPEC>`"]
pub type GPIO23 = crate::Reg<gpio23::GPIO23_SPEC>;
#[doc = "Pad control register"]
pub mod gpio23;
#[doc = "GPIO24 register accessor: an alias for `Reg<GPIO24_SPEC>`"]
pub type GPIO24 = crate::Reg<gpio24::GPIO24_SPEC>;
#[doc = "Pad control register"]
pub mod gpio24;
#[doc = "GPIO25 register accessor: an alias for `Reg<GPIO25_SPEC>`"]
pub type GPIO25 = crate::Reg<gpio25::GPIO25_SPEC>;
#[doc = "Pad control register"]
pub mod gpio25;
#[doc = "GPIO26 register accessor: an alias for `Reg<GPIO26_SPEC>`"]
pub type GPIO26 = crate::Reg<gpio26::GPIO26_SPEC>;
#[doc = "Pad control register"]
pub mod gpio26;
#[doc = "GPIO27 register accessor: an alias for `Reg<GPIO27_SPEC>`"]
pub type GPIO27 = crate::Reg<gpio27::GPIO27_SPEC>;
#[doc = "Pad control register"]
pub mod gpio27;
#[doc = "GPIO28 register accessor: an alias for `Reg<GPIO28_SPEC>`"]
pub type GPIO28 = crate::Reg<gpio28::GPIO28_SPEC>;
#[doc = "Pad control register"]
pub mod gpio28;
#[doc = "GPIO29 register accessor: an alias for `Reg<GPIO29_SPEC>`"]
pub type GPIO29 = crate::Reg<gpio29::GPIO29_SPEC>;
#[doc = "Pad control register"]
pub mod gpio29;
#[doc = "SWCLK register accessor: an alias for `Reg<SWCLK_SPEC>`"]
pub type SWCLK = crate::Reg<swclk::SWCLK_SPEC>;
#[doc = "Pad control register"]
pub mod swclk;
#[doc = "SWD register accessor: an alias for `Reg<SWD_SPEC>`"]
pub type SWD = crate::Reg<swd::SWD_SPEC>;
#[doc = "Pad control register"]
pub mod swd;
