#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSENAP` reader - VBUSEN Active Polarity"]
pub type VBUSENAP_R = crate::BitReader<bool>;
#[doc = "Field `VBUSENAP` writer - VBUSEN Active Polarity"]
pub type VBUSENAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SELFPOWERED` reader - PHY Power"]
pub type SELFPOWERED_R = crate::BitReader<bool>;
#[doc = "Field `SELFPOWERED` writer - PHY Power"]
pub type SELFPOWERED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LEMOSCCTRL` reader - Low Energy Mode Oscillator Control"]
pub type LEMOSCCTRL_R = crate::FieldReader<u8, LEMOSCCTRL_A>;
#[doc = "Low Energy Mode Oscillator Control\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEMOSCCTRL_A {
    #[doc = "0: Low Energy Mode has no effect on neither USBC or USHFRCO."]
    NONE = 0,
    #[doc = "1: The USBC clock is gated when Low Energy Mode is active."]
    GATE = 1,
}
impl From<LEMOSCCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LEMOSCCTRL_A) -> Self {
        variant as _
    }
}
impl LEMOSCCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LEMOSCCTRL_A> {
        match self.bits {
            0 => Some(LEMOSCCTRL_A::NONE),
            1 => Some(LEMOSCCTRL_A::GATE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LEMOSCCTRL_A::NONE
    }
    #[doc = "Checks if the value of the field is `GATE`"]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        *self == LEMOSCCTRL_A::GATE
    }
}
#[doc = "Field `LEMOSCCTRL` writer - Low Energy Mode Oscillator Control"]
pub type LEMOSCCTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, LEMOSCCTRL_A, 2, O>;
impl<'a, const O: u8> LEMOSCCTRL_W<'a, O> {
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LEMOSCCTRL_A::NONE)
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline(always)]
    pub fn gate(self) -> &'a mut W {
        self.variant(LEMOSCCTRL_A::GATE)
    }
}
#[doc = "Field `LEMPHYCTRL` reader - Low Energy Mode USB PHY Control"]
pub type LEMPHYCTRL_R = crate::BitReader<bool>;
#[doc = "Field `LEMPHYCTRL` writer - Low Energy Mode USB PHY Control"]
pub type LEMPHYCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LEMIDLEEN` reader - Low Energy Mode on Bus Idle Enable"]
pub type LEMIDLEEN_R = crate::BitReader<bool>;
#[doc = "Field `LEMIDLEEN` writer - Low Energy Mode on Bus Idle Enable"]
pub type LEMIDLEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `IDCDEN` reader - ID Pull-up Enable"]
pub type IDCDEN_R = crate::BitReader<bool>;
#[doc = "Field `IDCDEN` writer - ID Pull-up Enable"]
pub type IDCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OTGCLKCDIS` reader - OTG CLKC Disable"]
pub type OTGCLKCDIS_R = crate::BitReader<bool>;
#[doc = "Field `OTGCLKCDIS` writer - OTG CLKC Disable"]
pub type OTGCLKCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OTGIDINDIS` reader - OTG ID Input Disable"]
pub type OTGIDINDIS_R = crate::BitReader<bool>;
#[doc = "Field `OTGIDINDIS` writer - OTG ID Input Disable"]
pub type OTGIDINDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OTGPHYCTRLDIS` reader - OTG Control Signals to PHY Disable"]
pub type OTGPHYCTRLDIS_R = crate::BitReader<bool>;
#[doc = "Field `OTGPHYCTRLDIS` writer - OTG Control Signals to PHY Disable"]
pub type OTGPHYCTRLDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DCDEN` reader - Data Contact Detection Enable"]
pub type DCDEN_R = crate::FieldReader<u8, DCDEN_A>;
#[doc = "Data Contact Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCDEN_A {
    #[doc = "0: DCD is disabled."]
    DISABLED = 0,
    #[doc = "2: Only DCD timeout will be initiated."]
    TIMEOUT = 2,
    #[doc = "3: Full DCD operation (physical contact and timeout) will be initiated."]
    ENABLED = 3,
}
impl From<DCDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DCDEN_A) -> Self {
        variant as _
    }
}
impl DCDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCDEN_A> {
        match self.bits {
            0 => Some(DCDEN_A::DISABLED),
            2 => Some(DCDEN_A::TIMEOUT),
            3 => Some(DCDEN_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == DCDEN_A::TIMEOUT
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCDEN_A::ENABLED
    }
}
#[doc = "Field `DCDEN` writer - Data Contact Detection Enable"]
pub type DCDEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, DCDEN_A, 2, O>;
impl<'a, const O: u8> DCDEN_W<'a, O> {
    #[doc = "DCD is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDEN_A::DISABLED)
    }
    #[doc = "Only DCD timeout will be initiated."]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(DCDEN_A::TIMEOUT)
    }
    #[doc = "Full DCD operation (physical contact and timeout) will be initiated."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDEN_A::ENABLED)
    }
}
#[doc = "Field `PDEN` reader - Primary Detection Enable"]
pub type PDEN_R = crate::BitReader<bool>;
#[doc = "Field `PDEN` writer - Primary Detection Enable"]
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SDEN` reader - Secondary Detection Enable"]
pub type SDEN_R = crate::BitReader<bool>;
#[doc = "Field `SDEN` writer - Secondary Detection Enable"]
pub type SDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline(always)]
    pub fn vbusenap(&self) -> VBUSENAP_R {
        VBUSENAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - PHY Power"]
    #[inline(always)]
    pub fn selfpowered(&self) -> SELFPOWERED_R {
        SELFPOWERED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&self) -> LEMOSCCTRL_R {
        LEMOSCCTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&self) -> LEMPHYCTRL_R {
        LEMPHYCTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&self) -> LEMIDLEEN_R {
        LEMIDLEEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - ID Pull-up Enable"]
    #[inline(always)]
    pub fn idcden(&self) -> IDCDEN_R {
        IDCDEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 25 - OTG CLKC Disable"]
    #[inline(always)]
    pub fn otgclkcdis(&self) -> OTGCLKCDIS_R {
        OTGCLKCDIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - OTG ID Input Disable"]
    #[inline(always)]
    pub fn otgidindis(&self) -> OTGIDINDIS_R {
        OTGIDINDIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - OTG Control Signals to PHY Disable"]
    #[inline(always)]
    pub fn otgphyctrldis(&self) -> OTGPHYCTRLDIS_R {
        OTGPHYCTRLDIS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Data Contact Detection Enable"]
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Primary Detection Enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Secondary Detection Enable"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vbusenap(&mut self) -> VBUSENAP_W<0> {
        VBUSENAP_W::new(self)
    }
    #[doc = "Bit 3 - PHY Power"]
    #[inline(always)]
    #[must_use]
    pub fn selfpowered(&mut self) -> SELFPOWERED_W<3> {
        SELFPOWERED_W::new(self)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    #[must_use]
    pub fn lemoscctrl(&mut self) -> LEMOSCCTRL_W<4> {
        LEMOSCCTRL_W::new(self)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    #[must_use]
    pub fn lemphyctrl(&mut self) -> LEMPHYCTRL_W<7> {
        LEMPHYCTRL_W::new(self)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lemidleen(&mut self) -> LEMIDLEEN_W<9> {
        LEMIDLEEN_W::new(self)
    }
    #[doc = "Bit 12 - ID Pull-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn idcden(&mut self) -> IDCDEN_W<12> {
        IDCDEN_W::new(self)
    }
    #[doc = "Bit 25 - OTG CLKC Disable"]
    #[inline(always)]
    #[must_use]
    pub fn otgclkcdis(&mut self) -> OTGCLKCDIS_W<25> {
        OTGCLKCDIS_W::new(self)
    }
    #[doc = "Bit 26 - OTG ID Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn otgidindis(&mut self) -> OTGIDINDIS_W<26> {
        OTGIDINDIS_W::new(self)
    }
    #[doc = "Bit 27 - OTG Control Signals to PHY Disable"]
    #[inline(always)]
    #[must_use]
    pub fn otgphyctrldis(&mut self) -> OTGPHYCTRLDIS_W<27> {
        OTGPHYCTRLDIS_W::new(self)
    }
    #[doc = "Bits 28:29 - Data Contact Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcden(&mut self) -> DCDEN_W<28> {
        DCDEN_W::new(self)
    }
    #[doc = "Bit 30 - Primary Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<30> {
        PDEN_W::new(self)
    }
    #[doc = "Bit 31 - Secondary Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SDEN_W<31> {
        SDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x20"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
