#[doc = "Register `HFBUSPRESC` reader"]
pub struct R(crate::R<HFBUSPRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFBUSPRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFBUSPRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFBUSPRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFBUSPRESC` writer"]
pub struct W(crate::W<HFBUSPRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFBUSPRESC_SPEC>;
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
impl From<crate::W<HFBUSPRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFBUSPRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - HFBUSCLK Prescaler"]
pub type PRESC_R = crate::FieldReader<u16, PRESC_A>;
#[doc = "HFBUSCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u16 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Field `PRESC` writer - HFBUSCLK Prescaler"]
pub type PRESC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HFBUSPRESC_SPEC, u16, PRESC_A, 9, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
}
impl R {
    #[doc = "Bits 8:16 - HFBUSCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:16 - HFBUSCLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<8> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Bus Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfbuspresc](index.html) module"]
pub struct HFBUSPRESC_SPEC;
impl crate::RegisterSpec for HFBUSPRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfbuspresc::R](R) reader structure"]
impl crate::Readable for HFBUSPRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfbuspresc::W](W) writer structure"]
impl crate::Writable for HFBUSPRESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFBUSPRESC to value 0"]
impl crate::Resettable for HFBUSPRESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
