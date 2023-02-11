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
pub type ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `WRITE` writer - Set WRITE Interrupt Flag"]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CHOF` writer - Set CHOF Interrupt Flag"]
pub type CHOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CMOF` writer - Set CMOF Interrupt Flag"]
pub type CMOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `PWRUPF` writer - Set PWRUPF Interrupt Flag"]
pub type PWRUPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `ICACHERR` writer - Set ICACHERR Interrupt Flag"]
pub type ICACHERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `WDATAOV` writer - Set WDATAOV Interrupt Flag"]
pub type WDATAOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `LVEWRITE` writer - Set LVEWRITE Interrupt Flag"]
pub type LVEWRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `RAMERR1B` writer - Set RAMERR1B Interrupt Flag"]
pub type RAMERR1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `RAMERR2B` writer - Set RAMERR2B Interrupt Flag"]
pub type RAMERR2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `RAM1ERR1B` writer - Set RAM1ERR1B Interrupt Flag"]
pub type RAM1ERR1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `RAM1ERR2B` writer - Set RAM1ERR2B Interrupt Flag"]
pub type RAM1ERR2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `RAM2ERR1B` writer - Set RAM2ERR1B Interrupt Flag"]
pub type RAM2ERR1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `RAM2ERR2B` writer - Set RAM2ERR2B Interrupt Flag"]
pub type RAM2ERR2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set ERASE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<0> {
        ERASE_W::new(self)
    }
    #[doc = "Bit 1 - Set WRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<1> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 2 - Set CHOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> CHOF_W<2> {
        CHOF_W::new(self)
    }
    #[doc = "Bit 3 - Set CMOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CMOF_W<3> {
        CMOF_W::new(self)
    }
    #[doc = "Bit 4 - Set PWRUPF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PWRUPF_W<4> {
        PWRUPF_W::new(self)
    }
    #[doc = "Bit 5 - Set ICACHERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> ICACHERR_W<5> {
        ICACHERR_W::new(self)
    }
    #[doc = "Bit 6 - Set WDATAOV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdataov(&mut self) -> WDATAOV_W<6> {
        WDATAOV_W::new(self)
    }
    #[doc = "Bit 8 - Set LVEWRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lvewrite(&mut self) -> LVEWRITE_W<8> {
        LVEWRITE_W::new(self)
    }
    #[doc = "Bit 16 - Set RAMERR1B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerr1b(&mut self) -> RAMERR1B_W<16> {
        RAMERR1B_W::new(self)
    }
    #[doc = "Bit 17 - Set RAMERR2B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerr2b(&mut self) -> RAMERR2B_W<17> {
        RAMERR2B_W::new(self)
    }
    #[doc = "Bit 18 - Set RAM1ERR1B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ram1err1b(&mut self) -> RAM1ERR1B_W<18> {
        RAM1ERR1B_W::new(self)
    }
    #[doc = "Bit 19 - Set RAM1ERR2B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ram1err2b(&mut self) -> RAM1ERR2B_W<19> {
        RAM1ERR2B_W::new(self)
    }
    #[doc = "Bit 20 - Set RAM2ERR1B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ram2err1b(&mut self) -> RAM2ERR1B_W<20> {
        RAM2ERR1B_W::new(self)
    }
    #[doc = "Bit 21 - Set RAM2ERR2B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ram2err2b(&mut self) -> RAM2ERR2B_W<21> {
        RAM2ERR2B_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
