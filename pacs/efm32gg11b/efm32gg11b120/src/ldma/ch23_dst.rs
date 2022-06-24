#[doc = "Register `CH23_DST` reader"]
pub struct R(crate::R<CH23_DST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH23_DST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH23_DST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH23_DST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH23_DST` writer"]
pub struct W(crate::W<CH23_DST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH23_DST_SPEC>;
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
impl From<crate::W<CH23_DST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH23_DST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTADDR` reader - Destination Data Address"]
pub type DSTADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSTADDR` writer - Destination Data Address"]
pub type DSTADDR_W<'a> = crate::FieldWriter<'a, u32, CH23_DST_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn dstaddr(&self) -> DSTADDR_R {
        DSTADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn dstaddr(&mut self) -> DSTADDR_W {
        DSTADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch23_dst](index.html) module"]
pub struct CH23_DST_SPEC;
impl crate::RegisterSpec for CH23_DST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch23_dst::R](R) reader structure"]
impl crate::Readable for CH23_DST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch23_dst::W](W) writer structure"]
impl crate::Writable for CH23_DST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH23_DST to value 0"]
impl crate::Resettable for CH23_DST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
