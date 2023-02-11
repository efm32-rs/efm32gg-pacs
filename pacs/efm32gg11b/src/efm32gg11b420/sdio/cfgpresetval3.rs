#[doc = "Register `CFGPRESETVAL3` reader"]
pub struct R(crate::R<CFGPRESETVAL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGPRESETVAL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGPRESETVAL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGPRESETVAL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGPRESETVAL3` writer"]
pub struct W(crate::W<CFGPRESETVAL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGPRESETVAL3_SPEC>;
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
impl From<crate::W<CFGPRESETVAL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGPRESETVAL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDR104SDCLKFREQ` reader - SDR104 SD_CLK Frequency"]
pub type SDR104SDCLKFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDR104SDCLKFREQ` writer - SDR104 SD_CLK Frequency"]
pub type SDR104SDCLKFREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGPRESETVAL3_SPEC, u16, u16, 10, O>;
#[doc = "Field `SDR104CLKGENEN` reader - SDR104 SD_CLK Gen Enable"]
pub type SDR104CLKGENEN_R = crate::BitReader<bool>;
#[doc = "Field `SDR104CLKGENEN` writer - SDR104 SD_CLK Gen Enable"]
pub type SDR104CLKGENEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGPRESETVAL3_SPEC, bool, O>;
#[doc = "Field `SDR104DRVST` reader - SDR104 SD Drive Strength"]
pub type SDR104DRVST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDR104DRVST` writer - SDR104 SD Drive Strength"]
pub type SDR104DRVST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGPRESETVAL3_SPEC, u8, u8, 2, O>;
#[doc = "Field `DDR50SDCLKFREQ` reader - Preset Value for DDR50 Speed of SD_CLK"]
pub type DDR50SDCLKFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DDR50SDCLKFREQ` writer - Preset Value for DDR50 Speed of SD_CLK"]
pub type DDR50SDCLKFREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGPRESETVAL3_SPEC, u16, u16, 10, O>;
#[doc = "Field `DDR50CLKGENEN` reader - DDR50 Speed Clock Gen Enable"]
pub type DDR50CLKGENEN_R = crate::BitReader<bool>;
#[doc = "Field `DDR50CLKGENEN` writer - DDR50 Speed Clock Gen Enable"]
pub type DDR50CLKGENEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGPRESETVAL3_SPEC, bool, O>;
#[doc = "Field `DDR50DRVST` reader - DDR50 Speed Drive Strength"]
pub type DDR50DRVST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDR50DRVST` writer - DDR50 Speed Drive Strength"]
pub type DDR50DRVST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGPRESETVAL3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr104sdclkfreq(&self) -> SDR104SDCLKFREQ_R {
        SDR104SDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr104clkgenen(&self) -> SDR104CLKGENEN_R {
        SDR104CLKGENEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr104drvst(&self) -> SDR104DRVST_R {
        SDR104DRVST_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn ddr50sdclkfreq(&self) -> DDR50SDCLKFREQ_R {
        DDR50SDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn ddr50clkgenen(&self) -> DDR50CLKGENEN_R {
        DDR50CLKGENEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn ddr50drvst(&self) -> DDR50DRVST_R {
        DDR50DRVST_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn sdr104sdclkfreq(&mut self) -> SDR104SDCLKFREQ_W<0> {
        SDR104SDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdr104clkgenen(&mut self) -> SDR104CLKGENEN_W<10> {
        SDR104CLKGENEN_W::new(self)
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn sdr104drvst(&mut self) -> SDR104DRVST_W<11> {
        SDR104DRVST_W::new(self)
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn ddr50sdclkfreq(&mut self) -> DDR50SDCLKFREQ_W<16> {
        DDR50SDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddr50clkgenen(&mut self) -> DDR50CLKGENEN_W<26> {
        DDR50CLKGENEN_W::new(self)
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn ddr50drvst(&mut self) -> DDR50DRVST_W<27> {
        DDR50DRVST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Configuration Preset Value 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgpresetval3](index.html) module"]
pub struct CFGPRESETVAL3_SPEC;
impl crate::RegisterSpec for CFGPRESETVAL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgpresetval3::R](R) reader structure"]
impl crate::Readable for CFGPRESETVAL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgpresetval3::W](W) writer structure"]
impl crate::Writable for CFGPRESETVAL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGPRESETVAL3 to value 0"]
impl crate::Resettable for CFGPRESETVAL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
