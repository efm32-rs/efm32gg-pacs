#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network control register"]
    pub networkctrl: NETWORKCTRL,
    #[doc = "0x04 - Network configuration register"]
    pub networkcfg: NETWORKCFG,
    #[doc = "0x08 - Network status register"]
    pub networkstatus: NETWORKSTATUS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - DMA Configuration Register"]
    pub dmacfg: DMACFG,
    #[doc = "0x14 - Transmit status register"]
    pub txstatus: TXSTATUS,
    #[doc = "0x18 - Start address of the receive buffer queue"]
    pub rxqptr: RXQPTR,
    #[doc = "0x1c - Start address of the transmit buffer queue"]
    pub txqptr: TXQPTR,
    #[doc = "0x20 - Receive status register"]
    pub rxstatus: RXSTATUS,
    #[doc = "0x24 - Interrupt status register"]
    pub ifcr: IFCR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub iens: IENS,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub ienc: IENC,
    #[doc = "0x30 - Interrupt mask register"]
    pub ienro: IENRO,
    #[doc = "0x34 - PHY management register"]
    pub phymngmnt: PHYMNGMNT,
    #[doc = "0x38 - Received Pause Quantum Register"]
    pub rxpausequant: RXPAUSEQUANT,
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    pub txpausequant: TXPAUSEQUANT,
    #[doc = "0x40 - TX Partial Store and Forward"]
    pub pbuftxcutthru: PBUFTXCUTTHRU,
    #[doc = "0x44 - RX Partial Store and Forward"]
    pub pbufrxcutthru: PBUFRXCUTTHRU,
    #[doc = "0x48 - Maximum Jumbo Frame Size."]
    pub jumbomaxlen: JUMBOMAXLEN,
    _reserved18: [u8; 0x10],
    #[doc = "0x5c - Interrupt moderation register"]
    pub imod: IMOD,
    #[doc = "0x60 - System wake time"]
    pub syswaketime: SYSWAKETIME,
    _reserved20: [u8; 0x1c],
    #[doc = "0x80 - Hash Register Bottom \\[31:0\\]"]
    pub hashbottom: HASHBOTTOM,
    #[doc = "0x84 - Hash Register Top \\[63:32\\]"]
    pub hashtop: HASHTOP,
    #[doc = "0x88 - Specific Address 1 Bottom"]
    pub specaddr1bottom: SPECADDR1BOTTOM,
    #[doc = "0x8c - Specific Address 1 Top"]
    pub specaddr1top: SPECADDR1TOP,
    #[doc = "0x90 - Specific Address 2 Bottom"]
    pub specaddr2bottom: SPECADDR2BOTTOM,
    #[doc = "0x94 - Specific Address 2 Top"]
    pub specaddr2top: SPECADDR2TOP,
    #[doc = "0x98 - Specific Address 3 Bottom"]
    pub specaddr3bottom: SPECADDR3BOTTOM,
    #[doc = "0x9c - Specific Address 3 Top"]
    pub specaddr3top: SPECADDR3TOP,
    #[doc = "0xa0 - Specific Address 4 Bottom"]
    pub specaddr4bottom: SPECADDR4BOTTOM,
    #[doc = "0xa4 - Specific Address 4 Top"]
    pub specaddr4top: SPECADDR4TOP,
    #[doc = "0xa8 - Type ID Match 1"]
    pub spectype1: SPECTYPE1,
    #[doc = "0xac - Type ID Match 2"]
    pub spectype2: SPECTYPE2,
    #[doc = "0xb0 - Type ID Match 3"]
    pub spectype3: SPECTYPE3,
    #[doc = "0xb4 - Type ID Match 4"]
    pub spectype4: SPECTYPE4,
    #[doc = "0xb8 - Wake on LAN Register"]
    pub wolreg: WOLREG,
    #[doc = "0xbc - IPG stretch register"]
    pub stretchratio: STRETCHRATIO,
    #[doc = "0xc0 - Stacked VLAN Register"]
    pub stackedvlan: STACKEDVLAN,
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    pub txpfcpause: TXPFCPAUSE,
    #[doc = "0xc8 - Specific Address Mask 1 Bottom 31:0"]
    pub maskadd1bottom: MASKADD1BOTTOM,
    #[doc = "0xcc - Specific Address Mask 1 Top 47:32"]
    pub maskadd1top: MASKADD1TOP,
    _reserved40: [u8; 0x04],
    #[doc = "0xd4 - PTP RX unicast IP destination address"]
    pub rxptpunicast: RXPTPUNICAST,
    #[doc = "0xd8 - PTP TX unicast IP destination address"]
    pub txptpunicast: TXPTPUNICAST,
    #[doc = "0xdc - TSU timer comparison value nanoseconds"]
    pub tsunseccmp: TSUNSECCMP,
    #[doc = "0xe0 - TSU timer comparison value seconds \\[31:0\\]"]
    pub tsuseccmp: TSUSECCMP,
    #[doc = "0xe4 - TSU timer comparison value seconds \\[47:32\\]"]
    pub tsumsbseccmp: TSUMSBSECCMP,
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds Register 47:32"]
    pub tsuptptxmsbsec: TSUPTPTXMSBSEC,
    #[doc = "0xec - PTP Event Frame Received Seconds Register 47:32"]
    pub tsuptprxmsbsec: TSUPTPRXMSBSEC,
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds Register 47:32"]
    pub tsupeertxmsbsec: TSUPEERTXMSBSEC,
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds Register 47:32"]
    pub tsupeerrxmsbsec: TSUPEERRXMSBSEC,
    _reserved49: [u8; 0x08],
    #[doc = "0x100 - Octets transmitted 31:0"]
    pub octetstxedbottom: OCTETSTXEDBOTTOM,
    #[doc = "0x104 - Octets Transmitted 47:32"]
    pub octetstxedtop: OCTETSTXEDTOP,
    #[doc = "0x108 - Frames Transmitted"]
    pub framestxedok: FRAMESTXEDOK,
    #[doc = "0x10c - Broadcast Frames Transmitted"]
    pub broadcasttxed: BROADCASTTXED,
    #[doc = "0x110 - Multicast Frames Transmitted"]
    pub multicasttxed: MULTICASTTXED,
    #[doc = "0x114 - Pause Frames Transmitted"]
    pub pframestxed: PFRAMESTXED,
    #[doc = "0x118 - 64 Byte Frames Transmitted"]
    pub framestxed64: FRAMESTXED64,
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted"]
    pub framestxed65: FRAMESTXED65,
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted"]
    pub framestxed128: FRAMESTXED128,
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted"]
    pub framestxed256: FRAMESTXED256,
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted"]
    pub framestxed512: FRAMESTXED512,
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted"]
    pub framestxed1024: FRAMESTXED1024,
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted"]
    pub framestxed1519: FRAMESTXED1519,
    #[doc = "0x134 - Transmit Under Runs"]
    pub txunderruns: TXUNDERRUNS,
    #[doc = "0x138 - Single Collision Frames"]
    pub singlecols: SINGLECOLS,
    #[doc = "0x13c - Multiple Collision Frames"]
    pub multicols: MULTICOLS,
    #[doc = "0x140 - Excessive Collisions"]
    pub excesscols: EXCESSCOLS,
    #[doc = "0x144 - Late Collisions"]
    pub latecols: LATECOLS,
    #[doc = "0x148 - Deferred Transmission Frames"]
    pub deferredframes: DEFERREDFRAMES,
    #[doc = "0x14c - Carrier Sense Errors"]
    pub crserrs: CRSERRS,
    #[doc = "0x150 - Octets Received 31:0"]
    pub octetsrxedbottom: OCTETSRXEDBOTTOM,
    #[doc = "0x154 - Octets Received 47:32"]
    pub octetsrxedtop: OCTETSRXEDTOP,
    #[doc = "0x158 - Frames Received"]
    pub framesrxedok: FRAMESRXEDOK,
    #[doc = "0x15c - Broadcast Frames Received"]
    pub broadcastrxed: BROADCASTRXED,
    #[doc = "0x160 - Multicast Frames Received"]
    pub multicastrxed: MULTICASTRXED,
    #[doc = "0x164 - Pause Frames Received"]
    pub pframesrxed: PFRAMESRXED,
    #[doc = "0x168 - 64 Byte Frames Received"]
    pub framesrxed64: FRAMESRXED64,
    #[doc = "0x16c - 65 to 127 Byte Frames Received"]
    pub framesrxed65: FRAMESRXED65,
    #[doc = "0x170 - 128 to 255 Byte Frames Received"]
    pub framesrxed128: FRAMESRXED128,
    #[doc = "0x174 - 256 to 511 Byte Frames Received"]
    pub framesrxed256: FRAMESRXED256,
    #[doc = "0x178 - 512 to 1023 Byte Frames Received"]
    pub framesrxed512: FRAMESRXED512,
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received"]
    pub framesrxed1024: FRAMESRXED1024,
    #[doc = "0x180 - 1519 to maximum Byte Frames Received"]
    pub framesrxed1519: FRAMESRXED1519,
    #[doc = "0x184 - Undersized Frames Received"]
    pub undersizeframes: UNDERSIZEFRAMES,
    #[doc = "0x188 - Oversize Frames Received"]
    pub excessiverxlen: EXCESSIVERXLEN,
    #[doc = "0x18c - Jabbers Received"]
    pub rxjabbers: RXJABBERS,
    #[doc = "0x190 - Frame Check Sequence Errors"]
    pub fcserrs: FCSERRS,
    #[doc = "0x194 - Length Field Frame Errors"]
    pub rxlenerrs: RXLENERRS,
    #[doc = "0x198 - Receive Symbol Errors"]
    pub rxsymbolerrs: RXSYMBOLERRS,
    #[doc = "0x19c - Alignment Errors"]
    pub alignerrs: ALIGNERRS,
    #[doc = "0x1a0 - Receive Resource Errors"]
    pub rxresourceerrs: RXRESOURCEERRS,
    #[doc = "0x1a4 - Receive Overruns"]
    pub rxoverruns: RXOVERRUNS,
    #[doc = "0x1a8 - IP Header Checksum Errors"]
    pub rxipckerrs: RXIPCKERRS,
    #[doc = "0x1ac - TCP Checksum Errors"]
    pub rxtcpckerrs: RXTCPCKERRS,
    #[doc = "0x1b0 - UDP Checksum Errors"]
    pub rxudpckerrs: RXUDPCKERRS,
    #[doc = "0x1b4 - Receive DMA Flushed Packets"]
    pub autoflushedpkts: AUTOFLUSHEDPKTS,
    _reserved95: [u8; 0x04],
    #[doc = "0x1bc - 1588 Timer Increment Register subscript nsec"]
    pub tsutimerincrsubnsec: TSUTIMERINCRSUBNSEC,
    #[doc = "0x1c0 - 1588 Timer Seconds Register 47:32"]
    pub tsutimermsbsec: TSUTIMERMSBSEC,
    _reserved97: [u8; 0x0c],
    #[doc = "0x1d0 - 1588 Timer Seconds Register 31:0"]
    pub tsutimersec: TSUTIMERSEC,
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    pub tsutimernsec: TSUTIMERNSEC,
    #[doc = "0x1d8 - This register returns all zeroes when read."]
    pub tsutimeradjust: TSUTIMERADJUST,
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    pub tsutimerincr: TSUTIMERINCR,
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Register 31:0"]
    pub tsuptptxsec: TSUPTPTXSEC,
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    pub tsuptptxnsec: TSUPTPTXNSEC,
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Register 31:0"]
    pub tsuptprxsec: TSUPTPRXSEC,
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    pub tsuptprxnsec: TSUPTPRXNSEC,
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Register 31:0"]
    pub tsupeertxsec: TSUPEERTXSEC,
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    pub tsupeertxnsec: TSUPEERTXNSEC,
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Register 31:0"]
    pub tsupeerrxsec: TSUPEERRXSEC,
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    pub tsupeerrxnsec: TSUPEERRXNSEC,
    _reserved109: [u8; 0x60],
    #[doc = "0x260 - Transmit Pause Quantum Register 1"]
    pub txpausequant1: TXPAUSEQUANT1,
    #[doc = "0x264 - Transmit Pause Quantum Register 2"]
    pub txpausequant2: TXPAUSEQUANT2,
    #[doc = "0x268 - Transmit Pause Quantum Register 3"]
    pub txpausequant3: TXPAUSEQUANT3,
    _reserved112: [u8; 0x04],
    #[doc = "0x270 - Received LPI transitions"]
    pub rxlpi: RXLPI,
    #[doc = "0x274 - Received LPI time"]
    pub rxlpitime: RXLPITIME,
    #[doc = "0x278 - Transmit LPI transitions"]
    pub txlpi: TXLPI,
    #[doc = "0x27c - Transmit LPI time"]
    pub txlpitime: TXLPITIME,
    _reserved116: [u8; 0x024c],
    #[doc = "0x4cc - TX BD control register"]
    pub txbdctrl: TXBDCTRL,
    #[doc = "0x4d0 - RX BD control register"]
    pub rxbdctrl: RXBDCTRL,
    _reserved118: [u8; 0x072c],
    #[doc = "0xc00 - I/O Route Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0xc04 - I/O Route Location Register 0"]
    pub routeloc0: ROUTELOC0,
    _reserved120: [u8; 0x04],
    #[doc = "0xc0c - I/O Route Location Register 1"]
    pub routeloc1: ROUTELOC1,
    #[doc = "0xc10 - Ethernet control register"]
    pub ctrl: CTRL,
}
#[doc = "NETWORKCTRL (rw) register accessor: an alias for `Reg<NETWORKCTRL_SPEC>`"]
pub type NETWORKCTRL = crate::Reg<networkctrl::NETWORKCTRL_SPEC>;
#[doc = "Network control register"]
pub mod networkctrl;
#[doc = "NETWORKCFG (rw) register accessor: an alias for `Reg<NETWORKCFG_SPEC>`"]
pub type NETWORKCFG = crate::Reg<networkcfg::NETWORKCFG_SPEC>;
#[doc = "Network configuration register"]
pub mod networkcfg;
#[doc = "NETWORKSTATUS (r) register accessor: an alias for `Reg<NETWORKSTATUS_SPEC>`"]
pub type NETWORKSTATUS = crate::Reg<networkstatus::NETWORKSTATUS_SPEC>;
#[doc = "Network status register"]
pub mod networkstatus;
#[doc = "DMACFG (rw) register accessor: an alias for `Reg<DMACFG_SPEC>`"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "TXSTATUS (rw) register accessor: an alias for `Reg<TXSTATUS_SPEC>`"]
pub type TXSTATUS = crate::Reg<txstatus::TXSTATUS_SPEC>;
#[doc = "Transmit status register"]
pub mod txstatus;
#[doc = "RXQPTR (rw) register accessor: an alias for `Reg<RXQPTR_SPEC>`"]
pub type RXQPTR = crate::Reg<rxqptr::RXQPTR_SPEC>;
#[doc = "Start address of the receive buffer queue"]
pub mod rxqptr;
#[doc = "TXQPTR (rw) register accessor: an alias for `Reg<TXQPTR_SPEC>`"]
pub type TXQPTR = crate::Reg<txqptr::TXQPTR_SPEC>;
#[doc = "Start address of the transmit buffer queue"]
pub mod txqptr;
#[doc = "RXSTATUS (rw) register accessor: an alias for `Reg<RXSTATUS_SPEC>`"]
pub type RXSTATUS = crate::Reg<rxstatus::RXSTATUS_SPEC>;
#[doc = "Receive status register"]
pub mod rxstatus;
#[doc = "IFCR (rw) register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Interrupt status register"]
pub mod ifcr;
#[doc = "IENS (w) register accessor: an alias for `Reg<IENS_SPEC>`"]
pub type IENS = crate::Reg<iens::IENS_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod iens;
#[doc = "IENC (w) register accessor: an alias for `Reg<IENC_SPEC>`"]
pub type IENC = crate::Reg<ienc::IENC_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod ienc;
#[doc = "IENRO (rw) register accessor: an alias for `Reg<IENRO_SPEC>`"]
pub type IENRO = crate::Reg<ienro::IENRO_SPEC>;
#[doc = "Interrupt mask register"]
pub mod ienro;
#[doc = "PHYMNGMNT (rw) register accessor: an alias for `Reg<PHYMNGMNT_SPEC>`"]
pub type PHYMNGMNT = crate::Reg<phymngmnt::PHYMNGMNT_SPEC>;
#[doc = "PHY management register"]
pub mod phymngmnt;
#[doc = "RXPAUSEQUANT (r) register accessor: an alias for `Reg<RXPAUSEQUANT_SPEC>`"]
pub type RXPAUSEQUANT = crate::Reg<rxpausequant::RXPAUSEQUANT_SPEC>;
#[doc = "Received Pause Quantum Register"]
pub mod rxpausequant;
#[doc = "TXPAUSEQUANT (rw) register accessor: an alias for `Reg<TXPAUSEQUANT_SPEC>`"]
pub type TXPAUSEQUANT = crate::Reg<txpausequant::TXPAUSEQUANT_SPEC>;
#[doc = "Transmit Pause Quantum Register"]
pub mod txpausequant;
#[doc = "PBUFTXCUTTHRU (rw) register accessor: an alias for `Reg<PBUFTXCUTTHRU_SPEC>`"]
pub type PBUFTXCUTTHRU = crate::Reg<pbuftxcutthru::PBUFTXCUTTHRU_SPEC>;
#[doc = "TX Partial Store and Forward"]
pub mod pbuftxcutthru;
#[doc = "PBUFRXCUTTHRU (rw) register accessor: an alias for `Reg<PBUFRXCUTTHRU_SPEC>`"]
pub type PBUFRXCUTTHRU = crate::Reg<pbufrxcutthru::PBUFRXCUTTHRU_SPEC>;
#[doc = "RX Partial Store and Forward"]
pub mod pbufrxcutthru;
#[doc = "JUMBOMAXLEN (rw) register accessor: an alias for `Reg<JUMBOMAXLEN_SPEC>`"]
pub type JUMBOMAXLEN = crate::Reg<jumbomaxlen::JUMBOMAXLEN_SPEC>;
#[doc = "Maximum Jumbo Frame Size."]
pub mod jumbomaxlen;
#[doc = "IMOD (rw) register accessor: an alias for `Reg<IMOD_SPEC>`"]
pub type IMOD = crate::Reg<imod::IMOD_SPEC>;
#[doc = "Interrupt moderation register"]
pub mod imod;
#[doc = "SYSWAKETIME (rw) register accessor: an alias for `Reg<SYSWAKETIME_SPEC>`"]
pub type SYSWAKETIME = crate::Reg<syswaketime::SYSWAKETIME_SPEC>;
#[doc = "System wake time"]
pub mod syswaketime;
#[doc = "HASHBOTTOM (rw) register accessor: an alias for `Reg<HASHBOTTOM_SPEC>`"]
pub type HASHBOTTOM = crate::Reg<hashbottom::HASHBOTTOM_SPEC>;
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub mod hashbottom;
#[doc = "HASHTOP (rw) register accessor: an alias for `Reg<HASHTOP_SPEC>`"]
pub type HASHTOP = crate::Reg<hashtop::HASHTOP_SPEC>;
#[doc = "Hash Register Top \\[63:32\\]"]
pub mod hashtop;
#[doc = "SPECADDR1BOTTOM (rw) register accessor: an alias for `Reg<SPECADDR1BOTTOM_SPEC>`"]
pub type SPECADDR1BOTTOM = crate::Reg<specaddr1bottom::SPECADDR1BOTTOM_SPEC>;
#[doc = "Specific Address 1 Bottom"]
pub mod specaddr1bottom;
#[doc = "SPECADDR1TOP (rw) register accessor: an alias for `Reg<SPECADDR1TOP_SPEC>`"]
pub type SPECADDR1TOP = crate::Reg<specaddr1top::SPECADDR1TOP_SPEC>;
#[doc = "Specific Address 1 Top"]
pub mod specaddr1top;
#[doc = "SPECADDR2BOTTOM (rw) register accessor: an alias for `Reg<SPECADDR2BOTTOM_SPEC>`"]
pub type SPECADDR2BOTTOM = crate::Reg<specaddr2bottom::SPECADDR2BOTTOM_SPEC>;
#[doc = "Specific Address 2 Bottom"]
pub mod specaddr2bottom;
#[doc = "SPECADDR2TOP (rw) register accessor: an alias for `Reg<SPECADDR2TOP_SPEC>`"]
pub type SPECADDR2TOP = crate::Reg<specaddr2top::SPECADDR2TOP_SPEC>;
#[doc = "Specific Address 2 Top"]
pub mod specaddr2top;
#[doc = "SPECADDR3BOTTOM (rw) register accessor: an alias for `Reg<SPECADDR3BOTTOM_SPEC>`"]
pub type SPECADDR3BOTTOM = crate::Reg<specaddr3bottom::SPECADDR3BOTTOM_SPEC>;
#[doc = "Specific Address 3 Bottom"]
pub mod specaddr3bottom;
#[doc = "SPECADDR3TOP (rw) register accessor: an alias for `Reg<SPECADDR3TOP_SPEC>`"]
pub type SPECADDR3TOP = crate::Reg<specaddr3top::SPECADDR3TOP_SPEC>;
#[doc = "Specific Address 3 Top"]
pub mod specaddr3top;
#[doc = "SPECADDR4BOTTOM (rw) register accessor: an alias for `Reg<SPECADDR4BOTTOM_SPEC>`"]
pub type SPECADDR4BOTTOM = crate::Reg<specaddr4bottom::SPECADDR4BOTTOM_SPEC>;
#[doc = "Specific Address 4 Bottom"]
pub mod specaddr4bottom;
#[doc = "SPECADDR4TOP (rw) register accessor: an alias for `Reg<SPECADDR4TOP_SPEC>`"]
pub type SPECADDR4TOP = crate::Reg<specaddr4top::SPECADDR4TOP_SPEC>;
#[doc = "Specific Address 4 Top"]
pub mod specaddr4top;
#[doc = "SPECTYPE1 (rw) register accessor: an alias for `Reg<SPECTYPE1_SPEC>`"]
pub type SPECTYPE1 = crate::Reg<spectype1::SPECTYPE1_SPEC>;
#[doc = "Type ID Match 1"]
pub mod spectype1;
#[doc = "SPECTYPE2 (rw) register accessor: an alias for `Reg<SPECTYPE2_SPEC>`"]
pub type SPECTYPE2 = crate::Reg<spectype2::SPECTYPE2_SPEC>;
#[doc = "Type ID Match 2"]
pub mod spectype2;
#[doc = "SPECTYPE3 (rw) register accessor: an alias for `Reg<SPECTYPE3_SPEC>`"]
pub type SPECTYPE3 = crate::Reg<spectype3::SPECTYPE3_SPEC>;
#[doc = "Type ID Match 3"]
pub mod spectype3;
#[doc = "SPECTYPE4 (rw) register accessor: an alias for `Reg<SPECTYPE4_SPEC>`"]
pub type SPECTYPE4 = crate::Reg<spectype4::SPECTYPE4_SPEC>;
#[doc = "Type ID Match 4"]
pub mod spectype4;
#[doc = "WOLREG (rw) register accessor: an alias for `Reg<WOLREG_SPEC>`"]
pub type WOLREG = crate::Reg<wolreg::WOLREG_SPEC>;
#[doc = "Wake on LAN Register"]
pub mod wolreg;
#[doc = "STRETCHRATIO (rw) register accessor: an alias for `Reg<STRETCHRATIO_SPEC>`"]
pub type STRETCHRATIO = crate::Reg<stretchratio::STRETCHRATIO_SPEC>;
#[doc = "IPG stretch register"]
pub mod stretchratio;
#[doc = "STACKEDVLAN (rw) register accessor: an alias for `Reg<STACKEDVLAN_SPEC>`"]
pub type STACKEDVLAN = crate::Reg<stackedvlan::STACKEDVLAN_SPEC>;
#[doc = "Stacked VLAN Register"]
pub mod stackedvlan;
#[doc = "TXPFCPAUSE (rw) register accessor: an alias for `Reg<TXPFCPAUSE_SPEC>`"]
pub type TXPFCPAUSE = crate::Reg<txpfcpause::TXPFCPAUSE_SPEC>;
#[doc = "Transmit PFC Pause Register"]
pub mod txpfcpause;
#[doc = "MASKADD1BOTTOM (rw) register accessor: an alias for `Reg<MASKADD1BOTTOM_SPEC>`"]
pub type MASKADD1BOTTOM = crate::Reg<maskadd1bottom::MASKADD1BOTTOM_SPEC>;
#[doc = "Specific Address Mask 1 Bottom 31:0"]
pub mod maskadd1bottom;
#[doc = "MASKADD1TOP (rw) register accessor: an alias for `Reg<MASKADD1TOP_SPEC>`"]
pub type MASKADD1TOP = crate::Reg<maskadd1top::MASKADD1TOP_SPEC>;
#[doc = "Specific Address Mask 1 Top 47:32"]
pub mod maskadd1top;
#[doc = "RXPTPUNICAST (rw) register accessor: an alias for `Reg<RXPTPUNICAST_SPEC>`"]
pub type RXPTPUNICAST = crate::Reg<rxptpunicast::RXPTPUNICAST_SPEC>;
#[doc = "PTP RX unicast IP destination address"]
pub mod rxptpunicast;
#[doc = "TXPTPUNICAST (rw) register accessor: an alias for `Reg<TXPTPUNICAST_SPEC>`"]
pub type TXPTPUNICAST = crate::Reg<txptpunicast::TXPTPUNICAST_SPEC>;
#[doc = "PTP TX unicast IP destination address"]
pub mod txptpunicast;
#[doc = "TSUNSECCMP (rw) register accessor: an alias for `Reg<TSUNSECCMP_SPEC>`"]
pub type TSUNSECCMP = crate::Reg<tsunseccmp::TSUNSECCMP_SPEC>;
#[doc = "TSU timer comparison value nanoseconds"]
pub mod tsunseccmp;
#[doc = "TSUSECCMP (rw) register accessor: an alias for `Reg<TSUSECCMP_SPEC>`"]
pub type TSUSECCMP = crate::Reg<tsuseccmp::TSUSECCMP_SPEC>;
#[doc = "TSU timer comparison value seconds \\[31:0\\]"]
pub mod tsuseccmp;
#[doc = "TSUMSBSECCMP (rw) register accessor: an alias for `Reg<TSUMSBSECCMP_SPEC>`"]
pub type TSUMSBSECCMP = crate::Reg<tsumsbseccmp::TSUMSBSECCMP_SPEC>;
#[doc = "TSU timer comparison value seconds \\[47:32\\]"]
pub mod tsumsbseccmp;
#[doc = "TSUPTPTXMSBSEC (r) register accessor: an alias for `Reg<TSUPTPTXMSBSEC_SPEC>`"]
pub type TSUPTPTXMSBSEC = crate::Reg<tsuptptxmsbsec::TSUPTPTXMSBSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32"]
pub mod tsuptptxmsbsec;
#[doc = "TSUPTPRXMSBSEC (r) register accessor: an alias for `Reg<TSUPTPRXMSBSEC_SPEC>`"]
pub type TSUPTPRXMSBSEC = crate::Reg<tsuptprxmsbsec::TSUPTPRXMSBSEC_SPEC>;
#[doc = "PTP Event Frame Received Seconds Register 47:32"]
pub mod tsuptprxmsbsec;
#[doc = "TSUPEERTXMSBSEC (r) register accessor: an alias for `Reg<TSUPEERTXMSBSEC_SPEC>`"]
pub type TSUPEERTXMSBSEC = crate::Reg<tsupeertxmsbsec::TSUPEERTXMSBSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32"]
pub mod tsupeertxmsbsec;
#[doc = "TSUPEERRXMSBSEC (r) register accessor: an alias for `Reg<TSUPEERRXMSBSEC_SPEC>`"]
pub type TSUPEERRXMSBSEC = crate::Reg<tsupeerrxmsbsec::TSUPEERRXMSBSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Register 47:32"]
pub mod tsupeerrxmsbsec;
#[doc = "OCTETSTXEDBOTTOM (rw) register accessor: an alias for `Reg<OCTETSTXEDBOTTOM_SPEC>`"]
pub type OCTETSTXEDBOTTOM = crate::Reg<octetstxedbottom::OCTETSTXEDBOTTOM_SPEC>;
#[doc = "Octets transmitted 31:0"]
pub mod octetstxedbottom;
#[doc = "OCTETSTXEDTOP (rw) register accessor: an alias for `Reg<OCTETSTXEDTOP_SPEC>`"]
pub type OCTETSTXEDTOP = crate::Reg<octetstxedtop::OCTETSTXEDTOP_SPEC>;
#[doc = "Octets Transmitted 47:32"]
pub mod octetstxedtop;
#[doc = "FRAMESTXEDOK (rw) register accessor: an alias for `Reg<FRAMESTXEDOK_SPEC>`"]
pub type FRAMESTXEDOK = crate::Reg<framestxedok::FRAMESTXEDOK_SPEC>;
#[doc = "Frames Transmitted"]
pub mod framestxedok;
#[doc = "BROADCASTTXED (rw) register accessor: an alias for `Reg<BROADCASTTXED_SPEC>`"]
pub type BROADCASTTXED = crate::Reg<broadcasttxed::BROADCASTTXED_SPEC>;
#[doc = "Broadcast Frames Transmitted"]
pub mod broadcasttxed;
#[doc = "MULTICASTTXED (rw) register accessor: an alias for `Reg<MULTICASTTXED_SPEC>`"]
pub type MULTICASTTXED = crate::Reg<multicasttxed::MULTICASTTXED_SPEC>;
#[doc = "Multicast Frames Transmitted"]
pub mod multicasttxed;
#[doc = "PFRAMESTXED (rw) register accessor: an alias for `Reg<PFRAMESTXED_SPEC>`"]
pub type PFRAMESTXED = crate::Reg<pframestxed::PFRAMESTXED_SPEC>;
#[doc = "Pause Frames Transmitted"]
pub mod pframestxed;
#[doc = "FRAMESTXED64 (rw) register accessor: an alias for `Reg<FRAMESTXED64_SPEC>`"]
pub type FRAMESTXED64 = crate::Reg<framestxed64::FRAMESTXED64_SPEC>;
#[doc = "64 Byte Frames Transmitted"]
pub mod framestxed64;
#[doc = "FRAMESTXED65 (rw) register accessor: an alias for `Reg<FRAMESTXED65_SPEC>`"]
pub type FRAMESTXED65 = crate::Reg<framestxed65::FRAMESTXED65_SPEC>;
#[doc = "65 to 127 Byte Frames Transmitted"]
pub mod framestxed65;
#[doc = "FRAMESTXED128 (rw) register accessor: an alias for `Reg<FRAMESTXED128_SPEC>`"]
pub type FRAMESTXED128 = crate::Reg<framestxed128::FRAMESTXED128_SPEC>;
#[doc = "128 to 255 Byte Frames Transmitted"]
pub mod framestxed128;
#[doc = "FRAMESTXED256 (rw) register accessor: an alias for `Reg<FRAMESTXED256_SPEC>`"]
pub type FRAMESTXED256 = crate::Reg<framestxed256::FRAMESTXED256_SPEC>;
#[doc = "256 to 511 Byte Frames Transmitted"]
pub mod framestxed256;
#[doc = "FRAMESTXED512 (rw) register accessor: an alias for `Reg<FRAMESTXED512_SPEC>`"]
pub type FRAMESTXED512 = crate::Reg<framestxed512::FRAMESTXED512_SPEC>;
#[doc = "512 to 1023 Byte Frames Transmitted"]
pub mod framestxed512;
#[doc = "FRAMESTXED1024 (rw) register accessor: an alias for `Reg<FRAMESTXED1024_SPEC>`"]
pub type FRAMESTXED1024 = crate::Reg<framestxed1024::FRAMESTXED1024_SPEC>;
#[doc = "1024 to 1518 Byte Frames Transmitted"]
pub mod framestxed1024;
#[doc = "FRAMESTXED1519 (rw) register accessor: an alias for `Reg<FRAMESTXED1519_SPEC>`"]
pub type FRAMESTXED1519 = crate::Reg<framestxed1519::FRAMESTXED1519_SPEC>;
#[doc = "Greater Than 1518 Byte Frames Transmitted"]
pub mod framestxed1519;
#[doc = "TXUNDERRUNS (rw) register accessor: an alias for `Reg<TXUNDERRUNS_SPEC>`"]
pub type TXUNDERRUNS = crate::Reg<txunderruns::TXUNDERRUNS_SPEC>;
#[doc = "Transmit Under Runs"]
pub mod txunderruns;
#[doc = "SINGLECOLS (rw) register accessor: an alias for `Reg<SINGLECOLS_SPEC>`"]
pub type SINGLECOLS = crate::Reg<singlecols::SINGLECOLS_SPEC>;
#[doc = "Single Collision Frames"]
pub mod singlecols;
#[doc = "MULTICOLS (rw) register accessor: an alias for `Reg<MULTICOLS_SPEC>`"]
pub type MULTICOLS = crate::Reg<multicols::MULTICOLS_SPEC>;
#[doc = "Multiple Collision Frames"]
pub mod multicols;
#[doc = "EXCESSCOLS (rw) register accessor: an alias for `Reg<EXCESSCOLS_SPEC>`"]
pub type EXCESSCOLS = crate::Reg<excesscols::EXCESSCOLS_SPEC>;
#[doc = "Excessive Collisions"]
pub mod excesscols;
#[doc = "LATECOLS (rw) register accessor: an alias for `Reg<LATECOLS_SPEC>`"]
pub type LATECOLS = crate::Reg<latecols::LATECOLS_SPEC>;
#[doc = "Late Collisions"]
pub mod latecols;
#[doc = "DEFERREDFRAMES (rw) register accessor: an alias for `Reg<DEFERREDFRAMES_SPEC>`"]
pub type DEFERREDFRAMES = crate::Reg<deferredframes::DEFERREDFRAMES_SPEC>;
#[doc = "Deferred Transmission Frames"]
pub mod deferredframes;
#[doc = "CRSERRS (rw) register accessor: an alias for `Reg<CRSERRS_SPEC>`"]
pub type CRSERRS = crate::Reg<crserrs::CRSERRS_SPEC>;
#[doc = "Carrier Sense Errors"]
pub mod crserrs;
#[doc = "OCTETSRXEDBOTTOM (rw) register accessor: an alias for `Reg<OCTETSRXEDBOTTOM_SPEC>`"]
pub type OCTETSRXEDBOTTOM = crate::Reg<octetsrxedbottom::OCTETSRXEDBOTTOM_SPEC>;
#[doc = "Octets Received 31:0"]
pub mod octetsrxedbottom;
#[doc = "OCTETSRXEDTOP (rw) register accessor: an alias for `Reg<OCTETSRXEDTOP_SPEC>`"]
pub type OCTETSRXEDTOP = crate::Reg<octetsrxedtop::OCTETSRXEDTOP_SPEC>;
#[doc = "Octets Received 47:32"]
pub mod octetsrxedtop;
#[doc = "FRAMESRXEDOK (rw) register accessor: an alias for `Reg<FRAMESRXEDOK_SPEC>`"]
pub type FRAMESRXEDOK = crate::Reg<framesrxedok::FRAMESRXEDOK_SPEC>;
#[doc = "Frames Received"]
pub mod framesrxedok;
#[doc = "BROADCASTRXED (rw) register accessor: an alias for `Reg<BROADCASTRXED_SPEC>`"]
pub type BROADCASTRXED = crate::Reg<broadcastrxed::BROADCASTRXED_SPEC>;
#[doc = "Broadcast Frames Received"]
pub mod broadcastrxed;
#[doc = "MULTICASTRXED (rw) register accessor: an alias for `Reg<MULTICASTRXED_SPEC>`"]
pub type MULTICASTRXED = crate::Reg<multicastrxed::MULTICASTRXED_SPEC>;
#[doc = "Multicast Frames Received"]
pub mod multicastrxed;
#[doc = "PFRAMESRXED (rw) register accessor: an alias for `Reg<PFRAMESRXED_SPEC>`"]
pub type PFRAMESRXED = crate::Reg<pframesrxed::PFRAMESRXED_SPEC>;
#[doc = "Pause Frames Received"]
pub mod pframesrxed;
#[doc = "FRAMESRXED64 (rw) register accessor: an alias for `Reg<FRAMESRXED64_SPEC>`"]
pub type FRAMESRXED64 = crate::Reg<framesrxed64::FRAMESRXED64_SPEC>;
#[doc = "64 Byte Frames Received"]
pub mod framesrxed64;
#[doc = "FRAMESRXED65 (rw) register accessor: an alias for `Reg<FRAMESRXED65_SPEC>`"]
pub type FRAMESRXED65 = crate::Reg<framesrxed65::FRAMESRXED65_SPEC>;
#[doc = "65 to 127 Byte Frames Received"]
pub mod framesrxed65;
#[doc = "FRAMESRXED128 (rw) register accessor: an alias for `Reg<FRAMESRXED128_SPEC>`"]
pub type FRAMESRXED128 = crate::Reg<framesrxed128::FRAMESRXED128_SPEC>;
#[doc = "128 to 255 Byte Frames Received"]
pub mod framesrxed128;
#[doc = "FRAMESRXED256 (rw) register accessor: an alias for `Reg<FRAMESRXED256_SPEC>`"]
pub type FRAMESRXED256 = crate::Reg<framesrxed256::FRAMESRXED256_SPEC>;
#[doc = "256 to 511 Byte Frames Received"]
pub mod framesrxed256;
#[doc = "FRAMESRXED512 (rw) register accessor: an alias for `Reg<FRAMESRXED512_SPEC>`"]
pub type FRAMESRXED512 = crate::Reg<framesrxed512::FRAMESRXED512_SPEC>;
#[doc = "512 to 1023 Byte Frames Received"]
pub mod framesrxed512;
#[doc = "FRAMESRXED1024 (rw) register accessor: an alias for `Reg<FRAMESRXED1024_SPEC>`"]
pub type FRAMESRXED1024 = crate::Reg<framesrxed1024::FRAMESRXED1024_SPEC>;
#[doc = "1024 to 1518 Byte Frames Received"]
pub mod framesrxed1024;
#[doc = "FRAMESRXED1519 (rw) register accessor: an alias for `Reg<FRAMESRXED1519_SPEC>`"]
pub type FRAMESRXED1519 = crate::Reg<framesrxed1519::FRAMESRXED1519_SPEC>;
#[doc = "1519 to maximum Byte Frames Received"]
pub mod framesrxed1519;
#[doc = "UNDERSIZEFRAMES (rw) register accessor: an alias for `Reg<UNDERSIZEFRAMES_SPEC>`"]
pub type UNDERSIZEFRAMES = crate::Reg<undersizeframes::UNDERSIZEFRAMES_SPEC>;
#[doc = "Undersized Frames Received"]
pub mod undersizeframes;
#[doc = "EXCESSIVERXLEN (rw) register accessor: an alias for `Reg<EXCESSIVERXLEN_SPEC>`"]
pub type EXCESSIVERXLEN = crate::Reg<excessiverxlen::EXCESSIVERXLEN_SPEC>;
#[doc = "Oversize Frames Received"]
pub mod excessiverxlen;
#[doc = "RXJABBERS (rw) register accessor: an alias for `Reg<RXJABBERS_SPEC>`"]
pub type RXJABBERS = crate::Reg<rxjabbers::RXJABBERS_SPEC>;
#[doc = "Jabbers Received"]
pub mod rxjabbers;
#[doc = "FCSERRS (rw) register accessor: an alias for `Reg<FCSERRS_SPEC>`"]
pub type FCSERRS = crate::Reg<fcserrs::FCSERRS_SPEC>;
#[doc = "Frame Check Sequence Errors"]
pub mod fcserrs;
#[doc = "RXLENERRS (rw) register accessor: an alias for `Reg<RXLENERRS_SPEC>`"]
pub type RXLENERRS = crate::Reg<rxlenerrs::RXLENERRS_SPEC>;
#[doc = "Length Field Frame Errors"]
pub mod rxlenerrs;
#[doc = "RXSYMBOLERRS (rw) register accessor: an alias for `Reg<RXSYMBOLERRS_SPEC>`"]
pub type RXSYMBOLERRS = crate::Reg<rxsymbolerrs::RXSYMBOLERRS_SPEC>;
#[doc = "Receive Symbol Errors"]
pub mod rxsymbolerrs;
#[doc = "ALIGNERRS (rw) register accessor: an alias for `Reg<ALIGNERRS_SPEC>`"]
pub type ALIGNERRS = crate::Reg<alignerrs::ALIGNERRS_SPEC>;
#[doc = "Alignment Errors"]
pub mod alignerrs;
#[doc = "RXRESOURCEERRS (rw) register accessor: an alias for `Reg<RXRESOURCEERRS_SPEC>`"]
pub type RXRESOURCEERRS = crate::Reg<rxresourceerrs::RXRESOURCEERRS_SPEC>;
#[doc = "Receive Resource Errors"]
pub mod rxresourceerrs;
#[doc = "RXOVERRUNS (rw) register accessor: an alias for `Reg<RXOVERRUNS_SPEC>`"]
pub type RXOVERRUNS = crate::Reg<rxoverruns::RXOVERRUNS_SPEC>;
#[doc = "Receive Overruns"]
pub mod rxoverruns;
#[doc = "RXIPCKERRS (rw) register accessor: an alias for `Reg<RXIPCKERRS_SPEC>`"]
pub type RXIPCKERRS = crate::Reg<rxipckerrs::RXIPCKERRS_SPEC>;
#[doc = "IP Header Checksum Errors"]
pub mod rxipckerrs;
#[doc = "RXTCPCKERRS (rw) register accessor: an alias for `Reg<RXTCPCKERRS_SPEC>`"]
pub type RXTCPCKERRS = crate::Reg<rxtcpckerrs::RXTCPCKERRS_SPEC>;
#[doc = "TCP Checksum Errors"]
pub mod rxtcpckerrs;
#[doc = "RXUDPCKERRS (rw) register accessor: an alias for `Reg<RXUDPCKERRS_SPEC>`"]
pub type RXUDPCKERRS = crate::Reg<rxudpckerrs::RXUDPCKERRS_SPEC>;
#[doc = "UDP Checksum Errors"]
pub mod rxudpckerrs;
#[doc = "AUTOFLUSHEDPKTS (rw) register accessor: an alias for `Reg<AUTOFLUSHEDPKTS_SPEC>`"]
pub type AUTOFLUSHEDPKTS = crate::Reg<autoflushedpkts::AUTOFLUSHEDPKTS_SPEC>;
#[doc = "Receive DMA Flushed Packets"]
pub mod autoflushedpkts;
#[doc = "TSUTIMERINCRSUBNSEC (rw) register accessor: an alias for `Reg<TSUTIMERINCRSUBNSEC_SPEC>`"]
pub type TSUTIMERINCRSUBNSEC = crate::Reg<tsutimerincrsubnsec::TSUTIMERINCRSUBNSEC_SPEC>;
#[doc = "1588 Timer Increment Register subscript nsec"]
pub mod tsutimerincrsubnsec;
#[doc = "TSUTIMERMSBSEC (rw) register accessor: an alias for `Reg<TSUTIMERMSBSEC_SPEC>`"]
pub type TSUTIMERMSBSEC = crate::Reg<tsutimermsbsec::TSUTIMERMSBSEC_SPEC>;
#[doc = "1588 Timer Seconds Register 47:32"]
pub mod tsutimermsbsec;
#[doc = "TSUTIMERSEC (rw) register accessor: an alias for `Reg<TSUTIMERSEC_SPEC>`"]
pub type TSUTIMERSEC = crate::Reg<tsutimersec::TSUTIMERSEC_SPEC>;
#[doc = "1588 Timer Seconds Register 31:0"]
pub mod tsutimersec;
#[doc = "TSUTIMERNSEC (rw) register accessor: an alias for `Reg<TSUTIMERNSEC_SPEC>`"]
pub type TSUTIMERNSEC = crate::Reg<tsutimernsec::TSUTIMERNSEC_SPEC>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tsutimernsec;
#[doc = "TSUTIMERADJUST (rw) register accessor: an alias for `Reg<TSUTIMERADJUST_SPEC>`"]
pub type TSUTIMERADJUST = crate::Reg<tsutimeradjust::TSUTIMERADJUST_SPEC>;
#[doc = "This register returns all zeroes when read."]
pub mod tsutimeradjust;
#[doc = "TSUTIMERINCR (rw) register accessor: an alias for `Reg<TSUTIMERINCR_SPEC>`"]
pub type TSUTIMERINCR = crate::Reg<tsutimerincr::TSUTIMERINCR_SPEC>;
#[doc = "1588 Timer Increment Register"]
pub mod tsutimerincr;
#[doc = "TSUPTPTXSEC (r) register accessor: an alias for `Reg<TSUPTPTXSEC_SPEC>`"]
pub type TSUPTPTXSEC = crate::Reg<tsuptptxsec::TSUPTPTXSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0"]
pub mod tsuptptxsec;
#[doc = "TSUPTPTXNSEC (r) register accessor: an alias for `Reg<TSUPTPTXNSEC_SPEC>`"]
pub type TSUPTPTXNSEC = crate::Reg<tsuptptxnsec::TSUPTPTXNSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod tsuptptxnsec;
#[doc = "TSUPTPRXSEC (r) register accessor: an alias for `Reg<TSUPTPRXSEC_SPEC>`"]
pub type TSUPTPRXSEC = crate::Reg<tsuptprxsec::TSUPTPRXSEC_SPEC>;
#[doc = "PTP Event Frame Received Seconds Register 31:0"]
pub mod tsuptprxsec;
#[doc = "TSUPTPRXNSEC (r) register accessor: an alias for `Reg<TSUPTPRXNSEC_SPEC>`"]
pub type TSUPTPRXNSEC = crate::Reg<tsuptprxnsec::TSUPTPRXNSEC_SPEC>;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod tsuptprxnsec;
#[doc = "TSUPEERTXSEC (r) register accessor: an alias for `Reg<TSUPEERTXSEC_SPEC>`"]
pub type TSUPEERTXSEC = crate::Reg<tsupeertxsec::TSUPEERTXSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0"]
pub mod tsupeertxsec;
#[doc = "TSUPEERTXNSEC (r) register accessor: an alias for `Reg<TSUPEERTXNSEC_SPEC>`"]
pub type TSUPEERTXNSEC = crate::Reg<tsupeertxnsec::TSUPEERTXNSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod tsupeertxnsec;
#[doc = "TSUPEERRXSEC (r) register accessor: an alias for `Reg<TSUPEERRXSEC_SPEC>`"]
pub type TSUPEERRXSEC = crate::Reg<tsupeerrxsec::TSUPEERRXSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0"]
pub mod tsupeerrxsec;
#[doc = "TSUPEERRXNSEC (r) register accessor: an alias for `Reg<TSUPEERRXNSEC_SPEC>`"]
pub type TSUPEERRXNSEC = crate::Reg<tsupeerrxnsec::TSUPEERRXNSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod tsupeerrxnsec;
#[doc = "TXPAUSEQUANT1 (rw) register accessor: an alias for `Reg<TXPAUSEQUANT1_SPEC>`"]
pub type TXPAUSEQUANT1 = crate::Reg<txpausequant1::TXPAUSEQUANT1_SPEC>;
#[doc = "Transmit Pause Quantum Register 1"]
pub mod txpausequant1;
#[doc = "TXPAUSEQUANT2 (rw) register accessor: an alias for `Reg<TXPAUSEQUANT2_SPEC>`"]
pub type TXPAUSEQUANT2 = crate::Reg<txpausequant2::TXPAUSEQUANT2_SPEC>;
#[doc = "Transmit Pause Quantum Register 2"]
pub mod txpausequant2;
#[doc = "TXPAUSEQUANT3 (rw) register accessor: an alias for `Reg<TXPAUSEQUANT3_SPEC>`"]
pub type TXPAUSEQUANT3 = crate::Reg<txpausequant3::TXPAUSEQUANT3_SPEC>;
#[doc = "Transmit Pause Quantum Register 3"]
pub mod txpausequant3;
#[doc = "RXLPI (rw) register accessor: an alias for `Reg<RXLPI_SPEC>`"]
pub type RXLPI = crate::Reg<rxlpi::RXLPI_SPEC>;
#[doc = "Received LPI transitions"]
pub mod rxlpi;
#[doc = "RXLPITIME (rw) register accessor: an alias for `Reg<RXLPITIME_SPEC>`"]
pub type RXLPITIME = crate::Reg<rxlpitime::RXLPITIME_SPEC>;
#[doc = "Received LPI time"]
pub mod rxlpitime;
#[doc = "TXLPI (rw) register accessor: an alias for `Reg<TXLPI_SPEC>`"]
pub type TXLPI = crate::Reg<txlpi::TXLPI_SPEC>;
#[doc = "Transmit LPI transitions"]
pub mod txlpi;
#[doc = "TXLPITIME (rw) register accessor: an alias for `Reg<TXLPITIME_SPEC>`"]
pub type TXLPITIME = crate::Reg<txlpitime::TXLPITIME_SPEC>;
#[doc = "Transmit LPI time"]
pub mod txlpitime;
#[doc = "TXBDCTRL (rw) register accessor: an alias for `Reg<TXBDCTRL_SPEC>`"]
pub type TXBDCTRL = crate::Reg<txbdctrl::TXBDCTRL_SPEC>;
#[doc = "TX BD control register"]
pub mod txbdctrl;
#[doc = "RXBDCTRL (rw) register accessor: an alias for `Reg<RXBDCTRL_SPEC>`"]
pub type RXBDCTRL = crate::Reg<rxbdctrl::RXBDCTRL_SPEC>;
#[doc = "RX BD control register"]
pub mod rxbdctrl;
#[doc = "ROUTEPEN (rw) register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Route Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: an alias for `Reg<ROUTELOC1_SPEC>`"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Route Location Register 1"]
pub mod routeloc1;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Ethernet control register"]
pub mod ctrl;
