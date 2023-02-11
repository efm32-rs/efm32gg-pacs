#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
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
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSDETH` writer - Clear VBUSDETH Interrupt Flag"]
pub type VBUSDETH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `VBUSDETL` writer - Clear VBUSDETL Interrupt Flag"]
pub type VBUSDETL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `ERR` writer - Clear ERR Interrupt Flag"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `DCD` writer - Clear DCD Interrupt Flag"]
pub type DCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `PD` writer - Clear PD Interrupt Flag"]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `SD` writer - Clear SD Interrupt Flag"]
pub type SD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear VBUSDETH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vbusdeth(&mut self) -> VBUSDETH_W<0> {
        VBUSDETH_W::new(self)
    }
    #[doc = "Bit 1 - Clear VBUSDETL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vbusdetl(&mut self) -> VBUSDETL_W<1> {
        VBUSDETL_W::new(self)
    }
    #[doc = "Bit 8 - Clear ERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<8> {
        ERR_W::new(self)
    }
    #[doc = "Bit 9 - Clear DCD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcd(&mut self) -> DCD_W<9> {
        DCD_W::new(self)
    }
    #[doc = "Bit 10 - Clear PD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<10> {
        PD_W::new(self)
    }
    #[doc = "Bit 11 - Clear SD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sd(&mut self) -> SD_W<11> {
        SD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
