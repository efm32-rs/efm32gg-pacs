#[doc = "Register `ROUTELOC1` reader"]
pub struct R(crate::R<ROUTELOC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC1` writer"]
pub struct W(crate::W<ROUTELOC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC1_SPEC>;
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
impl From<crate::W<ROUTELOC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDLOC` reader - I/O Location for CMD Pin"]
pub type CMDLOC_R = crate::FieldReader<u8, CMDLOC_A>;
#[doc = "I/O Location for CMD Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<CMDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDLOC_A) -> Self {
        variant as _
    }
}
impl CMDLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDLOC_A> {
        match self.bits {
            0 => Some(CMDLOC_A::LOC0),
            1 => Some(CMDLOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CMDLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CMDLOC_A::LOC1
    }
}
#[doc = "Field `CMDLOC` writer - I/O Location for CMD Pin"]
pub type CMDLOC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, CMDLOC_A, 6, O>;
impl<'a, const O: u8> CMDLOC_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CMDLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CMDLOC_A::LOC1)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline(always)]
    pub fn cmdloc(&self) -> CMDLOC_R {
        CMDLOC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline(always)]
    #[must_use]
    pub fn cmdloc(&mut self) -> CMDLOC_W<0> {
        CMDLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O LOCATION Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc1](index.html) module"]
pub struct ROUTELOC1_SPEC;
impl crate::RegisterSpec for ROUTELOC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc1::R](R) reader structure"]
impl crate::Readable for ROUTELOC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc1::W](W) writer structure"]
impl crate::Writable for ROUTELOC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTELOC1 to value 0"]
impl crate::Resettable for ROUTELOC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
