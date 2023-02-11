#[doc = "Register `FLASHCMDADDR` reader"]
pub struct R(crate::R<FLASHCMDADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHCMDADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHCMDADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHCMDADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHCMDADDR` writer"]
pub struct W(crate::W<FLASHCMDADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHCMDADDR_SPEC>;
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
impl From<crate::W<FLASHCMDADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHCMDADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Command Address"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Command Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASHCMDADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Command Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Command Address Register (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcmdaddr](index.html) module"]
pub struct FLASHCMDADDR_SPEC;
impl crate::RegisterSpec for FLASHCMDADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashcmdaddr::R](R) reader structure"]
impl crate::Readable for FLASHCMDADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashcmdaddr::W](W) writer structure"]
impl crate::Writable for FLASHCMDADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASHCMDADDR to value 0"]
impl crate::Resettable for FLASHCMDADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
