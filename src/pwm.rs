#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and status register"]
    pub ch0_csr: crate::Reg<ch0_csr::CH0_CSR_SPEC>,
    #[doc = "0x04 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch0_div: crate::Reg<ch0_div::CH0_DIV_SPEC>,
    #[doc = "0x08 - Direct access to the PWM counter"]
    pub ch0_ctr: crate::Reg<ch0_ctr::CH0_CTR_SPEC>,
    #[doc = "0x0c - Counter compare values"]
    pub ch0_cc: crate::Reg<ch0_cc::CH0_CC_SPEC>,
    #[doc = "0x10 - Counter wrap value"]
    pub ch0_top: crate::Reg<ch0_top::CH0_TOP_SPEC>,
    #[doc = "0x14 - Control and status register"]
    pub ch1_csr: crate::Reg<ch1_csr::CH1_CSR_SPEC>,
    #[doc = "0x18 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch1_div: crate::Reg<ch1_div::CH1_DIV_SPEC>,
    #[doc = "0x1c - Direct access to the PWM counter"]
    pub ch1_ctr: crate::Reg<ch1_ctr::CH1_CTR_SPEC>,
    #[doc = "0x20 - Counter compare values"]
    pub ch1_cc: crate::Reg<ch1_cc::CH1_CC_SPEC>,
    #[doc = "0x24 - Counter wrap value"]
    pub ch1_top: crate::Reg<ch1_top::CH1_TOP_SPEC>,
    #[doc = "0x28 - Control and status register"]
    pub ch2_csr: crate::Reg<ch2_csr::CH2_CSR_SPEC>,
    #[doc = "0x2c - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch2_div: crate::Reg<ch2_div::CH2_DIV_SPEC>,
    #[doc = "0x30 - Direct access to the PWM counter"]
    pub ch2_ctr: crate::Reg<ch2_ctr::CH2_CTR_SPEC>,
    #[doc = "0x34 - Counter compare values"]
    pub ch2_cc: crate::Reg<ch2_cc::CH2_CC_SPEC>,
    #[doc = "0x38 - Counter wrap value"]
    pub ch2_top: crate::Reg<ch2_top::CH2_TOP_SPEC>,
    #[doc = "0x3c - Control and status register"]
    pub ch3_csr: crate::Reg<ch3_csr::CH3_CSR_SPEC>,
    #[doc = "0x40 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch3_div: crate::Reg<ch3_div::CH3_DIV_SPEC>,
    #[doc = "0x44 - Direct access to the PWM counter"]
    pub ch3_ctr: crate::Reg<ch3_ctr::CH3_CTR_SPEC>,
    #[doc = "0x48 - Counter compare values"]
    pub ch3_cc: crate::Reg<ch3_cc::CH3_CC_SPEC>,
    #[doc = "0x4c - Counter wrap value"]
    pub ch3_top: crate::Reg<ch3_top::CH3_TOP_SPEC>,
    #[doc = "0x50 - Control and status register"]
    pub ch4_csr: crate::Reg<ch4_csr::CH4_CSR_SPEC>,
    #[doc = "0x54 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch4_div: crate::Reg<ch4_div::CH4_DIV_SPEC>,
    #[doc = "0x58 - Direct access to the PWM counter"]
    pub ch4_ctr: crate::Reg<ch4_ctr::CH4_CTR_SPEC>,
    #[doc = "0x5c - Counter compare values"]
    pub ch4_cc: crate::Reg<ch4_cc::CH4_CC_SPEC>,
    #[doc = "0x60 - Counter wrap value"]
    pub ch4_top: crate::Reg<ch4_top::CH4_TOP_SPEC>,
    #[doc = "0x64 - Control and status register"]
    pub ch5_csr: crate::Reg<ch5_csr::CH5_CSR_SPEC>,
    #[doc = "0x68 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch5_div: crate::Reg<ch5_div::CH5_DIV_SPEC>,
    #[doc = "0x6c - Direct access to the PWM counter"]
    pub ch5_ctr: crate::Reg<ch5_ctr::CH5_CTR_SPEC>,
    #[doc = "0x70 - Counter compare values"]
    pub ch5_cc: crate::Reg<ch5_cc::CH5_CC_SPEC>,
    #[doc = "0x74 - Counter wrap value"]
    pub ch5_top: crate::Reg<ch5_top::CH5_TOP_SPEC>,
    #[doc = "0x78 - Control and status register"]
    pub ch6_csr: crate::Reg<ch6_csr::CH6_CSR_SPEC>,
    #[doc = "0x7c - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch6_div: crate::Reg<ch6_div::CH6_DIV_SPEC>,
    #[doc = "0x80 - Direct access to the PWM counter"]
    pub ch6_ctr: crate::Reg<ch6_ctr::CH6_CTR_SPEC>,
    #[doc = "0x84 - Counter compare values"]
    pub ch6_cc: crate::Reg<ch6_cc::CH6_CC_SPEC>,
    #[doc = "0x88 - Counter wrap value"]
    pub ch6_top: crate::Reg<ch6_top::CH6_TOP_SPEC>,
    #[doc = "0x8c - Control and status register"]
    pub ch7_csr: crate::Reg<ch7_csr::CH7_CSR_SPEC>,
    #[doc = "0x90 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch7_div: crate::Reg<ch7_div::CH7_DIV_SPEC>,
    #[doc = "0x94 - Direct access to the PWM counter"]
    pub ch7_ctr: crate::Reg<ch7_ctr::CH7_CTR_SPEC>,
    #[doc = "0x98 - Counter compare values"]
    pub ch7_cc: crate::Reg<ch7_cc::CH7_CC_SPEC>,
    #[doc = "0x9c - Counter wrap value"]
    pub ch7_top: crate::Reg<ch7_top::CH7_TOP_SPEC>,
    #[doc = "0xa0 - This register aliases the CSR_EN bits for all channels.\\n Writing to this register allows multiple channels to be enabled\\n or disabled simultaneously, so they can run in perfect sync.\\n For each channel, there is only one physical EN register bit,\\n which can be accessed through here or CHx_CSR."]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0xa4 - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0xa8 - Interrupt Enable"]
    pub inte: crate::Reg<inte::INTE_SPEC>,
    #[doc = "0xac - Interrupt Force"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0xb0 - Interrupt status after masking & forcing"]
    pub ints: crate::Reg<ints::INTS_SPEC>,
}
#[doc = "CH0_CSR register accessor: an alias for `Reg<CH0_CSR_SPEC>`"]
pub type CH0_CSR = crate::Reg<ch0_csr::CH0_CSR_SPEC>;
#[doc = "Control and status register"]
pub mod ch0_csr;
#[doc = "CH0_DIV register accessor: an alias for `Reg<CH0_DIV_SPEC>`"]
pub type CH0_DIV = crate::Reg<ch0_div::CH0_DIV_SPEC>;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch0_div;
#[doc = "CH0_CTR register accessor: an alias for `Reg<CH0_CTR_SPEC>`"]
pub type CH0_CTR = crate::Reg<ch0_ctr::CH0_CTR_SPEC>;
#[doc = "Direct access to the PWM counter"]
pub mod ch0_ctr;
#[doc = "CH0_CC register accessor: an alias for `Reg<CH0_CC_SPEC>`"]
pub type CH0_CC = crate::Reg<ch0_cc::CH0_CC_SPEC>;
#[doc = "Counter compare values"]
pub mod ch0_cc;
#[doc = "CH0_TOP register accessor: an alias for `Reg<CH0_TOP_SPEC>`"]
pub type CH0_TOP = crate::Reg<ch0_top::CH0_TOP_SPEC>;
#[doc = "Counter wrap value"]
pub mod ch0_top;
#[doc = "CH1_CSR register accessor: an alias for `Reg<CH1_CSR_SPEC>`"]
pub type CH1_CSR = crate::Reg<ch1_csr::CH1_CSR_SPEC>;
#[doc = "Control and status register"]
pub mod ch1_csr;
#[doc = "CH1_DIV register accessor: an alias for `Reg<CH1_DIV_SPEC>`"]
pub type CH1_DIV = crate::Reg<ch1_div::CH1_DIV_SPEC>;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch1_div;
#[doc = "CH1_CTR register accessor: an alias for `Reg<CH1_CTR_SPEC>`"]
pub type CH1_CTR = crate::Reg<ch1_ctr::CH1_CTR_SPEC>;
#[doc = "Direct access to the PWM counter"]
pub mod ch1_ctr;
#[doc = "CH1_CC register accessor: an alias for `Reg<CH1_CC_SPEC>`"]
pub type CH1_CC = crate::Reg<ch1_cc::CH1_CC_SPEC>;
#[doc = "Counter compare values"]
pub mod ch1_cc;
#[doc = "CH1_TOP register accessor: an alias for `Reg<CH1_TOP_SPEC>`"]
pub type CH1_TOP = crate::Reg<ch1_top::CH1_TOP_SPEC>;
#[doc = "Counter wrap value"]
pub mod ch1_top;
#[doc = "CH2_CSR register accessor: an alias for `Reg<CH2_CSR_SPEC>`"]
pub type CH2_CSR = crate::Reg<ch2_csr::CH2_CSR_SPEC>;
#[doc = "Control and status register"]
pub mod ch2_csr;
#[doc = "CH2_DIV register accessor: an alias for `Reg<CH2_DIV_SPEC>`"]
pub type CH2_DIV = crate::Reg<ch2_div::CH2_DIV_SPEC>;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch2_div;
#[doc = "CH2_CTR register accessor: an alias for `Reg<CH2_CTR_SPEC>`"]
pub type CH2_CTR = crate::Reg<ch2_ctr::CH2_CTR_SPEC>;
#[doc = "Direct access to the PWM counter"]
pub mod ch2_ctr;
#[doc = "CH2_CC register accessor: an alias for `Reg<CH2_CC_SPEC>`"]
pub type CH2_CC = crate::Reg<ch2_cc::CH2_CC_SPEC>;
#[doc = "Counter compare values"]
pub mod ch2_cc;
#[doc = "CH2_TOP register accessor: an alias for `Reg<CH2_TOP_SPEC>`"]
pub type CH2_TOP = crate::Reg<ch2_top::CH2_TOP_SPEC>;
#[doc = "Counter wrap value"]
pub mod ch2_top;
#[doc = "CH3_CSR register accessor: an alias for `Reg<CH3_CSR_SPEC>`"]
pub type CH3_CSR = crate::Reg<ch3_csr::CH3_CSR_SPEC>;
#[doc = "Control and status register"]
pub mod ch3_csr;
#[doc = "CH3_DIV register accessor: an alias for `Reg<CH3_DIV_SPEC>`"]
pub type CH3_DIV = crate::Reg<ch3_div::CH3_DIV_SPEC>;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch3_div;
#[doc = "CH3_CTR register accessor: an alias for `Reg<CH3_CTR_SPEC>`"]
pub type CH3_CTR = crate::Reg<ch3_ctr::CH3_CTR_SPEC>;
#[doc = "Direct access to the PWM counter"]
pub mod ch3_ctr;
#[doc = "CH3_CC register accessor: an alias for `Reg<CH3_CC_SPEC>`"]
pub type CH3_CC = crate::Reg<ch3_cc::CH3_CC_SPEC>;
#[doc = "Counter compare values"]
pub mod ch3_cc;
#[doc = "CH3_TOP register accessor: an alias for `Reg<CH3_TOP_SPEC>`"]
pub type CH3_TOP = crate::Reg<ch3_top::CH3_TOP_SPEC>;
#[doc = "Counter wrap value"]
pub mod ch3_top;
#[doc = "CH4_CSR register accessor: an alias for `Reg<CH4_CSR_SPEC>`"]
pub type CH4_CSR = crate::Reg<ch4_csr::CH4_CSR_SPEC>;
#[doc = "Control and status register"]
pub mod ch4_csr;
#[doc = "CH4_DIV register accessor: an alias for `Reg<CH4_DIV_SPEC>`"]
pub type CH4_DIV = crate::Reg<ch4_div::CH4_DIV_SPEC>;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch4_div;
#[doc = "CH4_CTR register accessor: an alias for `Reg<CH4_CTR_SPEC>`"]
pub type CH4_CTR = crate::Reg<ch4_ctr::CH4_CTR_SPEC>;
#[doc = "Direct access to the PWM counter"]
pub mod ch4_ctr;
#[doc = "CH4_CC register accessor: an alias for `Reg<CH4_CC_SPEC>`"]
pub type CH4_CC = crate::Reg<ch4_cc::CH4_CC_SPEC>;
#[doc = "Counter compare values"]
pub mod ch4_cc;
#[doc = "CH4_TOP register accessor: an alias for `Reg<CH4_TOP_SPEC>`"]
pub type CH4_TOP = crate::Reg<ch4_top::CH4_TOP_SPEC>;
#[doc = "Counter wrap value"]
pub mod ch4_top;
#[doc = "CH5_CSR register accessor: an alias for `Reg<CH5_CSR_SPEC>`"]
pub type CH5_CSR = crate::Reg<ch5_csr::CH5_CSR_SPEC>;
#[doc = "Control and status register"]
pub mod ch5_csr;
#[doc = "CH5_DIV register accessor: an alias for `Reg<CH5_DIV_SPEC>`"]
pub type CH5_DIV = crate::Reg<ch5_div::CH5_DIV_SPEC>;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch5_div;
#[doc = "CH5_CTR register accessor: an alias for `Reg<CH5_CTR_SPEC>`"]
pub type CH5_CTR = crate::Reg<ch5_ctr::CH5_CTR_SPEC>;
#[doc = "Direct access to the PWM counter"]
pub mod ch5_ctr;
#[doc = "CH5_CC register accessor: an alias for `Reg<CH5_CC_SPEC>`"]
pub type CH5_CC = crate::Reg<ch5_cc::CH5_CC_SPEC>;
#[doc = "Counter compare values"]
pub mod ch5_cc;
#[doc = "CH5_TOP register accessor: an alias for `Reg<CH5_TOP_SPEC>`"]
pub type CH5_TOP = crate::Reg<ch5_top::CH5_TOP_SPEC>;
#[doc = "Counter wrap value"]
pub mod ch5_top;
#[doc = "CH6_CSR register accessor: an alias for `Reg<CH6_CSR_SPEC>`"]
pub type CH6_CSR = crate::Reg<ch6_csr::CH6_CSR_SPEC>;
#[doc = "Control and status register"]
pub mod ch6_csr;
#[doc = "CH6_DIV register accessor: an alias for `Reg<CH6_DIV_SPEC>`"]
pub type CH6_DIV = crate::Reg<ch6_div::CH6_DIV_SPEC>;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch6_div;
#[doc = "CH6_CTR register accessor: an alias for `Reg<CH6_CTR_SPEC>`"]
pub type CH6_CTR = crate::Reg<ch6_ctr::CH6_CTR_SPEC>;
#[doc = "Direct access to the PWM counter"]
pub mod ch6_ctr;
#[doc = "CH6_CC register accessor: an alias for `Reg<CH6_CC_SPEC>`"]
pub type CH6_CC = crate::Reg<ch6_cc::CH6_CC_SPEC>;
#[doc = "Counter compare values"]
pub mod ch6_cc;
#[doc = "CH6_TOP register accessor: an alias for `Reg<CH6_TOP_SPEC>`"]
pub type CH6_TOP = crate::Reg<ch6_top::CH6_TOP_SPEC>;
#[doc = "Counter wrap value"]
pub mod ch6_top;
#[doc = "CH7_CSR register accessor: an alias for `Reg<CH7_CSR_SPEC>`"]
pub type CH7_CSR = crate::Reg<ch7_csr::CH7_CSR_SPEC>;
#[doc = "Control and status register"]
pub mod ch7_csr;
#[doc = "CH7_DIV register accessor: an alias for `Reg<CH7_DIV_SPEC>`"]
pub type CH7_DIV = crate::Reg<ch7_div::CH7_DIV_SPEC>;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch7_div;
#[doc = "CH7_CTR register accessor: an alias for `Reg<CH7_CTR_SPEC>`"]
pub type CH7_CTR = crate::Reg<ch7_ctr::CH7_CTR_SPEC>;
#[doc = "Direct access to the PWM counter"]
pub mod ch7_ctr;
#[doc = "CH7_CC register accessor: an alias for `Reg<CH7_CC_SPEC>`"]
pub type CH7_CC = crate::Reg<ch7_cc::CH7_CC_SPEC>;
#[doc = "Counter compare values"]
pub mod ch7_cc;
#[doc = "CH7_TOP register accessor: an alias for `Reg<CH7_TOP_SPEC>`"]
pub type CH7_TOP = crate::Reg<ch7_top::CH7_TOP_SPEC>;
#[doc = "Counter wrap value"]
pub mod ch7_top;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "This register aliases the CSR_EN bits for all channels.\\n Writing to this register allows multiple channels to be enabled\\n or disabled simultaneously, so they can run in perfect sync.\\n For each channel, there is only one physical EN register bit,\\n which can be accessed through here or CHx_CSR."]
pub mod en;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE register accessor: an alias for `Reg<INTE_SPEC>`"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS register accessor: an alias for `Reg<INTS_SPEC>`"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing"]
pub mod ints;
