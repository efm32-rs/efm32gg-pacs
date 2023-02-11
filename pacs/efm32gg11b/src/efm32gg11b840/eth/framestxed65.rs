#[doc = "Register `FRAMESTXED65` reader"]
pub struct R(crate::R<FRAMESTXED65_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMESTXED65_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMESTXED65_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMESTXED65_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMESTXED65` writer"]
pub struct W(crate::W<FRAMESTXED65_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMESTXED65_SPEC>;
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
impl From<crate::W<FRAMESTXED65_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMESTXED65_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - 65 to127 byte frames transmitted without error"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - 65 to127 byte frames transmitted without error"]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRAMESTXED65_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 65 to127 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 65 to127 byte frames transmitted without error"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "65 to 127 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framestxed65](index.html) module"]
pub struct FRAMESTXED65_SPEC;
impl crate::RegisterSpec for FRAMESTXED65_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framestxed65::R](R) reader structure"]
impl crate::Readable for FRAMESTXED65_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framestxed65::W](W) writer structure"]
impl crate::Writable for FRAMESTXED65_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMESTXED65 to value 0"]
impl crate::Resettable for FRAMESTXED65_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
