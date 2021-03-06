#[doc = "Register `TFTPIXEL1` reader"]
pub struct R(crate::R<TFTPIXEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTPIXEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTPIXEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTPIXEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTPIXEL1` writer"]
pub struct W(crate::W<TFTPIXEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTPIXEL1_SPEC>;
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
impl From<crate::W<TFTPIXEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTPIXEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - RGB Data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - RGB Data"]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, TFTPIXEL1_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 0:23 - RGB Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - RGB Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Pixel 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftpixel1](index.html) module"]
pub struct TFTPIXEL1_SPEC;
impl crate::RegisterSpec for TFTPIXEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftpixel1::R](R) reader structure"]
impl crate::Readable for TFTPIXEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftpixel1::W](W) writer structure"]
impl crate::Writable for TFTPIXEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFTPIXEL1 to value 0"]
impl crate::Resettable for TFTPIXEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
