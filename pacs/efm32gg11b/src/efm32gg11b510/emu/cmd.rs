#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM4UNLATCH` writer - EM4 Unlatch"]
pub type EM4UNLATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `EM01VSCALE0` writer - EM01 Voltage Scale Command to Scale to Voltage Scale Level 0"]
pub type EM01VSCALE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `EM01VSCALE2` writer - EM01 Voltage Scale Command to Scale to Voltage Scale Level 2"]
pub type EM01VSCALE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - EM4 Unlatch"]
    #[inline(always)]
    #[must_use]
    pub fn em4unlatch(&mut self) -> EM4UNLATCH_W<0> {
        EM4UNLATCH_W::new(self)
    }
    #[doc = "Bit 4 - EM01 Voltage Scale Command to Scale to Voltage Scale Level 0"]
    #[inline(always)]
    #[must_use]
    pub fn em01vscale0(&mut self) -> EM01VSCALE0_W<4> {
        EM01VSCALE0_W::new(self)
    }
    #[doc = "Bit 6 - EM01 Voltage Scale Command to Scale to Voltage Scale Level 2"]
    #[inline(always)]
    #[must_use]
    pub fn em01vscale2(&mut self) -> EM01VSCALE2_W<6> {
        EM01VSCALE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
