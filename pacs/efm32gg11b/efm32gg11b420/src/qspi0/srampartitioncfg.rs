#[doc = "Register `SRAMPARTITIONCFG` reader"]
pub struct R(crate::R<SRAMPARTITIONCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMPARTITIONCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMPARTITIONCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMPARTITIONCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMPARTITIONCFG` writer"]
pub struct W(crate::W<SRAMPARTITIONCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMPARTITIONCFG_SPEC>;
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
impl From<crate::W<SRAMPARTITIONCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMPARTITIONCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Indirect Read Partition Size"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - Indirect Read Partition Size"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, SRAMPARTITIONCFG_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Indirect Read Partition Size"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirect Read Partition Size"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Partition Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srampartitioncfg](index.html) module"]
pub struct SRAMPARTITIONCFG_SPEC;
impl crate::RegisterSpec for SRAMPARTITIONCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srampartitioncfg::R](R) reader structure"]
impl crate::Readable for SRAMPARTITIONCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srampartitioncfg::W](W) writer structure"]
impl crate::Writable for SRAMPARTITIONCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAMPARTITIONCFG to value 0x80"]
impl crate::Resettable for SRAMPARTITIONCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
