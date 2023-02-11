#[doc = "Register `IRQSTATUS` reader"]
pub struct R(crate::R<IRQSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQSTATUS` writer"]
pub struct W(crate::W<IRQSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQSTATUS_SPEC>;
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
impl From<crate::W<IRQSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODEMFAIL` reader - Mode M Failure"]
pub type MODEMFAIL_R = crate::BitReader<bool>;
#[doc = "Field `MODEMFAIL` writer - Mode M Failure"]
pub type MODEMFAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `UNDERFLOWDET` reader - Underflow Detected"]
pub type UNDERFLOWDET_R = crate::BitReader<bool>;
#[doc = "Field `UNDERFLOWDET` writer - Underflow Detected"]
pub type UNDERFLOWDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `INDIRECTOPDONE` reader - Indirect Operation Complete"]
pub type INDIRECTOPDONE_R = crate::BitReader<bool>;
#[doc = "Field `INDIRECTOPDONE` writer - Indirect Operation Complete"]
pub type INDIRECTOPDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `INDIRECTREADREJECT` reader - Indirect Operation Was Requested but Could Not Be Accepted"]
pub type INDIRECTREADREJECT_R = crate::BitReader<bool>;
#[doc = "Field `INDIRECTREADREJECT` writer - Indirect Operation Was Requested but Could Not Be Accepted"]
pub type INDIRECTREADREJECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `PROTWRATTEMPT` reader - Write to Protected Area Was Attempted and Rejected"]
pub type PROTWRATTEMPT_R = crate::BitReader<bool>;
#[doc = "Field `PROTWRATTEMPT` writer - Write to Protected Area Was Attempted and Rejected"]
pub type PROTWRATTEMPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `ILLEGALACCESSDET` reader - Illegal Memory Access Has Been Detected"]
pub type ILLEGALACCESSDET_R = crate::BitReader<bool>;
#[doc = "Field `ILLEGALACCESSDET` writer - Illegal Memory Access Has Been Detected"]
pub type ILLEGALACCESSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `INDIRECTXFERLEVELBREACH` reader - Indirect Transfer Watermark Level Breached"]
pub type INDIRECTXFERLEVELBREACH_R = crate::BitReader<bool>;
#[doc = "Field `INDIRECTXFERLEVELBREACH` writer - Indirect Transfer Watermark Level Breached"]
pub type INDIRECTXFERLEVELBREACH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `RECVOVERFLOW` reader - Receive Overflow"]
pub type RECVOVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RECVOVERFLOW` writer - Receive Overflow"]
pub type RECVOVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `TXFIFONOTFULL` reader - Small TX FIFO Not Full"]
pub type TXFIFONOTFULL_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFONOTFULL` writer - Small TX FIFO Not Full"]
pub type TXFIFONOTFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `TXFIFOFULL` reader - Small TX FIFO Full"]
pub type TXFIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOFULL` writer - Small TX FIFO Full"]
pub type TXFIFOFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `RXFIFONOTEMPTY` reader - Small RX FIFO Not Empty"]
pub type RXFIFONOTEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFONOTEMPTY` writer - Small RX FIFO Not Empty"]
pub type RXFIFONOTEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `RXFIFOFULL` reader - Small RX FIFO Full"]
pub type RXFIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOFULL` writer - Small RX FIFO Full"]
pub type RXFIFOFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `INDRDSRAMFULL` reader - Indirect Read Partition Overflow"]
pub type INDRDSRAMFULL_R = crate::BitReader<bool>;
#[doc = "Field `INDRDSRAMFULL` writer - Indirect Read Partition Overflow"]
pub type INDRDSRAMFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `POLLEXPINT` reader - The Maximum Number of Programmed Polls Cycles is Expired"]
pub type POLLEXPINT_R = crate::BitReader<bool>;
#[doc = "Field `POLLEXPINT` writer - The Maximum Number of Programmed Polls Cycles is Expired"]
pub type POLLEXPINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `STIGREQINT` reader - The Controller is Ready for Getting Another STIG Request"]
pub type STIGREQINT_R = crate::BitReader<bool>;
#[doc = "Field `STIGREQINT` writer - The Controller is Ready for Getting Another STIG Request"]
pub type STIGREQINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `RXCRCDATAERR` reader - RX CRC Data Error"]
pub type RXCRCDATAERR_R = crate::BitReader<bool>;
#[doc = "Field `RXCRCDATAERR` writer - RX CRC Data Error"]
pub type RXCRCDATAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `RXCRCDATAVAL` reader - RX CRC Data Valid"]
pub type RXCRCDATAVAL_R = crate::BitReader<bool>;
#[doc = "Field `RXCRCDATAVAL` writer - RX CRC Data Valid"]
pub type RXCRCDATAVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
#[doc = "Field `TXCRCCHUNKBRK` reader - TX CRC Chunk Was Broken"]
pub type TXCRCCHUNKBRK_R = crate::BitReader<bool>;
#[doc = "Field `TXCRCCHUNKBRK` writer - TX CRC Chunk Was Broken"]
pub type TXCRCCHUNKBRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mode M Failure"]
    #[inline(always)]
    pub fn modemfail(&self) -> MODEMFAIL_R {
        MODEMFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Detected"]
    #[inline(always)]
    pub fn underflowdet(&self) -> UNDERFLOWDET_R {
        UNDERFLOWDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indirect Operation Complete"]
    #[inline(always)]
    pub fn indirectopdone(&self) -> INDIRECTOPDONE_R {
        INDIRECTOPDONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indirect Operation Was Requested but Could Not Be Accepted"]
    #[inline(always)]
    pub fn indirectreadreject(&self) -> INDIRECTREADREJECT_R {
        INDIRECTREADREJECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write to Protected Area Was Attempted and Rejected"]
    #[inline(always)]
    pub fn protwrattempt(&self) -> PROTWRATTEMPT_R {
        PROTWRATTEMPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Illegal Memory Access Has Been Detected"]
    #[inline(always)]
    pub fn illegalaccessdet(&self) -> ILLEGALACCESSDET_R {
        ILLEGALACCESSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indirect Transfer Watermark Level Breached"]
    #[inline(always)]
    pub fn indirectxferlevelbreach(&self) -> INDIRECTXFERLEVELBREACH_R {
        INDIRECTXFERLEVELBREACH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Overflow"]
    #[inline(always)]
    pub fn recvoverflow(&self) -> RECVOVERFLOW_R {
        RECVOVERFLOW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full"]
    #[inline(always)]
    pub fn txfifonotfull(&self) -> TXFIFONOTFULL_R {
        TXFIFONOTFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Small TX FIFO Full"]
    #[inline(always)]
    pub fn txfifofull(&self) -> TXFIFOFULL_R {
        TXFIFOFULL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rxfifonotempty(&self) -> RXFIFONOTEMPTY_R {
        RXFIFONOTEMPTY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Small RX FIFO Full"]
    #[inline(always)]
    pub fn rxfifofull(&self) -> RXFIFOFULL_R {
        RXFIFOFULL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow"]
    #[inline(always)]
    pub fn indrdsramfull(&self) -> INDRDSRAMFULL_R {
        INDRDSRAMFULL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The Maximum Number of Programmed Polls Cycles is Expired"]
    #[inline(always)]
    pub fn pollexpint(&self) -> POLLEXPINT_R {
        POLLEXPINT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The Controller is Ready for Getting Another STIG Request"]
    #[inline(always)]
    pub fn stigreqint(&self) -> STIGREQINT_R {
        STIGREQINT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RX CRC Data Error"]
    #[inline(always)]
    pub fn rxcrcdataerr(&self) -> RXCRCDATAERR_R {
        RXCRCDATAERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX CRC Data Valid"]
    #[inline(always)]
    pub fn rxcrcdataval(&self) -> RXCRCDATAVAL_R {
        RXCRCDATAVAL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken"]
    #[inline(always)]
    pub fn txcrcchunkbrk(&self) -> TXCRCCHUNKBRK_R {
        TXCRCCHUNKBRK_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode M Failure"]
    #[inline(always)]
    #[must_use]
    pub fn modemfail(&mut self) -> MODEMFAIL_W<0> {
        MODEMFAIL_W::new(self)
    }
    #[doc = "Bit 1 - Underflow Detected"]
    #[inline(always)]
    #[must_use]
    pub fn underflowdet(&mut self) -> UNDERFLOWDET_W<1> {
        UNDERFLOWDET_W::new(self)
    }
    #[doc = "Bit 2 - Indirect Operation Complete"]
    #[inline(always)]
    #[must_use]
    pub fn indirectopdone(&mut self) -> INDIRECTOPDONE_W<2> {
        INDIRECTOPDONE_W::new(self)
    }
    #[doc = "Bit 3 - Indirect Operation Was Requested but Could Not Be Accepted"]
    #[inline(always)]
    #[must_use]
    pub fn indirectreadreject(&mut self) -> INDIRECTREADREJECT_W<3> {
        INDIRECTREADREJECT_W::new(self)
    }
    #[doc = "Bit 4 - Write to Protected Area Was Attempted and Rejected"]
    #[inline(always)]
    #[must_use]
    pub fn protwrattempt(&mut self) -> PROTWRATTEMPT_W<4> {
        PROTWRATTEMPT_W::new(self)
    }
    #[doc = "Bit 5 - Illegal Memory Access Has Been Detected"]
    #[inline(always)]
    #[must_use]
    pub fn illegalaccessdet(&mut self) -> ILLEGALACCESSDET_W<5> {
        ILLEGALACCESSDET_W::new(self)
    }
    #[doc = "Bit 6 - Indirect Transfer Watermark Level Breached"]
    #[inline(always)]
    #[must_use]
    pub fn indirectxferlevelbreach(&mut self) -> INDIRECTXFERLEVELBREACH_W<6> {
        INDIRECTXFERLEVELBREACH_W::new(self)
    }
    #[doc = "Bit 7 - Receive Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn recvoverflow(&mut self) -> RECVOVERFLOW_W<7> {
        RECVOVERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full"]
    #[inline(always)]
    #[must_use]
    pub fn txfifonotfull(&mut self) -> TXFIFONOTFULL_W<8> {
        TXFIFONOTFULL_W::new(self)
    }
    #[doc = "Bit 9 - Small TX FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn txfifofull(&mut self) -> TXFIFOFULL_W<9> {
        TXFIFOFULL_W::new(self)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifonotempty(&mut self) -> RXFIFONOTEMPTY_W<10> {
        RXFIFONOTEMPTY_W::new(self)
    }
    #[doc = "Bit 11 - Small RX FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifofull(&mut self) -> RXFIFOFULL_W<11> {
        RXFIFOFULL_W::new(self)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn indrdsramfull(&mut self) -> INDRDSRAMFULL_W<12> {
        INDRDSRAMFULL_W::new(self)
    }
    #[doc = "Bit 13 - The Maximum Number of Programmed Polls Cycles is Expired"]
    #[inline(always)]
    #[must_use]
    pub fn pollexpint(&mut self) -> POLLEXPINT_W<13> {
        POLLEXPINT_W::new(self)
    }
    #[doc = "Bit 14 - The Controller is Ready for Getting Another STIG Request"]
    #[inline(always)]
    #[must_use]
    pub fn stigreqint(&mut self) -> STIGREQINT_W<14> {
        STIGREQINT_W::new(self)
    }
    #[doc = "Bit 16 - RX CRC Data Error"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcdataerr(&mut self) -> RXCRCDATAERR_W<16> {
        RXCRCDATAERR_W::new(self)
    }
    #[doc = "Bit 17 - RX CRC Data Valid"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcdataval(&mut self) -> RXCRCDATAVAL_W<17> {
        RXCRCDATAVAL_W::new(self)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken"]
    #[inline(always)]
    #[must_use]
    pub fn txcrcchunkbrk(&mut self) -> TXCRCCHUNKBRK_W<18> {
        TXCRCCHUNKBRK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstatus](index.html) module"]
pub struct IRQSTATUS_SPEC;
impl crate::RegisterSpec for IRQSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqstatus::R](R) reader structure"]
impl crate::Readable for IRQSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqstatus::W](W) writer structure"]
impl crate::Writable for IRQSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQSTATUS to value 0"]
impl crate::Resettable for IRQSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
