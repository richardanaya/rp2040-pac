#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO status"]
    pub gpio0_status: crate::Reg<gpio0_status::GPIO0_STATUS_SPEC>,
    #[doc = "0x04 - GPIO control including function select and overrides."]
    pub gpio0_ctrl: crate::Reg<gpio0_ctrl::GPIO0_CTRL_SPEC>,
    #[doc = "0x08 - GPIO status"]
    pub gpio1_status: crate::Reg<gpio1_status::GPIO1_STATUS_SPEC>,
    #[doc = "0x0c - GPIO control including function select and overrides."]
    pub gpio1_ctrl: crate::Reg<gpio1_ctrl::GPIO1_CTRL_SPEC>,
    #[doc = "0x10 - GPIO status"]
    pub gpio2_status: crate::Reg<gpio2_status::GPIO2_STATUS_SPEC>,
    #[doc = "0x14 - GPIO control including function select and overrides."]
    pub gpio2_ctrl: crate::Reg<gpio2_ctrl::GPIO2_CTRL_SPEC>,
    #[doc = "0x18 - GPIO status"]
    pub gpio3_status: crate::Reg<gpio3_status::GPIO3_STATUS_SPEC>,
    #[doc = "0x1c - GPIO control including function select and overrides."]
    pub gpio3_ctrl: crate::Reg<gpio3_ctrl::GPIO3_CTRL_SPEC>,
    #[doc = "0x20 - GPIO status"]
    pub gpio4_status: crate::Reg<gpio4_status::GPIO4_STATUS_SPEC>,
    #[doc = "0x24 - GPIO control including function select and overrides."]
    pub gpio4_ctrl: crate::Reg<gpio4_ctrl::GPIO4_CTRL_SPEC>,
    #[doc = "0x28 - GPIO status"]
    pub gpio5_status: crate::Reg<gpio5_status::GPIO5_STATUS_SPEC>,
    #[doc = "0x2c - GPIO control including function select and overrides."]
    pub gpio5_ctrl: crate::Reg<gpio5_ctrl::GPIO5_CTRL_SPEC>,
    #[doc = "0x30 - GPIO status"]
    pub gpio6_status: crate::Reg<gpio6_status::GPIO6_STATUS_SPEC>,
    #[doc = "0x34 - GPIO control including function select and overrides."]
    pub gpio6_ctrl: crate::Reg<gpio6_ctrl::GPIO6_CTRL_SPEC>,
    #[doc = "0x38 - GPIO status"]
    pub gpio7_status: crate::Reg<gpio7_status::GPIO7_STATUS_SPEC>,
    #[doc = "0x3c - GPIO control including function select and overrides."]
    pub gpio7_ctrl: crate::Reg<gpio7_ctrl::GPIO7_CTRL_SPEC>,
    #[doc = "0x40 - GPIO status"]
    pub gpio8_status: crate::Reg<gpio8_status::GPIO8_STATUS_SPEC>,
    #[doc = "0x44 - GPIO control including function select and overrides."]
    pub gpio8_ctrl: crate::Reg<gpio8_ctrl::GPIO8_CTRL_SPEC>,
    #[doc = "0x48 - GPIO status"]
    pub gpio9_status: crate::Reg<gpio9_status::GPIO9_STATUS_SPEC>,
    #[doc = "0x4c - GPIO control including function select and overrides."]
    pub gpio9_ctrl: crate::Reg<gpio9_ctrl::GPIO9_CTRL_SPEC>,
    #[doc = "0x50 - GPIO status"]
    pub gpio10_status: crate::Reg<gpio10_status::GPIO10_STATUS_SPEC>,
    #[doc = "0x54 - GPIO control including function select and overrides."]
    pub gpio10_ctrl: crate::Reg<gpio10_ctrl::GPIO10_CTRL_SPEC>,
    #[doc = "0x58 - GPIO status"]
    pub gpio11_status: crate::Reg<gpio11_status::GPIO11_STATUS_SPEC>,
    #[doc = "0x5c - GPIO control including function select and overrides."]
    pub gpio11_ctrl: crate::Reg<gpio11_ctrl::GPIO11_CTRL_SPEC>,
    #[doc = "0x60 - GPIO status"]
    pub gpio12_status: crate::Reg<gpio12_status::GPIO12_STATUS_SPEC>,
    #[doc = "0x64 - GPIO control including function select and overrides."]
    pub gpio12_ctrl: crate::Reg<gpio12_ctrl::GPIO12_CTRL_SPEC>,
    #[doc = "0x68 - GPIO status"]
    pub gpio13_status: crate::Reg<gpio13_status::GPIO13_STATUS_SPEC>,
    #[doc = "0x6c - GPIO control including function select and overrides."]
    pub gpio13_ctrl: crate::Reg<gpio13_ctrl::GPIO13_CTRL_SPEC>,
    #[doc = "0x70 - GPIO status"]
    pub gpio14_status: crate::Reg<gpio14_status::GPIO14_STATUS_SPEC>,
    #[doc = "0x74 - GPIO control including function select and overrides."]
    pub gpio14_ctrl: crate::Reg<gpio14_ctrl::GPIO14_CTRL_SPEC>,
    #[doc = "0x78 - GPIO status"]
    pub gpio15_status: crate::Reg<gpio15_status::GPIO15_STATUS_SPEC>,
    #[doc = "0x7c - GPIO control including function select and overrides."]
    pub gpio15_ctrl: crate::Reg<gpio15_ctrl::GPIO15_CTRL_SPEC>,
    #[doc = "0x80 - GPIO status"]
    pub gpio16_status: crate::Reg<gpio16_status::GPIO16_STATUS_SPEC>,
    #[doc = "0x84 - GPIO control including function select and overrides."]
    pub gpio16_ctrl: crate::Reg<gpio16_ctrl::GPIO16_CTRL_SPEC>,
    #[doc = "0x88 - GPIO status"]
    pub gpio17_status: crate::Reg<gpio17_status::GPIO17_STATUS_SPEC>,
    #[doc = "0x8c - GPIO control including function select and overrides."]
    pub gpio17_ctrl: crate::Reg<gpio17_ctrl::GPIO17_CTRL_SPEC>,
    #[doc = "0x90 - GPIO status"]
    pub gpio18_status: crate::Reg<gpio18_status::GPIO18_STATUS_SPEC>,
    #[doc = "0x94 - GPIO control including function select and overrides."]
    pub gpio18_ctrl: crate::Reg<gpio18_ctrl::GPIO18_CTRL_SPEC>,
    #[doc = "0x98 - GPIO status"]
    pub gpio19_status: crate::Reg<gpio19_status::GPIO19_STATUS_SPEC>,
    #[doc = "0x9c - GPIO control including function select and overrides."]
    pub gpio19_ctrl: crate::Reg<gpio19_ctrl::GPIO19_CTRL_SPEC>,
    #[doc = "0xa0 - GPIO status"]
    pub gpio20_status: crate::Reg<gpio20_status::GPIO20_STATUS_SPEC>,
    #[doc = "0xa4 - GPIO control including function select and overrides."]
    pub gpio20_ctrl: crate::Reg<gpio20_ctrl::GPIO20_CTRL_SPEC>,
    #[doc = "0xa8 - GPIO status"]
    pub gpio21_status: crate::Reg<gpio21_status::GPIO21_STATUS_SPEC>,
    #[doc = "0xac - GPIO control including function select and overrides."]
    pub gpio21_ctrl: crate::Reg<gpio21_ctrl::GPIO21_CTRL_SPEC>,
    #[doc = "0xb0 - GPIO status"]
    pub gpio22_status: crate::Reg<gpio22_status::GPIO22_STATUS_SPEC>,
    #[doc = "0xb4 - GPIO control including function select and overrides."]
    pub gpio22_ctrl: crate::Reg<gpio22_ctrl::GPIO22_CTRL_SPEC>,
    #[doc = "0xb8 - GPIO status"]
    pub gpio23_status: crate::Reg<gpio23_status::GPIO23_STATUS_SPEC>,
    #[doc = "0xbc - GPIO control including function select and overrides."]
    pub gpio23_ctrl: crate::Reg<gpio23_ctrl::GPIO23_CTRL_SPEC>,
    #[doc = "0xc0 - GPIO status"]
    pub gpio24_status: crate::Reg<gpio24_status::GPIO24_STATUS_SPEC>,
    #[doc = "0xc4 - GPIO control including function select and overrides."]
    pub gpio24_ctrl: crate::Reg<gpio24_ctrl::GPIO24_CTRL_SPEC>,
    #[doc = "0xc8 - GPIO status"]
    pub gpio25_status: crate::Reg<gpio25_status::GPIO25_STATUS_SPEC>,
    #[doc = "0xcc - GPIO control including function select and overrides."]
    pub gpio25_ctrl: crate::Reg<gpio25_ctrl::GPIO25_CTRL_SPEC>,
    #[doc = "0xd0 - GPIO status"]
    pub gpio26_status: crate::Reg<gpio26_status::GPIO26_STATUS_SPEC>,
    #[doc = "0xd4 - GPIO control including function select and overrides."]
    pub gpio26_ctrl: crate::Reg<gpio26_ctrl::GPIO26_CTRL_SPEC>,
    #[doc = "0xd8 - GPIO status"]
    pub gpio27_status: crate::Reg<gpio27_status::GPIO27_STATUS_SPEC>,
    #[doc = "0xdc - GPIO control including function select and overrides."]
    pub gpio27_ctrl: crate::Reg<gpio27_ctrl::GPIO27_CTRL_SPEC>,
    #[doc = "0xe0 - GPIO status"]
    pub gpio28_status: crate::Reg<gpio28_status::GPIO28_STATUS_SPEC>,
    #[doc = "0xe4 - GPIO control including function select and overrides."]
    pub gpio28_ctrl: crate::Reg<gpio28_ctrl::GPIO28_CTRL_SPEC>,
    #[doc = "0xe8 - GPIO status"]
    pub gpio29_status: crate::Reg<gpio29_status::GPIO29_STATUS_SPEC>,
    #[doc = "0xec - GPIO control including function select and overrides."]
    pub gpio29_ctrl: crate::Reg<gpio29_ctrl::GPIO29_CTRL_SPEC>,
    #[doc = "0xf0 - Raw Interrupts"]
    pub intr0: crate::Reg<intr0::INTR0_SPEC>,
    #[doc = "0xf4 - Raw Interrupts"]
    pub intr1: crate::Reg<intr1::INTR1_SPEC>,
    #[doc = "0xf8 - Raw Interrupts"]
    pub intr2: crate::Reg<intr2::INTR2_SPEC>,
    #[doc = "0xfc - Raw Interrupts"]
    pub intr3: crate::Reg<intr3::INTR3_SPEC>,
    #[doc = "0x100 - Interrupt Enable for proc0"]
    pub proc0_inte0: crate::Reg<proc0_inte0::PROC0_INTE0_SPEC>,
    #[doc = "0x104 - Interrupt Enable for proc0"]
    pub proc0_inte1: crate::Reg<proc0_inte1::PROC0_INTE1_SPEC>,
    #[doc = "0x108 - Interrupt Enable for proc0"]
    pub proc0_inte2: crate::Reg<proc0_inte2::PROC0_INTE2_SPEC>,
    #[doc = "0x10c - Interrupt Enable for proc0"]
    pub proc0_inte3: crate::Reg<proc0_inte3::PROC0_INTE3_SPEC>,
    #[doc = "0x110 - Interrupt Force for proc0"]
    pub proc0_intf0: crate::Reg<proc0_intf0::PROC0_INTF0_SPEC>,
    #[doc = "0x114 - Interrupt Force for proc0"]
    pub proc0_intf1: crate::Reg<proc0_intf1::PROC0_INTF1_SPEC>,
    #[doc = "0x118 - Interrupt Force for proc0"]
    pub proc0_intf2: crate::Reg<proc0_intf2::PROC0_INTF2_SPEC>,
    #[doc = "0x11c - Interrupt Force for proc0"]
    pub proc0_intf3: crate::Reg<proc0_intf3::PROC0_INTF3_SPEC>,
    #[doc = "0x120 - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints0: crate::Reg<proc0_ints0::PROC0_INTS0_SPEC>,
    #[doc = "0x124 - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints1: crate::Reg<proc0_ints1::PROC0_INTS1_SPEC>,
    #[doc = "0x128 - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints2: crate::Reg<proc0_ints2::PROC0_INTS2_SPEC>,
    #[doc = "0x12c - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints3: crate::Reg<proc0_ints3::PROC0_INTS3_SPEC>,
    #[doc = "0x130 - Interrupt Enable for proc1"]
    pub proc1_inte0: crate::Reg<proc1_inte0::PROC1_INTE0_SPEC>,
    #[doc = "0x134 - Interrupt Enable for proc1"]
    pub proc1_inte1: crate::Reg<proc1_inte1::PROC1_INTE1_SPEC>,
    #[doc = "0x138 - Interrupt Enable for proc1"]
    pub proc1_inte2: crate::Reg<proc1_inte2::PROC1_INTE2_SPEC>,
    #[doc = "0x13c - Interrupt Enable for proc1"]
    pub proc1_inte3: crate::Reg<proc1_inte3::PROC1_INTE3_SPEC>,
    #[doc = "0x140 - Interrupt Force for proc1"]
    pub proc1_intf0: crate::Reg<proc1_intf0::PROC1_INTF0_SPEC>,
    #[doc = "0x144 - Interrupt Force for proc1"]
    pub proc1_intf1: crate::Reg<proc1_intf1::PROC1_INTF1_SPEC>,
    #[doc = "0x148 - Interrupt Force for proc1"]
    pub proc1_intf2: crate::Reg<proc1_intf2::PROC1_INTF2_SPEC>,
    #[doc = "0x14c - Interrupt Force for proc1"]
    pub proc1_intf3: crate::Reg<proc1_intf3::PROC1_INTF3_SPEC>,
    #[doc = "0x150 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints0: crate::Reg<proc1_ints0::PROC1_INTS0_SPEC>,
    #[doc = "0x154 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints1: crate::Reg<proc1_ints1::PROC1_INTS1_SPEC>,
    #[doc = "0x158 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints2: crate::Reg<proc1_ints2::PROC1_INTS2_SPEC>,
    #[doc = "0x15c - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints3: crate::Reg<proc1_ints3::PROC1_INTS3_SPEC>,
    #[doc = "0x160 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte0: crate::Reg<dormant_wake_inte0::DORMANT_WAKE_INTE0_SPEC>,
    #[doc = "0x164 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte1: crate::Reg<dormant_wake_inte1::DORMANT_WAKE_INTE1_SPEC>,
    #[doc = "0x168 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte2: crate::Reg<dormant_wake_inte2::DORMANT_WAKE_INTE2_SPEC>,
    #[doc = "0x16c - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte3: crate::Reg<dormant_wake_inte3::DORMANT_WAKE_INTE3_SPEC>,
    #[doc = "0x170 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf0: crate::Reg<dormant_wake_intf0::DORMANT_WAKE_INTF0_SPEC>,
    #[doc = "0x174 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf1: crate::Reg<dormant_wake_intf1::DORMANT_WAKE_INTF1_SPEC>,
    #[doc = "0x178 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf2: crate::Reg<dormant_wake_intf2::DORMANT_WAKE_INTF2_SPEC>,
    #[doc = "0x17c - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf3: crate::Reg<dormant_wake_intf3::DORMANT_WAKE_INTF3_SPEC>,
    #[doc = "0x180 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints0: crate::Reg<dormant_wake_ints0::DORMANT_WAKE_INTS0_SPEC>,
    #[doc = "0x184 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints1: crate::Reg<dormant_wake_ints1::DORMANT_WAKE_INTS1_SPEC>,
    #[doc = "0x188 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints2: crate::Reg<dormant_wake_ints2::DORMANT_WAKE_INTS2_SPEC>,
    #[doc = "0x18c - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints3: crate::Reg<dormant_wake_ints3::DORMANT_WAKE_INTS3_SPEC>,
}
#[doc = "GPIO0_STATUS register accessor: an alias for `Reg<GPIO0_STATUS_SPEC>`"]
pub type GPIO0_STATUS = crate::Reg<gpio0_status::GPIO0_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio0_status;
#[doc = "GPIO0_CTRL register accessor: an alias for `Reg<GPIO0_CTRL_SPEC>`"]
pub type GPIO0_CTRL = crate::Reg<gpio0_ctrl::GPIO0_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio0_ctrl;
#[doc = "GPIO1_STATUS register accessor: an alias for `Reg<GPIO1_STATUS_SPEC>`"]
pub type GPIO1_STATUS = crate::Reg<gpio1_status::GPIO1_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio1_status;
#[doc = "GPIO1_CTRL register accessor: an alias for `Reg<GPIO1_CTRL_SPEC>`"]
pub type GPIO1_CTRL = crate::Reg<gpio1_ctrl::GPIO1_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio1_ctrl;
#[doc = "GPIO2_STATUS register accessor: an alias for `Reg<GPIO2_STATUS_SPEC>`"]
pub type GPIO2_STATUS = crate::Reg<gpio2_status::GPIO2_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio2_status;
#[doc = "GPIO2_CTRL register accessor: an alias for `Reg<GPIO2_CTRL_SPEC>`"]
pub type GPIO2_CTRL = crate::Reg<gpio2_ctrl::GPIO2_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio2_ctrl;
#[doc = "GPIO3_STATUS register accessor: an alias for `Reg<GPIO3_STATUS_SPEC>`"]
pub type GPIO3_STATUS = crate::Reg<gpio3_status::GPIO3_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio3_status;
#[doc = "GPIO3_CTRL register accessor: an alias for `Reg<GPIO3_CTRL_SPEC>`"]
pub type GPIO3_CTRL = crate::Reg<gpio3_ctrl::GPIO3_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio3_ctrl;
#[doc = "GPIO4_STATUS register accessor: an alias for `Reg<GPIO4_STATUS_SPEC>`"]
pub type GPIO4_STATUS = crate::Reg<gpio4_status::GPIO4_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio4_status;
#[doc = "GPIO4_CTRL register accessor: an alias for `Reg<GPIO4_CTRL_SPEC>`"]
pub type GPIO4_CTRL = crate::Reg<gpio4_ctrl::GPIO4_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio4_ctrl;
#[doc = "GPIO5_STATUS register accessor: an alias for `Reg<GPIO5_STATUS_SPEC>`"]
pub type GPIO5_STATUS = crate::Reg<gpio5_status::GPIO5_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio5_status;
#[doc = "GPIO5_CTRL register accessor: an alias for `Reg<GPIO5_CTRL_SPEC>`"]
pub type GPIO5_CTRL = crate::Reg<gpio5_ctrl::GPIO5_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio5_ctrl;
#[doc = "GPIO6_STATUS register accessor: an alias for `Reg<GPIO6_STATUS_SPEC>`"]
pub type GPIO6_STATUS = crate::Reg<gpio6_status::GPIO6_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio6_status;
#[doc = "GPIO6_CTRL register accessor: an alias for `Reg<GPIO6_CTRL_SPEC>`"]
pub type GPIO6_CTRL = crate::Reg<gpio6_ctrl::GPIO6_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio6_ctrl;
#[doc = "GPIO7_STATUS register accessor: an alias for `Reg<GPIO7_STATUS_SPEC>`"]
pub type GPIO7_STATUS = crate::Reg<gpio7_status::GPIO7_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio7_status;
#[doc = "GPIO7_CTRL register accessor: an alias for `Reg<GPIO7_CTRL_SPEC>`"]
pub type GPIO7_CTRL = crate::Reg<gpio7_ctrl::GPIO7_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio7_ctrl;
#[doc = "GPIO8_STATUS register accessor: an alias for `Reg<GPIO8_STATUS_SPEC>`"]
pub type GPIO8_STATUS = crate::Reg<gpio8_status::GPIO8_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio8_status;
#[doc = "GPIO8_CTRL register accessor: an alias for `Reg<GPIO8_CTRL_SPEC>`"]
pub type GPIO8_CTRL = crate::Reg<gpio8_ctrl::GPIO8_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio8_ctrl;
#[doc = "GPIO9_STATUS register accessor: an alias for `Reg<GPIO9_STATUS_SPEC>`"]
pub type GPIO9_STATUS = crate::Reg<gpio9_status::GPIO9_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio9_status;
#[doc = "GPIO9_CTRL register accessor: an alias for `Reg<GPIO9_CTRL_SPEC>`"]
pub type GPIO9_CTRL = crate::Reg<gpio9_ctrl::GPIO9_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio9_ctrl;
#[doc = "GPIO10_STATUS register accessor: an alias for `Reg<GPIO10_STATUS_SPEC>`"]
pub type GPIO10_STATUS = crate::Reg<gpio10_status::GPIO10_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio10_status;
#[doc = "GPIO10_CTRL register accessor: an alias for `Reg<GPIO10_CTRL_SPEC>`"]
pub type GPIO10_CTRL = crate::Reg<gpio10_ctrl::GPIO10_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio10_ctrl;
#[doc = "GPIO11_STATUS register accessor: an alias for `Reg<GPIO11_STATUS_SPEC>`"]
pub type GPIO11_STATUS = crate::Reg<gpio11_status::GPIO11_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio11_status;
#[doc = "GPIO11_CTRL register accessor: an alias for `Reg<GPIO11_CTRL_SPEC>`"]
pub type GPIO11_CTRL = crate::Reg<gpio11_ctrl::GPIO11_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio11_ctrl;
#[doc = "GPIO12_STATUS register accessor: an alias for `Reg<GPIO12_STATUS_SPEC>`"]
pub type GPIO12_STATUS = crate::Reg<gpio12_status::GPIO12_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio12_status;
#[doc = "GPIO12_CTRL register accessor: an alias for `Reg<GPIO12_CTRL_SPEC>`"]
pub type GPIO12_CTRL = crate::Reg<gpio12_ctrl::GPIO12_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio12_ctrl;
#[doc = "GPIO13_STATUS register accessor: an alias for `Reg<GPIO13_STATUS_SPEC>`"]
pub type GPIO13_STATUS = crate::Reg<gpio13_status::GPIO13_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio13_status;
#[doc = "GPIO13_CTRL register accessor: an alias for `Reg<GPIO13_CTRL_SPEC>`"]
pub type GPIO13_CTRL = crate::Reg<gpio13_ctrl::GPIO13_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio13_ctrl;
#[doc = "GPIO14_STATUS register accessor: an alias for `Reg<GPIO14_STATUS_SPEC>`"]
pub type GPIO14_STATUS = crate::Reg<gpio14_status::GPIO14_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio14_status;
#[doc = "GPIO14_CTRL register accessor: an alias for `Reg<GPIO14_CTRL_SPEC>`"]
pub type GPIO14_CTRL = crate::Reg<gpio14_ctrl::GPIO14_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio14_ctrl;
#[doc = "GPIO15_STATUS register accessor: an alias for `Reg<GPIO15_STATUS_SPEC>`"]
pub type GPIO15_STATUS = crate::Reg<gpio15_status::GPIO15_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio15_status;
#[doc = "GPIO15_CTRL register accessor: an alias for `Reg<GPIO15_CTRL_SPEC>`"]
pub type GPIO15_CTRL = crate::Reg<gpio15_ctrl::GPIO15_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio15_ctrl;
#[doc = "GPIO16_STATUS register accessor: an alias for `Reg<GPIO16_STATUS_SPEC>`"]
pub type GPIO16_STATUS = crate::Reg<gpio16_status::GPIO16_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio16_status;
#[doc = "GPIO16_CTRL register accessor: an alias for `Reg<GPIO16_CTRL_SPEC>`"]
pub type GPIO16_CTRL = crate::Reg<gpio16_ctrl::GPIO16_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio16_ctrl;
#[doc = "GPIO17_STATUS register accessor: an alias for `Reg<GPIO17_STATUS_SPEC>`"]
pub type GPIO17_STATUS = crate::Reg<gpio17_status::GPIO17_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio17_status;
#[doc = "GPIO17_CTRL register accessor: an alias for `Reg<GPIO17_CTRL_SPEC>`"]
pub type GPIO17_CTRL = crate::Reg<gpio17_ctrl::GPIO17_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio17_ctrl;
#[doc = "GPIO18_STATUS register accessor: an alias for `Reg<GPIO18_STATUS_SPEC>`"]
pub type GPIO18_STATUS = crate::Reg<gpio18_status::GPIO18_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio18_status;
#[doc = "GPIO18_CTRL register accessor: an alias for `Reg<GPIO18_CTRL_SPEC>`"]
pub type GPIO18_CTRL = crate::Reg<gpio18_ctrl::GPIO18_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio18_ctrl;
#[doc = "GPIO19_STATUS register accessor: an alias for `Reg<GPIO19_STATUS_SPEC>`"]
pub type GPIO19_STATUS = crate::Reg<gpio19_status::GPIO19_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio19_status;
#[doc = "GPIO19_CTRL register accessor: an alias for `Reg<GPIO19_CTRL_SPEC>`"]
pub type GPIO19_CTRL = crate::Reg<gpio19_ctrl::GPIO19_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio19_ctrl;
#[doc = "GPIO20_STATUS register accessor: an alias for `Reg<GPIO20_STATUS_SPEC>`"]
pub type GPIO20_STATUS = crate::Reg<gpio20_status::GPIO20_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio20_status;
#[doc = "GPIO20_CTRL register accessor: an alias for `Reg<GPIO20_CTRL_SPEC>`"]
pub type GPIO20_CTRL = crate::Reg<gpio20_ctrl::GPIO20_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio20_ctrl;
#[doc = "GPIO21_STATUS register accessor: an alias for `Reg<GPIO21_STATUS_SPEC>`"]
pub type GPIO21_STATUS = crate::Reg<gpio21_status::GPIO21_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio21_status;
#[doc = "GPIO21_CTRL register accessor: an alias for `Reg<GPIO21_CTRL_SPEC>`"]
pub type GPIO21_CTRL = crate::Reg<gpio21_ctrl::GPIO21_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio21_ctrl;
#[doc = "GPIO22_STATUS register accessor: an alias for `Reg<GPIO22_STATUS_SPEC>`"]
pub type GPIO22_STATUS = crate::Reg<gpio22_status::GPIO22_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio22_status;
#[doc = "GPIO22_CTRL register accessor: an alias for `Reg<GPIO22_CTRL_SPEC>`"]
pub type GPIO22_CTRL = crate::Reg<gpio22_ctrl::GPIO22_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio22_ctrl;
#[doc = "GPIO23_STATUS register accessor: an alias for `Reg<GPIO23_STATUS_SPEC>`"]
pub type GPIO23_STATUS = crate::Reg<gpio23_status::GPIO23_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio23_status;
#[doc = "GPIO23_CTRL register accessor: an alias for `Reg<GPIO23_CTRL_SPEC>`"]
pub type GPIO23_CTRL = crate::Reg<gpio23_ctrl::GPIO23_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio23_ctrl;
#[doc = "GPIO24_STATUS register accessor: an alias for `Reg<GPIO24_STATUS_SPEC>`"]
pub type GPIO24_STATUS = crate::Reg<gpio24_status::GPIO24_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio24_status;
#[doc = "GPIO24_CTRL register accessor: an alias for `Reg<GPIO24_CTRL_SPEC>`"]
pub type GPIO24_CTRL = crate::Reg<gpio24_ctrl::GPIO24_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio24_ctrl;
#[doc = "GPIO25_STATUS register accessor: an alias for `Reg<GPIO25_STATUS_SPEC>`"]
pub type GPIO25_STATUS = crate::Reg<gpio25_status::GPIO25_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio25_status;
#[doc = "GPIO25_CTRL register accessor: an alias for `Reg<GPIO25_CTRL_SPEC>`"]
pub type GPIO25_CTRL = crate::Reg<gpio25_ctrl::GPIO25_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio25_ctrl;
#[doc = "GPIO26_STATUS register accessor: an alias for `Reg<GPIO26_STATUS_SPEC>`"]
pub type GPIO26_STATUS = crate::Reg<gpio26_status::GPIO26_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio26_status;
#[doc = "GPIO26_CTRL register accessor: an alias for `Reg<GPIO26_CTRL_SPEC>`"]
pub type GPIO26_CTRL = crate::Reg<gpio26_ctrl::GPIO26_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio26_ctrl;
#[doc = "GPIO27_STATUS register accessor: an alias for `Reg<GPIO27_STATUS_SPEC>`"]
pub type GPIO27_STATUS = crate::Reg<gpio27_status::GPIO27_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio27_status;
#[doc = "GPIO27_CTRL register accessor: an alias for `Reg<GPIO27_CTRL_SPEC>`"]
pub type GPIO27_CTRL = crate::Reg<gpio27_ctrl::GPIO27_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio27_ctrl;
#[doc = "GPIO28_STATUS register accessor: an alias for `Reg<GPIO28_STATUS_SPEC>`"]
pub type GPIO28_STATUS = crate::Reg<gpio28_status::GPIO28_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio28_status;
#[doc = "GPIO28_CTRL register accessor: an alias for `Reg<GPIO28_CTRL_SPEC>`"]
pub type GPIO28_CTRL = crate::Reg<gpio28_ctrl::GPIO28_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio28_ctrl;
#[doc = "GPIO29_STATUS register accessor: an alias for `Reg<GPIO29_STATUS_SPEC>`"]
pub type GPIO29_STATUS = crate::Reg<gpio29_status::GPIO29_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio29_status;
#[doc = "GPIO29_CTRL register accessor: an alias for `Reg<GPIO29_CTRL_SPEC>`"]
pub type GPIO29_CTRL = crate::Reg<gpio29_ctrl::GPIO29_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio29_ctrl;
#[doc = "INTR0 register accessor: an alias for `Reg<INTR0_SPEC>`"]
pub type INTR0 = crate::Reg<intr0::INTR0_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr0;
#[doc = "INTR1 register accessor: an alias for `Reg<INTR1_SPEC>`"]
pub type INTR1 = crate::Reg<intr1::INTR1_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr1;
#[doc = "INTR2 register accessor: an alias for `Reg<INTR2_SPEC>`"]
pub type INTR2 = crate::Reg<intr2::INTR2_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr2;
#[doc = "INTR3 register accessor: an alias for `Reg<INTR3_SPEC>`"]
pub type INTR3 = crate::Reg<intr3::INTR3_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr3;
#[doc = "PROC0_INTE0 register accessor: an alias for `Reg<PROC0_INTE0_SPEC>`"]
pub type PROC0_INTE0 = crate::Reg<proc0_inte0::PROC0_INTE0_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte0;
#[doc = "PROC0_INTE1 register accessor: an alias for `Reg<PROC0_INTE1_SPEC>`"]
pub type PROC0_INTE1 = crate::Reg<proc0_inte1::PROC0_INTE1_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte1;
#[doc = "PROC0_INTE2 register accessor: an alias for `Reg<PROC0_INTE2_SPEC>`"]
pub type PROC0_INTE2 = crate::Reg<proc0_inte2::PROC0_INTE2_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte2;
#[doc = "PROC0_INTE3 register accessor: an alias for `Reg<PROC0_INTE3_SPEC>`"]
pub type PROC0_INTE3 = crate::Reg<proc0_inte3::PROC0_INTE3_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte3;
#[doc = "PROC0_INTF0 register accessor: an alias for `Reg<PROC0_INTF0_SPEC>`"]
pub type PROC0_INTF0 = crate::Reg<proc0_intf0::PROC0_INTF0_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf0;
#[doc = "PROC0_INTF1 register accessor: an alias for `Reg<PROC0_INTF1_SPEC>`"]
pub type PROC0_INTF1 = crate::Reg<proc0_intf1::PROC0_INTF1_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf1;
#[doc = "PROC0_INTF2 register accessor: an alias for `Reg<PROC0_INTF2_SPEC>`"]
pub type PROC0_INTF2 = crate::Reg<proc0_intf2::PROC0_INTF2_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf2;
#[doc = "PROC0_INTF3 register accessor: an alias for `Reg<PROC0_INTF3_SPEC>`"]
pub type PROC0_INTF3 = crate::Reg<proc0_intf3::PROC0_INTF3_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf3;
#[doc = "PROC0_INTS0 register accessor: an alias for `Reg<PROC0_INTS0_SPEC>`"]
pub type PROC0_INTS0 = crate::Reg<proc0_ints0::PROC0_INTS0_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints0;
#[doc = "PROC0_INTS1 register accessor: an alias for `Reg<PROC0_INTS1_SPEC>`"]
pub type PROC0_INTS1 = crate::Reg<proc0_ints1::PROC0_INTS1_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints1;
#[doc = "PROC0_INTS2 register accessor: an alias for `Reg<PROC0_INTS2_SPEC>`"]
pub type PROC0_INTS2 = crate::Reg<proc0_ints2::PROC0_INTS2_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints2;
#[doc = "PROC0_INTS3 register accessor: an alias for `Reg<PROC0_INTS3_SPEC>`"]
pub type PROC0_INTS3 = crate::Reg<proc0_ints3::PROC0_INTS3_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints3;
#[doc = "PROC1_INTE0 register accessor: an alias for `Reg<PROC1_INTE0_SPEC>`"]
pub type PROC1_INTE0 = crate::Reg<proc1_inte0::PROC1_INTE0_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte0;
#[doc = "PROC1_INTE1 register accessor: an alias for `Reg<PROC1_INTE1_SPEC>`"]
pub type PROC1_INTE1 = crate::Reg<proc1_inte1::PROC1_INTE1_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte1;
#[doc = "PROC1_INTE2 register accessor: an alias for `Reg<PROC1_INTE2_SPEC>`"]
pub type PROC1_INTE2 = crate::Reg<proc1_inte2::PROC1_INTE2_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte2;
#[doc = "PROC1_INTE3 register accessor: an alias for `Reg<PROC1_INTE3_SPEC>`"]
pub type PROC1_INTE3 = crate::Reg<proc1_inte3::PROC1_INTE3_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte3;
#[doc = "PROC1_INTF0 register accessor: an alias for `Reg<PROC1_INTF0_SPEC>`"]
pub type PROC1_INTF0 = crate::Reg<proc1_intf0::PROC1_INTF0_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf0;
#[doc = "PROC1_INTF1 register accessor: an alias for `Reg<PROC1_INTF1_SPEC>`"]
pub type PROC1_INTF1 = crate::Reg<proc1_intf1::PROC1_INTF1_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf1;
#[doc = "PROC1_INTF2 register accessor: an alias for `Reg<PROC1_INTF2_SPEC>`"]
pub type PROC1_INTF2 = crate::Reg<proc1_intf2::PROC1_INTF2_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf2;
#[doc = "PROC1_INTF3 register accessor: an alias for `Reg<PROC1_INTF3_SPEC>`"]
pub type PROC1_INTF3 = crate::Reg<proc1_intf3::PROC1_INTF3_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf3;
#[doc = "PROC1_INTS0 register accessor: an alias for `Reg<PROC1_INTS0_SPEC>`"]
pub type PROC1_INTS0 = crate::Reg<proc1_ints0::PROC1_INTS0_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints0;
#[doc = "PROC1_INTS1 register accessor: an alias for `Reg<PROC1_INTS1_SPEC>`"]
pub type PROC1_INTS1 = crate::Reg<proc1_ints1::PROC1_INTS1_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints1;
#[doc = "PROC1_INTS2 register accessor: an alias for `Reg<PROC1_INTS2_SPEC>`"]
pub type PROC1_INTS2 = crate::Reg<proc1_ints2::PROC1_INTS2_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints2;
#[doc = "PROC1_INTS3 register accessor: an alias for `Reg<PROC1_INTS3_SPEC>`"]
pub type PROC1_INTS3 = crate::Reg<proc1_ints3::PROC1_INTS3_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints3;
#[doc = "DORMANT_WAKE_INTE0 register accessor: an alias for `Reg<DORMANT_WAKE_INTE0_SPEC>`"]
pub type DORMANT_WAKE_INTE0 = crate::Reg<dormant_wake_inte0::DORMANT_WAKE_INTE0_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte0;
#[doc = "DORMANT_WAKE_INTE1 register accessor: an alias for `Reg<DORMANT_WAKE_INTE1_SPEC>`"]
pub type DORMANT_WAKE_INTE1 = crate::Reg<dormant_wake_inte1::DORMANT_WAKE_INTE1_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte1;
#[doc = "DORMANT_WAKE_INTE2 register accessor: an alias for `Reg<DORMANT_WAKE_INTE2_SPEC>`"]
pub type DORMANT_WAKE_INTE2 = crate::Reg<dormant_wake_inte2::DORMANT_WAKE_INTE2_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte2;
#[doc = "DORMANT_WAKE_INTE3 register accessor: an alias for `Reg<DORMANT_WAKE_INTE3_SPEC>`"]
pub type DORMANT_WAKE_INTE3 = crate::Reg<dormant_wake_inte3::DORMANT_WAKE_INTE3_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte3;
#[doc = "DORMANT_WAKE_INTF0 register accessor: an alias for `Reg<DORMANT_WAKE_INTF0_SPEC>`"]
pub type DORMANT_WAKE_INTF0 = crate::Reg<dormant_wake_intf0::DORMANT_WAKE_INTF0_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf0;
#[doc = "DORMANT_WAKE_INTF1 register accessor: an alias for `Reg<DORMANT_WAKE_INTF1_SPEC>`"]
pub type DORMANT_WAKE_INTF1 = crate::Reg<dormant_wake_intf1::DORMANT_WAKE_INTF1_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf1;
#[doc = "DORMANT_WAKE_INTF2 register accessor: an alias for `Reg<DORMANT_WAKE_INTF2_SPEC>`"]
pub type DORMANT_WAKE_INTF2 = crate::Reg<dormant_wake_intf2::DORMANT_WAKE_INTF2_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf2;
#[doc = "DORMANT_WAKE_INTF3 register accessor: an alias for `Reg<DORMANT_WAKE_INTF3_SPEC>`"]
pub type DORMANT_WAKE_INTF3 = crate::Reg<dormant_wake_intf3::DORMANT_WAKE_INTF3_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf3;
#[doc = "DORMANT_WAKE_INTS0 register accessor: an alias for `Reg<DORMANT_WAKE_INTS0_SPEC>`"]
pub type DORMANT_WAKE_INTS0 = crate::Reg<dormant_wake_ints0::DORMANT_WAKE_INTS0_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints0;
#[doc = "DORMANT_WAKE_INTS1 register accessor: an alias for `Reg<DORMANT_WAKE_INTS1_SPEC>`"]
pub type DORMANT_WAKE_INTS1 = crate::Reg<dormant_wake_ints1::DORMANT_WAKE_INTS1_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints1;
#[doc = "DORMANT_WAKE_INTS2 register accessor: an alias for `Reg<DORMANT_WAKE_INTS2_SPEC>`"]
pub type DORMANT_WAKE_INTS2 = crate::Reg<dormant_wake_ints2::DORMANT_WAKE_INTS2_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints2;
#[doc = "DORMANT_WAKE_INTS3 register accessor: an alias for `Reg<DORMANT_WAKE_INTS3_SPEC>`"]
pub type DORMANT_WAKE_INTS3 = crate::Reg<dormant_wake_ints3::DORMANT_WAKE_INTS3_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints3;
