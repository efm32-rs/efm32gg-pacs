#[doc = "Register `COMPB_COMP` reader"]
pub struct R(crate::R<COMPB_COMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPB_COMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPB_COMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPB_COMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPB_COMP` writer"]
pub struct W(crate::W<COMPB_COMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPB_COMP_SPEC>;
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
impl From<crate::W<COMPB_COMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPB_COMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - Compare Value"]
pub type COMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMP` writer - Compare Value"]
pub type COMP_W<'a> = crate::FieldWriter<'a, u32, COMPB_COMP_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 0:23 - Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compb_comp](index.html) module"]
pub struct COMPB_COMP_SPEC;
impl crate::RegisterSpec for COMPB_COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compb_comp::R](R) reader structure"]
impl crate::Readable for COMPB_COMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compb_comp::W](W) writer structure"]
impl crate::Writable for COMPB_COMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMPB_COMP to value 0"]
impl crate::Resettable for COMPB_COMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
