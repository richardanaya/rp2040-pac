#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Channel 0 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch0_read_addr: crate::Reg<ch0_read_addr::CH0_READ_ADDR_SPEC>,
    #[doc = "0x04 - DMA Channel 0 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch0_write_addr: crate::Reg<ch0_write_addr::CH0_WRITE_ADDR_SPEC>,
    #[doc = "0x08 - DMA Channel 0 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch0_trans_count: crate::Reg<ch0_trans_count::CH0_TRANS_COUNT_SPEC>,
    #[doc = "0x0c - DMA Channel 0 Control and Status"]
    pub ch0_ctrl_trig: crate::Reg<ch0_ctrl_trig::CH0_CTRL_TRIG_SPEC>,
    #[doc = "0x10 - Alias for channel 0 CTRL register"]
    pub ch0_al1_ctrl: crate::Reg<ch0_al1_ctrl::CH0_AL1_CTRL_SPEC>,
    #[doc = "0x14 - Alias for channel 0 READ_ADDR register"]
    pub ch0_al1_read_addr: crate::Reg<ch0_al1_read_addr::CH0_AL1_READ_ADDR_SPEC>,
    #[doc = "0x18 - Alias for channel 0 WRITE_ADDR register"]
    pub ch0_al1_write_addr: crate::Reg<ch0_al1_write_addr::CH0_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x1c - Alias for channel 0 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch0_al1_trans_count_trig:
        crate::Reg<ch0_al1_trans_count_trig::CH0_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x20 - Alias for channel 0 CTRL register"]
    pub ch0_al2_ctrl: crate::Reg<ch0_al2_ctrl::CH0_AL2_CTRL_SPEC>,
    #[doc = "0x24 - Alias for channel 0 TRANS_COUNT register"]
    pub ch0_al2_trans_count: crate::Reg<ch0_al2_trans_count::CH0_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x28 - Alias for channel 0 READ_ADDR register"]
    pub ch0_al2_read_addr: crate::Reg<ch0_al2_read_addr::CH0_AL2_READ_ADDR_SPEC>,
    #[doc = "0x2c - Alias for channel 0 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch0_al2_write_addr_trig: crate::Reg<ch0_al2_write_addr_trig::CH0_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x30 - Alias for channel 0 CTRL register"]
    pub ch0_al3_ctrl: crate::Reg<ch0_al3_ctrl::CH0_AL3_CTRL_SPEC>,
    #[doc = "0x34 - Alias for channel 0 WRITE_ADDR register"]
    pub ch0_al3_write_addr: crate::Reg<ch0_al3_write_addr::CH0_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x38 - Alias for channel 0 TRANS_COUNT register"]
    pub ch0_al3_trans_count: crate::Reg<ch0_al3_trans_count::CH0_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x3c - Alias for channel 0 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch0_al3_read_addr_trig: crate::Reg<ch0_al3_read_addr_trig::CH0_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x40 - DMA Channel 1 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch1_read_addr: crate::Reg<ch1_read_addr::CH1_READ_ADDR_SPEC>,
    #[doc = "0x44 - DMA Channel 1 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch1_write_addr: crate::Reg<ch1_write_addr::CH1_WRITE_ADDR_SPEC>,
    #[doc = "0x48 - DMA Channel 1 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch1_trans_count: crate::Reg<ch1_trans_count::CH1_TRANS_COUNT_SPEC>,
    #[doc = "0x4c - DMA Channel 1 Control and Status"]
    pub ch1_ctrl_trig: crate::Reg<ch1_ctrl_trig::CH1_CTRL_TRIG_SPEC>,
    #[doc = "0x50 - Alias for channel 1 CTRL register"]
    pub ch1_al1_ctrl: crate::Reg<ch1_al1_ctrl::CH1_AL1_CTRL_SPEC>,
    #[doc = "0x54 - Alias for channel 1 READ_ADDR register"]
    pub ch1_al1_read_addr: crate::Reg<ch1_al1_read_addr::CH1_AL1_READ_ADDR_SPEC>,
    #[doc = "0x58 - Alias for channel 1 WRITE_ADDR register"]
    pub ch1_al1_write_addr: crate::Reg<ch1_al1_write_addr::CH1_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x5c - Alias for channel 1 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch1_al1_trans_count_trig:
        crate::Reg<ch1_al1_trans_count_trig::CH1_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x60 - Alias for channel 1 CTRL register"]
    pub ch1_al2_ctrl: crate::Reg<ch1_al2_ctrl::CH1_AL2_CTRL_SPEC>,
    #[doc = "0x64 - Alias for channel 1 TRANS_COUNT register"]
    pub ch1_al2_trans_count: crate::Reg<ch1_al2_trans_count::CH1_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x68 - Alias for channel 1 READ_ADDR register"]
    pub ch1_al2_read_addr: crate::Reg<ch1_al2_read_addr::CH1_AL2_READ_ADDR_SPEC>,
    #[doc = "0x6c - Alias for channel 1 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch1_al2_write_addr_trig: crate::Reg<ch1_al2_write_addr_trig::CH1_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x70 - Alias for channel 1 CTRL register"]
    pub ch1_al3_ctrl: crate::Reg<ch1_al3_ctrl::CH1_AL3_CTRL_SPEC>,
    #[doc = "0x74 - Alias for channel 1 WRITE_ADDR register"]
    pub ch1_al3_write_addr: crate::Reg<ch1_al3_write_addr::CH1_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x78 - Alias for channel 1 TRANS_COUNT register"]
    pub ch1_al3_trans_count: crate::Reg<ch1_al3_trans_count::CH1_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x7c - Alias for channel 1 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch1_al3_read_addr_trig: crate::Reg<ch1_al3_read_addr_trig::CH1_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x80 - DMA Channel 2 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch2_read_addr: crate::Reg<ch2_read_addr::CH2_READ_ADDR_SPEC>,
    #[doc = "0x84 - DMA Channel 2 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch2_write_addr: crate::Reg<ch2_write_addr::CH2_WRITE_ADDR_SPEC>,
    #[doc = "0x88 - DMA Channel 2 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch2_trans_count: crate::Reg<ch2_trans_count::CH2_TRANS_COUNT_SPEC>,
    #[doc = "0x8c - DMA Channel 2 Control and Status"]
    pub ch2_ctrl_trig: crate::Reg<ch2_ctrl_trig::CH2_CTRL_TRIG_SPEC>,
    #[doc = "0x90 - Alias for channel 2 CTRL register"]
    pub ch2_al1_ctrl: crate::Reg<ch2_al1_ctrl::CH2_AL1_CTRL_SPEC>,
    #[doc = "0x94 - Alias for channel 2 READ_ADDR register"]
    pub ch2_al1_read_addr: crate::Reg<ch2_al1_read_addr::CH2_AL1_READ_ADDR_SPEC>,
    #[doc = "0x98 - Alias for channel 2 WRITE_ADDR register"]
    pub ch2_al1_write_addr: crate::Reg<ch2_al1_write_addr::CH2_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x9c - Alias for channel 2 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch2_al1_trans_count_trig:
        crate::Reg<ch2_al1_trans_count_trig::CH2_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0xa0 - Alias for channel 2 CTRL register"]
    pub ch2_al2_ctrl: crate::Reg<ch2_al2_ctrl::CH2_AL2_CTRL_SPEC>,
    #[doc = "0xa4 - Alias for channel 2 TRANS_COUNT register"]
    pub ch2_al2_trans_count: crate::Reg<ch2_al2_trans_count::CH2_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0xa8 - Alias for channel 2 READ_ADDR register"]
    pub ch2_al2_read_addr: crate::Reg<ch2_al2_read_addr::CH2_AL2_READ_ADDR_SPEC>,
    #[doc = "0xac - Alias for channel 2 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch2_al2_write_addr_trig: crate::Reg<ch2_al2_write_addr_trig::CH2_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0xb0 - Alias for channel 2 CTRL register"]
    pub ch2_al3_ctrl: crate::Reg<ch2_al3_ctrl::CH2_AL3_CTRL_SPEC>,
    #[doc = "0xb4 - Alias for channel 2 WRITE_ADDR register"]
    pub ch2_al3_write_addr: crate::Reg<ch2_al3_write_addr::CH2_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0xb8 - Alias for channel 2 TRANS_COUNT register"]
    pub ch2_al3_trans_count: crate::Reg<ch2_al3_trans_count::CH2_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0xbc - Alias for channel 2 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch2_al3_read_addr_trig: crate::Reg<ch2_al3_read_addr_trig::CH2_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0xc0 - DMA Channel 3 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch3_read_addr: crate::Reg<ch3_read_addr::CH3_READ_ADDR_SPEC>,
    #[doc = "0xc4 - DMA Channel 3 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch3_write_addr: crate::Reg<ch3_write_addr::CH3_WRITE_ADDR_SPEC>,
    #[doc = "0xc8 - DMA Channel 3 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch3_trans_count: crate::Reg<ch3_trans_count::CH3_TRANS_COUNT_SPEC>,
    #[doc = "0xcc - DMA Channel 3 Control and Status"]
    pub ch3_ctrl_trig: crate::Reg<ch3_ctrl_trig::CH3_CTRL_TRIG_SPEC>,
    #[doc = "0xd0 - Alias for channel 3 CTRL register"]
    pub ch3_al1_ctrl: crate::Reg<ch3_al1_ctrl::CH3_AL1_CTRL_SPEC>,
    #[doc = "0xd4 - Alias for channel 3 READ_ADDR register"]
    pub ch3_al1_read_addr: crate::Reg<ch3_al1_read_addr::CH3_AL1_READ_ADDR_SPEC>,
    #[doc = "0xd8 - Alias for channel 3 WRITE_ADDR register"]
    pub ch3_al1_write_addr: crate::Reg<ch3_al1_write_addr::CH3_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0xdc - Alias for channel 3 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch3_al1_trans_count_trig:
        crate::Reg<ch3_al1_trans_count_trig::CH3_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0xe0 - Alias for channel 3 CTRL register"]
    pub ch3_al2_ctrl: crate::Reg<ch3_al2_ctrl::CH3_AL2_CTRL_SPEC>,
    #[doc = "0xe4 - Alias for channel 3 TRANS_COUNT register"]
    pub ch3_al2_trans_count: crate::Reg<ch3_al2_trans_count::CH3_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0xe8 - Alias for channel 3 READ_ADDR register"]
    pub ch3_al2_read_addr: crate::Reg<ch3_al2_read_addr::CH3_AL2_READ_ADDR_SPEC>,
    #[doc = "0xec - Alias for channel 3 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch3_al2_write_addr_trig: crate::Reg<ch3_al2_write_addr_trig::CH3_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0xf0 - Alias for channel 3 CTRL register"]
    pub ch3_al3_ctrl: crate::Reg<ch3_al3_ctrl::CH3_AL3_CTRL_SPEC>,
    #[doc = "0xf4 - Alias for channel 3 WRITE_ADDR register"]
    pub ch3_al3_write_addr: crate::Reg<ch3_al3_write_addr::CH3_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0xf8 - Alias for channel 3 TRANS_COUNT register"]
    pub ch3_al3_trans_count: crate::Reg<ch3_al3_trans_count::CH3_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0xfc - Alias for channel 3 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch3_al3_read_addr_trig: crate::Reg<ch3_al3_read_addr_trig::CH3_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x100 - DMA Channel 4 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch4_read_addr: crate::Reg<ch4_read_addr::CH4_READ_ADDR_SPEC>,
    #[doc = "0x104 - DMA Channel 4 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch4_write_addr: crate::Reg<ch4_write_addr::CH4_WRITE_ADDR_SPEC>,
    #[doc = "0x108 - DMA Channel 4 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch4_trans_count: crate::Reg<ch4_trans_count::CH4_TRANS_COUNT_SPEC>,
    #[doc = "0x10c - DMA Channel 4 Control and Status"]
    pub ch4_ctrl_trig: crate::Reg<ch4_ctrl_trig::CH4_CTRL_TRIG_SPEC>,
    #[doc = "0x110 - Alias for channel 4 CTRL register"]
    pub ch4_al1_ctrl: crate::Reg<ch4_al1_ctrl::CH4_AL1_CTRL_SPEC>,
    #[doc = "0x114 - Alias for channel 4 READ_ADDR register"]
    pub ch4_al1_read_addr: crate::Reg<ch4_al1_read_addr::CH4_AL1_READ_ADDR_SPEC>,
    #[doc = "0x118 - Alias for channel 4 WRITE_ADDR register"]
    pub ch4_al1_write_addr: crate::Reg<ch4_al1_write_addr::CH4_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x11c - Alias for channel 4 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch4_al1_trans_count_trig:
        crate::Reg<ch4_al1_trans_count_trig::CH4_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x120 - Alias for channel 4 CTRL register"]
    pub ch4_al2_ctrl: crate::Reg<ch4_al2_ctrl::CH4_AL2_CTRL_SPEC>,
    #[doc = "0x124 - Alias for channel 4 TRANS_COUNT register"]
    pub ch4_al2_trans_count: crate::Reg<ch4_al2_trans_count::CH4_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x128 - Alias for channel 4 READ_ADDR register"]
    pub ch4_al2_read_addr: crate::Reg<ch4_al2_read_addr::CH4_AL2_READ_ADDR_SPEC>,
    #[doc = "0x12c - Alias for channel 4 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch4_al2_write_addr_trig: crate::Reg<ch4_al2_write_addr_trig::CH4_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x130 - Alias for channel 4 CTRL register"]
    pub ch4_al3_ctrl: crate::Reg<ch4_al3_ctrl::CH4_AL3_CTRL_SPEC>,
    #[doc = "0x134 - Alias for channel 4 WRITE_ADDR register"]
    pub ch4_al3_write_addr: crate::Reg<ch4_al3_write_addr::CH4_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x138 - Alias for channel 4 TRANS_COUNT register"]
    pub ch4_al3_trans_count: crate::Reg<ch4_al3_trans_count::CH4_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x13c - Alias for channel 4 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch4_al3_read_addr_trig: crate::Reg<ch4_al3_read_addr_trig::CH4_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x140 - DMA Channel 5 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch5_read_addr: crate::Reg<ch5_read_addr::CH5_READ_ADDR_SPEC>,
    #[doc = "0x144 - DMA Channel 5 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch5_write_addr: crate::Reg<ch5_write_addr::CH5_WRITE_ADDR_SPEC>,
    #[doc = "0x148 - DMA Channel 5 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch5_trans_count: crate::Reg<ch5_trans_count::CH5_TRANS_COUNT_SPEC>,
    #[doc = "0x14c - DMA Channel 5 Control and Status"]
    pub ch5_ctrl_trig: crate::Reg<ch5_ctrl_trig::CH5_CTRL_TRIG_SPEC>,
    #[doc = "0x150 - Alias for channel 5 CTRL register"]
    pub ch5_al1_ctrl: crate::Reg<ch5_al1_ctrl::CH5_AL1_CTRL_SPEC>,
    #[doc = "0x154 - Alias for channel 5 READ_ADDR register"]
    pub ch5_al1_read_addr: crate::Reg<ch5_al1_read_addr::CH5_AL1_READ_ADDR_SPEC>,
    #[doc = "0x158 - Alias for channel 5 WRITE_ADDR register"]
    pub ch5_al1_write_addr: crate::Reg<ch5_al1_write_addr::CH5_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x15c - Alias for channel 5 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch5_al1_trans_count_trig:
        crate::Reg<ch5_al1_trans_count_trig::CH5_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x160 - Alias for channel 5 CTRL register"]
    pub ch5_al2_ctrl: crate::Reg<ch5_al2_ctrl::CH5_AL2_CTRL_SPEC>,
    #[doc = "0x164 - Alias for channel 5 TRANS_COUNT register"]
    pub ch5_al2_trans_count: crate::Reg<ch5_al2_trans_count::CH5_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x168 - Alias for channel 5 READ_ADDR register"]
    pub ch5_al2_read_addr: crate::Reg<ch5_al2_read_addr::CH5_AL2_READ_ADDR_SPEC>,
    #[doc = "0x16c - Alias for channel 5 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch5_al2_write_addr_trig: crate::Reg<ch5_al2_write_addr_trig::CH5_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x170 - Alias for channel 5 CTRL register"]
    pub ch5_al3_ctrl: crate::Reg<ch5_al3_ctrl::CH5_AL3_CTRL_SPEC>,
    #[doc = "0x174 - Alias for channel 5 WRITE_ADDR register"]
    pub ch5_al3_write_addr: crate::Reg<ch5_al3_write_addr::CH5_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x178 - Alias for channel 5 TRANS_COUNT register"]
    pub ch5_al3_trans_count: crate::Reg<ch5_al3_trans_count::CH5_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x17c - Alias for channel 5 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch5_al3_read_addr_trig: crate::Reg<ch5_al3_read_addr_trig::CH5_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x180 - DMA Channel 6 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch6_read_addr: crate::Reg<ch6_read_addr::CH6_READ_ADDR_SPEC>,
    #[doc = "0x184 - DMA Channel 6 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch6_write_addr: crate::Reg<ch6_write_addr::CH6_WRITE_ADDR_SPEC>,
    #[doc = "0x188 - DMA Channel 6 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch6_trans_count: crate::Reg<ch6_trans_count::CH6_TRANS_COUNT_SPEC>,
    #[doc = "0x18c - DMA Channel 6 Control and Status"]
    pub ch6_ctrl_trig: crate::Reg<ch6_ctrl_trig::CH6_CTRL_TRIG_SPEC>,
    #[doc = "0x190 - Alias for channel 6 CTRL register"]
    pub ch6_al1_ctrl: crate::Reg<ch6_al1_ctrl::CH6_AL1_CTRL_SPEC>,
    #[doc = "0x194 - Alias for channel 6 READ_ADDR register"]
    pub ch6_al1_read_addr: crate::Reg<ch6_al1_read_addr::CH6_AL1_READ_ADDR_SPEC>,
    #[doc = "0x198 - Alias for channel 6 WRITE_ADDR register"]
    pub ch6_al1_write_addr: crate::Reg<ch6_al1_write_addr::CH6_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x19c - Alias for channel 6 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch6_al1_trans_count_trig:
        crate::Reg<ch6_al1_trans_count_trig::CH6_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x1a0 - Alias for channel 6 CTRL register"]
    pub ch6_al2_ctrl: crate::Reg<ch6_al2_ctrl::CH6_AL2_CTRL_SPEC>,
    #[doc = "0x1a4 - Alias for channel 6 TRANS_COUNT register"]
    pub ch6_al2_trans_count: crate::Reg<ch6_al2_trans_count::CH6_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x1a8 - Alias for channel 6 READ_ADDR register"]
    pub ch6_al2_read_addr: crate::Reg<ch6_al2_read_addr::CH6_AL2_READ_ADDR_SPEC>,
    #[doc = "0x1ac - Alias for channel 6 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch6_al2_write_addr_trig: crate::Reg<ch6_al2_write_addr_trig::CH6_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x1b0 - Alias for channel 6 CTRL register"]
    pub ch6_al3_ctrl: crate::Reg<ch6_al3_ctrl::CH6_AL3_CTRL_SPEC>,
    #[doc = "0x1b4 - Alias for channel 6 WRITE_ADDR register"]
    pub ch6_al3_write_addr: crate::Reg<ch6_al3_write_addr::CH6_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x1b8 - Alias for channel 6 TRANS_COUNT register"]
    pub ch6_al3_trans_count: crate::Reg<ch6_al3_trans_count::CH6_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x1bc - Alias for channel 6 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch6_al3_read_addr_trig: crate::Reg<ch6_al3_read_addr_trig::CH6_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x1c0 - DMA Channel 7 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch7_read_addr: crate::Reg<ch7_read_addr::CH7_READ_ADDR_SPEC>,
    #[doc = "0x1c4 - DMA Channel 7 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch7_write_addr: crate::Reg<ch7_write_addr::CH7_WRITE_ADDR_SPEC>,
    #[doc = "0x1c8 - DMA Channel 7 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch7_trans_count: crate::Reg<ch7_trans_count::CH7_TRANS_COUNT_SPEC>,
    #[doc = "0x1cc - DMA Channel 7 Control and Status"]
    pub ch7_ctrl_trig: crate::Reg<ch7_ctrl_trig::CH7_CTRL_TRIG_SPEC>,
    #[doc = "0x1d0 - Alias for channel 7 CTRL register"]
    pub ch7_al1_ctrl: crate::Reg<ch7_al1_ctrl::CH7_AL1_CTRL_SPEC>,
    #[doc = "0x1d4 - Alias for channel 7 READ_ADDR register"]
    pub ch7_al1_read_addr: crate::Reg<ch7_al1_read_addr::CH7_AL1_READ_ADDR_SPEC>,
    #[doc = "0x1d8 - Alias for channel 7 WRITE_ADDR register"]
    pub ch7_al1_write_addr: crate::Reg<ch7_al1_write_addr::CH7_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x1dc - Alias for channel 7 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch7_al1_trans_count_trig:
        crate::Reg<ch7_al1_trans_count_trig::CH7_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x1e0 - Alias for channel 7 CTRL register"]
    pub ch7_al2_ctrl: crate::Reg<ch7_al2_ctrl::CH7_AL2_CTRL_SPEC>,
    #[doc = "0x1e4 - Alias for channel 7 TRANS_COUNT register"]
    pub ch7_al2_trans_count: crate::Reg<ch7_al2_trans_count::CH7_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x1e8 - Alias for channel 7 READ_ADDR register"]
    pub ch7_al2_read_addr: crate::Reg<ch7_al2_read_addr::CH7_AL2_READ_ADDR_SPEC>,
    #[doc = "0x1ec - Alias for channel 7 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch7_al2_write_addr_trig: crate::Reg<ch7_al2_write_addr_trig::CH7_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x1f0 - Alias for channel 7 CTRL register"]
    pub ch7_al3_ctrl: crate::Reg<ch7_al3_ctrl::CH7_AL3_CTRL_SPEC>,
    #[doc = "0x1f4 - Alias for channel 7 WRITE_ADDR register"]
    pub ch7_al3_write_addr: crate::Reg<ch7_al3_write_addr::CH7_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x1f8 - Alias for channel 7 TRANS_COUNT register"]
    pub ch7_al3_trans_count: crate::Reg<ch7_al3_trans_count::CH7_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x1fc - Alias for channel 7 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch7_al3_read_addr_trig: crate::Reg<ch7_al3_read_addr_trig::CH7_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x200 - DMA Channel 8 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch8_read_addr: crate::Reg<ch8_read_addr::CH8_READ_ADDR_SPEC>,
    #[doc = "0x204 - DMA Channel 8 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch8_write_addr: crate::Reg<ch8_write_addr::CH8_WRITE_ADDR_SPEC>,
    #[doc = "0x208 - DMA Channel 8 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch8_trans_count: crate::Reg<ch8_trans_count::CH8_TRANS_COUNT_SPEC>,
    #[doc = "0x20c - DMA Channel 8 Control and Status"]
    pub ch8_ctrl_trig: crate::Reg<ch8_ctrl_trig::CH8_CTRL_TRIG_SPEC>,
    #[doc = "0x210 - Alias for channel 8 CTRL register"]
    pub ch8_al1_ctrl: crate::Reg<ch8_al1_ctrl::CH8_AL1_CTRL_SPEC>,
    #[doc = "0x214 - Alias for channel 8 READ_ADDR register"]
    pub ch8_al1_read_addr: crate::Reg<ch8_al1_read_addr::CH8_AL1_READ_ADDR_SPEC>,
    #[doc = "0x218 - Alias for channel 8 WRITE_ADDR register"]
    pub ch8_al1_write_addr: crate::Reg<ch8_al1_write_addr::CH8_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x21c - Alias for channel 8 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch8_al1_trans_count_trig:
        crate::Reg<ch8_al1_trans_count_trig::CH8_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x220 - Alias for channel 8 CTRL register"]
    pub ch8_al2_ctrl: crate::Reg<ch8_al2_ctrl::CH8_AL2_CTRL_SPEC>,
    #[doc = "0x224 - Alias for channel 8 TRANS_COUNT register"]
    pub ch8_al2_trans_count: crate::Reg<ch8_al2_trans_count::CH8_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x228 - Alias for channel 8 READ_ADDR register"]
    pub ch8_al2_read_addr: crate::Reg<ch8_al2_read_addr::CH8_AL2_READ_ADDR_SPEC>,
    #[doc = "0x22c - Alias for channel 8 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch8_al2_write_addr_trig: crate::Reg<ch8_al2_write_addr_trig::CH8_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x230 - Alias for channel 8 CTRL register"]
    pub ch8_al3_ctrl: crate::Reg<ch8_al3_ctrl::CH8_AL3_CTRL_SPEC>,
    #[doc = "0x234 - Alias for channel 8 WRITE_ADDR register"]
    pub ch8_al3_write_addr: crate::Reg<ch8_al3_write_addr::CH8_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x238 - Alias for channel 8 TRANS_COUNT register"]
    pub ch8_al3_trans_count: crate::Reg<ch8_al3_trans_count::CH8_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x23c - Alias for channel 8 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch8_al3_read_addr_trig: crate::Reg<ch8_al3_read_addr_trig::CH8_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x240 - DMA Channel 9 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch9_read_addr: crate::Reg<ch9_read_addr::CH9_READ_ADDR_SPEC>,
    #[doc = "0x244 - DMA Channel 9 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch9_write_addr: crate::Reg<ch9_write_addr::CH9_WRITE_ADDR_SPEC>,
    #[doc = "0x248 - DMA Channel 9 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch9_trans_count: crate::Reg<ch9_trans_count::CH9_TRANS_COUNT_SPEC>,
    #[doc = "0x24c - DMA Channel 9 Control and Status"]
    pub ch9_ctrl_trig: crate::Reg<ch9_ctrl_trig::CH9_CTRL_TRIG_SPEC>,
    #[doc = "0x250 - Alias for channel 9 CTRL register"]
    pub ch9_al1_ctrl: crate::Reg<ch9_al1_ctrl::CH9_AL1_CTRL_SPEC>,
    #[doc = "0x254 - Alias for channel 9 READ_ADDR register"]
    pub ch9_al1_read_addr: crate::Reg<ch9_al1_read_addr::CH9_AL1_READ_ADDR_SPEC>,
    #[doc = "0x258 - Alias for channel 9 WRITE_ADDR register"]
    pub ch9_al1_write_addr: crate::Reg<ch9_al1_write_addr::CH9_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x25c - Alias for channel 9 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch9_al1_trans_count_trig:
        crate::Reg<ch9_al1_trans_count_trig::CH9_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x260 - Alias for channel 9 CTRL register"]
    pub ch9_al2_ctrl: crate::Reg<ch9_al2_ctrl::CH9_AL2_CTRL_SPEC>,
    #[doc = "0x264 - Alias for channel 9 TRANS_COUNT register"]
    pub ch9_al2_trans_count: crate::Reg<ch9_al2_trans_count::CH9_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x268 - Alias for channel 9 READ_ADDR register"]
    pub ch9_al2_read_addr: crate::Reg<ch9_al2_read_addr::CH9_AL2_READ_ADDR_SPEC>,
    #[doc = "0x26c - Alias for channel 9 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch9_al2_write_addr_trig: crate::Reg<ch9_al2_write_addr_trig::CH9_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x270 - Alias for channel 9 CTRL register"]
    pub ch9_al3_ctrl: crate::Reg<ch9_al3_ctrl::CH9_AL3_CTRL_SPEC>,
    #[doc = "0x274 - Alias for channel 9 WRITE_ADDR register"]
    pub ch9_al3_write_addr: crate::Reg<ch9_al3_write_addr::CH9_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x278 - Alias for channel 9 TRANS_COUNT register"]
    pub ch9_al3_trans_count: crate::Reg<ch9_al3_trans_count::CH9_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x27c - Alias for channel 9 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch9_al3_read_addr_trig: crate::Reg<ch9_al3_read_addr_trig::CH9_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x280 - DMA Channel 10 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch10_read_addr: crate::Reg<ch10_read_addr::CH10_READ_ADDR_SPEC>,
    #[doc = "0x284 - DMA Channel 10 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch10_write_addr: crate::Reg<ch10_write_addr::CH10_WRITE_ADDR_SPEC>,
    #[doc = "0x288 - DMA Channel 10 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch10_trans_count: crate::Reg<ch10_trans_count::CH10_TRANS_COUNT_SPEC>,
    #[doc = "0x28c - DMA Channel 10 Control and Status"]
    pub ch10_ctrl_trig: crate::Reg<ch10_ctrl_trig::CH10_CTRL_TRIG_SPEC>,
    #[doc = "0x290 - Alias for channel 10 CTRL register"]
    pub ch10_al1_ctrl: crate::Reg<ch10_al1_ctrl::CH10_AL1_CTRL_SPEC>,
    #[doc = "0x294 - Alias for channel 10 READ_ADDR register"]
    pub ch10_al1_read_addr: crate::Reg<ch10_al1_read_addr::CH10_AL1_READ_ADDR_SPEC>,
    #[doc = "0x298 - Alias for channel 10 WRITE_ADDR register"]
    pub ch10_al1_write_addr: crate::Reg<ch10_al1_write_addr::CH10_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x29c - Alias for channel 10 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch10_al1_trans_count_trig:
        crate::Reg<ch10_al1_trans_count_trig::CH10_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x2a0 - Alias for channel 10 CTRL register"]
    pub ch10_al2_ctrl: crate::Reg<ch10_al2_ctrl::CH10_AL2_CTRL_SPEC>,
    #[doc = "0x2a4 - Alias for channel 10 TRANS_COUNT register"]
    pub ch10_al2_trans_count: crate::Reg<ch10_al2_trans_count::CH10_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x2a8 - Alias for channel 10 READ_ADDR register"]
    pub ch10_al2_read_addr: crate::Reg<ch10_al2_read_addr::CH10_AL2_READ_ADDR_SPEC>,
    #[doc = "0x2ac - Alias for channel 10 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch10_al2_write_addr_trig:
        crate::Reg<ch10_al2_write_addr_trig::CH10_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x2b0 - Alias for channel 10 CTRL register"]
    pub ch10_al3_ctrl: crate::Reg<ch10_al3_ctrl::CH10_AL3_CTRL_SPEC>,
    #[doc = "0x2b4 - Alias for channel 10 WRITE_ADDR register"]
    pub ch10_al3_write_addr: crate::Reg<ch10_al3_write_addr::CH10_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x2b8 - Alias for channel 10 TRANS_COUNT register"]
    pub ch10_al3_trans_count: crate::Reg<ch10_al3_trans_count::CH10_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x2bc - Alias for channel 10 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch10_al3_read_addr_trig: crate::Reg<ch10_al3_read_addr_trig::CH10_AL3_READ_ADDR_TRIG_SPEC>,
    #[doc = "0x2c0 - DMA Channel 11 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch11_read_addr: crate::Reg<ch11_read_addr::CH11_READ_ADDR_SPEC>,
    #[doc = "0x2c4 - DMA Channel 11 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch11_write_addr: crate::Reg<ch11_write_addr::CH11_WRITE_ADDR_SPEC>,
    #[doc = "0x2c8 - DMA Channel 11 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch11_trans_count: crate::Reg<ch11_trans_count::CH11_TRANS_COUNT_SPEC>,
    #[doc = "0x2cc - DMA Channel 11 Control and Status"]
    pub ch11_ctrl_trig: crate::Reg<ch11_ctrl_trig::CH11_CTRL_TRIG_SPEC>,
    #[doc = "0x2d0 - Alias for channel 11 CTRL register"]
    pub ch11_al1_ctrl: crate::Reg<ch11_al1_ctrl::CH11_AL1_CTRL_SPEC>,
    #[doc = "0x2d4 - Alias for channel 11 READ_ADDR register"]
    pub ch11_al1_read_addr: crate::Reg<ch11_al1_read_addr::CH11_AL1_READ_ADDR_SPEC>,
    #[doc = "0x2d8 - Alias for channel 11 WRITE_ADDR register"]
    pub ch11_al1_write_addr: crate::Reg<ch11_al1_write_addr::CH11_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x2dc - Alias for channel 11 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch11_al1_trans_count_trig:
        crate::Reg<ch11_al1_trans_count_trig::CH11_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x2e0 - Alias for channel 11 CTRL register"]
    pub ch11_al2_ctrl: crate::Reg<ch11_al2_ctrl::CH11_AL2_CTRL_SPEC>,
    #[doc = "0x2e4 - Alias for channel 11 TRANS_COUNT register"]
    pub ch11_al2_trans_count: crate::Reg<ch11_al2_trans_count::CH11_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x2e8 - Alias for channel 11 READ_ADDR register"]
    pub ch11_al2_read_addr: crate::Reg<ch11_al2_read_addr::CH11_AL2_READ_ADDR_SPEC>,
    #[doc = "0x2ec - Alias for channel 11 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch11_al2_write_addr_trig:
        crate::Reg<ch11_al2_write_addr_trig::CH11_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x2f0 - Alias for channel 11 CTRL register"]
    pub ch11_al3_ctrl: crate::Reg<ch11_al3_ctrl::CH11_AL3_CTRL_SPEC>,
    #[doc = "0x2f4 - Alias for channel 11 WRITE_ADDR register"]
    pub ch11_al3_write_addr: crate::Reg<ch11_al3_write_addr::CH11_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x2f8 - Alias for channel 11 TRANS_COUNT register"]
    pub ch11_al3_trans_count: crate::Reg<ch11_al3_trans_count::CH11_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x2fc - Alias for channel 11 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch11_al3_read_addr_trig: crate::Reg<ch11_al3_read_addr_trig::CH11_AL3_READ_ADDR_TRIG_SPEC>,
    _reserved192: [u8; 256usize],
    #[doc = "0x400 - Interrupt Status (raw)"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x404 - Interrupt Enables for IRQ 0"]
    pub inte0: crate::Reg<inte0::INTE0_SPEC>,
    #[doc = "0x408 - Force Interrupts"]
    pub intf0: crate::Reg<intf0::INTF0_SPEC>,
    #[doc = "0x40c - Interrupt Status for IRQ 0"]
    pub ints0: crate::Reg<ints0::INTS0_SPEC>,
    _reserved196: [u8; 4usize],
    #[doc = "0x414 - Interrupt Enables for IRQ 1"]
    pub inte1: crate::Reg<inte1::INTE1_SPEC>,
    #[doc = "0x418 - Force Interrupts for IRQ 1"]
    pub intf1: crate::Reg<intf1::INTF1_SPEC>,
    #[doc = "0x41c - Interrupt Status (masked) for IRQ 1"]
    pub ints1: crate::Reg<ints1::INTS1_SPEC>,
    #[doc = "0x420 - Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer0: crate::Reg<timer0::TIMER0_SPEC>,
    #[doc = "0x424 - Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer1: crate::Reg<timer1::TIMER1_SPEC>,
    _reserved201: [u8; 8usize],
    #[doc = "0x430 - Trigger one or more channels simultaneously"]
    pub multi_chan_trigger: crate::Reg<multi_chan_trigger::MULTI_CHAN_TRIGGER_SPEC>,
    #[doc = "0x434 - Sniffer Control"]
    pub sniff_ctrl: crate::Reg<sniff_ctrl::SNIFF_CTRL_SPEC>,
    #[doc = "0x438 - Data accumulator for sniff hardware\\n Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    pub sniff_data: crate::Reg<sniff_data::SNIFF_DATA_SPEC>,
    _reserved204: [u8; 4usize],
    #[doc = "0x440 - Debug RAF, WAF, TDF levels"]
    pub fifo_levels: crate::Reg<fifo_levels::FIFO_LEVELS_SPEC>,
    #[doc = "0x444 - Abort an in-progress transfer sequence on one or more channels"]
    pub chan_abort: crate::Reg<chan_abort::CHAN_ABORT_SPEC>,
    #[doc = "0x448 - The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    pub n_channels: crate::Reg<n_channels::N_CHANNELS_SPEC>,
    _reserved207: [u8; 948usize],
    #[doc = "0x800 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch0_dbg_ctdreq: crate::Reg<ch0_dbg_ctdreq::CH0_DBG_CTDREQ_SPEC>,
    #[doc = "0x804 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch0_dbg_tcr: crate::Reg<ch0_dbg_tcr::CH0_DBG_TCR_SPEC>,
    _reserved209: [u8; 56usize],
    #[doc = "0x840 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch1_dbg_ctdreq: crate::Reg<ch1_dbg_ctdreq::CH1_DBG_CTDREQ_SPEC>,
    #[doc = "0x844 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch1_dbg_tcr: crate::Reg<ch1_dbg_tcr::CH1_DBG_TCR_SPEC>,
    _reserved211: [u8; 56usize],
    #[doc = "0x880 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch2_dbg_ctdreq: crate::Reg<ch2_dbg_ctdreq::CH2_DBG_CTDREQ_SPEC>,
    #[doc = "0x884 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch2_dbg_tcr: crate::Reg<ch2_dbg_tcr::CH2_DBG_TCR_SPEC>,
    _reserved213: [u8; 56usize],
    #[doc = "0x8c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch3_dbg_ctdreq: crate::Reg<ch3_dbg_ctdreq::CH3_DBG_CTDREQ_SPEC>,
    #[doc = "0x8c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch3_dbg_tcr: crate::Reg<ch3_dbg_tcr::CH3_DBG_TCR_SPEC>,
    _reserved215: [u8; 56usize],
    #[doc = "0x900 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch4_dbg_ctdreq: crate::Reg<ch4_dbg_ctdreq::CH4_DBG_CTDREQ_SPEC>,
    #[doc = "0x904 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch4_dbg_tcr: crate::Reg<ch4_dbg_tcr::CH4_DBG_TCR_SPEC>,
    _reserved217: [u8; 56usize],
    #[doc = "0x940 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch5_dbg_ctdreq: crate::Reg<ch5_dbg_ctdreq::CH5_DBG_CTDREQ_SPEC>,
    #[doc = "0x944 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch5_dbg_tcr: crate::Reg<ch5_dbg_tcr::CH5_DBG_TCR_SPEC>,
    _reserved219: [u8; 56usize],
    #[doc = "0x980 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch6_dbg_ctdreq: crate::Reg<ch6_dbg_ctdreq::CH6_DBG_CTDREQ_SPEC>,
    #[doc = "0x984 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch6_dbg_tcr: crate::Reg<ch6_dbg_tcr::CH6_DBG_TCR_SPEC>,
    _reserved221: [u8; 56usize],
    #[doc = "0x9c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch7_dbg_ctdreq: crate::Reg<ch7_dbg_ctdreq::CH7_DBG_CTDREQ_SPEC>,
    #[doc = "0x9c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch7_dbg_tcr: crate::Reg<ch7_dbg_tcr::CH7_DBG_TCR_SPEC>,
    _reserved223: [u8; 56usize],
    #[doc = "0xa00 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch8_dbg_ctdreq: crate::Reg<ch8_dbg_ctdreq::CH8_DBG_CTDREQ_SPEC>,
    #[doc = "0xa04 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch8_dbg_tcr: crate::Reg<ch8_dbg_tcr::CH8_DBG_TCR_SPEC>,
    _reserved225: [u8; 56usize],
    #[doc = "0xa40 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch9_dbg_ctdreq: crate::Reg<ch9_dbg_ctdreq::CH9_DBG_CTDREQ_SPEC>,
    #[doc = "0xa44 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch9_dbg_tcr: crate::Reg<ch9_dbg_tcr::CH9_DBG_TCR_SPEC>,
    _reserved227: [u8; 56usize],
    #[doc = "0xa80 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch10_dbg_ctdreq: crate::Reg<ch10_dbg_ctdreq::CH10_DBG_CTDREQ_SPEC>,
    #[doc = "0xa84 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch10_dbg_tcr: crate::Reg<ch10_dbg_tcr::CH10_DBG_TCR_SPEC>,
    _reserved229: [u8; 56usize],
    #[doc = "0xac0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch11_dbg_ctdreq: crate::Reg<ch11_dbg_ctdreq::CH11_DBG_CTDREQ_SPEC>,
    #[doc = "0xac4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch11_dbg_tcr: crate::Reg<ch11_dbg_tcr::CH11_DBG_TCR_SPEC>,
}
#[doc = "CH0_READ_ADDR register accessor: an alias for `Reg<CH0_READ_ADDR_SPEC>`"]
pub type CH0_READ_ADDR = crate::Reg<ch0_read_addr::CH0_READ_ADDR_SPEC>;
#[doc = "DMA Channel 0 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch0_read_addr;
#[doc = "CH0_WRITE_ADDR register accessor: an alias for `Reg<CH0_WRITE_ADDR_SPEC>`"]
pub type CH0_WRITE_ADDR = crate::Reg<ch0_write_addr::CH0_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 0 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch0_write_addr;
#[doc = "CH0_TRANS_COUNT register accessor: an alias for `Reg<CH0_TRANS_COUNT_SPEC>`"]
pub type CH0_TRANS_COUNT = crate::Reg<ch0_trans_count::CH0_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 0 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch0_trans_count;
#[doc = "CH0_CTRL_TRIG register accessor: an alias for `Reg<CH0_CTRL_TRIG_SPEC>`"]
pub type CH0_CTRL_TRIG = crate::Reg<ch0_ctrl_trig::CH0_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch0_ctrl_trig;
#[doc = "CH0_AL1_CTRL register accessor: an alias for `Reg<CH0_AL1_CTRL_SPEC>`"]
pub type CH0_AL1_CTRL = crate::Reg<ch0_al1_ctrl::CH0_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 0 CTRL register"]
pub mod ch0_al1_ctrl;
#[doc = "CH0_AL1_READ_ADDR register accessor: an alias for `Reg<CH0_AL1_READ_ADDR_SPEC>`"]
pub type CH0_AL1_READ_ADDR = crate::Reg<ch0_al1_read_addr::CH0_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch0_al1_read_addr;
#[doc = "CH0_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH0_AL1_WRITE_ADDR_SPEC>`"]
pub type CH0_AL1_WRITE_ADDR = crate::Reg<ch0_al1_write_addr::CH0_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch0_al1_write_addr;
#[doc = "CH0_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH0_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH0_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch0_al1_trans_count_trig::CH0_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 0 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch0_al1_trans_count_trig;
#[doc = "CH0_AL2_CTRL register accessor: an alias for `Reg<CH0_AL2_CTRL_SPEC>`"]
pub type CH0_AL2_CTRL = crate::Reg<ch0_al2_ctrl::CH0_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 0 CTRL register"]
pub mod ch0_al2_ctrl;
#[doc = "CH0_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH0_AL2_TRANS_COUNT_SPEC>`"]
pub type CH0_AL2_TRANS_COUNT = crate::Reg<ch0_al2_trans_count::CH0_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch0_al2_trans_count;
#[doc = "CH0_AL2_READ_ADDR register accessor: an alias for `Reg<CH0_AL2_READ_ADDR_SPEC>`"]
pub type CH0_AL2_READ_ADDR = crate::Reg<ch0_al2_read_addr::CH0_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch0_al2_read_addr;
#[doc = "CH0_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH0_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH0_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch0_al2_write_addr_trig::CH0_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 0 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch0_al2_write_addr_trig;
#[doc = "CH0_AL3_CTRL register accessor: an alias for `Reg<CH0_AL3_CTRL_SPEC>`"]
pub type CH0_AL3_CTRL = crate::Reg<ch0_al3_ctrl::CH0_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 0 CTRL register"]
pub mod ch0_al3_ctrl;
#[doc = "CH0_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH0_AL3_WRITE_ADDR_SPEC>`"]
pub type CH0_AL3_WRITE_ADDR = crate::Reg<ch0_al3_write_addr::CH0_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch0_al3_write_addr;
#[doc = "CH0_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH0_AL3_TRANS_COUNT_SPEC>`"]
pub type CH0_AL3_TRANS_COUNT = crate::Reg<ch0_al3_trans_count::CH0_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch0_al3_trans_count;
#[doc = "CH0_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH0_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH0_AL3_READ_ADDR_TRIG = crate::Reg<ch0_al3_read_addr_trig::CH0_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 0 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch0_al3_read_addr_trig;
#[doc = "CH1_READ_ADDR register accessor: an alias for `Reg<CH1_READ_ADDR_SPEC>`"]
pub type CH1_READ_ADDR = crate::Reg<ch1_read_addr::CH1_READ_ADDR_SPEC>;
#[doc = "DMA Channel 1 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch1_read_addr;
#[doc = "CH1_WRITE_ADDR register accessor: an alias for `Reg<CH1_WRITE_ADDR_SPEC>`"]
pub type CH1_WRITE_ADDR = crate::Reg<ch1_write_addr::CH1_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 1 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch1_write_addr;
#[doc = "CH1_TRANS_COUNT register accessor: an alias for `Reg<CH1_TRANS_COUNT_SPEC>`"]
pub type CH1_TRANS_COUNT = crate::Reg<ch1_trans_count::CH1_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 1 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch1_trans_count;
#[doc = "CH1_CTRL_TRIG register accessor: an alias for `Reg<CH1_CTRL_TRIG_SPEC>`"]
pub type CH1_CTRL_TRIG = crate::Reg<ch1_ctrl_trig::CH1_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 1 Control and Status"]
pub mod ch1_ctrl_trig;
#[doc = "CH1_AL1_CTRL register accessor: an alias for `Reg<CH1_AL1_CTRL_SPEC>`"]
pub type CH1_AL1_CTRL = crate::Reg<ch1_al1_ctrl::CH1_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 1 CTRL register"]
pub mod ch1_al1_ctrl;
#[doc = "CH1_AL1_READ_ADDR register accessor: an alias for `Reg<CH1_AL1_READ_ADDR_SPEC>`"]
pub type CH1_AL1_READ_ADDR = crate::Reg<ch1_al1_read_addr::CH1_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 1 READ_ADDR register"]
pub mod ch1_al1_read_addr;
#[doc = "CH1_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH1_AL1_WRITE_ADDR_SPEC>`"]
pub type CH1_AL1_WRITE_ADDR = crate::Reg<ch1_al1_write_addr::CH1_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 1 WRITE_ADDR register"]
pub mod ch1_al1_write_addr;
#[doc = "CH1_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH1_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH1_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch1_al1_trans_count_trig::CH1_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 1 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch1_al1_trans_count_trig;
#[doc = "CH1_AL2_CTRL register accessor: an alias for `Reg<CH1_AL2_CTRL_SPEC>`"]
pub type CH1_AL2_CTRL = crate::Reg<ch1_al2_ctrl::CH1_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 1 CTRL register"]
pub mod ch1_al2_ctrl;
#[doc = "CH1_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH1_AL2_TRANS_COUNT_SPEC>`"]
pub type CH1_AL2_TRANS_COUNT = crate::Reg<ch1_al2_trans_count::CH1_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 1 TRANS_COUNT register"]
pub mod ch1_al2_trans_count;
#[doc = "CH1_AL2_READ_ADDR register accessor: an alias for `Reg<CH1_AL2_READ_ADDR_SPEC>`"]
pub type CH1_AL2_READ_ADDR = crate::Reg<ch1_al2_read_addr::CH1_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 1 READ_ADDR register"]
pub mod ch1_al2_read_addr;
#[doc = "CH1_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH1_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH1_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch1_al2_write_addr_trig::CH1_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 1 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch1_al2_write_addr_trig;
#[doc = "CH1_AL3_CTRL register accessor: an alias for `Reg<CH1_AL3_CTRL_SPEC>`"]
pub type CH1_AL3_CTRL = crate::Reg<ch1_al3_ctrl::CH1_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 1 CTRL register"]
pub mod ch1_al3_ctrl;
#[doc = "CH1_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH1_AL3_WRITE_ADDR_SPEC>`"]
pub type CH1_AL3_WRITE_ADDR = crate::Reg<ch1_al3_write_addr::CH1_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 1 WRITE_ADDR register"]
pub mod ch1_al3_write_addr;
#[doc = "CH1_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH1_AL3_TRANS_COUNT_SPEC>`"]
pub type CH1_AL3_TRANS_COUNT = crate::Reg<ch1_al3_trans_count::CH1_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 1 TRANS_COUNT register"]
pub mod ch1_al3_trans_count;
#[doc = "CH1_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH1_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH1_AL3_READ_ADDR_TRIG = crate::Reg<ch1_al3_read_addr_trig::CH1_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 1 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch1_al3_read_addr_trig;
#[doc = "CH2_READ_ADDR register accessor: an alias for `Reg<CH2_READ_ADDR_SPEC>`"]
pub type CH2_READ_ADDR = crate::Reg<ch2_read_addr::CH2_READ_ADDR_SPEC>;
#[doc = "DMA Channel 2 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch2_read_addr;
#[doc = "CH2_WRITE_ADDR register accessor: an alias for `Reg<CH2_WRITE_ADDR_SPEC>`"]
pub type CH2_WRITE_ADDR = crate::Reg<ch2_write_addr::CH2_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 2 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch2_write_addr;
#[doc = "CH2_TRANS_COUNT register accessor: an alias for `Reg<CH2_TRANS_COUNT_SPEC>`"]
pub type CH2_TRANS_COUNT = crate::Reg<ch2_trans_count::CH2_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 2 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch2_trans_count;
#[doc = "CH2_CTRL_TRIG register accessor: an alias for `Reg<CH2_CTRL_TRIG_SPEC>`"]
pub type CH2_CTRL_TRIG = crate::Reg<ch2_ctrl_trig::CH2_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 2 Control and Status"]
pub mod ch2_ctrl_trig;
#[doc = "CH2_AL1_CTRL register accessor: an alias for `Reg<CH2_AL1_CTRL_SPEC>`"]
pub type CH2_AL1_CTRL = crate::Reg<ch2_al1_ctrl::CH2_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 2 CTRL register"]
pub mod ch2_al1_ctrl;
#[doc = "CH2_AL1_READ_ADDR register accessor: an alias for `Reg<CH2_AL1_READ_ADDR_SPEC>`"]
pub type CH2_AL1_READ_ADDR = crate::Reg<ch2_al1_read_addr::CH2_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 2 READ_ADDR register"]
pub mod ch2_al1_read_addr;
#[doc = "CH2_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH2_AL1_WRITE_ADDR_SPEC>`"]
pub type CH2_AL1_WRITE_ADDR = crate::Reg<ch2_al1_write_addr::CH2_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 2 WRITE_ADDR register"]
pub mod ch2_al1_write_addr;
#[doc = "CH2_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH2_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH2_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch2_al1_trans_count_trig::CH2_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 2 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch2_al1_trans_count_trig;
#[doc = "CH2_AL2_CTRL register accessor: an alias for `Reg<CH2_AL2_CTRL_SPEC>`"]
pub type CH2_AL2_CTRL = crate::Reg<ch2_al2_ctrl::CH2_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 2 CTRL register"]
pub mod ch2_al2_ctrl;
#[doc = "CH2_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH2_AL2_TRANS_COUNT_SPEC>`"]
pub type CH2_AL2_TRANS_COUNT = crate::Reg<ch2_al2_trans_count::CH2_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 2 TRANS_COUNT register"]
pub mod ch2_al2_trans_count;
#[doc = "CH2_AL2_READ_ADDR register accessor: an alias for `Reg<CH2_AL2_READ_ADDR_SPEC>`"]
pub type CH2_AL2_READ_ADDR = crate::Reg<ch2_al2_read_addr::CH2_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 2 READ_ADDR register"]
pub mod ch2_al2_read_addr;
#[doc = "CH2_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH2_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH2_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch2_al2_write_addr_trig::CH2_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 2 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch2_al2_write_addr_trig;
#[doc = "CH2_AL3_CTRL register accessor: an alias for `Reg<CH2_AL3_CTRL_SPEC>`"]
pub type CH2_AL3_CTRL = crate::Reg<ch2_al3_ctrl::CH2_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 2 CTRL register"]
pub mod ch2_al3_ctrl;
#[doc = "CH2_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH2_AL3_WRITE_ADDR_SPEC>`"]
pub type CH2_AL3_WRITE_ADDR = crate::Reg<ch2_al3_write_addr::CH2_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 2 WRITE_ADDR register"]
pub mod ch2_al3_write_addr;
#[doc = "CH2_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH2_AL3_TRANS_COUNT_SPEC>`"]
pub type CH2_AL3_TRANS_COUNT = crate::Reg<ch2_al3_trans_count::CH2_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 2 TRANS_COUNT register"]
pub mod ch2_al3_trans_count;
#[doc = "CH2_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH2_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH2_AL3_READ_ADDR_TRIG = crate::Reg<ch2_al3_read_addr_trig::CH2_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 2 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch2_al3_read_addr_trig;
#[doc = "CH3_READ_ADDR register accessor: an alias for `Reg<CH3_READ_ADDR_SPEC>`"]
pub type CH3_READ_ADDR = crate::Reg<ch3_read_addr::CH3_READ_ADDR_SPEC>;
#[doc = "DMA Channel 3 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch3_read_addr;
#[doc = "CH3_WRITE_ADDR register accessor: an alias for `Reg<CH3_WRITE_ADDR_SPEC>`"]
pub type CH3_WRITE_ADDR = crate::Reg<ch3_write_addr::CH3_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 3 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch3_write_addr;
#[doc = "CH3_TRANS_COUNT register accessor: an alias for `Reg<CH3_TRANS_COUNT_SPEC>`"]
pub type CH3_TRANS_COUNT = crate::Reg<ch3_trans_count::CH3_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 3 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch3_trans_count;
#[doc = "CH3_CTRL_TRIG register accessor: an alias for `Reg<CH3_CTRL_TRIG_SPEC>`"]
pub type CH3_CTRL_TRIG = crate::Reg<ch3_ctrl_trig::CH3_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 3 Control and Status"]
pub mod ch3_ctrl_trig;
#[doc = "CH3_AL1_CTRL register accessor: an alias for `Reg<CH3_AL1_CTRL_SPEC>`"]
pub type CH3_AL1_CTRL = crate::Reg<ch3_al1_ctrl::CH3_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 3 CTRL register"]
pub mod ch3_al1_ctrl;
#[doc = "CH3_AL1_READ_ADDR register accessor: an alias for `Reg<CH3_AL1_READ_ADDR_SPEC>`"]
pub type CH3_AL1_READ_ADDR = crate::Reg<ch3_al1_read_addr::CH3_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 3 READ_ADDR register"]
pub mod ch3_al1_read_addr;
#[doc = "CH3_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH3_AL1_WRITE_ADDR_SPEC>`"]
pub type CH3_AL1_WRITE_ADDR = crate::Reg<ch3_al1_write_addr::CH3_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 3 WRITE_ADDR register"]
pub mod ch3_al1_write_addr;
#[doc = "CH3_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH3_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH3_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch3_al1_trans_count_trig::CH3_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 3 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch3_al1_trans_count_trig;
#[doc = "CH3_AL2_CTRL register accessor: an alias for `Reg<CH3_AL2_CTRL_SPEC>`"]
pub type CH3_AL2_CTRL = crate::Reg<ch3_al2_ctrl::CH3_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 3 CTRL register"]
pub mod ch3_al2_ctrl;
#[doc = "CH3_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH3_AL2_TRANS_COUNT_SPEC>`"]
pub type CH3_AL2_TRANS_COUNT = crate::Reg<ch3_al2_trans_count::CH3_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 3 TRANS_COUNT register"]
pub mod ch3_al2_trans_count;
#[doc = "CH3_AL2_READ_ADDR register accessor: an alias for `Reg<CH3_AL2_READ_ADDR_SPEC>`"]
pub type CH3_AL2_READ_ADDR = crate::Reg<ch3_al2_read_addr::CH3_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 3 READ_ADDR register"]
pub mod ch3_al2_read_addr;
#[doc = "CH3_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH3_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH3_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch3_al2_write_addr_trig::CH3_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 3 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch3_al2_write_addr_trig;
#[doc = "CH3_AL3_CTRL register accessor: an alias for `Reg<CH3_AL3_CTRL_SPEC>`"]
pub type CH3_AL3_CTRL = crate::Reg<ch3_al3_ctrl::CH3_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 3 CTRL register"]
pub mod ch3_al3_ctrl;
#[doc = "CH3_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH3_AL3_WRITE_ADDR_SPEC>`"]
pub type CH3_AL3_WRITE_ADDR = crate::Reg<ch3_al3_write_addr::CH3_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 3 WRITE_ADDR register"]
pub mod ch3_al3_write_addr;
#[doc = "CH3_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH3_AL3_TRANS_COUNT_SPEC>`"]
pub type CH3_AL3_TRANS_COUNT = crate::Reg<ch3_al3_trans_count::CH3_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 3 TRANS_COUNT register"]
pub mod ch3_al3_trans_count;
#[doc = "CH3_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH3_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH3_AL3_READ_ADDR_TRIG = crate::Reg<ch3_al3_read_addr_trig::CH3_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 3 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch3_al3_read_addr_trig;
#[doc = "CH4_READ_ADDR register accessor: an alias for `Reg<CH4_READ_ADDR_SPEC>`"]
pub type CH4_READ_ADDR = crate::Reg<ch4_read_addr::CH4_READ_ADDR_SPEC>;
#[doc = "DMA Channel 4 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch4_read_addr;
#[doc = "CH4_WRITE_ADDR register accessor: an alias for `Reg<CH4_WRITE_ADDR_SPEC>`"]
pub type CH4_WRITE_ADDR = crate::Reg<ch4_write_addr::CH4_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 4 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch4_write_addr;
#[doc = "CH4_TRANS_COUNT register accessor: an alias for `Reg<CH4_TRANS_COUNT_SPEC>`"]
pub type CH4_TRANS_COUNT = crate::Reg<ch4_trans_count::CH4_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 4 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch4_trans_count;
#[doc = "CH4_CTRL_TRIG register accessor: an alias for `Reg<CH4_CTRL_TRIG_SPEC>`"]
pub type CH4_CTRL_TRIG = crate::Reg<ch4_ctrl_trig::CH4_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 4 Control and Status"]
pub mod ch4_ctrl_trig;
#[doc = "CH4_AL1_CTRL register accessor: an alias for `Reg<CH4_AL1_CTRL_SPEC>`"]
pub type CH4_AL1_CTRL = crate::Reg<ch4_al1_ctrl::CH4_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 4 CTRL register"]
pub mod ch4_al1_ctrl;
#[doc = "CH4_AL1_READ_ADDR register accessor: an alias for `Reg<CH4_AL1_READ_ADDR_SPEC>`"]
pub type CH4_AL1_READ_ADDR = crate::Reg<ch4_al1_read_addr::CH4_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 4 READ_ADDR register"]
pub mod ch4_al1_read_addr;
#[doc = "CH4_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH4_AL1_WRITE_ADDR_SPEC>`"]
pub type CH4_AL1_WRITE_ADDR = crate::Reg<ch4_al1_write_addr::CH4_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 4 WRITE_ADDR register"]
pub mod ch4_al1_write_addr;
#[doc = "CH4_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH4_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH4_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch4_al1_trans_count_trig::CH4_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 4 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch4_al1_trans_count_trig;
#[doc = "CH4_AL2_CTRL register accessor: an alias for `Reg<CH4_AL2_CTRL_SPEC>`"]
pub type CH4_AL2_CTRL = crate::Reg<ch4_al2_ctrl::CH4_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 4 CTRL register"]
pub mod ch4_al2_ctrl;
#[doc = "CH4_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH4_AL2_TRANS_COUNT_SPEC>`"]
pub type CH4_AL2_TRANS_COUNT = crate::Reg<ch4_al2_trans_count::CH4_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 4 TRANS_COUNT register"]
pub mod ch4_al2_trans_count;
#[doc = "CH4_AL2_READ_ADDR register accessor: an alias for `Reg<CH4_AL2_READ_ADDR_SPEC>`"]
pub type CH4_AL2_READ_ADDR = crate::Reg<ch4_al2_read_addr::CH4_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 4 READ_ADDR register"]
pub mod ch4_al2_read_addr;
#[doc = "CH4_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH4_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH4_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch4_al2_write_addr_trig::CH4_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 4 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch4_al2_write_addr_trig;
#[doc = "CH4_AL3_CTRL register accessor: an alias for `Reg<CH4_AL3_CTRL_SPEC>`"]
pub type CH4_AL3_CTRL = crate::Reg<ch4_al3_ctrl::CH4_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 4 CTRL register"]
pub mod ch4_al3_ctrl;
#[doc = "CH4_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH4_AL3_WRITE_ADDR_SPEC>`"]
pub type CH4_AL3_WRITE_ADDR = crate::Reg<ch4_al3_write_addr::CH4_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 4 WRITE_ADDR register"]
pub mod ch4_al3_write_addr;
#[doc = "CH4_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH4_AL3_TRANS_COUNT_SPEC>`"]
pub type CH4_AL3_TRANS_COUNT = crate::Reg<ch4_al3_trans_count::CH4_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 4 TRANS_COUNT register"]
pub mod ch4_al3_trans_count;
#[doc = "CH4_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH4_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH4_AL3_READ_ADDR_TRIG = crate::Reg<ch4_al3_read_addr_trig::CH4_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 4 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch4_al3_read_addr_trig;
#[doc = "CH5_READ_ADDR register accessor: an alias for `Reg<CH5_READ_ADDR_SPEC>`"]
pub type CH5_READ_ADDR = crate::Reg<ch5_read_addr::CH5_READ_ADDR_SPEC>;
#[doc = "DMA Channel 5 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch5_read_addr;
#[doc = "CH5_WRITE_ADDR register accessor: an alias for `Reg<CH5_WRITE_ADDR_SPEC>`"]
pub type CH5_WRITE_ADDR = crate::Reg<ch5_write_addr::CH5_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 5 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch5_write_addr;
#[doc = "CH5_TRANS_COUNT register accessor: an alias for `Reg<CH5_TRANS_COUNT_SPEC>`"]
pub type CH5_TRANS_COUNT = crate::Reg<ch5_trans_count::CH5_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 5 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch5_trans_count;
#[doc = "CH5_CTRL_TRIG register accessor: an alias for `Reg<CH5_CTRL_TRIG_SPEC>`"]
pub type CH5_CTRL_TRIG = crate::Reg<ch5_ctrl_trig::CH5_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 5 Control and Status"]
pub mod ch5_ctrl_trig;
#[doc = "CH5_AL1_CTRL register accessor: an alias for `Reg<CH5_AL1_CTRL_SPEC>`"]
pub type CH5_AL1_CTRL = crate::Reg<ch5_al1_ctrl::CH5_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 5 CTRL register"]
pub mod ch5_al1_ctrl;
#[doc = "CH5_AL1_READ_ADDR register accessor: an alias for `Reg<CH5_AL1_READ_ADDR_SPEC>`"]
pub type CH5_AL1_READ_ADDR = crate::Reg<ch5_al1_read_addr::CH5_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 5 READ_ADDR register"]
pub mod ch5_al1_read_addr;
#[doc = "CH5_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH5_AL1_WRITE_ADDR_SPEC>`"]
pub type CH5_AL1_WRITE_ADDR = crate::Reg<ch5_al1_write_addr::CH5_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 5 WRITE_ADDR register"]
pub mod ch5_al1_write_addr;
#[doc = "CH5_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH5_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH5_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch5_al1_trans_count_trig::CH5_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 5 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch5_al1_trans_count_trig;
#[doc = "CH5_AL2_CTRL register accessor: an alias for `Reg<CH5_AL2_CTRL_SPEC>`"]
pub type CH5_AL2_CTRL = crate::Reg<ch5_al2_ctrl::CH5_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 5 CTRL register"]
pub mod ch5_al2_ctrl;
#[doc = "CH5_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH5_AL2_TRANS_COUNT_SPEC>`"]
pub type CH5_AL2_TRANS_COUNT = crate::Reg<ch5_al2_trans_count::CH5_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 5 TRANS_COUNT register"]
pub mod ch5_al2_trans_count;
#[doc = "CH5_AL2_READ_ADDR register accessor: an alias for `Reg<CH5_AL2_READ_ADDR_SPEC>`"]
pub type CH5_AL2_READ_ADDR = crate::Reg<ch5_al2_read_addr::CH5_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 5 READ_ADDR register"]
pub mod ch5_al2_read_addr;
#[doc = "CH5_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH5_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH5_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch5_al2_write_addr_trig::CH5_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 5 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch5_al2_write_addr_trig;
#[doc = "CH5_AL3_CTRL register accessor: an alias for `Reg<CH5_AL3_CTRL_SPEC>`"]
pub type CH5_AL3_CTRL = crate::Reg<ch5_al3_ctrl::CH5_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 5 CTRL register"]
pub mod ch5_al3_ctrl;
#[doc = "CH5_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH5_AL3_WRITE_ADDR_SPEC>`"]
pub type CH5_AL3_WRITE_ADDR = crate::Reg<ch5_al3_write_addr::CH5_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 5 WRITE_ADDR register"]
pub mod ch5_al3_write_addr;
#[doc = "CH5_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH5_AL3_TRANS_COUNT_SPEC>`"]
pub type CH5_AL3_TRANS_COUNT = crate::Reg<ch5_al3_trans_count::CH5_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 5 TRANS_COUNT register"]
pub mod ch5_al3_trans_count;
#[doc = "CH5_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH5_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH5_AL3_READ_ADDR_TRIG = crate::Reg<ch5_al3_read_addr_trig::CH5_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 5 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch5_al3_read_addr_trig;
#[doc = "CH6_READ_ADDR register accessor: an alias for `Reg<CH6_READ_ADDR_SPEC>`"]
pub type CH6_READ_ADDR = crate::Reg<ch6_read_addr::CH6_READ_ADDR_SPEC>;
#[doc = "DMA Channel 6 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch6_read_addr;
#[doc = "CH6_WRITE_ADDR register accessor: an alias for `Reg<CH6_WRITE_ADDR_SPEC>`"]
pub type CH6_WRITE_ADDR = crate::Reg<ch6_write_addr::CH6_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 6 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch6_write_addr;
#[doc = "CH6_TRANS_COUNT register accessor: an alias for `Reg<CH6_TRANS_COUNT_SPEC>`"]
pub type CH6_TRANS_COUNT = crate::Reg<ch6_trans_count::CH6_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 6 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch6_trans_count;
#[doc = "CH6_CTRL_TRIG register accessor: an alias for `Reg<CH6_CTRL_TRIG_SPEC>`"]
pub type CH6_CTRL_TRIG = crate::Reg<ch6_ctrl_trig::CH6_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 6 Control and Status"]
pub mod ch6_ctrl_trig;
#[doc = "CH6_AL1_CTRL register accessor: an alias for `Reg<CH6_AL1_CTRL_SPEC>`"]
pub type CH6_AL1_CTRL = crate::Reg<ch6_al1_ctrl::CH6_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 6 CTRL register"]
pub mod ch6_al1_ctrl;
#[doc = "CH6_AL1_READ_ADDR register accessor: an alias for `Reg<CH6_AL1_READ_ADDR_SPEC>`"]
pub type CH6_AL1_READ_ADDR = crate::Reg<ch6_al1_read_addr::CH6_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 6 READ_ADDR register"]
pub mod ch6_al1_read_addr;
#[doc = "CH6_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH6_AL1_WRITE_ADDR_SPEC>`"]
pub type CH6_AL1_WRITE_ADDR = crate::Reg<ch6_al1_write_addr::CH6_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 6 WRITE_ADDR register"]
pub mod ch6_al1_write_addr;
#[doc = "CH6_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH6_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH6_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch6_al1_trans_count_trig::CH6_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 6 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch6_al1_trans_count_trig;
#[doc = "CH6_AL2_CTRL register accessor: an alias for `Reg<CH6_AL2_CTRL_SPEC>`"]
pub type CH6_AL2_CTRL = crate::Reg<ch6_al2_ctrl::CH6_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 6 CTRL register"]
pub mod ch6_al2_ctrl;
#[doc = "CH6_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH6_AL2_TRANS_COUNT_SPEC>`"]
pub type CH6_AL2_TRANS_COUNT = crate::Reg<ch6_al2_trans_count::CH6_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 6 TRANS_COUNT register"]
pub mod ch6_al2_trans_count;
#[doc = "CH6_AL2_READ_ADDR register accessor: an alias for `Reg<CH6_AL2_READ_ADDR_SPEC>`"]
pub type CH6_AL2_READ_ADDR = crate::Reg<ch6_al2_read_addr::CH6_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 6 READ_ADDR register"]
pub mod ch6_al2_read_addr;
#[doc = "CH6_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH6_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH6_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch6_al2_write_addr_trig::CH6_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 6 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch6_al2_write_addr_trig;
#[doc = "CH6_AL3_CTRL register accessor: an alias for `Reg<CH6_AL3_CTRL_SPEC>`"]
pub type CH6_AL3_CTRL = crate::Reg<ch6_al3_ctrl::CH6_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 6 CTRL register"]
pub mod ch6_al3_ctrl;
#[doc = "CH6_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH6_AL3_WRITE_ADDR_SPEC>`"]
pub type CH6_AL3_WRITE_ADDR = crate::Reg<ch6_al3_write_addr::CH6_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 6 WRITE_ADDR register"]
pub mod ch6_al3_write_addr;
#[doc = "CH6_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH6_AL3_TRANS_COUNT_SPEC>`"]
pub type CH6_AL3_TRANS_COUNT = crate::Reg<ch6_al3_trans_count::CH6_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 6 TRANS_COUNT register"]
pub mod ch6_al3_trans_count;
#[doc = "CH6_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH6_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH6_AL3_READ_ADDR_TRIG = crate::Reg<ch6_al3_read_addr_trig::CH6_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 6 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch6_al3_read_addr_trig;
#[doc = "CH7_READ_ADDR register accessor: an alias for `Reg<CH7_READ_ADDR_SPEC>`"]
pub type CH7_READ_ADDR = crate::Reg<ch7_read_addr::CH7_READ_ADDR_SPEC>;
#[doc = "DMA Channel 7 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch7_read_addr;
#[doc = "CH7_WRITE_ADDR register accessor: an alias for `Reg<CH7_WRITE_ADDR_SPEC>`"]
pub type CH7_WRITE_ADDR = crate::Reg<ch7_write_addr::CH7_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 7 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch7_write_addr;
#[doc = "CH7_TRANS_COUNT register accessor: an alias for `Reg<CH7_TRANS_COUNT_SPEC>`"]
pub type CH7_TRANS_COUNT = crate::Reg<ch7_trans_count::CH7_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 7 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch7_trans_count;
#[doc = "CH7_CTRL_TRIG register accessor: an alias for `Reg<CH7_CTRL_TRIG_SPEC>`"]
pub type CH7_CTRL_TRIG = crate::Reg<ch7_ctrl_trig::CH7_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 7 Control and Status"]
pub mod ch7_ctrl_trig;
#[doc = "CH7_AL1_CTRL register accessor: an alias for `Reg<CH7_AL1_CTRL_SPEC>`"]
pub type CH7_AL1_CTRL = crate::Reg<ch7_al1_ctrl::CH7_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 7 CTRL register"]
pub mod ch7_al1_ctrl;
#[doc = "CH7_AL1_READ_ADDR register accessor: an alias for `Reg<CH7_AL1_READ_ADDR_SPEC>`"]
pub type CH7_AL1_READ_ADDR = crate::Reg<ch7_al1_read_addr::CH7_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 7 READ_ADDR register"]
pub mod ch7_al1_read_addr;
#[doc = "CH7_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH7_AL1_WRITE_ADDR_SPEC>`"]
pub type CH7_AL1_WRITE_ADDR = crate::Reg<ch7_al1_write_addr::CH7_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 7 WRITE_ADDR register"]
pub mod ch7_al1_write_addr;
#[doc = "CH7_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH7_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH7_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch7_al1_trans_count_trig::CH7_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 7 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch7_al1_trans_count_trig;
#[doc = "CH7_AL2_CTRL register accessor: an alias for `Reg<CH7_AL2_CTRL_SPEC>`"]
pub type CH7_AL2_CTRL = crate::Reg<ch7_al2_ctrl::CH7_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 7 CTRL register"]
pub mod ch7_al2_ctrl;
#[doc = "CH7_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH7_AL2_TRANS_COUNT_SPEC>`"]
pub type CH7_AL2_TRANS_COUNT = crate::Reg<ch7_al2_trans_count::CH7_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 7 TRANS_COUNT register"]
pub mod ch7_al2_trans_count;
#[doc = "CH7_AL2_READ_ADDR register accessor: an alias for `Reg<CH7_AL2_READ_ADDR_SPEC>`"]
pub type CH7_AL2_READ_ADDR = crate::Reg<ch7_al2_read_addr::CH7_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 7 READ_ADDR register"]
pub mod ch7_al2_read_addr;
#[doc = "CH7_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH7_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH7_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch7_al2_write_addr_trig::CH7_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 7 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch7_al2_write_addr_trig;
#[doc = "CH7_AL3_CTRL register accessor: an alias for `Reg<CH7_AL3_CTRL_SPEC>`"]
pub type CH7_AL3_CTRL = crate::Reg<ch7_al3_ctrl::CH7_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 7 CTRL register"]
pub mod ch7_al3_ctrl;
#[doc = "CH7_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH7_AL3_WRITE_ADDR_SPEC>`"]
pub type CH7_AL3_WRITE_ADDR = crate::Reg<ch7_al3_write_addr::CH7_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 7 WRITE_ADDR register"]
pub mod ch7_al3_write_addr;
#[doc = "CH7_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH7_AL3_TRANS_COUNT_SPEC>`"]
pub type CH7_AL3_TRANS_COUNT = crate::Reg<ch7_al3_trans_count::CH7_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 7 TRANS_COUNT register"]
pub mod ch7_al3_trans_count;
#[doc = "CH7_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH7_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH7_AL3_READ_ADDR_TRIG = crate::Reg<ch7_al3_read_addr_trig::CH7_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 7 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch7_al3_read_addr_trig;
#[doc = "CH8_READ_ADDR register accessor: an alias for `Reg<CH8_READ_ADDR_SPEC>`"]
pub type CH8_READ_ADDR = crate::Reg<ch8_read_addr::CH8_READ_ADDR_SPEC>;
#[doc = "DMA Channel 8 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch8_read_addr;
#[doc = "CH8_WRITE_ADDR register accessor: an alias for `Reg<CH8_WRITE_ADDR_SPEC>`"]
pub type CH8_WRITE_ADDR = crate::Reg<ch8_write_addr::CH8_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 8 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch8_write_addr;
#[doc = "CH8_TRANS_COUNT register accessor: an alias for `Reg<CH8_TRANS_COUNT_SPEC>`"]
pub type CH8_TRANS_COUNT = crate::Reg<ch8_trans_count::CH8_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 8 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch8_trans_count;
#[doc = "CH8_CTRL_TRIG register accessor: an alias for `Reg<CH8_CTRL_TRIG_SPEC>`"]
pub type CH8_CTRL_TRIG = crate::Reg<ch8_ctrl_trig::CH8_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 8 Control and Status"]
pub mod ch8_ctrl_trig;
#[doc = "CH8_AL1_CTRL register accessor: an alias for `Reg<CH8_AL1_CTRL_SPEC>`"]
pub type CH8_AL1_CTRL = crate::Reg<ch8_al1_ctrl::CH8_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 8 CTRL register"]
pub mod ch8_al1_ctrl;
#[doc = "CH8_AL1_READ_ADDR register accessor: an alias for `Reg<CH8_AL1_READ_ADDR_SPEC>`"]
pub type CH8_AL1_READ_ADDR = crate::Reg<ch8_al1_read_addr::CH8_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 8 READ_ADDR register"]
pub mod ch8_al1_read_addr;
#[doc = "CH8_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH8_AL1_WRITE_ADDR_SPEC>`"]
pub type CH8_AL1_WRITE_ADDR = crate::Reg<ch8_al1_write_addr::CH8_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 8 WRITE_ADDR register"]
pub mod ch8_al1_write_addr;
#[doc = "CH8_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH8_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH8_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch8_al1_trans_count_trig::CH8_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 8 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch8_al1_trans_count_trig;
#[doc = "CH8_AL2_CTRL register accessor: an alias for `Reg<CH8_AL2_CTRL_SPEC>`"]
pub type CH8_AL2_CTRL = crate::Reg<ch8_al2_ctrl::CH8_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 8 CTRL register"]
pub mod ch8_al2_ctrl;
#[doc = "CH8_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH8_AL2_TRANS_COUNT_SPEC>`"]
pub type CH8_AL2_TRANS_COUNT = crate::Reg<ch8_al2_trans_count::CH8_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 8 TRANS_COUNT register"]
pub mod ch8_al2_trans_count;
#[doc = "CH8_AL2_READ_ADDR register accessor: an alias for `Reg<CH8_AL2_READ_ADDR_SPEC>`"]
pub type CH8_AL2_READ_ADDR = crate::Reg<ch8_al2_read_addr::CH8_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 8 READ_ADDR register"]
pub mod ch8_al2_read_addr;
#[doc = "CH8_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH8_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH8_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch8_al2_write_addr_trig::CH8_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 8 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch8_al2_write_addr_trig;
#[doc = "CH8_AL3_CTRL register accessor: an alias for `Reg<CH8_AL3_CTRL_SPEC>`"]
pub type CH8_AL3_CTRL = crate::Reg<ch8_al3_ctrl::CH8_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 8 CTRL register"]
pub mod ch8_al3_ctrl;
#[doc = "CH8_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH8_AL3_WRITE_ADDR_SPEC>`"]
pub type CH8_AL3_WRITE_ADDR = crate::Reg<ch8_al3_write_addr::CH8_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 8 WRITE_ADDR register"]
pub mod ch8_al3_write_addr;
#[doc = "CH8_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH8_AL3_TRANS_COUNT_SPEC>`"]
pub type CH8_AL3_TRANS_COUNT = crate::Reg<ch8_al3_trans_count::CH8_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 8 TRANS_COUNT register"]
pub mod ch8_al3_trans_count;
#[doc = "CH8_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH8_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH8_AL3_READ_ADDR_TRIG = crate::Reg<ch8_al3_read_addr_trig::CH8_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 8 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch8_al3_read_addr_trig;
#[doc = "CH9_READ_ADDR register accessor: an alias for `Reg<CH9_READ_ADDR_SPEC>`"]
pub type CH9_READ_ADDR = crate::Reg<ch9_read_addr::CH9_READ_ADDR_SPEC>;
#[doc = "DMA Channel 9 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch9_read_addr;
#[doc = "CH9_WRITE_ADDR register accessor: an alias for `Reg<CH9_WRITE_ADDR_SPEC>`"]
pub type CH9_WRITE_ADDR = crate::Reg<ch9_write_addr::CH9_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 9 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch9_write_addr;
#[doc = "CH9_TRANS_COUNT register accessor: an alias for `Reg<CH9_TRANS_COUNT_SPEC>`"]
pub type CH9_TRANS_COUNT = crate::Reg<ch9_trans_count::CH9_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 9 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch9_trans_count;
#[doc = "CH9_CTRL_TRIG register accessor: an alias for `Reg<CH9_CTRL_TRIG_SPEC>`"]
pub type CH9_CTRL_TRIG = crate::Reg<ch9_ctrl_trig::CH9_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 9 Control and Status"]
pub mod ch9_ctrl_trig;
#[doc = "CH9_AL1_CTRL register accessor: an alias for `Reg<CH9_AL1_CTRL_SPEC>`"]
pub type CH9_AL1_CTRL = crate::Reg<ch9_al1_ctrl::CH9_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 9 CTRL register"]
pub mod ch9_al1_ctrl;
#[doc = "CH9_AL1_READ_ADDR register accessor: an alias for `Reg<CH9_AL1_READ_ADDR_SPEC>`"]
pub type CH9_AL1_READ_ADDR = crate::Reg<ch9_al1_read_addr::CH9_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 9 READ_ADDR register"]
pub mod ch9_al1_read_addr;
#[doc = "CH9_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH9_AL1_WRITE_ADDR_SPEC>`"]
pub type CH9_AL1_WRITE_ADDR = crate::Reg<ch9_al1_write_addr::CH9_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 9 WRITE_ADDR register"]
pub mod ch9_al1_write_addr;
#[doc = "CH9_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH9_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH9_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch9_al1_trans_count_trig::CH9_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 9 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch9_al1_trans_count_trig;
#[doc = "CH9_AL2_CTRL register accessor: an alias for `Reg<CH9_AL2_CTRL_SPEC>`"]
pub type CH9_AL2_CTRL = crate::Reg<ch9_al2_ctrl::CH9_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 9 CTRL register"]
pub mod ch9_al2_ctrl;
#[doc = "CH9_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH9_AL2_TRANS_COUNT_SPEC>`"]
pub type CH9_AL2_TRANS_COUNT = crate::Reg<ch9_al2_trans_count::CH9_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 9 TRANS_COUNT register"]
pub mod ch9_al2_trans_count;
#[doc = "CH9_AL2_READ_ADDR register accessor: an alias for `Reg<CH9_AL2_READ_ADDR_SPEC>`"]
pub type CH9_AL2_READ_ADDR = crate::Reg<ch9_al2_read_addr::CH9_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 9 READ_ADDR register"]
pub mod ch9_al2_read_addr;
#[doc = "CH9_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH9_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH9_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch9_al2_write_addr_trig::CH9_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 9 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch9_al2_write_addr_trig;
#[doc = "CH9_AL3_CTRL register accessor: an alias for `Reg<CH9_AL3_CTRL_SPEC>`"]
pub type CH9_AL3_CTRL = crate::Reg<ch9_al3_ctrl::CH9_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 9 CTRL register"]
pub mod ch9_al3_ctrl;
#[doc = "CH9_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH9_AL3_WRITE_ADDR_SPEC>`"]
pub type CH9_AL3_WRITE_ADDR = crate::Reg<ch9_al3_write_addr::CH9_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 9 WRITE_ADDR register"]
pub mod ch9_al3_write_addr;
#[doc = "CH9_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH9_AL3_TRANS_COUNT_SPEC>`"]
pub type CH9_AL3_TRANS_COUNT = crate::Reg<ch9_al3_trans_count::CH9_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 9 TRANS_COUNT register"]
pub mod ch9_al3_trans_count;
#[doc = "CH9_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH9_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH9_AL3_READ_ADDR_TRIG = crate::Reg<ch9_al3_read_addr_trig::CH9_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 9 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch9_al3_read_addr_trig;
#[doc = "CH10_READ_ADDR register accessor: an alias for `Reg<CH10_READ_ADDR_SPEC>`"]
pub type CH10_READ_ADDR = crate::Reg<ch10_read_addr::CH10_READ_ADDR_SPEC>;
#[doc = "DMA Channel 10 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch10_read_addr;
#[doc = "CH10_WRITE_ADDR register accessor: an alias for `Reg<CH10_WRITE_ADDR_SPEC>`"]
pub type CH10_WRITE_ADDR = crate::Reg<ch10_write_addr::CH10_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 10 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch10_write_addr;
#[doc = "CH10_TRANS_COUNT register accessor: an alias for `Reg<CH10_TRANS_COUNT_SPEC>`"]
pub type CH10_TRANS_COUNT = crate::Reg<ch10_trans_count::CH10_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 10 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch10_trans_count;
#[doc = "CH10_CTRL_TRIG register accessor: an alias for `Reg<CH10_CTRL_TRIG_SPEC>`"]
pub type CH10_CTRL_TRIG = crate::Reg<ch10_ctrl_trig::CH10_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 10 Control and Status"]
pub mod ch10_ctrl_trig;
#[doc = "CH10_AL1_CTRL register accessor: an alias for `Reg<CH10_AL1_CTRL_SPEC>`"]
pub type CH10_AL1_CTRL = crate::Reg<ch10_al1_ctrl::CH10_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 10 CTRL register"]
pub mod ch10_al1_ctrl;
#[doc = "CH10_AL1_READ_ADDR register accessor: an alias for `Reg<CH10_AL1_READ_ADDR_SPEC>`"]
pub type CH10_AL1_READ_ADDR = crate::Reg<ch10_al1_read_addr::CH10_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 10 READ_ADDR register"]
pub mod ch10_al1_read_addr;
#[doc = "CH10_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH10_AL1_WRITE_ADDR_SPEC>`"]
pub type CH10_AL1_WRITE_ADDR = crate::Reg<ch10_al1_write_addr::CH10_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 10 WRITE_ADDR register"]
pub mod ch10_al1_write_addr;
#[doc = "CH10_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH10_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH10_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch10_al1_trans_count_trig::CH10_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 10 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch10_al1_trans_count_trig;
#[doc = "CH10_AL2_CTRL register accessor: an alias for `Reg<CH10_AL2_CTRL_SPEC>`"]
pub type CH10_AL2_CTRL = crate::Reg<ch10_al2_ctrl::CH10_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 10 CTRL register"]
pub mod ch10_al2_ctrl;
#[doc = "CH10_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH10_AL2_TRANS_COUNT_SPEC>`"]
pub type CH10_AL2_TRANS_COUNT = crate::Reg<ch10_al2_trans_count::CH10_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 10 TRANS_COUNT register"]
pub mod ch10_al2_trans_count;
#[doc = "CH10_AL2_READ_ADDR register accessor: an alias for `Reg<CH10_AL2_READ_ADDR_SPEC>`"]
pub type CH10_AL2_READ_ADDR = crate::Reg<ch10_al2_read_addr::CH10_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 10 READ_ADDR register"]
pub mod ch10_al2_read_addr;
#[doc = "CH10_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH10_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH10_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch10_al2_write_addr_trig::CH10_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 10 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch10_al2_write_addr_trig;
#[doc = "CH10_AL3_CTRL register accessor: an alias for `Reg<CH10_AL3_CTRL_SPEC>`"]
pub type CH10_AL3_CTRL = crate::Reg<ch10_al3_ctrl::CH10_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 10 CTRL register"]
pub mod ch10_al3_ctrl;
#[doc = "CH10_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH10_AL3_WRITE_ADDR_SPEC>`"]
pub type CH10_AL3_WRITE_ADDR = crate::Reg<ch10_al3_write_addr::CH10_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 10 WRITE_ADDR register"]
pub mod ch10_al3_write_addr;
#[doc = "CH10_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH10_AL3_TRANS_COUNT_SPEC>`"]
pub type CH10_AL3_TRANS_COUNT = crate::Reg<ch10_al3_trans_count::CH10_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 10 TRANS_COUNT register"]
pub mod ch10_al3_trans_count;
#[doc = "CH10_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH10_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH10_AL3_READ_ADDR_TRIG =
    crate::Reg<ch10_al3_read_addr_trig::CH10_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 10 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch10_al3_read_addr_trig;
#[doc = "CH11_READ_ADDR register accessor: an alias for `Reg<CH11_READ_ADDR_SPEC>`"]
pub type CH11_READ_ADDR = crate::Reg<ch11_read_addr::CH11_READ_ADDR_SPEC>;
#[doc = "DMA Channel 11 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch11_read_addr;
#[doc = "CH11_WRITE_ADDR register accessor: an alias for `Reg<CH11_WRITE_ADDR_SPEC>`"]
pub type CH11_WRITE_ADDR = crate::Reg<ch11_write_addr::CH11_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 11 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch11_write_addr;
#[doc = "CH11_TRANS_COUNT register accessor: an alias for `Reg<CH11_TRANS_COUNT_SPEC>`"]
pub type CH11_TRANS_COUNT = crate::Reg<ch11_trans_count::CH11_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 11 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch11_trans_count;
#[doc = "CH11_CTRL_TRIG register accessor: an alias for `Reg<CH11_CTRL_TRIG_SPEC>`"]
pub type CH11_CTRL_TRIG = crate::Reg<ch11_ctrl_trig::CH11_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 11 Control and Status"]
pub mod ch11_ctrl_trig;
#[doc = "CH11_AL1_CTRL register accessor: an alias for `Reg<CH11_AL1_CTRL_SPEC>`"]
pub type CH11_AL1_CTRL = crate::Reg<ch11_al1_ctrl::CH11_AL1_CTRL_SPEC>;
#[doc = "Alias for channel 11 CTRL register"]
pub mod ch11_al1_ctrl;
#[doc = "CH11_AL1_READ_ADDR register accessor: an alias for `Reg<CH11_AL1_READ_ADDR_SPEC>`"]
pub type CH11_AL1_READ_ADDR = crate::Reg<ch11_al1_read_addr::CH11_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 11 READ_ADDR register"]
pub mod ch11_al1_read_addr;
#[doc = "CH11_AL1_WRITE_ADDR register accessor: an alias for `Reg<CH11_AL1_WRITE_ADDR_SPEC>`"]
pub type CH11_AL1_WRITE_ADDR = crate::Reg<ch11_al1_write_addr::CH11_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 11 WRITE_ADDR register"]
pub mod ch11_al1_write_addr;
#[doc = "CH11_AL1_TRANS_COUNT_TRIG register accessor: an alias for `Reg<CH11_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH11_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch11_al1_trans_count_trig::CH11_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 11 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch11_al1_trans_count_trig;
#[doc = "CH11_AL2_CTRL register accessor: an alias for `Reg<CH11_AL2_CTRL_SPEC>`"]
pub type CH11_AL2_CTRL = crate::Reg<ch11_al2_ctrl::CH11_AL2_CTRL_SPEC>;
#[doc = "Alias for channel 11 CTRL register"]
pub mod ch11_al2_ctrl;
#[doc = "CH11_AL2_TRANS_COUNT register accessor: an alias for `Reg<CH11_AL2_TRANS_COUNT_SPEC>`"]
pub type CH11_AL2_TRANS_COUNT = crate::Reg<ch11_al2_trans_count::CH11_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 11 TRANS_COUNT register"]
pub mod ch11_al2_trans_count;
#[doc = "CH11_AL2_READ_ADDR register accessor: an alias for `Reg<CH11_AL2_READ_ADDR_SPEC>`"]
pub type CH11_AL2_READ_ADDR = crate::Reg<ch11_al2_read_addr::CH11_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 11 READ_ADDR register"]
pub mod ch11_al2_read_addr;
#[doc = "CH11_AL2_WRITE_ADDR_TRIG register accessor: an alias for `Reg<CH11_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH11_AL2_WRITE_ADDR_TRIG =
    crate::Reg<ch11_al2_write_addr_trig::CH11_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 11 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch11_al2_write_addr_trig;
#[doc = "CH11_AL3_CTRL register accessor: an alias for `Reg<CH11_AL3_CTRL_SPEC>`"]
pub type CH11_AL3_CTRL = crate::Reg<ch11_al3_ctrl::CH11_AL3_CTRL_SPEC>;
#[doc = "Alias for channel 11 CTRL register"]
pub mod ch11_al3_ctrl;
#[doc = "CH11_AL3_WRITE_ADDR register accessor: an alias for `Reg<CH11_AL3_WRITE_ADDR_SPEC>`"]
pub type CH11_AL3_WRITE_ADDR = crate::Reg<ch11_al3_write_addr::CH11_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 11 WRITE_ADDR register"]
pub mod ch11_al3_write_addr;
#[doc = "CH11_AL3_TRANS_COUNT register accessor: an alias for `Reg<CH11_AL3_TRANS_COUNT_SPEC>`"]
pub type CH11_AL3_TRANS_COUNT = crate::Reg<ch11_al3_trans_count::CH11_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 11 TRANS_COUNT register"]
pub mod ch11_al3_trans_count;
#[doc = "CH11_AL3_READ_ADDR_TRIG register accessor: an alias for `Reg<CH11_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH11_AL3_READ_ADDR_TRIG =
    crate::Reg<ch11_al3_read_addr_trig::CH11_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 11 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch11_al3_read_addr_trig;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt Status (raw)"]
pub mod intr;
#[doc = "INTE0 register accessor: an alias for `Reg<INTE0_SPEC>`"]
pub type INTE0 = crate::Reg<inte0::INTE0_SPEC>;
#[doc = "Interrupt Enables for IRQ 0"]
pub mod inte0;
#[doc = "INTF0 register accessor: an alias for `Reg<INTF0_SPEC>`"]
pub type INTF0 = crate::Reg<intf0::INTF0_SPEC>;
#[doc = "Force Interrupts"]
pub mod intf0;
#[doc = "INTS0 register accessor: an alias for `Reg<INTS0_SPEC>`"]
pub type INTS0 = crate::Reg<ints0::INTS0_SPEC>;
#[doc = "Interrupt Status for IRQ 0"]
pub mod ints0;
#[doc = "INTE1 register accessor: an alias for `Reg<INTE1_SPEC>`"]
pub type INTE1 = crate::Reg<inte1::INTE1_SPEC>;
#[doc = "Interrupt Enables for IRQ 1"]
pub mod inte1;
#[doc = "INTF1 register accessor: an alias for `Reg<INTF1_SPEC>`"]
pub type INTF1 = crate::Reg<intf1::INTF1_SPEC>;
#[doc = "Force Interrupts for IRQ 1"]
pub mod intf1;
#[doc = "INTS1 register accessor: an alias for `Reg<INTS1_SPEC>`"]
pub type INTS1 = crate::Reg<ints1::INTS1_SPEC>;
#[doc = "Interrupt Status (masked) for IRQ 1"]
pub mod ints1;
#[doc = "TIMER0 register accessor: an alias for `Reg<TIMER0_SPEC>`"]
pub type TIMER0 = crate::Reg<timer0::TIMER0_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer0;
#[doc = "TIMER1 register accessor: an alias for `Reg<TIMER1_SPEC>`"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer1;
#[doc = "MULTI_CHAN_TRIGGER register accessor: an alias for `Reg<MULTI_CHAN_TRIGGER_SPEC>`"]
pub type MULTI_CHAN_TRIGGER = crate::Reg<multi_chan_trigger::MULTI_CHAN_TRIGGER_SPEC>;
#[doc = "Trigger one or more channels simultaneously"]
pub mod multi_chan_trigger;
#[doc = "SNIFF_CTRL register accessor: an alias for `Reg<SNIFF_CTRL_SPEC>`"]
pub type SNIFF_CTRL = crate::Reg<sniff_ctrl::SNIFF_CTRL_SPEC>;
#[doc = "Sniffer Control"]
pub mod sniff_ctrl;
#[doc = "SNIFF_DATA register accessor: an alias for `Reg<SNIFF_DATA_SPEC>`"]
pub type SNIFF_DATA = crate::Reg<sniff_data::SNIFF_DATA_SPEC>;
#[doc = "Data accumulator for sniff hardware\\n Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
pub mod sniff_data;
#[doc = "FIFO_LEVELS register accessor: an alias for `Reg<FIFO_LEVELS_SPEC>`"]
pub type FIFO_LEVELS = crate::Reg<fifo_levels::FIFO_LEVELS_SPEC>;
#[doc = "Debug RAF, WAF, TDF levels"]
pub mod fifo_levels;
#[doc = "CHAN_ABORT register accessor: an alias for `Reg<CHAN_ABORT_SPEC>`"]
pub type CHAN_ABORT = crate::Reg<chan_abort::CHAN_ABORT_SPEC>;
#[doc = "Abort an in-progress transfer sequence on one or more channels"]
pub mod chan_abort;
#[doc = "N_CHANNELS register accessor: an alias for `Reg<N_CHANNELS_SPEC>`"]
pub type N_CHANNELS = crate::Reg<n_channels::N_CHANNELS_SPEC>;
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
pub mod n_channels;
#[doc = "CH0_DBG_CTDREQ register accessor: an alias for `Reg<CH0_DBG_CTDREQ_SPEC>`"]
pub type CH0_DBG_CTDREQ = crate::Reg<ch0_dbg_ctdreq::CH0_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch0_dbg_ctdreq;
#[doc = "CH0_DBG_TCR register accessor: an alias for `Reg<CH0_DBG_TCR_SPEC>`"]
pub type CH0_DBG_TCR = crate::Reg<ch0_dbg_tcr::CH0_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch0_dbg_tcr;
#[doc = "CH1_DBG_CTDREQ register accessor: an alias for `Reg<CH1_DBG_CTDREQ_SPEC>`"]
pub type CH1_DBG_CTDREQ = crate::Reg<ch1_dbg_ctdreq::CH1_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch1_dbg_ctdreq;
#[doc = "CH1_DBG_TCR register accessor: an alias for `Reg<CH1_DBG_TCR_SPEC>`"]
pub type CH1_DBG_TCR = crate::Reg<ch1_dbg_tcr::CH1_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch1_dbg_tcr;
#[doc = "CH2_DBG_CTDREQ register accessor: an alias for `Reg<CH2_DBG_CTDREQ_SPEC>`"]
pub type CH2_DBG_CTDREQ = crate::Reg<ch2_dbg_ctdreq::CH2_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch2_dbg_ctdreq;
#[doc = "CH2_DBG_TCR register accessor: an alias for `Reg<CH2_DBG_TCR_SPEC>`"]
pub type CH2_DBG_TCR = crate::Reg<ch2_dbg_tcr::CH2_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch2_dbg_tcr;
#[doc = "CH3_DBG_CTDREQ register accessor: an alias for `Reg<CH3_DBG_CTDREQ_SPEC>`"]
pub type CH3_DBG_CTDREQ = crate::Reg<ch3_dbg_ctdreq::CH3_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch3_dbg_ctdreq;
#[doc = "CH3_DBG_TCR register accessor: an alias for `Reg<CH3_DBG_TCR_SPEC>`"]
pub type CH3_DBG_TCR = crate::Reg<ch3_dbg_tcr::CH3_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch3_dbg_tcr;
#[doc = "CH4_DBG_CTDREQ register accessor: an alias for `Reg<CH4_DBG_CTDREQ_SPEC>`"]
pub type CH4_DBG_CTDREQ = crate::Reg<ch4_dbg_ctdreq::CH4_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch4_dbg_ctdreq;
#[doc = "CH4_DBG_TCR register accessor: an alias for `Reg<CH4_DBG_TCR_SPEC>`"]
pub type CH4_DBG_TCR = crate::Reg<ch4_dbg_tcr::CH4_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch4_dbg_tcr;
#[doc = "CH5_DBG_CTDREQ register accessor: an alias for `Reg<CH5_DBG_CTDREQ_SPEC>`"]
pub type CH5_DBG_CTDREQ = crate::Reg<ch5_dbg_ctdreq::CH5_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch5_dbg_ctdreq;
#[doc = "CH5_DBG_TCR register accessor: an alias for `Reg<CH5_DBG_TCR_SPEC>`"]
pub type CH5_DBG_TCR = crate::Reg<ch5_dbg_tcr::CH5_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch5_dbg_tcr;
#[doc = "CH6_DBG_CTDREQ register accessor: an alias for `Reg<CH6_DBG_CTDREQ_SPEC>`"]
pub type CH6_DBG_CTDREQ = crate::Reg<ch6_dbg_ctdreq::CH6_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch6_dbg_ctdreq;
#[doc = "CH6_DBG_TCR register accessor: an alias for `Reg<CH6_DBG_TCR_SPEC>`"]
pub type CH6_DBG_TCR = crate::Reg<ch6_dbg_tcr::CH6_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch6_dbg_tcr;
#[doc = "CH7_DBG_CTDREQ register accessor: an alias for `Reg<CH7_DBG_CTDREQ_SPEC>`"]
pub type CH7_DBG_CTDREQ = crate::Reg<ch7_dbg_ctdreq::CH7_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch7_dbg_ctdreq;
#[doc = "CH7_DBG_TCR register accessor: an alias for `Reg<CH7_DBG_TCR_SPEC>`"]
pub type CH7_DBG_TCR = crate::Reg<ch7_dbg_tcr::CH7_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch7_dbg_tcr;
#[doc = "CH8_DBG_CTDREQ register accessor: an alias for `Reg<CH8_DBG_CTDREQ_SPEC>`"]
pub type CH8_DBG_CTDREQ = crate::Reg<ch8_dbg_ctdreq::CH8_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch8_dbg_ctdreq;
#[doc = "CH8_DBG_TCR register accessor: an alias for `Reg<CH8_DBG_TCR_SPEC>`"]
pub type CH8_DBG_TCR = crate::Reg<ch8_dbg_tcr::CH8_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch8_dbg_tcr;
#[doc = "CH9_DBG_CTDREQ register accessor: an alias for `Reg<CH9_DBG_CTDREQ_SPEC>`"]
pub type CH9_DBG_CTDREQ = crate::Reg<ch9_dbg_ctdreq::CH9_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch9_dbg_ctdreq;
#[doc = "CH9_DBG_TCR register accessor: an alias for `Reg<CH9_DBG_TCR_SPEC>`"]
pub type CH9_DBG_TCR = crate::Reg<ch9_dbg_tcr::CH9_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch9_dbg_tcr;
#[doc = "CH10_DBG_CTDREQ register accessor: an alias for `Reg<CH10_DBG_CTDREQ_SPEC>`"]
pub type CH10_DBG_CTDREQ = crate::Reg<ch10_dbg_ctdreq::CH10_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch10_dbg_ctdreq;
#[doc = "CH10_DBG_TCR register accessor: an alias for `Reg<CH10_DBG_TCR_SPEC>`"]
pub type CH10_DBG_TCR = crate::Reg<ch10_dbg_tcr::CH10_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch10_dbg_tcr;
#[doc = "CH11_DBG_CTDREQ register accessor: an alias for `Reg<CH11_DBG_CTDREQ_SPEC>`"]
pub type CH11_DBG_CTDREQ = crate::Reg<ch11_dbg_ctdreq::CH11_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch11_dbg_ctdreq;
#[doc = "CH11_DBG_TCR register accessor: an alias for `Reg<CH11_DBG_TCR_SPEC>`"]
pub type CH11_DBG_TCR = crate::Reg<ch11_dbg_tcr::CH11_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch11_dbg_tcr;
