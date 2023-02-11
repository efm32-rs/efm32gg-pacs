#[doc = "Register `SPECADDR3TOP` reader"]
pub struct R(crate::R<SPECADDR3TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPECADDR3TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPECADDR3TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPECADDR3TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPECADDR3TOP` writer"]
pub struct W(crate::W<SPECADDR3TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPECADDR3TOP_SPEC>;
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
impl From<crate::W<SPECADDR3TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPECADDR3TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Specific address 3 MSB"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - Specific address 3 MSB"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPECADDR3TOP_SPEC, u16, u16, 16, O>;
#[doc = "Field `FILTERTYPE` reader - MAC SA or DA selection"]
pub type FILTERTYPE_R = crate::BitReader<bool>;
#[doc = "Field `FILTERTYPE` writer - MAC SA or DA selection"]
pub type FILTERTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPECADDR3TOP_SPEC, bool, O>;
#[doc = "Field `FILTERBYTEMASK` reader - Filter byte Mask"]
pub type FILTERBYTEMASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTERBYTEMASK` writer - Filter byte Mask"]
pub type FILTERBYTEMASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPECADDR3TOP_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:15 - Specific address 3 MSB"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&self) -> FILTERTYPE_R {
        FILTERTYPE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Filter byte Mask"]
    #[inline(always)]
    pub fn filterbytemask(&self) -> FILTERBYTEMASK_R {
        FILTERBYTEMASK_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific address 3 MSB"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    #[must_use]
    pub fn filtertype(&mut self) -> FILTERTYPE_W<16> {
        FILTERTYPE_W::new(self)
    }
    #[doc = "Bits 24:29 - Filter byte Mask"]
    #[inline(always)]
    #[must_use]
    pub fn filterbytemask(&mut self) -> FILTERBYTEMASK_W<24> {
        FILTERBYTEMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specific Address 3 Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [specaddr3top](index.html) module"]
pub struct SPECADDR3TOP_SPEC;
impl crate::RegisterSpec for SPECADDR3TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [specaddr3top::R](R) reader structure"]
impl crate::Readable for SPECADDR3TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [specaddr3top::W](W) writer structure"]
impl crate::Writable for SPECADDR3TOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPECADDR3TOP to value 0"]
impl crate::Resettable for SPECADDR3TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
