#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCINTRSUP` reader - Asynchronous Interrupt Support"]
pub type ASYNCINTRSUP_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCINTRSUP` writer - Asynchronous Interrupt Support"]
pub type ASYNCINTRSUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `SLOTTYPE` reader - Slot Type"]
pub type SLOTTYPE_R = crate::FieldReader<u8, SLOTTYPE_A>;
#[doc = "Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLOTTYPE_A {
    #[doc = "0: Removable SD Card Slot"]
    RMSDSLOT = 0,
    #[doc = "1: Embedded SD Card Slot"]
    EMSDSLOT = 1,
    #[doc = "2: Shared SD Card Slot"]
    SHBUSSLOT = 2,
}
impl From<SLOTTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOTTYPE_A) -> Self {
        variant as _
    }
}
impl SLOTTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLOTTYPE_A> {
        match self.bits {
            0 => Some(SLOTTYPE_A::RMSDSLOT),
            1 => Some(SLOTTYPE_A::EMSDSLOT),
            2 => Some(SLOTTYPE_A::SHBUSSLOT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RMSDSLOT`"]
    #[inline(always)]
    pub fn is_rmsdslot(&self) -> bool {
        *self == SLOTTYPE_A::RMSDSLOT
    }
    #[doc = "Checks if the value of the field is `EMSDSLOT`"]
    #[inline(always)]
    pub fn is_emsdslot(&self) -> bool {
        *self == SLOTTYPE_A::EMSDSLOT
    }
    #[doc = "Checks if the value of the field is `SHBUSSLOT`"]
    #[inline(always)]
    pub fn is_shbusslot(&self) -> bool {
        *self == SLOTTYPE_A::SHBUSSLOT
    }
}
#[doc = "Field `SLOTTYPE` writer - Slot Type"]
pub type SLOTTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, SLOTTYPE_A, 2, O>;
impl<'a, const O: u8> SLOTTYPE_W<'a, O> {
    #[doc = "Removable SD Card Slot"]
    #[inline(always)]
    pub fn rmsdslot(self) -> &'a mut W {
        self.variant(SLOTTYPE_A::RMSDSLOT)
    }
    #[doc = "Embedded SD Card Slot"]
    #[inline(always)]
    pub fn emsdslot(self) -> &'a mut W {
        self.variant(SLOTTYPE_A::EMSDSLOT)
    }
    #[doc = "Shared SD Card Slot"]
    #[inline(always)]
    pub fn shbusslot(self) -> &'a mut W {
        self.variant(SLOTTYPE_A::SHBUSSLOT)
    }
}
#[doc = "Field `CSDR50SUP` reader - Core Support SDR50"]
pub type CSDR50SUP_R = crate::BitReader<bool>;
#[doc = "Field `CSDR50SUP` writer - Core Support SDR50"]
pub type CSDR50SUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `CSDR104SUP` reader - Support SDR104"]
pub type CSDR104SUP_R = crate::BitReader<bool>;
#[doc = "Field `CSDR104SUP` writer - Support SDR104"]
pub type CSDR104SUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `CDDR50SUP` reader - Support DDR50"]
pub type CDDR50SUP_R = crate::BitReader<bool>;
#[doc = "Field `CDDR50SUP` writer - Support DDR50"]
pub type CDDR50SUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `CDRVASUP` reader - Support Type a Driver"]
pub type CDRVASUP_R = crate::BitReader<bool>;
#[doc = "Field `CDRVASUP` writer - Support Type a Driver"]
pub type CDRVASUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `CDRVCSUP` reader - Support Type C Driver"]
pub type CDRVCSUP_R = crate::BitReader<bool>;
#[doc = "Field `CDRVCSUP` writer - Support Type C Driver"]
pub type CDRVCSUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `CDRVDSUP` reader - Support Type D Driver"]
pub type CDRVDSUP_R = crate::BitReader<bool>;
#[doc = "Field `CDRVDSUP` writer - Support Type D Driver"]
pub type CDRVDSUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `RETUNTMRCTL` reader - Retuning Timer Control"]
pub type RETUNTMRCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RETUNTMRCTL` writer - Retuning Timer Control"]
pub type RETUNTMRCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TUNSDR50` reader - Tuning for SDR50"]
pub type TUNSDR50_R = crate::BitReader<bool>;
#[doc = "Field `TUNSDR50` writer - Tuning for SDR50"]
pub type TUNSDR50_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `RETUNMODES` reader - Retuning Modes"]
pub type RETUNMODES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RETUNMODES` writer - Retuning Modes"]
pub type RETUNMODES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPISUP` reader - SPI Support"]
pub type SPISUP_R = crate::BitReader<bool>;
#[doc = "Field `SPISUP` writer - SPI Support"]
pub type SPISUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `ASYNCWKUPEN` reader - Asynchronous Wakeup Enable"]
pub type ASYNCWKUPEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCWKUPEN` writer - Asynchronous Wakeup Enable"]
pub type ASYNCWKUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asyncintrsup(&self) -> ASYNCINTRSUP_R {
        ASYNCINTRSUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Slot Type"]
    #[inline(always)]
    pub fn slottype(&self) -> SLOTTYPE_R {
        SLOTTYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Core Support SDR50"]
    #[inline(always)]
    pub fn csdr50sup(&self) -> CSDR50SUP_R {
        CSDR50SUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Support SDR104"]
    #[inline(always)]
    pub fn csdr104sup(&self) -> CSDR104SUP_R {
        CSDR104SUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Support DDR50"]
    #[inline(always)]
    pub fn cddr50sup(&self) -> CDDR50SUP_R {
        CDDR50SUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Support Type a Driver"]
    #[inline(always)]
    pub fn cdrvasup(&self) -> CDRVASUP_R {
        CDRVASUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Support Type C Driver"]
    #[inline(always)]
    pub fn cdrvcsup(&self) -> CDRVCSUP_R {
        CDRVCSUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Support Type D Driver"]
    #[inline(always)]
    pub fn cdrvdsup(&self) -> CDRVDSUP_R {
        CDRVDSUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - Retuning Timer Control"]
    #[inline(always)]
    pub fn retuntmrctl(&self) -> RETUNTMRCTL_R {
        RETUNTMRCTL_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Tuning for SDR50"]
    #[inline(always)]
    pub fn tunsdr50(&self) -> TUNSDR50_R {
        TUNSDR50_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Retuning Modes"]
    #[inline(always)]
    pub fn retunmodes(&self) -> RETUNMODES_R {
        RETUNMODES_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SPI Support"]
    #[inline(always)]
    pub fn spisup(&self) -> SPISUP_R {
        SPISUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Asynchronous Wakeup Enable"]
    #[inline(always)]
    pub fn asyncwkupen(&self) -> ASYNCWKUPEN_R {
        ASYNCWKUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asynchronous Interrupt Support"]
    #[inline(always)]
    #[must_use]
    pub fn asyncintrsup(&mut self) -> ASYNCINTRSUP_W<0> {
        ASYNCINTRSUP_W::new(self)
    }
    #[doc = "Bits 1:2 - Slot Type"]
    #[inline(always)]
    #[must_use]
    pub fn slottype(&mut self) -> SLOTTYPE_W<1> {
        SLOTTYPE_W::new(self)
    }
    #[doc = "Bit 3 - Core Support SDR50"]
    #[inline(always)]
    #[must_use]
    pub fn csdr50sup(&mut self) -> CSDR50SUP_W<3> {
        CSDR50SUP_W::new(self)
    }
    #[doc = "Bit 4 - Support SDR104"]
    #[inline(always)]
    #[must_use]
    pub fn csdr104sup(&mut self) -> CSDR104SUP_W<4> {
        CSDR104SUP_W::new(self)
    }
    #[doc = "Bit 5 - Support DDR50"]
    #[inline(always)]
    #[must_use]
    pub fn cddr50sup(&mut self) -> CDDR50SUP_W<5> {
        CDDR50SUP_W::new(self)
    }
    #[doc = "Bit 6 - Support Type a Driver"]
    #[inline(always)]
    #[must_use]
    pub fn cdrvasup(&mut self) -> CDRVASUP_W<6> {
        CDRVASUP_W::new(self)
    }
    #[doc = "Bit 7 - Support Type C Driver"]
    #[inline(always)]
    #[must_use]
    pub fn cdrvcsup(&mut self) -> CDRVCSUP_W<7> {
        CDRVCSUP_W::new(self)
    }
    #[doc = "Bit 8 - Support Type D Driver"]
    #[inline(always)]
    #[must_use]
    pub fn cdrvdsup(&mut self) -> CDRVDSUP_W<8> {
        CDRVDSUP_W::new(self)
    }
    #[doc = "Bits 9:12 - Retuning Timer Control"]
    #[inline(always)]
    #[must_use]
    pub fn retuntmrctl(&mut self) -> RETUNTMRCTL_W<9> {
        RETUNTMRCTL_W::new(self)
    }
    #[doc = "Bit 13 - Tuning for SDR50"]
    #[inline(always)]
    #[must_use]
    pub fn tunsdr50(&mut self) -> TUNSDR50_W<13> {
        TUNSDR50_W::new(self)
    }
    #[doc = "Bits 14:15 - Retuning Modes"]
    #[inline(always)]
    #[must_use]
    pub fn retunmodes(&mut self) -> RETUNMODES_W<14> {
        RETUNMODES_W::new(self)
    }
    #[doc = "Bit 16 - SPI Support"]
    #[inline(always)]
    #[must_use]
    pub fn spisup(&mut self) -> SPISUP_W<16> {
        SPISUP_W::new(self)
    }
    #[doc = "Bit 18 - Asynchronous Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asyncwkupen(&mut self) -> ASYNCWKUPEN_W<18> {
        ASYNCWKUPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
