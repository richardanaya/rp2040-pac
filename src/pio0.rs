#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - FIFO status register"]
    pub fstat: crate::Reg<fstat::FSTAT_SPEC>,
    #[doc = "0x08 - FIFO debug register"]
    pub fdebug: crate::Reg<fdebug::FDEBUG_SPEC>,
    #[doc = "0x0c - FIFO levels"]
    pub flevel: crate::Reg<flevel::FLEVEL_SPEC>,
    #[doc = "0x10 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub txf0: crate::Reg<txf0::TXF0_SPEC>,
    #[doc = "0x14 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub txf1: crate::Reg<txf1::TXF1_SPEC>,
    #[doc = "0x18 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub txf2: crate::Reg<txf2::TXF2_SPEC>,
    #[doc = "0x1c - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub txf3: crate::Reg<txf3::TXF3_SPEC>,
    #[doc = "0x20 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub rxf0: crate::Reg<rxf0::RXF0_SPEC>,
    #[doc = "0x24 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub rxf1: crate::Reg<rxf1::RXF1_SPEC>,
    #[doc = "0x28 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub rxf2: crate::Reg<rxf2::RXF2_SPEC>,
    #[doc = "0x2c - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub rxf3: crate::Reg<rxf3::RXF3_SPEC>,
    #[doc = "0x30 - Interrupt request register. Write 1 to clear"]
    pub irq: crate::Reg<irq::IRQ_SPEC>,
    #[doc = "0x34 - Writing a 1 to each of these bits will forcibly assert the corresponding IRQ.\\n Note this is different to the INTF register: writing here affects PIO internal\\n state. INTF just asserts the processor-facing IRQ signal for testing ISRs,\\n and is not visible to the state machines."]
    pub irq_force: crate::Reg<irq_force::IRQ_FORCE_SPEC>,
    #[doc = "0x38 - There is a 2-flipflop synchronizer on each GPIO input, which protects\\n PIO logic from metastabilities. This increases input delay, and for fast\\n synchronous IO (e.g. SPI) these synchronizers may need to be bypassed.\\n Each bit in this register corresponds to one GPIO.\\n 0 -> input is synchronized (default)\\n 1 -> synchronizer is bypassed\\n If in doubt, leave this register as all zeroes."]
    pub input_sync_bypass: crate::Reg<input_sync_bypass::INPUT_SYNC_BYPASS_SPEC>,
    #[doc = "0x3c - Read to sample the pad output values PIO is currently driving to the GPIOs."]
    pub dbg_padout: crate::Reg<dbg_padout::DBG_PADOUT_SPEC>,
    #[doc = "0x40 - Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs."]
    pub dbg_padoe: crate::Reg<dbg_padoe::DBG_PADOE_SPEC>,
    #[doc = "0x44 - The PIO hardware has some free parameters that may vary between chip products.\\n These should be provided in the chip datasheet, but are also exposed here."]
    pub dbg_cfginfo: crate::Reg<dbg_cfginfo::DBG_CFGINFO_SPEC>,
    #[doc = "0x48 - Write-only access to instruction memory location 0"]
    pub instr_mem0: crate::Reg<instr_mem0::INSTR_MEM0_SPEC>,
    #[doc = "0x4c - Write-only access to instruction memory location 1"]
    pub instr_mem1: crate::Reg<instr_mem1::INSTR_MEM1_SPEC>,
    #[doc = "0x50 - Write-only access to instruction memory location 2"]
    pub instr_mem2: crate::Reg<instr_mem2::INSTR_MEM2_SPEC>,
    #[doc = "0x54 - Write-only access to instruction memory location 3"]
    pub instr_mem3: crate::Reg<instr_mem3::INSTR_MEM3_SPEC>,
    #[doc = "0x58 - Write-only access to instruction memory location 4"]
    pub instr_mem4: crate::Reg<instr_mem4::INSTR_MEM4_SPEC>,
    #[doc = "0x5c - Write-only access to instruction memory location 5"]
    pub instr_mem5: crate::Reg<instr_mem5::INSTR_MEM5_SPEC>,
    #[doc = "0x60 - Write-only access to instruction memory location 6"]
    pub instr_mem6: crate::Reg<instr_mem6::INSTR_MEM6_SPEC>,
    #[doc = "0x64 - Write-only access to instruction memory location 7"]
    pub instr_mem7: crate::Reg<instr_mem7::INSTR_MEM7_SPEC>,
    #[doc = "0x68 - Write-only access to instruction memory location 8"]
    pub instr_mem8: crate::Reg<instr_mem8::INSTR_MEM8_SPEC>,
    #[doc = "0x6c - Write-only access to instruction memory location 9"]
    pub instr_mem9: crate::Reg<instr_mem9::INSTR_MEM9_SPEC>,
    #[doc = "0x70 - Write-only access to instruction memory location 10"]
    pub instr_mem10: crate::Reg<instr_mem10::INSTR_MEM10_SPEC>,
    #[doc = "0x74 - Write-only access to instruction memory location 11"]
    pub instr_mem11: crate::Reg<instr_mem11::INSTR_MEM11_SPEC>,
    #[doc = "0x78 - Write-only access to instruction memory location 12"]
    pub instr_mem12: crate::Reg<instr_mem12::INSTR_MEM12_SPEC>,
    #[doc = "0x7c - Write-only access to instruction memory location 13"]
    pub instr_mem13: crate::Reg<instr_mem13::INSTR_MEM13_SPEC>,
    #[doc = "0x80 - Write-only access to instruction memory location 14"]
    pub instr_mem14: crate::Reg<instr_mem14::INSTR_MEM14_SPEC>,
    #[doc = "0x84 - Write-only access to instruction memory location 15"]
    pub instr_mem15: crate::Reg<instr_mem15::INSTR_MEM15_SPEC>,
    #[doc = "0x88 - Write-only access to instruction memory location 16"]
    pub instr_mem16: crate::Reg<instr_mem16::INSTR_MEM16_SPEC>,
    #[doc = "0x8c - Write-only access to instruction memory location 17"]
    pub instr_mem17: crate::Reg<instr_mem17::INSTR_MEM17_SPEC>,
    #[doc = "0x90 - Write-only access to instruction memory location 18"]
    pub instr_mem18: crate::Reg<instr_mem18::INSTR_MEM18_SPEC>,
    #[doc = "0x94 - Write-only access to instruction memory location 19"]
    pub instr_mem19: crate::Reg<instr_mem19::INSTR_MEM19_SPEC>,
    #[doc = "0x98 - Write-only access to instruction memory location 20"]
    pub instr_mem20: crate::Reg<instr_mem20::INSTR_MEM20_SPEC>,
    #[doc = "0x9c - Write-only access to instruction memory location 21"]
    pub instr_mem21: crate::Reg<instr_mem21::INSTR_MEM21_SPEC>,
    #[doc = "0xa0 - Write-only access to instruction memory location 22"]
    pub instr_mem22: crate::Reg<instr_mem22::INSTR_MEM22_SPEC>,
    #[doc = "0xa4 - Write-only access to instruction memory location 23"]
    pub instr_mem23: crate::Reg<instr_mem23::INSTR_MEM23_SPEC>,
    #[doc = "0xa8 - Write-only access to instruction memory location 24"]
    pub instr_mem24: crate::Reg<instr_mem24::INSTR_MEM24_SPEC>,
    #[doc = "0xac - Write-only access to instruction memory location 25"]
    pub instr_mem25: crate::Reg<instr_mem25::INSTR_MEM25_SPEC>,
    #[doc = "0xb0 - Write-only access to instruction memory location 26"]
    pub instr_mem26: crate::Reg<instr_mem26::INSTR_MEM26_SPEC>,
    #[doc = "0xb4 - Write-only access to instruction memory location 27"]
    pub instr_mem27: crate::Reg<instr_mem27::INSTR_MEM27_SPEC>,
    #[doc = "0xb8 - Write-only access to instruction memory location 28"]
    pub instr_mem28: crate::Reg<instr_mem28::INSTR_MEM28_SPEC>,
    #[doc = "0xbc - Write-only access to instruction memory location 29"]
    pub instr_mem29: crate::Reg<instr_mem29::INSTR_MEM29_SPEC>,
    #[doc = "0xc0 - Write-only access to instruction memory location 30"]
    pub instr_mem30: crate::Reg<instr_mem30::INSTR_MEM30_SPEC>,
    #[doc = "0xc4 - Write-only access to instruction memory location 31"]
    pub instr_mem31: crate::Reg<instr_mem31::INSTR_MEM31_SPEC>,
    #[doc = "0xc8 - Clock divider register for state machine 0\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm0_clkdiv: crate::Reg<sm0_clkdiv::SM0_CLKDIV_SPEC>,
    #[doc = "0xcc - Execution/behavioural settings for state machine 0"]
    pub sm0_execctrl: crate::Reg<sm0_execctrl::SM0_EXECCTRL_SPEC>,
    #[doc = "0xd0 - Control behaviour of the input/output shift registers for state machine 0"]
    pub sm0_shiftctrl: crate::Reg<sm0_shiftctrl::SM0_SHIFTCTRL_SPEC>,
    #[doc = "0xd4 - Current instruction address of state machine 0"]
    pub sm0_addr: crate::Reg<sm0_addr::SM0_ADDR_SPEC>,
    #[doc = "0xd8 - Instruction currently being executed by state machine 0\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm0_instr: crate::Reg<sm0_instr::SM0_INSTR_SPEC>,
    #[doc = "0xdc - State machine pin control"]
    pub sm0_pinctrl: crate::Reg<sm0_pinctrl::SM0_PINCTRL_SPEC>,
    #[doc = "0xe0 - Clock divider register for state machine 1\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm1_clkdiv: crate::Reg<sm1_clkdiv::SM1_CLKDIV_SPEC>,
    #[doc = "0xe4 - Execution/behavioural settings for state machine 1"]
    pub sm1_execctrl: crate::Reg<sm1_execctrl::SM1_EXECCTRL_SPEC>,
    #[doc = "0xe8 - Control behaviour of the input/output shift registers for state machine 1"]
    pub sm1_shiftctrl: crate::Reg<sm1_shiftctrl::SM1_SHIFTCTRL_SPEC>,
    #[doc = "0xec - Current instruction address of state machine 1"]
    pub sm1_addr: crate::Reg<sm1_addr::SM1_ADDR_SPEC>,
    #[doc = "0xf0 - Instruction currently being executed by state machine 1\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm1_instr: crate::Reg<sm1_instr::SM1_INSTR_SPEC>,
    #[doc = "0xf4 - State machine pin control"]
    pub sm1_pinctrl: crate::Reg<sm1_pinctrl::SM1_PINCTRL_SPEC>,
    #[doc = "0xf8 - Clock divider register for state machine 2\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm2_clkdiv: crate::Reg<sm2_clkdiv::SM2_CLKDIV_SPEC>,
    #[doc = "0xfc - Execution/behavioural settings for state machine 2"]
    pub sm2_execctrl: crate::Reg<sm2_execctrl::SM2_EXECCTRL_SPEC>,
    #[doc = "0x100 - Control behaviour of the input/output shift registers for state machine 2"]
    pub sm2_shiftctrl: crate::Reg<sm2_shiftctrl::SM2_SHIFTCTRL_SPEC>,
    #[doc = "0x104 - Current instruction address of state machine 2"]
    pub sm2_addr: crate::Reg<sm2_addr::SM2_ADDR_SPEC>,
    #[doc = "0x108 - Instruction currently being executed by state machine 2\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm2_instr: crate::Reg<sm2_instr::SM2_INSTR_SPEC>,
    #[doc = "0x10c - State machine pin control"]
    pub sm2_pinctrl: crate::Reg<sm2_pinctrl::SM2_PINCTRL_SPEC>,
    #[doc = "0x110 - Clock divider register for state machine 3\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm3_clkdiv: crate::Reg<sm3_clkdiv::SM3_CLKDIV_SPEC>,
    #[doc = "0x114 - Execution/behavioural settings for state machine 3"]
    pub sm3_execctrl: crate::Reg<sm3_execctrl::SM3_EXECCTRL_SPEC>,
    #[doc = "0x118 - Control behaviour of the input/output shift registers for state machine 3"]
    pub sm3_shiftctrl: crate::Reg<sm3_shiftctrl::SM3_SHIFTCTRL_SPEC>,
    #[doc = "0x11c - Current instruction address of state machine 3"]
    pub sm3_addr: crate::Reg<sm3_addr::SM3_ADDR_SPEC>,
    #[doc = "0x120 - Instruction currently being executed by state machine 3\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm3_instr: crate::Reg<sm3_instr::SM3_INSTR_SPEC>,
    #[doc = "0x124 - State machine pin control"]
    pub sm3_pinctrl: crate::Reg<sm3_pinctrl::SM3_PINCTRL_SPEC>,
    #[doc = "0x128 - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x12c - Interrupt Enable for irq0"]
    pub irq0_inte: crate::Reg<irq0_inte::IRQ0_INTE_SPEC>,
    #[doc = "0x130 - Interrupt Force for irq0"]
    pub irq0_intf: crate::Reg<irq0_intf::IRQ0_INTF_SPEC>,
    #[doc = "0x134 - Interrupt status after masking & forcing for irq0"]
    pub irq0_ints: crate::Reg<irq0_ints::IRQ0_INTS_SPEC>,
    #[doc = "0x138 - Interrupt Enable for irq1"]
    pub irq1_inte: crate::Reg<irq1_inte::IRQ1_INTE_SPEC>,
    #[doc = "0x13c - Interrupt Force for irq1"]
    pub irq1_intf: crate::Reg<irq1_intf::IRQ1_INTF_SPEC>,
    #[doc = "0x140 - Interrupt status after masking & forcing for irq1"]
    pub irq1_ints: crate::Reg<irq1_ints::IRQ1_INTS_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "PIO control register"]
pub mod ctrl;
#[doc = "FSTAT register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "FIFO status register"]
pub mod fstat;
#[doc = "FDEBUG register accessor: an alias for `Reg<FDEBUG_SPEC>`"]
pub type FDEBUG = crate::Reg<fdebug::FDEBUG_SPEC>;
#[doc = "FIFO debug register"]
pub mod fdebug;
#[doc = "FLEVEL register accessor: an alias for `Reg<FLEVEL_SPEC>`"]
pub type FLEVEL = crate::Reg<flevel::FLEVEL_SPEC>;
#[doc = "FIFO levels"]
pub mod flevel;
#[doc = "TXF0 register accessor: an alias for `Reg<TXF0_SPEC>`"]
pub type TXF0 = crate::Reg<txf0::TXF0_SPEC>;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
pub mod txf0;
#[doc = "TXF1 register accessor: an alias for `Reg<TXF1_SPEC>`"]
pub type TXF1 = crate::Reg<txf1::TXF1_SPEC>;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
pub mod txf1;
#[doc = "TXF2 register accessor: an alias for `Reg<TXF2_SPEC>`"]
pub type TXF2 = crate::Reg<txf2::TXF2_SPEC>;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
pub mod txf2;
#[doc = "TXF3 register accessor: an alias for `Reg<TXF3_SPEC>`"]
pub type TXF3 = crate::Reg<txf3::TXF3_SPEC>;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
pub mod txf3;
#[doc = "RXF0 register accessor: an alias for `Reg<RXF0_SPEC>`"]
pub type RXF0 = crate::Reg<rxf0::RXF0_SPEC>;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
pub mod rxf0;
#[doc = "RXF1 register accessor: an alias for `Reg<RXF1_SPEC>`"]
pub type RXF1 = crate::Reg<rxf1::RXF1_SPEC>;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
pub mod rxf1;
#[doc = "RXF2 register accessor: an alias for `Reg<RXF2_SPEC>`"]
pub type RXF2 = crate::Reg<rxf2::RXF2_SPEC>;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
pub mod rxf2;
#[doc = "RXF3 register accessor: an alias for `Reg<RXF3_SPEC>`"]
pub type RXF3 = crate::Reg<rxf3::RXF3_SPEC>;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
pub mod rxf3;
#[doc = "IRQ register accessor: an alias for `Reg<IRQ_SPEC>`"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "Interrupt request register. Write 1 to clear"]
pub mod irq;
#[doc = "IRQ_FORCE register accessor: an alias for `Reg<IRQ_FORCE_SPEC>`"]
pub type IRQ_FORCE = crate::Reg<irq_force::IRQ_FORCE_SPEC>;
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ.\\n Note this is different to the INTF register: writing here affects PIO internal\\n state. INTF just asserts the processor-facing IRQ signal for testing ISRs,\\n and is not visible to the state machines."]
pub mod irq_force;
#[doc = "INPUT_SYNC_BYPASS register accessor: an alias for `Reg<INPUT_SYNC_BYPASS_SPEC>`"]
pub type INPUT_SYNC_BYPASS = crate::Reg<input_sync_bypass::INPUT_SYNC_BYPASS_SPEC>;
#[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects\\n PIO logic from metastabilities. This increases input delay, and for fast\\n synchronous IO (e.g. SPI) these synchronizers may need to be bypassed.\\n Each bit in this register corresponds to one GPIO.\\n 0 -> input is synchronized (default)\\n 1 -> synchronizer is bypassed\\n If in doubt, leave this register as all zeroes."]
pub mod input_sync_bypass;
#[doc = "DBG_PADOUT register accessor: an alias for `Reg<DBG_PADOUT_SPEC>`"]
pub type DBG_PADOUT = crate::Reg<dbg_padout::DBG_PADOUT_SPEC>;
#[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs."]
pub mod dbg_padout;
#[doc = "DBG_PADOE register accessor: an alias for `Reg<DBG_PADOE_SPEC>`"]
pub type DBG_PADOE = crate::Reg<dbg_padoe::DBG_PADOE_SPEC>;
#[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs."]
pub mod dbg_padoe;
#[doc = "DBG_CFGINFO register accessor: an alias for `Reg<DBG_CFGINFO_SPEC>`"]
pub type DBG_CFGINFO = crate::Reg<dbg_cfginfo::DBG_CFGINFO_SPEC>;
#[doc = "The PIO hardware has some free parameters that may vary between chip products.\\n These should be provided in the chip datasheet, but are also exposed here."]
pub mod dbg_cfginfo;
#[doc = "INSTR_MEM0 register accessor: an alias for `Reg<INSTR_MEM0_SPEC>`"]
pub type INSTR_MEM0 = crate::Reg<instr_mem0::INSTR_MEM0_SPEC>;
#[doc = "Write-only access to instruction memory location 0"]
pub mod instr_mem0;
#[doc = "INSTR_MEM1 register accessor: an alias for `Reg<INSTR_MEM1_SPEC>`"]
pub type INSTR_MEM1 = crate::Reg<instr_mem1::INSTR_MEM1_SPEC>;
#[doc = "Write-only access to instruction memory location 1"]
pub mod instr_mem1;
#[doc = "INSTR_MEM2 register accessor: an alias for `Reg<INSTR_MEM2_SPEC>`"]
pub type INSTR_MEM2 = crate::Reg<instr_mem2::INSTR_MEM2_SPEC>;
#[doc = "Write-only access to instruction memory location 2"]
pub mod instr_mem2;
#[doc = "INSTR_MEM3 register accessor: an alias for `Reg<INSTR_MEM3_SPEC>`"]
pub type INSTR_MEM3 = crate::Reg<instr_mem3::INSTR_MEM3_SPEC>;
#[doc = "Write-only access to instruction memory location 3"]
pub mod instr_mem3;
#[doc = "INSTR_MEM4 register accessor: an alias for `Reg<INSTR_MEM4_SPEC>`"]
pub type INSTR_MEM4 = crate::Reg<instr_mem4::INSTR_MEM4_SPEC>;
#[doc = "Write-only access to instruction memory location 4"]
pub mod instr_mem4;
#[doc = "INSTR_MEM5 register accessor: an alias for `Reg<INSTR_MEM5_SPEC>`"]
pub type INSTR_MEM5 = crate::Reg<instr_mem5::INSTR_MEM5_SPEC>;
#[doc = "Write-only access to instruction memory location 5"]
pub mod instr_mem5;
#[doc = "INSTR_MEM6 register accessor: an alias for `Reg<INSTR_MEM6_SPEC>`"]
pub type INSTR_MEM6 = crate::Reg<instr_mem6::INSTR_MEM6_SPEC>;
#[doc = "Write-only access to instruction memory location 6"]
pub mod instr_mem6;
#[doc = "INSTR_MEM7 register accessor: an alias for `Reg<INSTR_MEM7_SPEC>`"]
pub type INSTR_MEM7 = crate::Reg<instr_mem7::INSTR_MEM7_SPEC>;
#[doc = "Write-only access to instruction memory location 7"]
pub mod instr_mem7;
#[doc = "INSTR_MEM8 register accessor: an alias for `Reg<INSTR_MEM8_SPEC>`"]
pub type INSTR_MEM8 = crate::Reg<instr_mem8::INSTR_MEM8_SPEC>;
#[doc = "Write-only access to instruction memory location 8"]
pub mod instr_mem8;
#[doc = "INSTR_MEM9 register accessor: an alias for `Reg<INSTR_MEM9_SPEC>`"]
pub type INSTR_MEM9 = crate::Reg<instr_mem9::INSTR_MEM9_SPEC>;
#[doc = "Write-only access to instruction memory location 9"]
pub mod instr_mem9;
#[doc = "INSTR_MEM10 register accessor: an alias for `Reg<INSTR_MEM10_SPEC>`"]
pub type INSTR_MEM10 = crate::Reg<instr_mem10::INSTR_MEM10_SPEC>;
#[doc = "Write-only access to instruction memory location 10"]
pub mod instr_mem10;
#[doc = "INSTR_MEM11 register accessor: an alias for `Reg<INSTR_MEM11_SPEC>`"]
pub type INSTR_MEM11 = crate::Reg<instr_mem11::INSTR_MEM11_SPEC>;
#[doc = "Write-only access to instruction memory location 11"]
pub mod instr_mem11;
#[doc = "INSTR_MEM12 register accessor: an alias for `Reg<INSTR_MEM12_SPEC>`"]
pub type INSTR_MEM12 = crate::Reg<instr_mem12::INSTR_MEM12_SPEC>;
#[doc = "Write-only access to instruction memory location 12"]
pub mod instr_mem12;
#[doc = "INSTR_MEM13 register accessor: an alias for `Reg<INSTR_MEM13_SPEC>`"]
pub type INSTR_MEM13 = crate::Reg<instr_mem13::INSTR_MEM13_SPEC>;
#[doc = "Write-only access to instruction memory location 13"]
pub mod instr_mem13;
#[doc = "INSTR_MEM14 register accessor: an alias for `Reg<INSTR_MEM14_SPEC>`"]
pub type INSTR_MEM14 = crate::Reg<instr_mem14::INSTR_MEM14_SPEC>;
#[doc = "Write-only access to instruction memory location 14"]
pub mod instr_mem14;
#[doc = "INSTR_MEM15 register accessor: an alias for `Reg<INSTR_MEM15_SPEC>`"]
pub type INSTR_MEM15 = crate::Reg<instr_mem15::INSTR_MEM15_SPEC>;
#[doc = "Write-only access to instruction memory location 15"]
pub mod instr_mem15;
#[doc = "INSTR_MEM16 register accessor: an alias for `Reg<INSTR_MEM16_SPEC>`"]
pub type INSTR_MEM16 = crate::Reg<instr_mem16::INSTR_MEM16_SPEC>;
#[doc = "Write-only access to instruction memory location 16"]
pub mod instr_mem16;
#[doc = "INSTR_MEM17 register accessor: an alias for `Reg<INSTR_MEM17_SPEC>`"]
pub type INSTR_MEM17 = crate::Reg<instr_mem17::INSTR_MEM17_SPEC>;
#[doc = "Write-only access to instruction memory location 17"]
pub mod instr_mem17;
#[doc = "INSTR_MEM18 register accessor: an alias for `Reg<INSTR_MEM18_SPEC>`"]
pub type INSTR_MEM18 = crate::Reg<instr_mem18::INSTR_MEM18_SPEC>;
#[doc = "Write-only access to instruction memory location 18"]
pub mod instr_mem18;
#[doc = "INSTR_MEM19 register accessor: an alias for `Reg<INSTR_MEM19_SPEC>`"]
pub type INSTR_MEM19 = crate::Reg<instr_mem19::INSTR_MEM19_SPEC>;
#[doc = "Write-only access to instruction memory location 19"]
pub mod instr_mem19;
#[doc = "INSTR_MEM20 register accessor: an alias for `Reg<INSTR_MEM20_SPEC>`"]
pub type INSTR_MEM20 = crate::Reg<instr_mem20::INSTR_MEM20_SPEC>;
#[doc = "Write-only access to instruction memory location 20"]
pub mod instr_mem20;
#[doc = "INSTR_MEM21 register accessor: an alias for `Reg<INSTR_MEM21_SPEC>`"]
pub type INSTR_MEM21 = crate::Reg<instr_mem21::INSTR_MEM21_SPEC>;
#[doc = "Write-only access to instruction memory location 21"]
pub mod instr_mem21;
#[doc = "INSTR_MEM22 register accessor: an alias for `Reg<INSTR_MEM22_SPEC>`"]
pub type INSTR_MEM22 = crate::Reg<instr_mem22::INSTR_MEM22_SPEC>;
#[doc = "Write-only access to instruction memory location 22"]
pub mod instr_mem22;
#[doc = "INSTR_MEM23 register accessor: an alias for `Reg<INSTR_MEM23_SPEC>`"]
pub type INSTR_MEM23 = crate::Reg<instr_mem23::INSTR_MEM23_SPEC>;
#[doc = "Write-only access to instruction memory location 23"]
pub mod instr_mem23;
#[doc = "INSTR_MEM24 register accessor: an alias for `Reg<INSTR_MEM24_SPEC>`"]
pub type INSTR_MEM24 = crate::Reg<instr_mem24::INSTR_MEM24_SPEC>;
#[doc = "Write-only access to instruction memory location 24"]
pub mod instr_mem24;
#[doc = "INSTR_MEM25 register accessor: an alias for `Reg<INSTR_MEM25_SPEC>`"]
pub type INSTR_MEM25 = crate::Reg<instr_mem25::INSTR_MEM25_SPEC>;
#[doc = "Write-only access to instruction memory location 25"]
pub mod instr_mem25;
#[doc = "INSTR_MEM26 register accessor: an alias for `Reg<INSTR_MEM26_SPEC>`"]
pub type INSTR_MEM26 = crate::Reg<instr_mem26::INSTR_MEM26_SPEC>;
#[doc = "Write-only access to instruction memory location 26"]
pub mod instr_mem26;
#[doc = "INSTR_MEM27 register accessor: an alias for `Reg<INSTR_MEM27_SPEC>`"]
pub type INSTR_MEM27 = crate::Reg<instr_mem27::INSTR_MEM27_SPEC>;
#[doc = "Write-only access to instruction memory location 27"]
pub mod instr_mem27;
#[doc = "INSTR_MEM28 register accessor: an alias for `Reg<INSTR_MEM28_SPEC>`"]
pub type INSTR_MEM28 = crate::Reg<instr_mem28::INSTR_MEM28_SPEC>;
#[doc = "Write-only access to instruction memory location 28"]
pub mod instr_mem28;
#[doc = "INSTR_MEM29 register accessor: an alias for `Reg<INSTR_MEM29_SPEC>`"]
pub type INSTR_MEM29 = crate::Reg<instr_mem29::INSTR_MEM29_SPEC>;
#[doc = "Write-only access to instruction memory location 29"]
pub mod instr_mem29;
#[doc = "INSTR_MEM30 register accessor: an alias for `Reg<INSTR_MEM30_SPEC>`"]
pub type INSTR_MEM30 = crate::Reg<instr_mem30::INSTR_MEM30_SPEC>;
#[doc = "Write-only access to instruction memory location 30"]
pub mod instr_mem30;
#[doc = "INSTR_MEM31 register accessor: an alias for `Reg<INSTR_MEM31_SPEC>`"]
pub type INSTR_MEM31 = crate::Reg<instr_mem31::INSTR_MEM31_SPEC>;
#[doc = "Write-only access to instruction memory location 31"]
pub mod instr_mem31;
#[doc = "SM0_CLKDIV register accessor: an alias for `Reg<SM0_CLKDIV_SPEC>`"]
pub type SM0_CLKDIV = crate::Reg<sm0_clkdiv::SM0_CLKDIV_SPEC>;
#[doc = "Clock divider register for state machine 0\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm0_clkdiv;
#[doc = "SM0_EXECCTRL register accessor: an alias for `Reg<SM0_EXECCTRL_SPEC>`"]
pub type SM0_EXECCTRL = crate::Reg<sm0_execctrl::SM0_EXECCTRL_SPEC>;
#[doc = "Execution/behavioural settings for state machine 0"]
pub mod sm0_execctrl;
#[doc = "SM0_SHIFTCTRL register accessor: an alias for `Reg<SM0_SHIFTCTRL_SPEC>`"]
pub type SM0_SHIFTCTRL = crate::Reg<sm0_shiftctrl::SM0_SHIFTCTRL_SPEC>;
#[doc = "Control behaviour of the input/output shift registers for state machine 0"]
pub mod sm0_shiftctrl;
#[doc = "SM0_ADDR register accessor: an alias for `Reg<SM0_ADDR_SPEC>`"]
pub type SM0_ADDR = crate::Reg<sm0_addr::SM0_ADDR_SPEC>;
#[doc = "Current instruction address of state machine 0"]
pub mod sm0_addr;
#[doc = "SM0_INSTR register accessor: an alias for `Reg<SM0_INSTR_SPEC>`"]
pub type SM0_INSTR = crate::Reg<sm0_instr::SM0_INSTR_SPEC>;
#[doc = "Instruction currently being executed by state machine 0\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm0_instr;
#[doc = "SM0_PINCTRL register accessor: an alias for `Reg<SM0_PINCTRL_SPEC>`"]
pub type SM0_PINCTRL = crate::Reg<sm0_pinctrl::SM0_PINCTRL_SPEC>;
#[doc = "State machine pin control"]
pub mod sm0_pinctrl;
#[doc = "SM1_CLKDIV register accessor: an alias for `Reg<SM1_CLKDIV_SPEC>`"]
pub type SM1_CLKDIV = crate::Reg<sm1_clkdiv::SM1_CLKDIV_SPEC>;
#[doc = "Clock divider register for state machine 1\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm1_clkdiv;
#[doc = "SM1_EXECCTRL register accessor: an alias for `Reg<SM1_EXECCTRL_SPEC>`"]
pub type SM1_EXECCTRL = crate::Reg<sm1_execctrl::SM1_EXECCTRL_SPEC>;
#[doc = "Execution/behavioural settings for state machine 1"]
pub mod sm1_execctrl;
#[doc = "SM1_SHIFTCTRL register accessor: an alias for `Reg<SM1_SHIFTCTRL_SPEC>`"]
pub type SM1_SHIFTCTRL = crate::Reg<sm1_shiftctrl::SM1_SHIFTCTRL_SPEC>;
#[doc = "Control behaviour of the input/output shift registers for state machine 1"]
pub mod sm1_shiftctrl;
#[doc = "SM1_ADDR register accessor: an alias for `Reg<SM1_ADDR_SPEC>`"]
pub type SM1_ADDR = crate::Reg<sm1_addr::SM1_ADDR_SPEC>;
#[doc = "Current instruction address of state machine 1"]
pub mod sm1_addr;
#[doc = "SM1_INSTR register accessor: an alias for `Reg<SM1_INSTR_SPEC>`"]
pub type SM1_INSTR = crate::Reg<sm1_instr::SM1_INSTR_SPEC>;
#[doc = "Instruction currently being executed by state machine 1\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm1_instr;
#[doc = "SM1_PINCTRL register accessor: an alias for `Reg<SM1_PINCTRL_SPEC>`"]
pub type SM1_PINCTRL = crate::Reg<sm1_pinctrl::SM1_PINCTRL_SPEC>;
#[doc = "State machine pin control"]
pub mod sm1_pinctrl;
#[doc = "SM2_CLKDIV register accessor: an alias for `Reg<SM2_CLKDIV_SPEC>`"]
pub type SM2_CLKDIV = crate::Reg<sm2_clkdiv::SM2_CLKDIV_SPEC>;
#[doc = "Clock divider register for state machine 2\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm2_clkdiv;
#[doc = "SM2_EXECCTRL register accessor: an alias for `Reg<SM2_EXECCTRL_SPEC>`"]
pub type SM2_EXECCTRL = crate::Reg<sm2_execctrl::SM2_EXECCTRL_SPEC>;
#[doc = "Execution/behavioural settings for state machine 2"]
pub mod sm2_execctrl;
#[doc = "SM2_SHIFTCTRL register accessor: an alias for `Reg<SM2_SHIFTCTRL_SPEC>`"]
pub type SM2_SHIFTCTRL = crate::Reg<sm2_shiftctrl::SM2_SHIFTCTRL_SPEC>;
#[doc = "Control behaviour of the input/output shift registers for state machine 2"]
pub mod sm2_shiftctrl;
#[doc = "SM2_ADDR register accessor: an alias for `Reg<SM2_ADDR_SPEC>`"]
pub type SM2_ADDR = crate::Reg<sm2_addr::SM2_ADDR_SPEC>;
#[doc = "Current instruction address of state machine 2"]
pub mod sm2_addr;
#[doc = "SM2_INSTR register accessor: an alias for `Reg<SM2_INSTR_SPEC>`"]
pub type SM2_INSTR = crate::Reg<sm2_instr::SM2_INSTR_SPEC>;
#[doc = "Instruction currently being executed by state machine 2\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm2_instr;
#[doc = "SM2_PINCTRL register accessor: an alias for `Reg<SM2_PINCTRL_SPEC>`"]
pub type SM2_PINCTRL = crate::Reg<sm2_pinctrl::SM2_PINCTRL_SPEC>;
#[doc = "State machine pin control"]
pub mod sm2_pinctrl;
#[doc = "SM3_CLKDIV register accessor: an alias for `Reg<SM3_CLKDIV_SPEC>`"]
pub type SM3_CLKDIV = crate::Reg<sm3_clkdiv::SM3_CLKDIV_SPEC>;
#[doc = "Clock divider register for state machine 3\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm3_clkdiv;
#[doc = "SM3_EXECCTRL register accessor: an alias for `Reg<SM3_EXECCTRL_SPEC>`"]
pub type SM3_EXECCTRL = crate::Reg<sm3_execctrl::SM3_EXECCTRL_SPEC>;
#[doc = "Execution/behavioural settings for state machine 3"]
pub mod sm3_execctrl;
#[doc = "SM3_SHIFTCTRL register accessor: an alias for `Reg<SM3_SHIFTCTRL_SPEC>`"]
pub type SM3_SHIFTCTRL = crate::Reg<sm3_shiftctrl::SM3_SHIFTCTRL_SPEC>;
#[doc = "Control behaviour of the input/output shift registers for state machine 3"]
pub mod sm3_shiftctrl;
#[doc = "SM3_ADDR register accessor: an alias for `Reg<SM3_ADDR_SPEC>`"]
pub type SM3_ADDR = crate::Reg<sm3_addr::SM3_ADDR_SPEC>;
#[doc = "Current instruction address of state machine 3"]
pub mod sm3_addr;
#[doc = "SM3_INSTR register accessor: an alias for `Reg<SM3_INSTR_SPEC>`"]
pub type SM3_INSTR = crate::Reg<sm3_instr::SM3_INSTR_SPEC>;
#[doc = "Instruction currently being executed by state machine 3\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm3_instr;
#[doc = "SM3_PINCTRL register accessor: an alias for `Reg<SM3_PINCTRL_SPEC>`"]
pub type SM3_PINCTRL = crate::Reg<sm3_pinctrl::SM3_PINCTRL_SPEC>;
#[doc = "State machine pin control"]
pub mod sm3_pinctrl;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "IRQ0_INTE register accessor: an alias for `Reg<IRQ0_INTE_SPEC>`"]
pub type IRQ0_INTE = crate::Reg<irq0_inte::IRQ0_INTE_SPEC>;
#[doc = "Interrupt Enable for irq0"]
pub mod irq0_inte;
#[doc = "IRQ0_INTF register accessor: an alias for `Reg<IRQ0_INTF_SPEC>`"]
pub type IRQ0_INTF = crate::Reg<irq0_intf::IRQ0_INTF_SPEC>;
#[doc = "Interrupt Force for irq0"]
pub mod irq0_intf;
#[doc = "IRQ0_INTS register accessor: an alias for `Reg<IRQ0_INTS_SPEC>`"]
pub type IRQ0_INTS = crate::Reg<irq0_ints::IRQ0_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for irq0"]
pub mod irq0_ints;
#[doc = "IRQ1_INTE register accessor: an alias for `Reg<IRQ1_INTE_SPEC>`"]
pub type IRQ1_INTE = crate::Reg<irq1_inte::IRQ1_INTE_SPEC>;
#[doc = "Interrupt Enable for irq1"]
pub mod irq1_inte;
#[doc = "IRQ1_INTF register accessor: an alias for `Reg<IRQ1_INTF_SPEC>`"]
pub type IRQ1_INTF = crate::Reg<irq1_intf::IRQ1_INTF_SPEC>;
#[doc = "Interrupt Force for irq1"]
pub mod irq1_intf;
#[doc = "IRQ1_INTS register accessor: an alias for `Reg<IRQ1_INTS_SPEC>`"]
pub type IRQ1_INTS = crate::Reg<irq1_ints::IRQ1_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for irq1"]
pub mod irq1_ints;
