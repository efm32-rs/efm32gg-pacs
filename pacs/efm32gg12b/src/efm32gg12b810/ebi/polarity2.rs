#[doc = "Register `POLARITY2` reader"]
pub struct R(crate::R<POLARITY2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POLARITY2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POLARITY2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POLARITY2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POLARITY2` writer"]
pub struct W(crate::W<POLARITY2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POLARITY2_SPEC>;
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
impl From<crate::W<POLARITY2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POLARITY2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSPOL` reader - Chip Select Polarity"]
pub type CSPOL_R = crate::BitReader<bool>;
#[doc = "Field `CSPOL` writer - Chip Select Polarity"]
pub type CSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITY2_SPEC, bool, O>;
#[doc = "Field `REPOL` reader - Read Enable Polarity"]
pub type REPOL_R = crate::BitReader<bool>;
#[doc = "Field `REPOL` writer - Read Enable Polarity"]
pub type REPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITY2_SPEC, bool, O>;
#[doc = "Field `WEPOL` reader - Write Enable Polarity"]
pub type WEPOL_R = crate::BitReader<bool>;
#[doc = "Field `WEPOL` writer - Write Enable Polarity"]
pub type WEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITY2_SPEC, bool, O>;
#[doc = "Field `ALEPOL` reader - Address Latch Polarity"]
pub type ALEPOL_R = crate::BitReader<bool>;
#[doc = "Field `ALEPOL` writer - Address Latch Polarity"]
pub type ALEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITY2_SPEC, bool, O>;
#[doc = "Field `ARDYPOL` reader - ARDY Polarity"]
pub type ARDYPOL_R = crate::BitReader<bool>;
#[doc = "Field `ARDYPOL` writer - ARDY Polarity"]
pub type ARDYPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITY2_SPEC, bool, O>;
#[doc = "Field `BLPOL` reader - BL Polarity"]
pub type BLPOL_R = crate::BitReader<bool>;
#[doc = "Field `BLPOL` writer - BL Polarity"]
pub type BLPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, POLARITY2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Enable Polarity"]
    #[inline(always)]
    pub fn repol(&self) -> REPOL_R {
        REPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Enable Polarity"]
    #[inline(always)]
    pub fn wepol(&self) -> WEPOL_R {
        WEPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn alepol(&self) -> ALEPOL_R {
        ALEPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARDY Polarity"]
    #[inline(always)]
    pub fn ardypol(&self) -> ARDYPOL_R {
        ARDYPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BL Polarity"]
    #[inline(always)]
    pub fn blpol(&self) -> BLPOL_R {
        BLPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Chip Select Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol(&mut self) -> CSPOL_W<0> {
        CSPOL_W::new(self)
    }
    #[doc = "Bit 1 - Read Enable Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn repol(&mut self) -> REPOL_W<1> {
        REPOL_W::new(self)
    }
    #[doc = "Bit 2 - Write Enable Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wepol(&mut self) -> WEPOL_W<2> {
        WEPOL_W::new(self)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn alepol(&mut self) -> ALEPOL_W<3> {
        ALEPOL_W::new(self)
    }
    #[doc = "Bit 4 - ARDY Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ardypol(&mut self) -> ARDYPOL_W<4> {
        ARDYPOL_W::new(self)
    }
    #[doc = "Bit 5 - BL Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn blpol(&mut self) -> BLPOL_W<5> {
        BLPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Polarity Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [polarity2](index.html) module"]
pub struct POLARITY2_SPEC;
impl crate::RegisterSpec for POLARITY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [polarity2::R](R) reader structure"]
impl crate::Readable for POLARITY2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [polarity2::W](W) writer structure"]
impl crate::Writable for POLARITY2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POLARITY2 to value 0"]
impl crate::Resettable for POLARITY2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
