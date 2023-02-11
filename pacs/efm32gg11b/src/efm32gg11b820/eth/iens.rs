#[doc = "Register `IENS` writer"]
pub struct W(crate::W<IENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MNGMNTDONE` writer - Enable management done interrupt"]
pub type MNGMNTDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `RXCMPLT` writer - Enable receive complete interrupt"]
pub type RXCMPLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `RXUSEDBITREAD` writer - Enable receive used bit read interrupt"]
pub type RXUSEDBITREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `TXUSEDBITREAD` writer - Enable transmit used bit read interrupt"]
pub type TXUSEDBITREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `TXUNDERRUN` writer - Enable transmit buffer under run interrupt"]
pub type TXUNDERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `RTRYLMTORLATECOL` writer - Enable retry limit exceeded or late collision interrupt"]
pub type RTRYLMTORLATECOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `AMBAERR` writer - Enable transmit frame corruption due to AMBA (AHB) error interrupt"]
pub type AMBAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `TXCMPLT` writer - Enable transmit complete interrupt"]
pub type TXCMPLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `RXOVERRUN` writer - Enable receive overrun interrupt"]
pub type RXOVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `RESPNOTOK` writer - Enable bresp/hresp not OK interrupt"]
pub type RESPNOTOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `NONZEROPFRMQUANT` writer - Enable pause frame with non-zero pause quantum interrupt"]
pub type NONZEROPFRMQUANT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PAUSETIMEZERO` writer - Enable pause time zero interrupt"]
pub type PAUSETIMEZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PFRMTX` writer - Enable pause frame transmitted interrupt"]
pub type PFRMTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PTPDLYREQFRMRX` writer - Enable PTP delay_req frame received interrupt"]
pub type PTPDLYREQFRMRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PTPSYNCFRMRX` writer - Enable PTP sync frame received interrupt"]
pub type PTPSYNCFRMRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PTPDLYREQFRMTX` writer - Enable PTP delay_req frame transmitted interrupt"]
pub type PTPDLYREQFRMTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PTPSYNCFRMTX` writer - Enable PTP sync frame transmitted interrupt"]
pub type PTPSYNCFRMTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PTPPDLYREQFRMRX` writer - Enable PTP pdelay_req frame received interrupt"]
pub type PTPPDLYREQFRMRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PTPPDLYRESPFRMRX` writer - Enable PTP pdelay_resp frame received interrupt"]
pub type PTPPDLYRESPFRMRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PTPPDLYREQFRMTX` writer - Enable PTP pdelay_req frame transmitted interrupt"]
pub type PTPPDLYREQFRMTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `PTPPDLYRESPFRMTX` writer - Enable PTP pdelay_resp frame transmitted interrupt"]
pub type PTPPDLYRESPFRMTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `TSUSECREGINCR` writer - Enable TSU seconds register increment interrupt"]
pub type TSUSECREGINCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `RXLPIINDC` writer - Enable RX LPI indication interrupt"]
pub type RXLPIINDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `WOLEVNTRX` writer - Enable WOL event received interrupt"]
pub type WOLEVNTRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
#[doc = "Field `TSUTIMERCOMP` writer - Enable TSU timer comparison interrupt."]
pub type TSUTIMERCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Enable management done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mngmntdone(&mut self) -> MNGMNTDONE_W<0> {
        MNGMNTDONE_W::new(self)
    }
    #[doc = "Bit 1 - Enable receive complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxcmplt(&mut self) -> RXCMPLT_W<1> {
        RXCMPLT_W::new(self)
    }
    #[doc = "Bit 2 - Enable receive used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxusedbitread(&mut self) -> RXUSEDBITREAD_W<2> {
        RXUSEDBITREAD_W::new(self)
    }
    #[doc = "Bit 3 - Enable transmit used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txusedbitread(&mut self) -> TXUSEDBITREAD_W<3> {
        TXUSEDBITREAD_W::new(self)
    }
    #[doc = "Bit 4 - Enable transmit buffer under run interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W<4> {
        TXUNDERRUN_W::new(self)
    }
    #[doc = "Bit 5 - Enable retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rtrylmtorlatecol(&mut self) -> RTRYLMTORLATECOL_W<5> {
        RTRYLMTORLATECOL_W::new(self)
    }
    #[doc = "Bit 6 - Enable transmit frame corruption due to AMBA (AHB) error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ambaerr(&mut self) -> AMBAERR_W<6> {
        AMBAERR_W::new(self)
    }
    #[doc = "Bit 7 - Enable transmit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txcmplt(&mut self) -> TXCMPLT_W<7> {
        TXCMPLT_W::new(self)
    }
    #[doc = "Bit 10 - Enable receive overrun interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W<10> {
        RXOVERRUN_W::new(self)
    }
    #[doc = "Bit 11 - Enable bresp/hresp not OK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn respnotok(&mut self) -> RESPNOTOK_W<11> {
        RESPNOTOK_W::new(self)
    }
    #[doc = "Bit 12 - Enable pause frame with non-zero pause quantum interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nonzeropfrmquant(&mut self) -> NONZEROPFRMQUANT_W<12> {
        NONZEROPFRMQUANT_W::new(self)
    }
    #[doc = "Bit 13 - Enable pause time zero interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pausetimezero(&mut self) -> PAUSETIMEZERO_W<13> {
        PAUSETIMEZERO_W::new(self)
    }
    #[doc = "Bit 14 - Enable pause frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pfrmtx(&mut self) -> PFRMTX_W<14> {
        PFRMTX_W::new(self)
    }
    #[doc = "Bit 18 - Enable PTP delay_req frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpdlyreqfrmrx(&mut self) -> PTPDLYREQFRMRX_W<18> {
        PTPDLYREQFRMRX_W::new(self)
    }
    #[doc = "Bit 19 - Enable PTP sync frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpsyncfrmrx(&mut self) -> PTPSYNCFRMRX_W<19> {
        PTPSYNCFRMRX_W::new(self)
    }
    #[doc = "Bit 20 - Enable PTP delay_req frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpdlyreqfrmtx(&mut self) -> PTPDLYREQFRMTX_W<20> {
        PTPDLYREQFRMTX_W::new(self)
    }
    #[doc = "Bit 21 - Enable PTP sync frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpsyncfrmtx(&mut self) -> PTPSYNCFRMTX_W<21> {
        PTPSYNCFRMTX_W::new(self)
    }
    #[doc = "Bit 22 - Enable PTP pdelay_req frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyreqfrmrx(&mut self) -> PTPPDLYREQFRMRX_W<22> {
        PTPPDLYREQFRMRX_W::new(self)
    }
    #[doc = "Bit 23 - Enable PTP pdelay_resp frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyrespfrmrx(&mut self) -> PTPPDLYRESPFRMRX_W<23> {
        PTPPDLYRESPFRMRX_W::new(self)
    }
    #[doc = "Bit 24 - Enable PTP pdelay_req frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyreqfrmtx(&mut self) -> PTPPDLYREQFRMTX_W<24> {
        PTPPDLYREQFRMTX_W::new(self)
    }
    #[doc = "Bit 25 - Enable PTP pdelay_resp frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyrespfrmtx(&mut self) -> PTPPDLYRESPFRMTX_W<25> {
        PTPPDLYRESPFRMTX_W::new(self)
    }
    #[doc = "Bit 26 - Enable TSU seconds register increment interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tsusecregincr(&mut self) -> TSUSECREGINCR_W<26> {
        TSUSECREGINCR_W::new(self)
    }
    #[doc = "Bit 27 - Enable RX LPI indication interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxlpiindc(&mut self) -> RXLPIINDC_W<27> {
        RXLPIINDC_W::new(self)
    }
    #[doc = "Bit 28 - Enable WOL event received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wolevntrx(&mut self) -> WOLEVNTRX_W<28> {
        WOLEVNTRX_W::new(self)
    }
    #[doc = "Bit 29 - Enable TSU timer comparison interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tsutimercomp(&mut self) -> TSUTIMERCOMP_W<29> {
        TSUTIMERCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iens](index.html) module"]
pub struct IENS_SPEC;
impl crate::RegisterSpec for IENS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iens::W](W) writer structure"]
impl crate::Writable for IENS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IENS to value 0"]
impl crate::Resettable for IENS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
