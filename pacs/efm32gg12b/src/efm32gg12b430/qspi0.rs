#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Octal-SPI Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x04 - Device Read Instruction Configuration Register"]
    pub devinstrrdconfig: DEVINSTRRDCONFIG,
    #[doc = "0x08 - Device Write Instruction Configuration Register"]
    pub devinstrwrconfig: DEVINSTRWRCONFIG,
    #[doc = "0x0c - Device Delay Register"]
    pub devdelay: DEVDELAY,
    #[doc = "0x10 - Read Data Capture Register"]
    pub rddatacapture: RDDATACAPTURE,
    #[doc = "0x14 - Device Size Configuration Register"]
    pub devsizeconfig: DEVSIZECONFIG,
    #[doc = "0x18 - SRAM Partition Configuration Register"]
    pub srampartitioncfg: SRAMPARTITIONCFG,
    #[doc = "0x1c - Indirect Address Trigger Register"]
    pub indahbaddrtrigger: INDAHBADDRTRIGGER,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - Remap Address Register"]
    pub remapaddr: REMAPADDR,
    #[doc = "0x28 - Mode Bit Configuration Register"]
    pub modebitconfig: MODEBITCONFIG,
    #[doc = "0x2c - SRAM Fill Register"]
    pub sramfill: SRAMFILL,
    #[doc = "0x30 - TX Threshold Register"]
    pub txthresh: TXTHRESH,
    #[doc = "0x34 - RX Threshold Register"]
    pub rxthresh: RXTHRESH,
    #[doc = "0x38 - Write Completion Control Register"]
    pub writecompletionctrl: WRITECOMPLETIONCTRL,
    #[doc = "0x3c - Polling Expiration Register"]
    pub noofpollsbefexp: NOOFPOLLSBEFEXP,
    #[doc = "0x40 - Interrupt Status Register"]
    pub irqstatus: IRQSTATUS,
    #[doc = "0x44 - Interrupt Mask"]
    pub irqmask: IRQMASK,
    _reserved17: [u8; 0x08],
    #[doc = "0x50 - Lower Write Protection Register"]
    pub lowerwrprot: LOWERWRPROT,
    #[doc = "0x54 - Upper Write Protection Register"]
    pub upperwrprot: UPPERWRPROT,
    #[doc = "0x58 - Write Protection Control Register"]
    pub wrprotctrl: WRPROTCTRL,
    _reserved20: [u8; 0x04],
    #[doc = "0x60 - Indirect Read Transfer Control Register"]
    pub indirectreadxferctrl: INDIRECTREADXFERCTRL,
    #[doc = "0x64 - Indirect Read Transfer Watermark Register"]
    pub indirectreadxferwatermark: INDIRECTREADXFERWATERMARK,
    #[doc = "0x68 - Indirect Read Transfer Start Address Register"]
    pub indirectreadxferstart: INDIRECTREADXFERSTART,
    #[doc = "0x6c - Indirect Read Transfer Number Bytes Register"]
    pub indirectreadxfernumbytes: INDIRECTREADXFERNUMBYTES,
    #[doc = "0x70 - Indirect Write Transfer Control Register"]
    pub indirectwritexferctrl: INDIRECTWRITEXFERCTRL,
    #[doc = "0x74 - Indirect Write Transfer Watermark Register"]
    pub indirectwritexferwatermark: INDIRECTWRITEXFERWATERMARK,
    #[doc = "0x78 - Indirect Write Transfer Start Address Register"]
    pub indirectwritexferstart: INDIRECTWRITEXFERSTART,
    #[doc = "0x7c - Indirect Write Transfer Number Bytes Register"]
    pub indirectwritexfernumbytes: INDIRECTWRITEXFERNUMBYTES,
    #[doc = "0x80 - Indirect Trigger Address Range Register"]
    pub indirecttriggeraddrrange: INDIRECTTRIGGERADDRRANGE,
    _reserved29: [u8; 0x08],
    #[doc = "0x8c - Flash Command Control Memory Register (STIG)"]
    pub flashcommandctrlmem: FLASHCOMMANDCTRLMEM,
    #[doc = "0x90 - Flash Command Control Register (STIG)"]
    pub flashcmdctrl: FLASHCMDCTRL,
    #[doc = "0x94 - Flash Command Address Register (STIG)"]
    pub flashcmdaddr: FLASHCMDADDR,
    _reserved32: [u8; 0x08],
    #[doc = "0xa0 - Flash Command Read Data Register (Lower) (STIG)"]
    pub flashrddatalower: FLASHRDDATALOWER,
    #[doc = "0xa4 - Flash Command Read Data Register (Upper) (STIG)"]
    pub flashrddataupper: FLASHRDDATAUPPER,
    #[doc = "0xa8 - Flash Command Write Data Register (Lower) (STIG)"]
    pub flashwrdatalower: FLASHWRDATALOWER,
    #[doc = "0xac - Flash Command Write Data Register (Upper) (STIG)"]
    pub flashwrdataupper: FLASHWRDATAUPPER,
    #[doc = "0xb0 - Polling Flash Status Register"]
    pub pollingflashstatus: POLLINGFLASHSTATUS,
    #[doc = "0xb4 - PHY Configuration Register"]
    pub phyconfiguration: PHYCONFIGURATION,
    _reserved38: [u8; 0x28],
    #[doc = "0xe0 - Opcode Extension Register (Lower)"]
    pub opcodeextlower: OPCODEEXTLOWER,
    #[doc = "0xe4 - Opcode Extension Register (Upper)"]
    pub opcodeextupper: OPCODEEXTUPPER,
    _reserved40: [u8; 0x14],
    #[doc = "0xfc - Module ID Register"]
    pub moduleid: MODULEID,
    _reserved41: [u8; 0x04],
    #[doc = "0x104 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x108 - I/O Route Location Register 0"]
    pub routeloc0: ROUTELOC0,
}
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Octal-SPI Configuration Register"]
pub mod config;
#[doc = "DEVINSTRRDCONFIG (rw) register accessor: an alias for `Reg<DEVINSTRRDCONFIG_SPEC>`"]
pub type DEVINSTRRDCONFIG = crate::Reg<devinstrrdconfig::DEVINSTRRDCONFIG_SPEC>;
#[doc = "Device Read Instruction Configuration Register"]
pub mod devinstrrdconfig;
#[doc = "DEVINSTRWRCONFIG (rw) register accessor: an alias for `Reg<DEVINSTRWRCONFIG_SPEC>`"]
pub type DEVINSTRWRCONFIG = crate::Reg<devinstrwrconfig::DEVINSTRWRCONFIG_SPEC>;
#[doc = "Device Write Instruction Configuration Register"]
pub mod devinstrwrconfig;
#[doc = "DEVDELAY (rw) register accessor: an alias for `Reg<DEVDELAY_SPEC>`"]
pub type DEVDELAY = crate::Reg<devdelay::DEVDELAY_SPEC>;
#[doc = "Device Delay Register"]
pub mod devdelay;
#[doc = "RDDATACAPTURE (rw) register accessor: an alias for `Reg<RDDATACAPTURE_SPEC>`"]
pub type RDDATACAPTURE = crate::Reg<rddatacapture::RDDATACAPTURE_SPEC>;
#[doc = "Read Data Capture Register"]
pub mod rddatacapture;
#[doc = "DEVSIZECONFIG (rw) register accessor: an alias for `Reg<DEVSIZECONFIG_SPEC>`"]
pub type DEVSIZECONFIG = crate::Reg<devsizeconfig::DEVSIZECONFIG_SPEC>;
#[doc = "Device Size Configuration Register"]
pub mod devsizeconfig;
#[doc = "SRAMPARTITIONCFG (rw) register accessor: an alias for `Reg<SRAMPARTITIONCFG_SPEC>`"]
pub type SRAMPARTITIONCFG = crate::Reg<srampartitioncfg::SRAMPARTITIONCFG_SPEC>;
#[doc = "SRAM Partition Configuration Register"]
pub mod srampartitioncfg;
#[doc = "INDAHBADDRTRIGGER (rw) register accessor: an alias for `Reg<INDAHBADDRTRIGGER_SPEC>`"]
pub type INDAHBADDRTRIGGER = crate::Reg<indahbaddrtrigger::INDAHBADDRTRIGGER_SPEC>;
#[doc = "Indirect Address Trigger Register"]
pub mod indahbaddrtrigger;
#[doc = "REMAPADDR (rw) register accessor: an alias for `Reg<REMAPADDR_SPEC>`"]
pub type REMAPADDR = crate::Reg<remapaddr::REMAPADDR_SPEC>;
#[doc = "Remap Address Register"]
pub mod remapaddr;
#[doc = "MODEBITCONFIG (rw) register accessor: an alias for `Reg<MODEBITCONFIG_SPEC>`"]
pub type MODEBITCONFIG = crate::Reg<modebitconfig::MODEBITCONFIG_SPEC>;
#[doc = "Mode Bit Configuration Register"]
pub mod modebitconfig;
#[doc = "SRAMFILL (r) register accessor: an alias for `Reg<SRAMFILL_SPEC>`"]
pub type SRAMFILL = crate::Reg<sramfill::SRAMFILL_SPEC>;
#[doc = "SRAM Fill Register"]
pub mod sramfill;
#[doc = "TXTHRESH (rw) register accessor: an alias for `Reg<TXTHRESH_SPEC>`"]
pub type TXTHRESH = crate::Reg<txthresh::TXTHRESH_SPEC>;
#[doc = "TX Threshold Register"]
pub mod txthresh;
#[doc = "RXTHRESH (rw) register accessor: an alias for `Reg<RXTHRESH_SPEC>`"]
pub type RXTHRESH = crate::Reg<rxthresh::RXTHRESH_SPEC>;
#[doc = "RX Threshold Register"]
pub mod rxthresh;
#[doc = "WRITECOMPLETIONCTRL (rw) register accessor: an alias for `Reg<WRITECOMPLETIONCTRL_SPEC>`"]
pub type WRITECOMPLETIONCTRL = crate::Reg<writecompletionctrl::WRITECOMPLETIONCTRL_SPEC>;
#[doc = "Write Completion Control Register"]
pub mod writecompletionctrl;
#[doc = "NOOFPOLLSBEFEXP (rw) register accessor: an alias for `Reg<NOOFPOLLSBEFEXP_SPEC>`"]
pub type NOOFPOLLSBEFEXP = crate::Reg<noofpollsbefexp::NOOFPOLLSBEFEXP_SPEC>;
#[doc = "Polling Expiration Register"]
pub mod noofpollsbefexp;
#[doc = "IRQSTATUS (rw) register accessor: an alias for `Reg<IRQSTATUS_SPEC>`"]
pub type IRQSTATUS = crate::Reg<irqstatus::IRQSTATUS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod irqstatus;
#[doc = "IRQMASK (rw) register accessor: an alias for `Reg<IRQMASK_SPEC>`"]
pub type IRQMASK = crate::Reg<irqmask::IRQMASK_SPEC>;
#[doc = "Interrupt Mask"]
pub mod irqmask;
#[doc = "LOWERWRPROT (rw) register accessor: an alias for `Reg<LOWERWRPROT_SPEC>`"]
pub type LOWERWRPROT = crate::Reg<lowerwrprot::LOWERWRPROT_SPEC>;
#[doc = "Lower Write Protection Register"]
pub mod lowerwrprot;
#[doc = "UPPERWRPROT (rw) register accessor: an alias for `Reg<UPPERWRPROT_SPEC>`"]
pub type UPPERWRPROT = crate::Reg<upperwrprot::UPPERWRPROT_SPEC>;
#[doc = "Upper Write Protection Register"]
pub mod upperwrprot;
#[doc = "WRPROTCTRL (rw) register accessor: an alias for `Reg<WRPROTCTRL_SPEC>`"]
pub type WRPROTCTRL = crate::Reg<wrprotctrl::WRPROTCTRL_SPEC>;
#[doc = "Write Protection Control Register"]
pub mod wrprotctrl;
#[doc = "INDIRECTREADXFERCTRL (rw) register accessor: an alias for `Reg<INDIRECTREADXFERCTRL_SPEC>`"]
pub type INDIRECTREADXFERCTRL = crate::Reg<indirectreadxferctrl::INDIRECTREADXFERCTRL_SPEC>;
#[doc = "Indirect Read Transfer Control Register"]
pub mod indirectreadxferctrl;
#[doc = "INDIRECTREADXFERWATERMARK (rw) register accessor: an alias for `Reg<INDIRECTREADXFERWATERMARK_SPEC>`"]
pub type INDIRECTREADXFERWATERMARK =
    crate::Reg<indirectreadxferwatermark::INDIRECTREADXFERWATERMARK_SPEC>;
#[doc = "Indirect Read Transfer Watermark Register"]
pub mod indirectreadxferwatermark;
#[doc = "INDIRECTREADXFERSTART (rw) register accessor: an alias for `Reg<INDIRECTREADXFERSTART_SPEC>`"]
pub type INDIRECTREADXFERSTART = crate::Reg<indirectreadxferstart::INDIRECTREADXFERSTART_SPEC>;
#[doc = "Indirect Read Transfer Start Address Register"]
pub mod indirectreadxferstart;
#[doc = "INDIRECTREADXFERNUMBYTES (rw) register accessor: an alias for `Reg<INDIRECTREADXFERNUMBYTES_SPEC>`"]
pub type INDIRECTREADXFERNUMBYTES =
    crate::Reg<indirectreadxfernumbytes::INDIRECTREADXFERNUMBYTES_SPEC>;
#[doc = "Indirect Read Transfer Number Bytes Register"]
pub mod indirectreadxfernumbytes;
#[doc = "INDIRECTWRITEXFERCTRL (rw) register accessor: an alias for `Reg<INDIRECTWRITEXFERCTRL_SPEC>`"]
pub type INDIRECTWRITEXFERCTRL = crate::Reg<indirectwritexferctrl::INDIRECTWRITEXFERCTRL_SPEC>;
#[doc = "Indirect Write Transfer Control Register"]
pub mod indirectwritexferctrl;
#[doc = "INDIRECTWRITEXFERWATERMARK (rw) register accessor: an alias for `Reg<INDIRECTWRITEXFERWATERMARK_SPEC>`"]
pub type INDIRECTWRITEXFERWATERMARK =
    crate::Reg<indirectwritexferwatermark::INDIRECTWRITEXFERWATERMARK_SPEC>;
#[doc = "Indirect Write Transfer Watermark Register"]
pub mod indirectwritexferwatermark;
#[doc = "INDIRECTWRITEXFERSTART (rw) register accessor: an alias for `Reg<INDIRECTWRITEXFERSTART_SPEC>`"]
pub type INDIRECTWRITEXFERSTART = crate::Reg<indirectwritexferstart::INDIRECTWRITEXFERSTART_SPEC>;
#[doc = "Indirect Write Transfer Start Address Register"]
pub mod indirectwritexferstart;
#[doc = "INDIRECTWRITEXFERNUMBYTES (rw) register accessor: an alias for `Reg<INDIRECTWRITEXFERNUMBYTES_SPEC>`"]
pub type INDIRECTWRITEXFERNUMBYTES =
    crate::Reg<indirectwritexfernumbytes::INDIRECTWRITEXFERNUMBYTES_SPEC>;
#[doc = "Indirect Write Transfer Number Bytes Register"]
pub mod indirectwritexfernumbytes;
#[doc = "INDIRECTTRIGGERADDRRANGE (rw) register accessor: an alias for `Reg<INDIRECTTRIGGERADDRRANGE_SPEC>`"]
pub type INDIRECTTRIGGERADDRRANGE =
    crate::Reg<indirecttriggeraddrrange::INDIRECTTRIGGERADDRRANGE_SPEC>;
#[doc = "Indirect Trigger Address Range Register"]
pub mod indirecttriggeraddrrange;
#[doc = "FLASHCOMMANDCTRLMEM (rw) register accessor: an alias for `Reg<FLASHCOMMANDCTRLMEM_SPEC>`"]
pub type FLASHCOMMANDCTRLMEM = crate::Reg<flashcommandctrlmem::FLASHCOMMANDCTRLMEM_SPEC>;
#[doc = "Flash Command Control Memory Register (STIG)"]
pub mod flashcommandctrlmem;
#[doc = "FLASHCMDCTRL (rw) register accessor: an alias for `Reg<FLASHCMDCTRL_SPEC>`"]
pub type FLASHCMDCTRL = crate::Reg<flashcmdctrl::FLASHCMDCTRL_SPEC>;
#[doc = "Flash Command Control Register (STIG)"]
pub mod flashcmdctrl;
#[doc = "FLASHCMDADDR (rw) register accessor: an alias for `Reg<FLASHCMDADDR_SPEC>`"]
pub type FLASHCMDADDR = crate::Reg<flashcmdaddr::FLASHCMDADDR_SPEC>;
#[doc = "Flash Command Address Register (STIG)"]
pub mod flashcmdaddr;
#[doc = "FLASHRDDATALOWER (r) register accessor: an alias for `Reg<FLASHRDDATALOWER_SPEC>`"]
pub type FLASHRDDATALOWER = crate::Reg<flashrddatalower::FLASHRDDATALOWER_SPEC>;
#[doc = "Flash Command Read Data Register (Lower) (STIG)"]
pub mod flashrddatalower;
#[doc = "FLASHRDDATAUPPER (r) register accessor: an alias for `Reg<FLASHRDDATAUPPER_SPEC>`"]
pub type FLASHRDDATAUPPER = crate::Reg<flashrddataupper::FLASHRDDATAUPPER_SPEC>;
#[doc = "Flash Command Read Data Register (Upper) (STIG)"]
pub mod flashrddataupper;
#[doc = "FLASHWRDATALOWER (rw) register accessor: an alias for `Reg<FLASHWRDATALOWER_SPEC>`"]
pub type FLASHWRDATALOWER = crate::Reg<flashwrdatalower::FLASHWRDATALOWER_SPEC>;
#[doc = "Flash Command Write Data Register (Lower) (STIG)"]
pub mod flashwrdatalower;
#[doc = "FLASHWRDATAUPPER (rw) register accessor: an alias for `Reg<FLASHWRDATAUPPER_SPEC>`"]
pub type FLASHWRDATAUPPER = crate::Reg<flashwrdataupper::FLASHWRDATAUPPER_SPEC>;
#[doc = "Flash Command Write Data Register (Upper) (STIG)"]
pub mod flashwrdataupper;
#[doc = "POLLINGFLASHSTATUS (rw) register accessor: an alias for `Reg<POLLINGFLASHSTATUS_SPEC>`"]
pub type POLLINGFLASHSTATUS = crate::Reg<pollingflashstatus::POLLINGFLASHSTATUS_SPEC>;
#[doc = "Polling Flash Status Register"]
pub mod pollingflashstatus;
#[doc = "PHYCONFIGURATION (rw) register accessor: an alias for `Reg<PHYCONFIGURATION_SPEC>`"]
pub type PHYCONFIGURATION = crate::Reg<phyconfiguration::PHYCONFIGURATION_SPEC>;
#[doc = "PHY Configuration Register"]
pub mod phyconfiguration;
#[doc = "OPCODEEXTLOWER (rw) register accessor: an alias for `Reg<OPCODEEXTLOWER_SPEC>`"]
pub type OPCODEEXTLOWER = crate::Reg<opcodeextlower::OPCODEEXTLOWER_SPEC>;
#[doc = "Opcode Extension Register (Lower)"]
pub mod opcodeextlower;
#[doc = "OPCODEEXTUPPER (rw) register accessor: an alias for `Reg<OPCODEEXTUPPER_SPEC>`"]
pub type OPCODEEXTUPPER = crate::Reg<opcodeextupper::OPCODEEXTUPPER_SPEC>;
#[doc = "Opcode Extension Register (Upper)"]
pub mod opcodeextupper;
#[doc = "MODULEID (r) register accessor: an alias for `Reg<MODULEID_SPEC>`"]
pub type MODULEID = crate::Reg<moduleid::MODULEID_SPEC>;
#[doc = "Module ID Register"]
pub mod moduleid;
#[doc = "ROUTEPEN (rw) register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
