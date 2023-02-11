#[doc = "Register `IFENC` reader"]
pub struct R(crate::R<IFENC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFENC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFENC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFENC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFENC` writer"]
pub struct W(crate::W<IFENC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFENC_SPEC>;
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
impl From<crate::W<IFENC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFENC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDCOMEN` reader - Command Complete Signal Enable"]
pub type CMDCOMEN_R = crate::BitReader<bool>;
#[doc = "Field `CMDCOMEN` writer - Command Complete Signal Enable"]
pub type CMDCOMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `TRANCOMEN` reader - Transfer Complete Signal Enable"]
pub type TRANCOMEN_R = crate::BitReader<bool>;
#[doc = "Field `TRANCOMEN` writer - Transfer Complete Signal Enable"]
pub type TRANCOMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `BLKGAPEVTEN` reader - Block Gap Event Signal Enable"]
pub type BLKGAPEVTEN_R = crate::BitReader<bool>;
#[doc = "Field `BLKGAPEVTEN` writer - Block Gap Event Signal Enable"]
pub type BLKGAPEVTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `DMAINTEN` reader - DMA Interrupt Signal Enable"]
pub type DMAINTEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAINTEN` writer - DMA Interrupt Signal Enable"]
pub type DMAINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `BUFWRRDYEN` reader - Buffer Write Ready Signal Enable"]
pub type BUFWRRDYEN_R = crate::BitReader<bool>;
#[doc = "Field `BUFWRRDYEN` writer - Buffer Write Ready Signal Enable"]
pub type BUFWRRDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `BUFRDRDYEN` reader - Buffer Read Ready Signal Enable"]
pub type BUFRDRDYEN_R = crate::BitReader<bool>;
#[doc = "Field `BUFRDRDYEN` writer - Buffer Read Ready Signal Enable"]
pub type BUFRDRDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `CARDINSEN` reader - Card Insertion Signal Enable"]
pub type CARDINSEN_R = crate::BitReader<bool>;
#[doc = "Field `CARDINSEN` writer - Card Insertion Signal Enable"]
pub type CARDINSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `CARDRMEN` reader - Card Removal Signal Enable"]
pub type CARDRMEN_R = crate::BitReader<bool>;
#[doc = "Field `CARDRMEN` writer - Card Removal Signal Enable"]
pub type CARDRMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `CARDINTEN` reader - Card Interrupt Signal Enable"]
pub type CARDINTEN_R = crate::BitReader<bool>;
#[doc = "Field `CARDINTEN` writer - Card Interrupt Signal Enable"]
pub type CARDINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `RETUNINGEVTEN` reader - Re-Tunning Event Signal Enable"]
pub type RETUNINGEVTEN_R = crate::BitReader<bool>;
#[doc = "Field `RETUNINGEVTEN` writer - Re-Tunning Event Signal Enable"]
pub type RETUNINGEVTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `BOOTACKRCVEN` reader - Boot Ack Received Signal Enable"]
pub type BOOTACKRCVEN_R = crate::BitReader<bool>;
#[doc = "Field `BOOTACKRCVEN` writer - Boot Ack Received Signal Enable"]
pub type BOOTACKRCVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `BOOTTERMINATEEN` reader - Boot Terminate Interrupt Signal Enable"]
pub type BOOTTERMINATEEN_R = crate::BitReader<bool>;
#[doc = "Field `BOOTTERMINATEEN` writer - Boot Terminate Interrupt Signal Enable"]
pub type BOOTTERMINATEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `CMDTOUTERREN` reader - Command Time-out Error Status Enable"]
pub type CMDTOUTERREN_R = crate::BitReader<bool>;
#[doc = "Field `CMDTOUTERREN` writer - Command Time-out Error Status Enable"]
pub type CMDTOUTERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `CMDCRCERREN` reader - Command CRC Error Status Enable"]
pub type CMDCRCERREN_R = crate::BitReader<bool>;
#[doc = "Field `CMDCRCERREN` writer - Command CRC Error Status Enable"]
pub type CMDCRCERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `CMDENDBITERREN` reader - Command End Bit Error Status Enable"]
pub type CMDENDBITERREN_R = crate::BitReader<bool>;
#[doc = "Field `CMDENDBITERREN` writer - Command End Bit Error Status Enable"]
pub type CMDENDBITERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `CMDINDEXERREN` reader - Command Index Error Status Enable"]
pub type CMDINDEXERREN_R = crate::BitReader<bool>;
#[doc = "Field `CMDINDEXERREN` writer - Command Index Error Status Enable"]
pub type CMDINDEXERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `DATTOUTERREN` reader - Data Timeout Error Status Enable"]
pub type DATTOUTERREN_R = crate::BitReader<bool>;
#[doc = "Field `DATTOUTERREN` writer - Data Timeout Error Status Enable"]
pub type DATTOUTERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `DATCRCERREN` reader - Data CRC Error Status Enable"]
pub type DATCRCERREN_R = crate::BitReader<bool>;
#[doc = "Field `DATCRCERREN` writer - Data CRC Error Status Enable"]
pub type DATCRCERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `DATENDBITERREN` reader - Data End Bit Error Status Enable"]
pub type DATENDBITERREN_R = crate::BitReader<bool>;
#[doc = "Field `DATENDBITERREN` writer - Data End Bit Error Status Enable"]
pub type DATENDBITERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `CURRENTLIMITERREN` reader - Current Limit Error Status Enable"]
pub type CURRENTLIMITERREN_R = crate::BitReader<bool>;
#[doc = "Field `CURRENTLIMITERREN` writer - Current Limit Error Status Enable"]
pub type CURRENTLIMITERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `AUTOCMDERREN` reader - Auto CMD12 Error Status Enable"]
pub type AUTOCMDERREN_R = crate::BitReader<bool>;
#[doc = "Field `AUTOCMDERREN` writer - Auto CMD12 Error Status Enable"]
pub type AUTOCMDERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `ADMAERREN` reader - ADMA Error Status Enable"]
pub type ADMAERREN_R = crate::BitReader<bool>;
#[doc = "Field `ADMAERREN` writer - ADMA Error Status Enable"]
pub type ADMAERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `TUNINGERREN` reader - Tuning Error Status Enable"]
pub type TUNINGERREN_R = crate::BitReader<bool>;
#[doc = "Field `TUNINGERREN` writer - Tuning Error Status Enable"]
pub type TUNINGERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
#[doc = "Field `TARGETRESPEN` reader - Target Response/Host Error Status Enable"]
pub type TARGETRESPEN_R = crate::BitReader<bool>;
#[doc = "Field `TARGETRESPEN` writer - Target Response/Host Error Status Enable"]
pub type TARGETRESPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFENC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomen(&self) -> CMDCOMEN_R {
        CMDCOMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomen(&self) -> TRANCOMEN_R {
        TRANCOMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevten(&self) -> BLKGAPEVTEN_R {
        BLKGAPEVTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmainten(&self) -> DMAINTEN_R {
        DMAINTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdyen(&self) -> BUFWRRDYEN_R {
        BUFWRRDYEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdyen(&self) -> BUFRDRDYEN_R {
        BUFRDRDYEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinsen(&self) -> CARDINSEN_R {
        CARDINSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardrmen(&self) -> CARDRMEN_R {
        CARDRMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardinten(&self) -> CARDINTEN_R {
        CARDINTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tunning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevten(&self) -> RETUNINGEVTEN_R {
        RETUNINGEVTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcven(&self) -> BOOTACKRCVEN_R {
        BOOTACKRCVEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminateen(&self) -> BOOTTERMINATEEN_R {
        BOOTTERMINATEEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Time-out Error Status Enable"]
    #[inline(always)]
    pub fn cmdtouterren(&self) -> CMDTOUTERREN_R {
        CMDTOUTERREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn cmdcrcerren(&self) -> CMDCRCERREN_R {
        CMDCRCERREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cmdendbiterren(&self) -> CMDENDBITERREN_R {
        CMDENDBITERREN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn cmdindexerren(&self) -> CMDINDEXERREN_R {
        CMDINDEXERREN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dattouterren(&self) -> DATTOUTERREN_R {
        DATTOUTERREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn datcrcerren(&self) -> DATCRCERREN_R {
        DATCRCERREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn datendbiterren(&self) -> DATENDBITERREN_R {
        DATENDBITERREN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable"]
    #[inline(always)]
    pub fn currentlimiterren(&self) -> CURRENTLIMITERREN_R {
        CURRENTLIMITERREN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn autocmderren(&self) -> AUTOCMDERREN_R {
        AUTOCMDERREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADMA Error Status Enable"]
    #[inline(always)]
    pub fn admaerren(&self) -> ADMAERREN_R {
        ADMAERREN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tuningerren(&self) -> TUNINGERREN_R {
        TUNINGERREN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Target Response/Host Error Status Enable"]
    #[inline(always)]
    pub fn targetrespen(&self) -> TARGETRESPEN_R {
        TARGETRESPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcomen(&mut self) -> CMDCOMEN_W<0> {
        CMDCOMEN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trancomen(&mut self) -> TRANCOMEN_W<1> {
        TRANCOMEN_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blkgapevten(&mut self) -> BLKGAPEVTEN_W<2> {
        BLKGAPEVTEN_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmainten(&mut self) -> DMAINTEN_W<3> {
        DMAINTEN_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufwrrdyen(&mut self) -> BUFWRRDYEN_W<4> {
        BUFWRRDYEN_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufrdrdyen(&mut self) -> BUFRDRDYEN_W<5> {
        BUFRDRDYEN_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cardinsen(&mut self) -> CARDINSEN_W<6> {
        CARDINSEN_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cardrmen(&mut self) -> CARDRMEN_W<7> {
        CARDRMEN_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cardinten(&mut self) -> CARDINTEN_W<8> {
        CARDINTEN_W::new(self)
    }
    #[doc = "Bit 12 - Re-Tunning Event Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn retuningevten(&mut self) -> RETUNINGEVTEN_W<12> {
        RETUNINGEVTEN_W::new(self)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootackrcven(&mut self) -> BOOTACKRCVEN_W<13> {
        BOOTACKRCVEN_W::new(self)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootterminateen(&mut self) -> BOOTTERMINATEEN_W<14> {
        BOOTTERMINATEEN_W::new(self)
    }
    #[doc = "Bit 16 - Command Time-out Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtouterren(&mut self) -> CMDTOUTERREN_W<16> {
        CMDTOUTERREN_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcerren(&mut self) -> CMDCRCERREN_W<17> {
        CMDCRCERREN_W::new(self)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendbiterren(&mut self) -> CMDENDBITERREN_W<18> {
        CMDENDBITERREN_W::new(self)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindexerren(&mut self) -> CMDINDEXERREN_W<19> {
        CMDINDEXERREN_W::new(self)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dattouterren(&mut self) -> DATTOUTERREN_W<20> {
        DATTOUTERREN_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datcrcerren(&mut self) -> DATCRCERREN_W<21> {
        DATCRCERREN_W::new(self)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datendbiterren(&mut self) -> DATENDBITERREN_W<22> {
        DATENDBITERREN_W::new(self)
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn currentlimiterren(&mut self) -> CURRENTLIMITERREN_W<23> {
        CURRENTLIMITERREN_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autocmderren(&mut self) -> AUTOCMDERREN_W<24> {
        AUTOCMDERREN_W::new(self)
    }
    #[doc = "Bit 25 - ADMA Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn admaerren(&mut self) -> ADMAERREN_W<25> {
        ADMAERREN_W::new(self)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuningerren(&mut self) -> TUNINGERREN_W<26> {
        TUNINGERREN_W::new(self)
    }
    #[doc = "Bit 28 - Target Response/Host Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn targetrespen(&mut self) -> TARGETRESPEN_W<28> {
        TARGETRESPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal and Error Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifenc](index.html) module"]
pub struct IFENC_SPEC;
impl crate::RegisterSpec for IFENC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifenc::R](R) reader structure"]
impl crate::Readable for IFENC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifenc::W](W) writer structure"]
impl crate::Writable for IFENC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFENC to value 0"]
impl crate::Resettable for IFENC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
