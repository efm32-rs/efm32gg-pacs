#[doc = "Register `CLOCKCTRL` reader"]
pub struct R(crate::R<CLOCKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCKCTRL` writer"]
pub struct W(crate::W<CLOCKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCKCTRL_SPEC>;
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
impl From<crate::W<CLOCKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTCLKEN` reader - Internal Clock Enable"]
pub type INTCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `INTCLKEN` writer - Internal Clock Enable"]
pub type INTCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCKCTRL_SPEC, bool, O>;
#[doc = "Field `INTCLKSTABLE` reader - Internal Clock Stable"]
pub type INTCLKSTABLE_R = crate::BitReader<bool>;
#[doc = "Field `SDCLKEN` reader - SDIO_CLK Pin Clock Enable"]
pub type SDCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `SDCLKEN` writer - SDIO_CLK Pin Clock Enable"]
pub type SDCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCKCTRL_SPEC, bool, O>;
#[doc = "Field `CLKGENSEL` reader - Clock Generator Select"]
pub type CLKGENSEL_R = crate::BitReader<bool>;
#[doc = "Field `CLKGENSEL` writer - Clock Generator Select"]
pub type CLKGENSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCKCTRL_SPEC, bool, O>;
#[doc = "Field `UPPSDCLKFRE` reader - Upper Bits of SD_CLK Frequency Select"]
pub type UPPSDCLKFRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPPSDCLKFRE` writer - Upper Bits of SD_CLK Frequency Select"]
pub type UPPSDCLKFRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCKCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SDCLKFREQSEL` reader - SD_CLK Frequency Select"]
pub type SDCLKFREQSEL_R = crate::FieldReader<u8, SDCLKFREQSEL_A>;
#[doc = "SD_CLK Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDCLKFREQSEL_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<SDCLKFREQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCLKFREQSEL_A) -> Self {
        variant as _
    }
}
impl SDCLKFREQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDCLKFREQSEL_A> {
        match self.bits {
            0 => Some(SDCLKFREQSEL_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == SDCLKFREQSEL_A::NODIVISION
    }
}
#[doc = "Field `SDCLKFREQSEL` writer - SD_CLK Frequency Select"]
pub type SDCLKFREQSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCKCTRL_SPEC, u8, SDCLKFREQSEL_A, 8, O>;
impl<'a, const O: u8> SDCLKFREQSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(SDCLKFREQSEL_A::NODIVISION)
    }
}
#[doc = "Field `DATTOUTCNTVAL` reader - Data Timeout Counter Value"]
pub type DATTOUTCNTVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATTOUTCNTVAL` writer - Data Timeout Counter Value"]
pub type DATTOUTCNTVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCKCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SFTRSTA` reader - Software Reset for All"]
pub type SFTRSTA_R = crate::BitReader<bool>;
#[doc = "Field `SFTRSTA` writer - Software Reset for All"]
pub type SFTRSTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCKCTRL_SPEC, bool, O>;
#[doc = "Field `SFTRSTCMD` reader - Software Reset for CMD Line"]
pub type SFTRSTCMD_R = crate::BitReader<bool>;
#[doc = "Field `SFTRSTCMD` writer - Software Reset for CMD Line"]
pub type SFTRSTCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCKCTRL_SPEC, bool, O>;
#[doc = "Field `SFTRSTDAT` reader - Software Reset for DAT Line"]
pub type SFTRSTDAT_R = crate::BitReader<bool>;
#[doc = "Field `SFTRSTDAT` writer - Software Reset for DAT Line"]
pub type SFTRSTDAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCKCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&self) -> INTCLKEN_R {
        INTCLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclkstable(&self) -> INTCLKSTABLE_R {
        INTCLKSTABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SDCLKEN_R {
        SDCLKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgensel(&self) -> CLKGENSEL_R {
        CLKGENSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn uppsdclkfre(&self) -> UPPSDCLKFRE_R {
        UPPSDCLKFRE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfreqsel(&self) -> SDCLKFREQSEL_R {
        SDCLKFREQSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dattoutcntval(&self) -> DATTOUTCNTVAL_R {
        DATTOUTCNTVAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline(always)]
    pub fn sftrsta(&self) -> SFTRSTA_R {
        SFTRSTA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sftrstcmd(&self) -> SFTRSTCMD_R {
        SFTRSTCMD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sftrstdat(&self) -> SFTRSTDAT_R {
        SFTRSTDAT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn intclken(&mut self) -> INTCLKEN_W<0> {
        INTCLKEN_W::new(self)
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdclken(&mut self) -> SDCLKEN_W<2> {
        SDCLKEN_W::new(self)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    #[must_use]
    pub fn clkgensel(&mut self) -> CLKGENSEL_W<5> {
        CLKGENSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn uppsdclkfre(&mut self) -> UPPSDCLKFRE_W<6> {
        UPPSDCLKFRE_W::new(self)
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfreqsel(&mut self) -> SDCLKFREQSEL_W<8> {
        SDCLKFREQSEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn dattoutcntval(&mut self) -> DATTOUTCNTVAL_W<16> {
        DATTOUTCNTVAL_W::new(self)
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline(always)]
    #[must_use]
    pub fn sftrsta(&mut self) -> SFTRSTA_W<24> {
        SFTRSTA_W::new(self)
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstcmd(&mut self) -> SFTRSTCMD_W<25> {
        SFTRSTCMD_W::new(self)
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstdat(&mut self) -> SFTRSTDAT_W<26> {
        SFTRSTDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control, Timeout Control and Software Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockctrl](index.html) module"]
pub struct CLOCKCTRL_SPEC;
impl crate::RegisterSpec for CLOCKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clockctrl::R](R) reader structure"]
impl crate::Readable for CLOCKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clockctrl::W](W) writer structure"]
impl crate::Writable for CLOCKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCKCTRL to value 0"]
impl crate::Resettable for CLOCKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
