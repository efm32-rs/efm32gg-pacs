#[doc = "Register `CFGPRESETVAL0` reader"]
pub struct R(crate::R<CFGPRESETVAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGPRESETVAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGPRESETVAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGPRESETVAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGPRESETVAL0` writer"]
pub struct W(crate::W<CFGPRESETVAL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGPRESETVAL0_SPEC>;
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
impl From<crate::W<CFGPRESETVAL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGPRESETVAL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INITSDCLKFREQ` reader - Initial SD_CLK Frequency"]
pub type INITSDCLKFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INITSDCLKFREQ` writer - Initial SD_CLK Frequency"]
pub type INITSDCLKFREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGPRESETVAL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `INITCLKGENEN` reader - Initial Clock Gen Enable"]
pub type INITCLKGENEN_R = crate::BitReader<bool>;
#[doc = "Field `INITCLKGENEN` writer - Initial Clock Gen Enable"]
pub type INITCLKGENEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGPRESETVAL0_SPEC, bool, O>;
#[doc = "Field `INITDRVST` reader - Initial Drive Strength"]
pub type INITDRVST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INITDRVST` writer - Initial Drive Strength"]
pub type INITDRVST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGPRESETVAL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `DSPSDCLKFREQ` reader - Preset Value for Default Speed of SD_CLK"]
pub type DSPSDCLKFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSPSDCLKFREQ` writer - Preset Value for Default Speed of SD_CLK"]
pub type DSPSDCLKFREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGPRESETVAL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `DSPCLKGENEN` reader - Default Speed Clock Gen Enable"]
pub type DSPCLKGENEN_R = crate::BitReader<bool>;
#[doc = "Field `DSPCLKGENEN` writer - Default Speed Clock Gen Enable"]
pub type DSPCLKGENEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGPRESETVAL0_SPEC, bool, O>;
#[doc = "Field `DSPDRVST` reader - Default Speed Drive Strength"]
pub type DSPDRVST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSPDRVST` writer - Default Speed Drive Strength"]
pub type DSPDRVST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGPRESETVAL0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline(always)]
    pub fn initsdclkfreq(&self) -> INITSDCLKFREQ_R {
        INITSDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline(always)]
    pub fn initclkgenen(&self) -> INITCLKGENEN_R {
        INITCLKGENEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline(always)]
    pub fn initdrvst(&self) -> INITDRVST_R {
        INITDRVST_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline(always)]
    pub fn dspsdclkfreq(&self) -> DSPSDCLKFREQ_R {
        DSPSDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn dspclkgenen(&self) -> DSPCLKGENEN_R {
        DSPCLKGENEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline(always)]
    pub fn dspdrvst(&self) -> DSPDRVST_R {
        DSPDRVST_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn initsdclkfreq(&mut self) -> INITSDCLKFREQ_W<0> {
        INITSDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn initclkgenen(&mut self) -> INITCLKGENEN_W<10> {
        INITCLKGENEN_W::new(self)
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn initdrvst(&mut self) -> INITDRVST_W<11> {
        INITDRVST_W::new(self)
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dspsdclkfreq(&mut self) -> DSPSDCLKFREQ_W<16> {
        DSPSDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dspclkgenen(&mut self) -> DSPCLKGENEN_W<26> {
        DSPCLKGENEN_W::new(self)
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn dspdrvst(&mut self) -> DSPDRVST_W<27> {
        DSPDRVST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Configuration Preset Value 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgpresetval0](index.html) module"]
pub struct CFGPRESETVAL0_SPEC;
impl crate::RegisterSpec for CFGPRESETVAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgpresetval0::R](R) reader structure"]
impl crate::Readable for CFGPRESETVAL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgpresetval0::W](W) writer structure"]
impl crate::Writable for CFGPRESETVAL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGPRESETVAL0 to value 0"]
impl crate::Resettable for CFGPRESETVAL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
