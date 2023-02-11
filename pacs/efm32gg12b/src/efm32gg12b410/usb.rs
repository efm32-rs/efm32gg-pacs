#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - System Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x18 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved7: [u8; 0x10],
    #[doc = "0x2c - Charger Detect Configuration Register"]
    pub cdconf: CDCONF,
    #[doc = "0x30 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x34 - Data TRIM 1 Values for USB DP and DM"]
    pub dattrim1: DATTRIM1,
    _reserved10: [u8; 0x04],
    #[doc = "0x3c - USB LEM Control Register"]
    pub lemctrl: LEMCTRL,
    #[doc = "0x40 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    _reserved12: [u8; 0x000d_dfbc],
    #[doc = "0xde000 - OTG Control and Status Register"]
    pub gotgctl: GOTGCTL,
    #[doc = "0xde004 - OTG Interrupt Register"]
    pub gotgint: GOTGINT,
    #[doc = "0xde008 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0xde00c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0xde010 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0xde014 - Interrupt Register"]
    pub gintsts: GINTSTS,
    #[doc = "0xde018 - Interrupt Mask Register"]
    pub gintmsk: GINTMSK,
    #[doc = "0xde01c - Receive Status Debug Read Register"]
    pub grxstsr: GRXSTSR,
    #[doc = "0xde020 - Receive Status Read /Pop Register"]
    pub grxstsp: GRXSTSP,
    #[doc = "0xde024 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0xde028 - Non-periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: GNPTXFSIZ,
    #[doc = "0xde02c - Non-periodic Transmit FIFO/Queue Status Register"]
    pub gnptxsts: GNPTXSTS,
    _reserved24: [u8; 0x10],
    #[doc = "0xde040 - Synopsys ID Register"]
    pub gsnpsid: GSNPSID,
    _reserved25: [u8; 0x18],
    #[doc = "0xde05c - Global DFIFO Configuration Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved26: [u8; 0xa0],
    #[doc = "0xde100 - Host Periodic Transmit FIFO Size Register"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0xde104 - Device IN Endpoint Transmit FIFO Size Register 1"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0xde108 - Device IN Endpoint Transmit FIFO Size Register 2"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0xde10c - Device IN Endpoint Transmit FIFO Size Register 3"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0xde110 - Device IN Endpoint Transmit FIFO Size Register 4"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0xde114 - Device IN Endpoint Transmit FIFO Size Register 5"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0xde118 - Device IN Endpoint Transmit FIFO Size Register 6"]
    pub dieptxf6: DIEPTXF6,
    _reserved33: [u8; 0x02e4],
    #[doc = "0xde400 - Host Configuration Register"]
    pub hcfg: HCFG,
    #[doc = "0xde404 - Host Frame Interval Register"]
    pub hfir: HFIR,
    #[doc = "0xde408 - Host Frame Number/Frame Time Remaining Register"]
    pub hfnum: HFNUM,
    _reserved36: [u8; 0x04],
    #[doc = "0xde410 - Host Periodic Transmit FIFO/Queue Status Register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0xde414 - Host All Channels Interrupt Register"]
    pub haint: HAINT,
    #[doc = "0xde418 - Host All Channels Interrupt Mask Register"]
    pub haintmsk: HAINTMSK,
    _reserved39: [u8; 0x24],
    #[doc = "0xde440 - Host Port Control and Status Register"]
    pub hprt: HPRT,
    _reserved40: [u8; 0xbc],
    #[doc = "0xde500 - Host Channel x Characteristics Register"]
    pub hc0_char: HC0_CHAR,
    #[doc = "0xde504 - Host Channel x Split Control Register"]
    pub hc0_splt: HC0_SPLT,
    #[doc = "0xde508 - Host Channel x Interrupt Register"]
    pub hc0_int: HC0_INT,
    #[doc = "0xde50c - Host Channel x Interrupt Mask Register"]
    pub hc0_intmsk: HC0_INTMSK,
    #[doc = "0xde510 - Host Channel x Transfer Size Register"]
    pub hc0_tsiz: HC0_TSIZ,
    #[doc = "0xde514 - Host Channel x DMA Address Register"]
    pub hc0_dmaaddr: HC0_DMAADDR,
    _reserved46: [u8; 0x08],
    #[doc = "0xde520 - Host Channel x Characteristics Register"]
    pub hc1_char: HC1_CHAR,
    #[doc = "0xde524 - Host Channel x Split Control Register"]
    pub hc1_splt: HC1_SPLT,
    #[doc = "0xde528 - Host Channel x Interrupt Register"]
    pub hc1_int: HC1_INT,
    #[doc = "0xde52c - Host Channel x Interrupt Mask Register"]
    pub hc1_intmsk: HC1_INTMSK,
    #[doc = "0xde530 - Host Channel x Transfer Size Register"]
    pub hc1_tsiz: HC1_TSIZ,
    #[doc = "0xde534 - Host Channel x DMA Address Register"]
    pub hc1_dmaaddr: HC1_DMAADDR,
    _reserved52: [u8; 0x08],
    #[doc = "0xde540 - Host Channel x Characteristics Register"]
    pub hc2_char: HC2_CHAR,
    #[doc = "0xde544 - Host Channel x Split Control Register"]
    pub hc2_splt: HC2_SPLT,
    #[doc = "0xde548 - Host Channel x Interrupt Register"]
    pub hc2_int: HC2_INT,
    #[doc = "0xde54c - Host Channel x Interrupt Mask Register"]
    pub hc2_intmsk: HC2_INTMSK,
    #[doc = "0xde550 - Host Channel x Transfer Size Register"]
    pub hc2_tsiz: HC2_TSIZ,
    #[doc = "0xde554 - Host Channel x DMA Address Register"]
    pub hc2_dmaaddr: HC2_DMAADDR,
    _reserved58: [u8; 0x08],
    #[doc = "0xde560 - Host Channel x Characteristics Register"]
    pub hc3_char: HC3_CHAR,
    #[doc = "0xde564 - Host Channel x Split Control Register"]
    pub hc3_splt: HC3_SPLT,
    #[doc = "0xde568 - Host Channel x Interrupt Register"]
    pub hc3_int: HC3_INT,
    #[doc = "0xde56c - Host Channel x Interrupt Mask Register"]
    pub hc3_intmsk: HC3_INTMSK,
    #[doc = "0xde570 - Host Channel x Transfer Size Register"]
    pub hc3_tsiz: HC3_TSIZ,
    #[doc = "0xde574 - Host Channel x DMA Address Register"]
    pub hc3_dmaaddr: HC3_DMAADDR,
    _reserved64: [u8; 0x08],
    #[doc = "0xde580 - Host Channel x Characteristics Register"]
    pub hc4_char: HC4_CHAR,
    #[doc = "0xde584 - Host Channel x Split Control Register"]
    pub hc4_splt: HC4_SPLT,
    #[doc = "0xde588 - Host Channel x Interrupt Register"]
    pub hc4_int: HC4_INT,
    #[doc = "0xde58c - Host Channel x Interrupt Mask Register"]
    pub hc4_intmsk: HC4_INTMSK,
    #[doc = "0xde590 - Host Channel x Transfer Size Register"]
    pub hc4_tsiz: HC4_TSIZ,
    #[doc = "0xde594 - Host Channel x DMA Address Register"]
    pub hc4_dmaaddr: HC4_DMAADDR,
    _reserved70: [u8; 0x08],
    #[doc = "0xde5a0 - Host Channel x Characteristics Register"]
    pub hc5_char: HC5_CHAR,
    #[doc = "0xde5a4 - Host Channel x Split Control Register"]
    pub hc5_splt: HC5_SPLT,
    #[doc = "0xde5a8 - Host Channel x Interrupt Register"]
    pub hc5_int: HC5_INT,
    #[doc = "0xde5ac - Host Channel x Interrupt Mask Register"]
    pub hc5_intmsk: HC5_INTMSK,
    #[doc = "0xde5b0 - Host Channel x Transfer Size Register"]
    pub hc5_tsiz: HC5_TSIZ,
    #[doc = "0xde5b4 - Host Channel x DMA Address Register"]
    pub hc5_dmaaddr: HC5_DMAADDR,
    _reserved76: [u8; 0x08],
    #[doc = "0xde5c0 - Host Channel x Characteristics Register"]
    pub hc6_char: HC6_CHAR,
    #[doc = "0xde5c4 - Host Channel x Split Control Register"]
    pub hc6_splt: HC6_SPLT,
    #[doc = "0xde5c8 - Host Channel x Interrupt Register"]
    pub hc6_int: HC6_INT,
    #[doc = "0xde5cc - Host Channel x Interrupt Mask Register"]
    pub hc6_intmsk: HC6_INTMSK,
    #[doc = "0xde5d0 - Host Channel x Transfer Size Register"]
    pub hc6_tsiz: HC6_TSIZ,
    #[doc = "0xde5d4 - Host Channel x DMA Address Register"]
    pub hc6_dmaaddr: HC6_DMAADDR,
    _reserved82: [u8; 0x08],
    #[doc = "0xde5e0 - Host Channel x Characteristics Register"]
    pub hc7_char: HC7_CHAR,
    #[doc = "0xde5e4 - Host Channel x Split Control Register"]
    pub hc7_splt: HC7_SPLT,
    #[doc = "0xde5e8 - Host Channel x Interrupt Register"]
    pub hc7_int: HC7_INT,
    #[doc = "0xde5ec - Host Channel x Interrupt Mask Register"]
    pub hc7_intmsk: HC7_INTMSK,
    #[doc = "0xde5f0 - Host Channel x Transfer Size Register"]
    pub hc7_tsiz: HC7_TSIZ,
    #[doc = "0xde5f4 - Host Channel x DMA Address Register"]
    pub hc7_dmaaddr: HC7_DMAADDR,
    _reserved88: [u8; 0x08],
    #[doc = "0xde600 - Host Channel x Characteristics Register"]
    pub hc8_char: HC8_CHAR,
    #[doc = "0xde604 - Host Channel x Split Control Register"]
    pub hc8_splt: HC8_SPLT,
    #[doc = "0xde608 - Host Channel x Interrupt Register"]
    pub hc8_int: HC8_INT,
    #[doc = "0xde60c - Host Channel x Interrupt Mask Register"]
    pub hc8_intmsk: HC8_INTMSK,
    #[doc = "0xde610 - Host Channel x Transfer Size Register"]
    pub hc8_tsiz: HC8_TSIZ,
    #[doc = "0xde614 - Host Channel x DMA Address Register"]
    pub hc8_dmaaddr: HC8_DMAADDR,
    _reserved94: [u8; 0x08],
    #[doc = "0xde620 - Host Channel x Characteristics Register"]
    pub hc9_char: HC9_CHAR,
    #[doc = "0xde624 - Host Channel x Split Control Register"]
    pub hc9_splt: HC9_SPLT,
    #[doc = "0xde628 - Host Channel x Interrupt Register"]
    pub hc9_int: HC9_INT,
    #[doc = "0xde62c - Host Channel x Interrupt Mask Register"]
    pub hc9_intmsk: HC9_INTMSK,
    #[doc = "0xde630 - Host Channel x Transfer Size Register"]
    pub hc9_tsiz: HC9_TSIZ,
    #[doc = "0xde634 - Host Channel x DMA Address Register"]
    pub hc9_dmaaddr: HC9_DMAADDR,
    _reserved100: [u8; 0x08],
    #[doc = "0xde640 - Host Channel x Characteristics Register"]
    pub hc10_char: HC10_CHAR,
    #[doc = "0xde644 - Host Channel x Split Control Register"]
    pub hc10_splt: HC10_SPLT,
    #[doc = "0xde648 - Host Channel x Interrupt Register"]
    pub hc10_int: HC10_INT,
    #[doc = "0xde64c - Host Channel x Interrupt Mask Register"]
    pub hc10_intmsk: HC10_INTMSK,
    #[doc = "0xde650 - Host Channel x Transfer Size Register"]
    pub hc10_tsiz: HC10_TSIZ,
    #[doc = "0xde654 - Host Channel x DMA Address Register"]
    pub hc10_dmaaddr: HC10_DMAADDR,
    _reserved106: [u8; 0x08],
    #[doc = "0xde660 - Host Channel x Characteristics Register"]
    pub hc11_char: HC11_CHAR,
    #[doc = "0xde664 - Host Channel x Split Control Register"]
    pub hc11_splt: HC11_SPLT,
    #[doc = "0xde668 - Host Channel x Interrupt Register"]
    pub hc11_int: HC11_INT,
    #[doc = "0xde66c - Host Channel x Interrupt Mask Register"]
    pub hc11_intmsk: HC11_INTMSK,
    #[doc = "0xde670 - Host Channel x Transfer Size Register"]
    pub hc11_tsiz: HC11_TSIZ,
    #[doc = "0xde674 - Host Channel x DMA Address Register"]
    pub hc11_dmaaddr: HC11_DMAADDR,
    _reserved112: [u8; 0x08],
    #[doc = "0xde680 - Host Channel x Characteristics Register"]
    pub hc12_char: HC12_CHAR,
    #[doc = "0xde684 - Host Channel x Split Control Register"]
    pub hc12_splt: HC12_SPLT,
    #[doc = "0xde688 - Host Channel x Interrupt Register"]
    pub hc12_int: HC12_INT,
    #[doc = "0xde68c - Host Channel x Interrupt Mask Register"]
    pub hc12_intmsk: HC12_INTMSK,
    #[doc = "0xde690 - Host Channel x Transfer Size Register"]
    pub hc12_tsiz: HC12_TSIZ,
    #[doc = "0xde694 - Host Channel x DMA Address Register"]
    pub hc12_dmaaddr: HC12_DMAADDR,
    _reserved118: [u8; 0x08],
    #[doc = "0xde6a0 - Host Channel x Characteristics Register"]
    pub hc13_char: HC13_CHAR,
    #[doc = "0xde6a4 - Host Channel x Split Control Register"]
    pub hc13_splt: HC13_SPLT,
    #[doc = "0xde6a8 - Host Channel x Interrupt Register"]
    pub hc13_int: HC13_INT,
    #[doc = "0xde6ac - Host Channel x Interrupt Mask Register"]
    pub hc13_intmsk: HC13_INTMSK,
    #[doc = "0xde6b0 - Host Channel x Transfer Size Register"]
    pub hc13_tsiz: HC13_TSIZ,
    #[doc = "0xde6b4 - Host Channel x DMA Address Register"]
    pub hc13_dmaaddr: HC13_DMAADDR,
    _reserved124: [u8; 0x0148],
    #[doc = "0xde800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0xde804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0xde808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved127: [u8; 0x04],
    #[doc = "0xde810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0xde814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0xde818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0xde81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved131: [u8; 0x08],
    #[doc = "0xde828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0xde82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: DVBUSPULSE,
    #[doc = "0xde830 - Device Threshold Control Register"]
    pub dthrctl: DTHRCTL,
    #[doc = "0xde834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved135: [u8; 0xc8],
    #[doc = "0xde900 - Device Control IN Endpoint 0 Control Register"]
    pub diep0ctl: DIEP0CTL,
    _reserved136: [u8; 0x04],
    #[doc = "0xde908 - Device IN Endpoint 0 Interrupt Register"]
    pub diep0int: DIEP0INT,
    _reserved137: [u8; 0x04],
    #[doc = "0xde910 - Device IN Endpoint 0 Transfer Size Register"]
    pub diep0tsiz: DIEP0TSIZ,
    #[doc = "0xde914 - Device IN Endpoint 0 DMA Address Register"]
    pub diep0dmaaddr: DIEP0DMAADDR,
    #[doc = "0xde918 - Device IN Endpoint Transmit FIFO Status Register 0"]
    pub diep0txfsts: DIEP0TXFSTS,
    _reserved140: [u8; 0x04],
    #[doc = "0xde920 - Device Control IN Endpoint x+1 Control Register"]
    pub diep0_ctl: DIEP0_CTL,
    _reserved141: [u8; 0x04],
    #[doc = "0xde928 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep0_int: DIEP0_INT,
    _reserved142: [u8; 0x04],
    #[doc = "0xde930 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep0_tsiz: DIEP0_TSIZ,
    #[doc = "0xde934 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep0_dmaaddr: DIEP0_DMAADDR,
    #[doc = "0xde938 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep0_dtxfsts: DIEP0_DTXFSTS,
    _reserved145: [u8; 0x04],
    #[doc = "0xde940 - Device Control IN Endpoint x+1 Control Register"]
    pub diep1_ctl: DIEP1_CTL,
    _reserved146: [u8; 0x04],
    #[doc = "0xde948 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep1_int: DIEP1_INT,
    _reserved147: [u8; 0x04],
    #[doc = "0xde950 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep1_tsiz: DIEP1_TSIZ,
    #[doc = "0xde954 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep1_dmaaddr: DIEP1_DMAADDR,
    #[doc = "0xde958 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep1_dtxfsts: DIEP1_DTXFSTS,
    _reserved150: [u8; 0x04],
    #[doc = "0xde960 - Device Control IN Endpoint x+1 Control Register"]
    pub diep2_ctl: DIEP2_CTL,
    _reserved151: [u8; 0x04],
    #[doc = "0xde968 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep2_int: DIEP2_INT,
    _reserved152: [u8; 0x04],
    #[doc = "0xde970 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep2_tsiz: DIEP2_TSIZ,
    #[doc = "0xde974 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep2_dmaaddr: DIEP2_DMAADDR,
    #[doc = "0xde978 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep2_dtxfsts: DIEP2_DTXFSTS,
    _reserved155: [u8; 0x04],
    #[doc = "0xde980 - Device Control IN Endpoint x+1 Control Register"]
    pub diep3_ctl: DIEP3_CTL,
    _reserved156: [u8; 0x04],
    #[doc = "0xde988 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep3_int: DIEP3_INT,
    _reserved157: [u8; 0x04],
    #[doc = "0xde990 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep3_tsiz: DIEP3_TSIZ,
    #[doc = "0xde994 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep3_dmaaddr: DIEP3_DMAADDR,
    #[doc = "0xde998 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep3_dtxfsts: DIEP3_DTXFSTS,
    _reserved160: [u8; 0x04],
    #[doc = "0xde9a0 - Device Control IN Endpoint x+1 Control Register"]
    pub diep4_ctl: DIEP4_CTL,
    _reserved161: [u8; 0x04],
    #[doc = "0xde9a8 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep4_int: DIEP4_INT,
    _reserved162: [u8; 0x04],
    #[doc = "0xde9b0 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep4_tsiz: DIEP4_TSIZ,
    #[doc = "0xde9b4 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep4_dmaaddr: DIEP4_DMAADDR,
    #[doc = "0xde9b8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep4_dtxfsts: DIEP4_DTXFSTS,
    _reserved165: [u8; 0x04],
    #[doc = "0xde9c0 - Device Control IN Endpoint x+1 Control Register"]
    pub diep5_ctl: DIEP5_CTL,
    _reserved166: [u8; 0x04],
    #[doc = "0xde9c8 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep5_int: DIEP5_INT,
    _reserved167: [u8; 0x04],
    #[doc = "0xde9d0 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep5_tsiz: DIEP5_TSIZ,
    #[doc = "0xde9d4 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep5_dmaaddr: DIEP5_DMAADDR,
    #[doc = "0xde9d8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep5_dtxfsts: DIEP5_DTXFSTS,
    _reserved170: [u8; 0x0124],
    #[doc = "0xdeb00 - Device Control OUT Endpoint 0 Control Register"]
    pub doep0ctl: DOEP0CTL,
    _reserved171: [u8; 0x04],
    #[doc = "0xdeb08 - Device OUT Endpoint 0 Interrupt Register"]
    pub doep0int: DOEP0INT,
    _reserved172: [u8; 0x04],
    #[doc = "0xdeb10 - Device OUT Endpoint 0 Transfer Size Register"]
    pub doep0tsiz: DOEP0TSIZ,
    #[doc = "0xdeb14 - Device OUT Endpoint 0 DMA Address Register"]
    pub doep0dmaaddr: DOEP0DMAADDR,
    _reserved174: [u8; 0x08],
    #[doc = "0xdeb20 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep0_ctl: DOEP0_CTL,
    _reserved175: [u8; 0x04],
    #[doc = "0xdeb28 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep0_int: DOEP0_INT,
    _reserved176: [u8; 0x04],
    #[doc = "0xdeb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep0_tsiz: DOEP0_TSIZ,
    #[doc = "0xdeb34 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved178: [u8; 0x08],
    #[doc = "0xdeb40 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep1_ctl: DOEP1_CTL,
    _reserved179: [u8; 0x04],
    #[doc = "0xdeb48 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep1_int: DOEP1_INT,
    _reserved180: [u8; 0x04],
    #[doc = "0xdeb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep1_tsiz: DOEP1_TSIZ,
    #[doc = "0xdeb54 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved182: [u8; 0x08],
    #[doc = "0xdeb60 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep2_ctl: DOEP2_CTL,
    _reserved183: [u8; 0x04],
    #[doc = "0xdeb68 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep2_int: DOEP2_INT,
    _reserved184: [u8; 0x04],
    #[doc = "0xdeb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep2_tsiz: DOEP2_TSIZ,
    #[doc = "0xdeb74 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved186: [u8; 0x08],
    #[doc = "0xdeb80 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep3_ctl: DOEP3_CTL,
    _reserved187: [u8; 0x04],
    #[doc = "0xdeb88 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep3_int: DOEP3_INT,
    _reserved188: [u8; 0x04],
    #[doc = "0xdeb90 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep3_tsiz: DOEP3_TSIZ,
    #[doc = "0xdeb94 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep3_dmaaddr: DOEP3_DMAADDR,
    _reserved190: [u8; 0x08],
    #[doc = "0xdeba0 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep4_ctl: DOEP4_CTL,
    _reserved191: [u8; 0x04],
    #[doc = "0xdeba8 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep4_int: DOEP4_INT,
    _reserved192: [u8; 0x04],
    #[doc = "0xdebb0 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep4_tsiz: DOEP4_TSIZ,
    #[doc = "0xdebb4 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep4_dmaaddr: DOEP4_DMAADDR,
    _reserved194: [u8; 0x08],
    #[doc = "0xdebc0 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep5_ctl: DOEP5_CTL,
    _reserved195: [u8; 0x04],
    #[doc = "0xdebc8 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep5_int: DOEP5_INT,
    _reserved196: [u8; 0x04],
    #[doc = "0xdebd0 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep5_tsiz: DOEP5_TSIZ,
    #[doc = "0xdebd4 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep5_dmaaddr: DOEP5_DMAADDR,
    _reserved198: [u8; 0x0228],
    #[doc = "0xdee00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "System Status Register"]
pub mod status;
#[doc = "IF (r) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "ROUTE (rw) register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CDCONF (rw) register accessor: an alias for `Reg<CDCONF_SPEC>`"]
pub type CDCONF = crate::Reg<cdconf::CDCONF_SPEC>;
#[doc = "Charger Detect Configuration Register"]
pub mod cdconf;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "DATTRIM1 (rw) register accessor: an alias for `Reg<DATTRIM1_SPEC>`"]
pub type DATTRIM1 = crate::Reg<dattrim1::DATTRIM1_SPEC>;
#[doc = "Data TRIM 1 Values for USB DP and DM"]
pub mod dattrim1;
#[doc = "LEMCTRL (rw) register accessor: an alias for `Reg<LEMCTRL_SPEC>`"]
pub type LEMCTRL = crate::Reg<lemctrl::LEMCTRL_SPEC>;
#[doc = "USB LEM Control Register"]
pub mod lemctrl;
#[doc = "ROUTELOC0 (rw) register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "GOTGCTL (rw) register accessor: an alias for `Reg<GOTGCTL_SPEC>`"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "OTG Control and Status Register"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: an alias for `Reg<GOTGINT_SPEC>`"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: an alias for `Reg<GAHBCFG_SPEC>`"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: an alias for `Reg<GUSBCFG_SPEC>`"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: an alias for `Reg<GINTSTS_SPEC>`"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: an alias for `Reg<GINTMSK_SPEC>`"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: an alias for `Reg<GRXSTSR_SPEC>`"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: an alias for `Reg<GRXSTSP_SPEC>`"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = "Receive Status Read /Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: an alias for `Reg<GRXFSIZ_SPEC>`"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: an alias for `Reg<GNPTXFSIZ_SPEC>`"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS (r) register accessor: an alias for `Reg<GNPTXSTS_SPEC>`"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "GSNPSID (r) register accessor: an alias for `Reg<GSNPSID_SPEC>`"]
pub type GSNPSID = crate::Reg<gsnpsid::GSNPSID_SPEC>;
#[doc = "Synopsys ID Register"]
pub mod gsnpsid;
#[doc = "GDFIFOCFG (rw) register accessor: an alias for `Reg<GDFIFOCFG_SPEC>`"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ (rw) register accessor: an alias for `Reg<HPTXFSIZ_SPEC>`"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: an alias for `Reg<DIEPTXF1_SPEC>`"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 1"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: an alias for `Reg<DIEPTXF2_SPEC>`"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 2"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: an alias for `Reg<DIEPTXF3_SPEC>`"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 3"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: an alias for `Reg<DIEPTXF4_SPEC>`"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 4"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: an alias for `Reg<DIEPTXF5_SPEC>`"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 5"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: an alias for `Reg<DIEPTXF6_SPEC>`"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 6"]
pub mod dieptxf6;
#[doc = "HCFG (rw) register accessor: an alias for `Reg<HCFG_SPEC>`"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: an alias for `Reg<HFNUM_SPEC>`"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "HPTXSTS (r) register accessor: an alias for `Reg<HPTXSTS_SPEC>`"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: an alias for `Reg<HPRT_SPEC>`"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "HC0_CHAR (rw) register accessor: an alias for `Reg<HC0_CHAR_SPEC>`"]
pub type HC0_CHAR = crate::Reg<hc0_char::HC0_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc0_char;
#[doc = "HC0_SPLT (rw) register accessor: an alias for `Reg<HC0_SPLT_SPEC>`"]
pub type HC0_SPLT = crate::Reg<hc0_splt::HC0_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc0_splt;
#[doc = "HC0_INT (rw) register accessor: an alias for `Reg<HC0_INT_SPEC>`"]
pub type HC0_INT = crate::Reg<hc0_int::HC0_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc0_int;
#[doc = "HC0_INTMSK (rw) register accessor: an alias for `Reg<HC0_INTMSK_SPEC>`"]
pub type HC0_INTMSK = crate::Reg<hc0_intmsk::HC0_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc0_intmsk;
#[doc = "HC0_TSIZ (rw) register accessor: an alias for `Reg<HC0_TSIZ_SPEC>`"]
pub type HC0_TSIZ = crate::Reg<hc0_tsiz::HC0_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc0_tsiz;
#[doc = "HC0_DMAADDR (rw) register accessor: an alias for `Reg<HC0_DMAADDR_SPEC>`"]
pub type HC0_DMAADDR = crate::Reg<hc0_dmaaddr::HC0_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc0_dmaaddr;
#[doc = "HC1_CHAR (rw) register accessor: an alias for `Reg<HC1_CHAR_SPEC>`"]
pub type HC1_CHAR = crate::Reg<hc1_char::HC1_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc1_char;
#[doc = "HC1_SPLT (rw) register accessor: an alias for `Reg<HC1_SPLT_SPEC>`"]
pub type HC1_SPLT = crate::Reg<hc1_splt::HC1_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc1_splt;
#[doc = "HC1_INT (rw) register accessor: an alias for `Reg<HC1_INT_SPEC>`"]
pub type HC1_INT = crate::Reg<hc1_int::HC1_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc1_int;
#[doc = "HC1_INTMSK (rw) register accessor: an alias for `Reg<HC1_INTMSK_SPEC>`"]
pub type HC1_INTMSK = crate::Reg<hc1_intmsk::HC1_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc1_intmsk;
#[doc = "HC1_TSIZ (rw) register accessor: an alias for `Reg<HC1_TSIZ_SPEC>`"]
pub type HC1_TSIZ = crate::Reg<hc1_tsiz::HC1_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc1_tsiz;
#[doc = "HC1_DMAADDR (rw) register accessor: an alias for `Reg<HC1_DMAADDR_SPEC>`"]
pub type HC1_DMAADDR = crate::Reg<hc1_dmaaddr::HC1_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc1_dmaaddr;
#[doc = "HC2_CHAR (rw) register accessor: an alias for `Reg<HC2_CHAR_SPEC>`"]
pub type HC2_CHAR = crate::Reg<hc2_char::HC2_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc2_char;
#[doc = "HC2_SPLT (rw) register accessor: an alias for `Reg<HC2_SPLT_SPEC>`"]
pub type HC2_SPLT = crate::Reg<hc2_splt::HC2_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc2_splt;
#[doc = "HC2_INT (rw) register accessor: an alias for `Reg<HC2_INT_SPEC>`"]
pub type HC2_INT = crate::Reg<hc2_int::HC2_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc2_int;
#[doc = "HC2_INTMSK (rw) register accessor: an alias for `Reg<HC2_INTMSK_SPEC>`"]
pub type HC2_INTMSK = crate::Reg<hc2_intmsk::HC2_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc2_intmsk;
#[doc = "HC2_TSIZ (rw) register accessor: an alias for `Reg<HC2_TSIZ_SPEC>`"]
pub type HC2_TSIZ = crate::Reg<hc2_tsiz::HC2_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc2_tsiz;
#[doc = "HC2_DMAADDR (rw) register accessor: an alias for `Reg<HC2_DMAADDR_SPEC>`"]
pub type HC2_DMAADDR = crate::Reg<hc2_dmaaddr::HC2_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc2_dmaaddr;
#[doc = "HC3_CHAR (rw) register accessor: an alias for `Reg<HC3_CHAR_SPEC>`"]
pub type HC3_CHAR = crate::Reg<hc3_char::HC3_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc3_char;
#[doc = "HC3_SPLT (rw) register accessor: an alias for `Reg<HC3_SPLT_SPEC>`"]
pub type HC3_SPLT = crate::Reg<hc3_splt::HC3_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc3_splt;
#[doc = "HC3_INT (rw) register accessor: an alias for `Reg<HC3_INT_SPEC>`"]
pub type HC3_INT = crate::Reg<hc3_int::HC3_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc3_int;
#[doc = "HC3_INTMSK (rw) register accessor: an alias for `Reg<HC3_INTMSK_SPEC>`"]
pub type HC3_INTMSK = crate::Reg<hc3_intmsk::HC3_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc3_intmsk;
#[doc = "HC3_TSIZ (rw) register accessor: an alias for `Reg<HC3_TSIZ_SPEC>`"]
pub type HC3_TSIZ = crate::Reg<hc3_tsiz::HC3_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc3_tsiz;
#[doc = "HC3_DMAADDR (rw) register accessor: an alias for `Reg<HC3_DMAADDR_SPEC>`"]
pub type HC3_DMAADDR = crate::Reg<hc3_dmaaddr::HC3_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc3_dmaaddr;
#[doc = "HC4_CHAR (rw) register accessor: an alias for `Reg<HC4_CHAR_SPEC>`"]
pub type HC4_CHAR = crate::Reg<hc4_char::HC4_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc4_char;
#[doc = "HC4_SPLT (rw) register accessor: an alias for `Reg<HC4_SPLT_SPEC>`"]
pub type HC4_SPLT = crate::Reg<hc4_splt::HC4_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc4_splt;
#[doc = "HC4_INT (rw) register accessor: an alias for `Reg<HC4_INT_SPEC>`"]
pub type HC4_INT = crate::Reg<hc4_int::HC4_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc4_int;
#[doc = "HC4_INTMSK (rw) register accessor: an alias for `Reg<HC4_INTMSK_SPEC>`"]
pub type HC4_INTMSK = crate::Reg<hc4_intmsk::HC4_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc4_intmsk;
#[doc = "HC4_TSIZ (rw) register accessor: an alias for `Reg<HC4_TSIZ_SPEC>`"]
pub type HC4_TSIZ = crate::Reg<hc4_tsiz::HC4_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc4_tsiz;
#[doc = "HC4_DMAADDR (rw) register accessor: an alias for `Reg<HC4_DMAADDR_SPEC>`"]
pub type HC4_DMAADDR = crate::Reg<hc4_dmaaddr::HC4_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc4_dmaaddr;
#[doc = "HC5_CHAR (rw) register accessor: an alias for `Reg<HC5_CHAR_SPEC>`"]
pub type HC5_CHAR = crate::Reg<hc5_char::HC5_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc5_char;
#[doc = "HC5_SPLT (rw) register accessor: an alias for `Reg<HC5_SPLT_SPEC>`"]
pub type HC5_SPLT = crate::Reg<hc5_splt::HC5_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc5_splt;
#[doc = "HC5_INT (rw) register accessor: an alias for `Reg<HC5_INT_SPEC>`"]
pub type HC5_INT = crate::Reg<hc5_int::HC5_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc5_int;
#[doc = "HC5_INTMSK (rw) register accessor: an alias for `Reg<HC5_INTMSK_SPEC>`"]
pub type HC5_INTMSK = crate::Reg<hc5_intmsk::HC5_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc5_intmsk;
#[doc = "HC5_TSIZ (rw) register accessor: an alias for `Reg<HC5_TSIZ_SPEC>`"]
pub type HC5_TSIZ = crate::Reg<hc5_tsiz::HC5_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc5_tsiz;
#[doc = "HC5_DMAADDR (rw) register accessor: an alias for `Reg<HC5_DMAADDR_SPEC>`"]
pub type HC5_DMAADDR = crate::Reg<hc5_dmaaddr::HC5_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc5_dmaaddr;
#[doc = "HC6_CHAR (rw) register accessor: an alias for `Reg<HC6_CHAR_SPEC>`"]
pub type HC6_CHAR = crate::Reg<hc6_char::HC6_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc6_char;
#[doc = "HC6_SPLT (rw) register accessor: an alias for `Reg<HC6_SPLT_SPEC>`"]
pub type HC6_SPLT = crate::Reg<hc6_splt::HC6_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc6_splt;
#[doc = "HC6_INT (rw) register accessor: an alias for `Reg<HC6_INT_SPEC>`"]
pub type HC6_INT = crate::Reg<hc6_int::HC6_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc6_int;
#[doc = "HC6_INTMSK (rw) register accessor: an alias for `Reg<HC6_INTMSK_SPEC>`"]
pub type HC6_INTMSK = crate::Reg<hc6_intmsk::HC6_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc6_intmsk;
#[doc = "HC6_TSIZ (rw) register accessor: an alias for `Reg<HC6_TSIZ_SPEC>`"]
pub type HC6_TSIZ = crate::Reg<hc6_tsiz::HC6_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc6_tsiz;
#[doc = "HC6_DMAADDR (rw) register accessor: an alias for `Reg<HC6_DMAADDR_SPEC>`"]
pub type HC6_DMAADDR = crate::Reg<hc6_dmaaddr::HC6_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc6_dmaaddr;
#[doc = "HC7_CHAR (rw) register accessor: an alias for `Reg<HC7_CHAR_SPEC>`"]
pub type HC7_CHAR = crate::Reg<hc7_char::HC7_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc7_char;
#[doc = "HC7_SPLT (rw) register accessor: an alias for `Reg<HC7_SPLT_SPEC>`"]
pub type HC7_SPLT = crate::Reg<hc7_splt::HC7_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc7_splt;
#[doc = "HC7_INT (rw) register accessor: an alias for `Reg<HC7_INT_SPEC>`"]
pub type HC7_INT = crate::Reg<hc7_int::HC7_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc7_int;
#[doc = "HC7_INTMSK (rw) register accessor: an alias for `Reg<HC7_INTMSK_SPEC>`"]
pub type HC7_INTMSK = crate::Reg<hc7_intmsk::HC7_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc7_intmsk;
#[doc = "HC7_TSIZ (rw) register accessor: an alias for `Reg<HC7_TSIZ_SPEC>`"]
pub type HC7_TSIZ = crate::Reg<hc7_tsiz::HC7_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc7_tsiz;
#[doc = "HC7_DMAADDR (rw) register accessor: an alias for `Reg<HC7_DMAADDR_SPEC>`"]
pub type HC7_DMAADDR = crate::Reg<hc7_dmaaddr::HC7_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc7_dmaaddr;
#[doc = "HC8_CHAR (rw) register accessor: an alias for `Reg<HC8_CHAR_SPEC>`"]
pub type HC8_CHAR = crate::Reg<hc8_char::HC8_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc8_char;
#[doc = "HC8_SPLT (rw) register accessor: an alias for `Reg<HC8_SPLT_SPEC>`"]
pub type HC8_SPLT = crate::Reg<hc8_splt::HC8_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc8_splt;
#[doc = "HC8_INT (rw) register accessor: an alias for `Reg<HC8_INT_SPEC>`"]
pub type HC8_INT = crate::Reg<hc8_int::HC8_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc8_int;
#[doc = "HC8_INTMSK (rw) register accessor: an alias for `Reg<HC8_INTMSK_SPEC>`"]
pub type HC8_INTMSK = crate::Reg<hc8_intmsk::HC8_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc8_intmsk;
#[doc = "HC8_TSIZ (rw) register accessor: an alias for `Reg<HC8_TSIZ_SPEC>`"]
pub type HC8_TSIZ = crate::Reg<hc8_tsiz::HC8_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc8_tsiz;
#[doc = "HC8_DMAADDR (rw) register accessor: an alias for `Reg<HC8_DMAADDR_SPEC>`"]
pub type HC8_DMAADDR = crate::Reg<hc8_dmaaddr::HC8_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc8_dmaaddr;
#[doc = "HC9_CHAR (rw) register accessor: an alias for `Reg<HC9_CHAR_SPEC>`"]
pub type HC9_CHAR = crate::Reg<hc9_char::HC9_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc9_char;
#[doc = "HC9_SPLT (rw) register accessor: an alias for `Reg<HC9_SPLT_SPEC>`"]
pub type HC9_SPLT = crate::Reg<hc9_splt::HC9_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc9_splt;
#[doc = "HC9_INT (rw) register accessor: an alias for `Reg<HC9_INT_SPEC>`"]
pub type HC9_INT = crate::Reg<hc9_int::HC9_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc9_int;
#[doc = "HC9_INTMSK (rw) register accessor: an alias for `Reg<HC9_INTMSK_SPEC>`"]
pub type HC9_INTMSK = crate::Reg<hc9_intmsk::HC9_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc9_intmsk;
#[doc = "HC9_TSIZ (rw) register accessor: an alias for `Reg<HC9_TSIZ_SPEC>`"]
pub type HC9_TSIZ = crate::Reg<hc9_tsiz::HC9_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc9_tsiz;
#[doc = "HC9_DMAADDR (rw) register accessor: an alias for `Reg<HC9_DMAADDR_SPEC>`"]
pub type HC9_DMAADDR = crate::Reg<hc9_dmaaddr::HC9_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc9_dmaaddr;
#[doc = "HC10_CHAR (rw) register accessor: an alias for `Reg<HC10_CHAR_SPEC>`"]
pub type HC10_CHAR = crate::Reg<hc10_char::HC10_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc10_char;
#[doc = "HC10_SPLT (rw) register accessor: an alias for `Reg<HC10_SPLT_SPEC>`"]
pub type HC10_SPLT = crate::Reg<hc10_splt::HC10_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc10_splt;
#[doc = "HC10_INT (rw) register accessor: an alias for `Reg<HC10_INT_SPEC>`"]
pub type HC10_INT = crate::Reg<hc10_int::HC10_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc10_int;
#[doc = "HC10_INTMSK (rw) register accessor: an alias for `Reg<HC10_INTMSK_SPEC>`"]
pub type HC10_INTMSK = crate::Reg<hc10_intmsk::HC10_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc10_intmsk;
#[doc = "HC10_TSIZ (rw) register accessor: an alias for `Reg<HC10_TSIZ_SPEC>`"]
pub type HC10_TSIZ = crate::Reg<hc10_tsiz::HC10_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc10_tsiz;
#[doc = "HC10_DMAADDR (rw) register accessor: an alias for `Reg<HC10_DMAADDR_SPEC>`"]
pub type HC10_DMAADDR = crate::Reg<hc10_dmaaddr::HC10_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc10_dmaaddr;
#[doc = "HC11_CHAR (rw) register accessor: an alias for `Reg<HC11_CHAR_SPEC>`"]
pub type HC11_CHAR = crate::Reg<hc11_char::HC11_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc11_char;
#[doc = "HC11_SPLT (rw) register accessor: an alias for `Reg<HC11_SPLT_SPEC>`"]
pub type HC11_SPLT = crate::Reg<hc11_splt::HC11_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc11_splt;
#[doc = "HC11_INT (rw) register accessor: an alias for `Reg<HC11_INT_SPEC>`"]
pub type HC11_INT = crate::Reg<hc11_int::HC11_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc11_int;
#[doc = "HC11_INTMSK (rw) register accessor: an alias for `Reg<HC11_INTMSK_SPEC>`"]
pub type HC11_INTMSK = crate::Reg<hc11_intmsk::HC11_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc11_intmsk;
#[doc = "HC11_TSIZ (rw) register accessor: an alias for `Reg<HC11_TSIZ_SPEC>`"]
pub type HC11_TSIZ = crate::Reg<hc11_tsiz::HC11_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc11_tsiz;
#[doc = "HC11_DMAADDR (rw) register accessor: an alias for `Reg<HC11_DMAADDR_SPEC>`"]
pub type HC11_DMAADDR = crate::Reg<hc11_dmaaddr::HC11_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc11_dmaaddr;
#[doc = "HC12_CHAR (rw) register accessor: an alias for `Reg<HC12_CHAR_SPEC>`"]
pub type HC12_CHAR = crate::Reg<hc12_char::HC12_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc12_char;
#[doc = "HC12_SPLT (rw) register accessor: an alias for `Reg<HC12_SPLT_SPEC>`"]
pub type HC12_SPLT = crate::Reg<hc12_splt::HC12_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc12_splt;
#[doc = "HC12_INT (rw) register accessor: an alias for `Reg<HC12_INT_SPEC>`"]
pub type HC12_INT = crate::Reg<hc12_int::HC12_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc12_int;
#[doc = "HC12_INTMSK (rw) register accessor: an alias for `Reg<HC12_INTMSK_SPEC>`"]
pub type HC12_INTMSK = crate::Reg<hc12_intmsk::HC12_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc12_intmsk;
#[doc = "HC12_TSIZ (rw) register accessor: an alias for `Reg<HC12_TSIZ_SPEC>`"]
pub type HC12_TSIZ = crate::Reg<hc12_tsiz::HC12_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc12_tsiz;
#[doc = "HC12_DMAADDR (rw) register accessor: an alias for `Reg<HC12_DMAADDR_SPEC>`"]
pub type HC12_DMAADDR = crate::Reg<hc12_dmaaddr::HC12_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc12_dmaaddr;
#[doc = "HC13_CHAR (rw) register accessor: an alias for `Reg<HC13_CHAR_SPEC>`"]
pub type HC13_CHAR = crate::Reg<hc13_char::HC13_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc13_char;
#[doc = "HC13_SPLT (rw) register accessor: an alias for `Reg<HC13_SPLT_SPEC>`"]
pub type HC13_SPLT = crate::Reg<hc13_splt::HC13_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc13_splt;
#[doc = "HC13_INT (rw) register accessor: an alias for `Reg<HC13_INT_SPEC>`"]
pub type HC13_INT = crate::Reg<hc13_int::HC13_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc13_int;
#[doc = "HC13_INTMSK (rw) register accessor: an alias for `Reg<HC13_INTMSK_SPEC>`"]
pub type HC13_INTMSK = crate::Reg<hc13_intmsk::HC13_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc13_intmsk;
#[doc = "HC13_TSIZ (rw) register accessor: an alias for `Reg<HC13_TSIZ_SPEC>`"]
pub type HC13_TSIZ = crate::Reg<hc13_tsiz::HC13_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc13_tsiz;
#[doc = "HC13_DMAADDR (rw) register accessor: an alias for `Reg<HC13_DMAADDR_SPEC>`"]
pub type HC13_DMAADDR = crate::Reg<hc13_dmaaddr::HC13_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc13_dmaaddr;
#[doc = "DCFG (rw) register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: an alias for `Reg<DTHRCTL_SPEC>`"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "Device Threshold Control Register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "DIEP0CTL (rw) register accessor: an alias for `Reg<DIEP0CTL_SPEC>`"]
pub type DIEP0CTL = crate::Reg<diep0ctl::DIEP0CTL_SPEC>;
#[doc = "Device Control IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "DIEP0INT (rw) register accessor: an alias for `Reg<DIEP0INT_SPEC>`"]
pub type DIEP0INT = crate::Reg<diep0int::DIEP0INT_SPEC>;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "DIEP0TSIZ (rw) register accessor: an alias for `Reg<DIEP0TSIZ_SPEC>`"]
pub type DIEP0TSIZ = crate::Reg<diep0tsiz::DIEP0TSIZ_SPEC>;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "DIEP0DMAADDR (rw) register accessor: an alias for `Reg<DIEP0DMAADDR_SPEC>`"]
pub type DIEP0DMAADDR = crate::Reg<diep0dmaaddr::DIEP0DMAADDR_SPEC>;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "DIEP0TXFSTS (r) register accessor: an alias for `Reg<DIEP0TXFSTS_SPEC>`"]
pub type DIEP0TXFSTS = crate::Reg<diep0txfsts::DIEP0TXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 0"]
pub mod diep0txfsts;
#[doc = "DIEP0_CTL (rw) register accessor: an alias for `Reg<DIEP0_CTL_SPEC>`"]
pub type DIEP0_CTL = crate::Reg<diep0_ctl::DIEP0_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "DIEP0_INT (rw) register accessor: an alias for `Reg<DIEP0_INT_SPEC>`"]
pub type DIEP0_INT = crate::Reg<diep0_int::DIEP0_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "DIEP0_TSIZ (rw) register accessor: an alias for `Reg<DIEP0_TSIZ_SPEC>`"]
pub type DIEP0_TSIZ = crate::Reg<diep0_tsiz::DIEP0_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "DIEP0_DMAADDR (rw) register accessor: an alias for `Reg<DIEP0_DMAADDR_SPEC>`"]
pub type DIEP0_DMAADDR = crate::Reg<diep0_dmaaddr::DIEP0_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "DIEP0_DTXFSTS (r) register accessor: an alias for `Reg<DIEP0_DTXFSTS_SPEC>`"]
pub type DIEP0_DTXFSTS = crate::Reg<diep0_dtxfsts::DIEP0_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep0_dtxfsts;
#[doc = "DIEP1_CTL (rw) register accessor: an alias for `Reg<DIEP1_CTL_SPEC>`"]
pub type DIEP1_CTL = crate::Reg<diep1_ctl::DIEP1_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "DIEP1_INT (rw) register accessor: an alias for `Reg<DIEP1_INT_SPEC>`"]
pub type DIEP1_INT = crate::Reg<diep1_int::DIEP1_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "DIEP1_TSIZ (rw) register accessor: an alias for `Reg<DIEP1_TSIZ_SPEC>`"]
pub type DIEP1_TSIZ = crate::Reg<diep1_tsiz::DIEP1_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "DIEP1_DMAADDR (rw) register accessor: an alias for `Reg<DIEP1_DMAADDR_SPEC>`"]
pub type DIEP1_DMAADDR = crate::Reg<diep1_dmaaddr::DIEP1_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "DIEP1_DTXFSTS (r) register accessor: an alias for `Reg<DIEP1_DTXFSTS_SPEC>`"]
pub type DIEP1_DTXFSTS = crate::Reg<diep1_dtxfsts::DIEP1_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep1_dtxfsts;
#[doc = "DIEP2_CTL (rw) register accessor: an alias for `Reg<DIEP2_CTL_SPEC>`"]
pub type DIEP2_CTL = crate::Reg<diep2_ctl::DIEP2_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "DIEP2_INT (rw) register accessor: an alias for `Reg<DIEP2_INT_SPEC>`"]
pub type DIEP2_INT = crate::Reg<diep2_int::DIEP2_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "DIEP2_TSIZ (rw) register accessor: an alias for `Reg<DIEP2_TSIZ_SPEC>`"]
pub type DIEP2_TSIZ = crate::Reg<diep2_tsiz::DIEP2_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "DIEP2_DMAADDR (rw) register accessor: an alias for `Reg<DIEP2_DMAADDR_SPEC>`"]
pub type DIEP2_DMAADDR = crate::Reg<diep2_dmaaddr::DIEP2_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "DIEP2_DTXFSTS (r) register accessor: an alias for `Reg<DIEP2_DTXFSTS_SPEC>`"]
pub type DIEP2_DTXFSTS = crate::Reg<diep2_dtxfsts::DIEP2_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep2_dtxfsts;
#[doc = "DIEP3_CTL (rw) register accessor: an alias for `Reg<DIEP3_CTL_SPEC>`"]
pub type DIEP3_CTL = crate::Reg<diep3_ctl::DIEP3_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep3_ctl;
#[doc = "DIEP3_INT (rw) register accessor: an alias for `Reg<DIEP3_INT_SPEC>`"]
pub type DIEP3_INT = crate::Reg<diep3_int::DIEP3_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep3_int;
#[doc = "DIEP3_TSIZ (rw) register accessor: an alias for `Reg<DIEP3_TSIZ_SPEC>`"]
pub type DIEP3_TSIZ = crate::Reg<diep3_tsiz::DIEP3_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep3_tsiz;
#[doc = "DIEP3_DMAADDR (rw) register accessor: an alias for `Reg<DIEP3_DMAADDR_SPEC>`"]
pub type DIEP3_DMAADDR = crate::Reg<diep3_dmaaddr::DIEP3_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep3_dmaaddr;
#[doc = "DIEP3_DTXFSTS (r) register accessor: an alias for `Reg<DIEP3_DTXFSTS_SPEC>`"]
pub type DIEP3_DTXFSTS = crate::Reg<diep3_dtxfsts::DIEP3_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep3_dtxfsts;
#[doc = "DIEP4_CTL (rw) register accessor: an alias for `Reg<DIEP4_CTL_SPEC>`"]
pub type DIEP4_CTL = crate::Reg<diep4_ctl::DIEP4_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep4_ctl;
#[doc = "DIEP4_INT (rw) register accessor: an alias for `Reg<DIEP4_INT_SPEC>`"]
pub type DIEP4_INT = crate::Reg<diep4_int::DIEP4_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep4_int;
#[doc = "DIEP4_TSIZ (rw) register accessor: an alias for `Reg<DIEP4_TSIZ_SPEC>`"]
pub type DIEP4_TSIZ = crate::Reg<diep4_tsiz::DIEP4_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep4_tsiz;
#[doc = "DIEP4_DMAADDR (rw) register accessor: an alias for `Reg<DIEP4_DMAADDR_SPEC>`"]
pub type DIEP4_DMAADDR = crate::Reg<diep4_dmaaddr::DIEP4_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep4_dmaaddr;
#[doc = "DIEP4_DTXFSTS (r) register accessor: an alias for `Reg<DIEP4_DTXFSTS_SPEC>`"]
pub type DIEP4_DTXFSTS = crate::Reg<diep4_dtxfsts::DIEP4_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep4_dtxfsts;
#[doc = "DIEP5_CTL (rw) register accessor: an alias for `Reg<DIEP5_CTL_SPEC>`"]
pub type DIEP5_CTL = crate::Reg<diep5_ctl::DIEP5_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep5_ctl;
#[doc = "DIEP5_INT (rw) register accessor: an alias for `Reg<DIEP5_INT_SPEC>`"]
pub type DIEP5_INT = crate::Reg<diep5_int::DIEP5_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep5_int;
#[doc = "DIEP5_TSIZ (rw) register accessor: an alias for `Reg<DIEP5_TSIZ_SPEC>`"]
pub type DIEP5_TSIZ = crate::Reg<diep5_tsiz::DIEP5_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep5_tsiz;
#[doc = "DIEP5_DMAADDR (rw) register accessor: an alias for `Reg<DIEP5_DMAADDR_SPEC>`"]
pub type DIEP5_DMAADDR = crate::Reg<diep5_dmaaddr::DIEP5_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep5_dmaaddr;
#[doc = "DIEP5_DTXFSTS (r) register accessor: an alias for `Reg<DIEP5_DTXFSTS_SPEC>`"]
pub type DIEP5_DTXFSTS = crate::Reg<diep5_dtxfsts::DIEP5_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep5_dtxfsts;
#[doc = "DOEP0CTL (rw) register accessor: an alias for `Reg<DOEP0CTL_SPEC>`"]
pub type DOEP0CTL = crate::Reg<doep0ctl::DOEP0CTL_SPEC>;
#[doc = "Device Control OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "DOEP0INT (rw) register accessor: an alias for `Reg<DOEP0INT_SPEC>`"]
pub type DOEP0INT = crate::Reg<doep0int::DOEP0INT_SPEC>;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "DOEP0TSIZ (rw) register accessor: an alias for `Reg<DOEP0TSIZ_SPEC>`"]
pub type DOEP0TSIZ = crate::Reg<doep0tsiz::DOEP0TSIZ_SPEC>;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "DOEP0DMAADDR (rw) register accessor: an alias for `Reg<DOEP0DMAADDR_SPEC>`"]
pub type DOEP0DMAADDR = crate::Reg<doep0dmaaddr::DOEP0DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "DOEP0_CTL (rw) register accessor: an alias for `Reg<DOEP0_CTL_SPEC>`"]
pub type DOEP0_CTL = crate::Reg<doep0_ctl::DOEP0_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "DOEP0_INT (rw) register accessor: an alias for `Reg<DOEP0_INT_SPEC>`"]
pub type DOEP0_INT = crate::Reg<doep0_int::DOEP0_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "DOEP0_TSIZ (rw) register accessor: an alias for `Reg<DOEP0_TSIZ_SPEC>`"]
pub type DOEP0_TSIZ = crate::Reg<doep0_tsiz::DOEP0_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "DOEP0_DMAADDR (rw) register accessor: an alias for `Reg<DOEP0_DMAADDR_SPEC>`"]
pub type DOEP0_DMAADDR = crate::Reg<doep0_dmaaddr::DOEP0_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "DOEP1_CTL (rw) register accessor: an alias for `Reg<DOEP1_CTL_SPEC>`"]
pub type DOEP1_CTL = crate::Reg<doep1_ctl::DOEP1_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "DOEP1_INT (rw) register accessor: an alias for `Reg<DOEP1_INT_SPEC>`"]
pub type DOEP1_INT = crate::Reg<doep1_int::DOEP1_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "DOEP1_TSIZ (rw) register accessor: an alias for `Reg<DOEP1_TSIZ_SPEC>`"]
pub type DOEP1_TSIZ = crate::Reg<doep1_tsiz::DOEP1_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "DOEP1_DMAADDR (rw) register accessor: an alias for `Reg<DOEP1_DMAADDR_SPEC>`"]
pub type DOEP1_DMAADDR = crate::Reg<doep1_dmaaddr::DOEP1_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "DOEP2_CTL (rw) register accessor: an alias for `Reg<DOEP2_CTL_SPEC>`"]
pub type DOEP2_CTL = crate::Reg<doep2_ctl::DOEP2_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "DOEP2_INT (rw) register accessor: an alias for `Reg<DOEP2_INT_SPEC>`"]
pub type DOEP2_INT = crate::Reg<doep2_int::DOEP2_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "DOEP2_TSIZ (rw) register accessor: an alias for `Reg<DOEP2_TSIZ_SPEC>`"]
pub type DOEP2_TSIZ = crate::Reg<doep2_tsiz::DOEP2_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "DOEP2_DMAADDR (rw) register accessor: an alias for `Reg<DOEP2_DMAADDR_SPEC>`"]
pub type DOEP2_DMAADDR = crate::Reg<doep2_dmaaddr::DOEP2_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "DOEP3_CTL (rw) register accessor: an alias for `Reg<DOEP3_CTL_SPEC>`"]
pub type DOEP3_CTL = crate::Reg<doep3_ctl::DOEP3_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep3_ctl;
#[doc = "DOEP3_INT (rw) register accessor: an alias for `Reg<DOEP3_INT_SPEC>`"]
pub type DOEP3_INT = crate::Reg<doep3_int::DOEP3_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep3_int;
#[doc = "DOEP3_TSIZ (rw) register accessor: an alias for `Reg<DOEP3_TSIZ_SPEC>`"]
pub type DOEP3_TSIZ = crate::Reg<doep3_tsiz::DOEP3_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep3_tsiz;
#[doc = "DOEP3_DMAADDR (rw) register accessor: an alias for `Reg<DOEP3_DMAADDR_SPEC>`"]
pub type DOEP3_DMAADDR = crate::Reg<doep3_dmaaddr::DOEP3_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep3_dmaaddr;
#[doc = "DOEP4_CTL (rw) register accessor: an alias for `Reg<DOEP4_CTL_SPEC>`"]
pub type DOEP4_CTL = crate::Reg<doep4_ctl::DOEP4_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep4_ctl;
#[doc = "DOEP4_INT (rw) register accessor: an alias for `Reg<DOEP4_INT_SPEC>`"]
pub type DOEP4_INT = crate::Reg<doep4_int::DOEP4_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep4_int;
#[doc = "DOEP4_TSIZ (rw) register accessor: an alias for `Reg<DOEP4_TSIZ_SPEC>`"]
pub type DOEP4_TSIZ = crate::Reg<doep4_tsiz::DOEP4_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep4_tsiz;
#[doc = "DOEP4_DMAADDR (rw) register accessor: an alias for `Reg<DOEP4_DMAADDR_SPEC>`"]
pub type DOEP4_DMAADDR = crate::Reg<doep4_dmaaddr::DOEP4_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep4_dmaaddr;
#[doc = "DOEP5_CTL (rw) register accessor: an alias for `Reg<DOEP5_CTL_SPEC>`"]
pub type DOEP5_CTL = crate::Reg<doep5_ctl::DOEP5_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep5_ctl;
#[doc = "DOEP5_INT (rw) register accessor: an alias for `Reg<DOEP5_INT_SPEC>`"]
pub type DOEP5_INT = crate::Reg<doep5_int::DOEP5_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep5_int;
#[doc = "DOEP5_TSIZ (rw) register accessor: an alias for `Reg<DOEP5_TSIZ_SPEC>`"]
pub type DOEP5_TSIZ = crate::Reg<doep5_tsiz::DOEP5_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep5_tsiz;
#[doc = "DOEP5_DMAADDR (rw) register accessor: an alias for `Reg<DOEP5_DMAADDR_SPEC>`"]
pub type DOEP5_DMAADDR = crate::Reg<doep5_dmaaddr::DOEP5_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep5_dmaaddr;
#[doc = "PCGCCTL (rw) register accessor: an alias for `Reg<PCGCCTL_SPEC>`"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
