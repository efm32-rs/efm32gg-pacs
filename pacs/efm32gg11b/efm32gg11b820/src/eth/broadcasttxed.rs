#[doc = "Register `BROADCASTTXED` reader"]
pub struct R(crate::R<BROADCASTTXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROADCASTTXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROADCASTTXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROADCASTTXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BROADCASTTXED` writer"]
pub struct W(crate::W<BROADCASTTXED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROADCASTTXED_SPEC>;
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
impl From<crate::W<BROADCASTTXED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROADCASTTXED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Broadcast frames transmitted without error"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Broadcast frames transmitted without error"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, BROADCASTTXED_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Broadcast frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Broadcast frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Broadcast Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [broadcasttxed](index.html) module"]
pub struct BROADCASTTXED_SPEC;
impl crate::RegisterSpec for BROADCASTTXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [broadcasttxed::R](R) reader structure"]
impl crate::Readable for BROADCASTTXED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [broadcasttxed::W](W) writer structure"]
impl crate::Writable for BROADCASTTXED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BROADCASTTXED to value 0"]
impl crate::Resettable for BROADCASTTXED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
