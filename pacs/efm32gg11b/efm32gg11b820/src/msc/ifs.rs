#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASE` writer - Set ERASE Interrupt Flag"]
pub type ERASE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `WRITE` writer - Set WRITE Interrupt Flag"]
pub type WRITE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `CHOF` writer - Set CHOF Interrupt Flag"]
pub type CHOF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `CMOF` writer - Set CMOF Interrupt Flag"]
pub type CMOF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `PWRUPF` writer - Set PWRUPF Interrupt Flag"]
pub type PWRUPF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
#[doc = "Field `ICACHERR` writer - Set ICACHERR Interrupt Flag"]
pub type ICACHERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 5>;
#[doc = "Field `WDATAOV` writer - Set WDATAOV Interrupt Flag"]
pub type WDATAOV_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 6>;
#[doc = "Field `LVEWRITE` writer - Set LVEWRITE Interrupt Flag"]
pub type LVEWRITE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 8>;
#[doc = "Field `RAMERR1B` writer - Set RAMERR1B Interrupt Flag"]
pub type RAMERR1B_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 16>;
#[doc = "Field `RAMERR2B` writer - Set RAMERR2B Interrupt Flag"]
pub type RAMERR2B_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 17>;
#[doc = "Field `RAM1ERR1B` writer - Set RAM1ERR1B Interrupt Flag"]
pub type RAM1ERR1B_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 18>;
#[doc = "Field `RAM1ERR2B` writer - Set RAM1ERR2B Interrupt Flag"]
pub type RAM1ERR2B_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 19>;
impl W {
    #[doc = "Bit 0 - Set ERASE Interrupt Flag"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W::new(self)
    }
    #[doc = "Bit 1 - Set WRITE Interrupt Flag"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W::new(self)
    }
    #[doc = "Bit 2 - Set CHOF Interrupt Flag"]
    #[inline(always)]
    pub fn chof(&mut self) -> CHOF_W {
        CHOF_W::new(self)
    }
    #[doc = "Bit 3 - Set CMOF Interrupt Flag"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CMOF_W {
        CMOF_W::new(self)
    }
    #[doc = "Bit 4 - Set PWRUPF Interrupt Flag"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PWRUPF_W {
        PWRUPF_W::new(self)
    }
    #[doc = "Bit 5 - Set ICACHERR Interrupt Flag"]
    #[inline(always)]
    pub fn icacherr(&mut self) -> ICACHERR_W {
        ICACHERR_W::new(self)
    }
    #[doc = "Bit 6 - Set WDATAOV Interrupt Flag"]
    #[inline(always)]
    pub fn wdataov(&mut self) -> WDATAOV_W {
        WDATAOV_W::new(self)
    }
    #[doc = "Bit 8 - Set LVEWRITE Interrupt Flag"]
    #[inline(always)]
    pub fn lvewrite(&mut self) -> LVEWRITE_W {
        LVEWRITE_W::new(self)
    }
    #[doc = "Bit 16 - Set RAMERR1B Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&mut self) -> RAMERR1B_W {
        RAMERR1B_W::new(self)
    }
    #[doc = "Bit 17 - Set RAMERR2B Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&mut self) -> RAMERR2B_W {
        RAMERR2B_W::new(self)
    }
    #[doc = "Bit 18 - Set RAM1ERR1B Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err1b(&mut self) -> RAM1ERR1B_W {
        RAM1ERR1B_W::new(self)
    }
    #[doc = "Bit 19 - Set RAM1ERR2B Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err2b(&mut self) -> RAM1ERR2B_W {
        RAM1ERR2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
