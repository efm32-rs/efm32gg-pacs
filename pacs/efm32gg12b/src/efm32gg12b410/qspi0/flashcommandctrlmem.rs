#[doc = "Register `FLASHCOMMANDCTRLMEM` reader"]
pub struct R(crate::R<FLASHCOMMANDCTRLMEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHCOMMANDCTRLMEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHCOMMANDCTRLMEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHCOMMANDCTRLMEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHCOMMANDCTRLMEM` writer"]
pub struct W(crate::W<FLASHCOMMANDCTRLMEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHCOMMANDCTRLMEM_SPEC>;
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
impl From<crate::W<FLASHCOMMANDCTRLMEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHCOMMANDCTRLMEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGERMEMBANKREQ` writer - Trigger the Memory Bank Data Request"]
pub type TRIGGERMEMBANKREQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLASHCOMMANDCTRLMEM_SPEC, bool, O>;
#[doc = "Field `MEMBANKREQINPROGRESS` reader - Memory Bank Data Request in Progress"]
pub type MEMBANKREQINPROGRESS_R = crate::BitReader<bool>;
#[doc = "Field `MEMBANKREADDATA` reader - Last Requested Data From the STIG Memory Bank"]
pub type MEMBANKREADDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBOFSTIGREADBYTES` reader - Number of Read Bytes for the Extended STIG"]
pub type NBOFSTIGREADBYTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBOFSTIGREADBYTES` writer - Number of Read Bytes for the Extended STIG"]
pub type NBOFSTIGREADBYTES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHCOMMANDCTRLMEM_SPEC, u8, u8, 3, O>;
#[doc = "Field `MEMBANKADDR` reader - Memory Bank Address"]
pub type MEMBANKADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEMBANKADDR` writer - Memory Bank Address"]
pub type MEMBANKADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHCOMMANDCTRLMEM_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bit 1 - Memory Bank Data Request in Progress"]
    #[inline(always)]
    pub fn membankreqinprogress(&self) -> MEMBANKREQINPROGRESS_R {
        MEMBANKREQINPROGRESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Last Requested Data From the STIG Memory Bank"]
    #[inline(always)]
    pub fn membankreaddata(&self) -> MEMBANKREADDATA_R {
        MEMBANKREADDATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Number of Read Bytes for the Extended STIG"]
    #[inline(always)]
    pub fn nbofstigreadbytes(&self) -> NBOFSTIGREADBYTES_R {
        NBOFSTIGREADBYTES_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:28 - Memory Bank Address"]
    #[inline(always)]
    pub fn membankaddr(&self) -> MEMBANKADDR_R {
        MEMBANKADDR_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger the Memory Bank Data Request"]
    #[inline(always)]
    #[must_use]
    pub fn triggermembankreq(&mut self) -> TRIGGERMEMBANKREQ_W<0> {
        TRIGGERMEMBANKREQ_W::new(self)
    }
    #[doc = "Bits 16:18 - Number of Read Bytes for the Extended STIG"]
    #[inline(always)]
    #[must_use]
    pub fn nbofstigreadbytes(&mut self) -> NBOFSTIGREADBYTES_W<16> {
        NBOFSTIGREADBYTES_W::new(self)
    }
    #[doc = "Bits 20:28 - Memory Bank Address"]
    #[inline(always)]
    #[must_use]
    pub fn membankaddr(&mut self) -> MEMBANKADDR_W<20> {
        MEMBANKADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Command Control Memory Register (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcommandctrlmem](index.html) module"]
pub struct FLASHCOMMANDCTRLMEM_SPEC;
impl crate::RegisterSpec for FLASHCOMMANDCTRLMEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashcommandctrlmem::R](R) reader structure"]
impl crate::Readable for FLASHCOMMANDCTRLMEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashcommandctrlmem::W](W) writer structure"]
impl crate::Writable for FLASHCOMMANDCTRLMEM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASHCOMMANDCTRLMEM to value 0"]
impl crate::Resettable for FLASHCOMMANDCTRLMEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
