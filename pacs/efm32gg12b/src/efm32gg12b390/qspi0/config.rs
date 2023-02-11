#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENBSPI` reader - QSPI Enable"]
pub type ENBSPI_R = crate::BitReader<bool>;
#[doc = "Field `ENBSPI` writer - QSPI Enable"]
pub type ENBSPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SELCLKPOL` reader - Clock Polarity, CPOL"]
pub type SELCLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `SELCLKPOL` writer - Clock Polarity, CPOL"]
pub type SELCLKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SELCLKPHASE` reader - Clock Phase, CPHA"]
pub type SELCLKPHASE_R = crate::BitReader<bool>;
#[doc = "Field `SELCLKPHASE` writer - Clock Phase, CPHA"]
pub type SELCLKPHASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `PHYMODEENABLE` reader - PHY Mode Enable"]
pub type PHYMODEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PHYMODEENABLE` writer - PHY Mode Enable"]
pub type PHYMODEENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENBDEVHOLD` reader - Enable Device Hold"]
pub type ENBDEVHOLD_R = crate::BitReader<bool>;
#[doc = "Field `ENBDEVHOLD` writer - Enable Device Hold"]
pub type ENBDEVHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENBDEVRST` reader - Enable Device Reset"]
pub type ENBDEVRST_R = crate::BitReader<bool>;
#[doc = "Field `ENBDEVRST` writer - Enable Device Reset"]
pub type ENBDEVRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `DEVRSTCONFIG` reader - Device Reset Configuration"]
pub type DEVRSTCONFIG_R = crate::BitReader<bool>;
#[doc = "Field `DEVRSTCONFIG` writer - Device Reset Configuration"]
pub type DEVRSTCONFIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENBDIRACCCTLR` reader - Enable Direct Access Controller"]
pub type ENBDIRACCCTLR_R = crate::BitReader<bool>;
#[doc = "Field `ENBDIRACCCTLR` writer - Enable Direct Access Controller"]
pub type ENBDIRACCCTLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENBLEGACYIPMODE` reader - Legacy IP Mode Enable"]
pub type ENBLEGACYIPMODE_R = crate::BitReader<bool>;
#[doc = "Field `ENBLEGACYIPMODE` writer - Legacy IP Mode Enable"]
pub type ENBLEGACYIPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `PERIPHSELDEC` reader - Peripheral Select Decode"]
pub type PERIPHSELDEC_R = crate::BitReader<bool>;
#[doc = "Field `PERIPHSELDEC` writer - Peripheral Select Decode"]
pub type PERIPHSELDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `PERIPHCSLINES` reader - Peripheral Chip Select Lines"]
pub type PERIPHCSLINES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERIPHCSLINES` writer - Peripheral Chip Select Lines"]
pub type PERIPHCSLINES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `WRPROTFLASH` reader - Write Protect Flash Pin"]
pub type WRPROTFLASH_R = crate::BitReader<bool>;
#[doc = "Field `WRPROTFLASH` writer - Write Protect Flash Pin"]
pub type WRPROTFLASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENBAHBADDRREMAP` reader - Enable Address Remapping"]
pub type ENBAHBADDRREMAP_R = crate::BitReader<bool>;
#[doc = "Field `ENBAHBADDRREMAP` writer - Enable Address Remapping"]
pub type ENBAHBADDRREMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENTERXIPMODE` reader - Enter XIP Mode on Next READ"]
pub type ENTERXIPMODE_R = crate::BitReader<bool>;
#[doc = "Field `ENTERXIPMODE` writer - Enter XIP Mode on Next READ"]
pub type ENTERXIPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENTERXIPMODEIMM` reader - Enter XIP Mode Immediately"]
pub type ENTERXIPMODEIMM_R = crate::BitReader<bool>;
#[doc = "Field `ENTERXIPMODEIMM` writer - Enter XIP Mode Immediately"]
pub type ENTERXIPMODEIMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `MSTRBAUDDIV` reader - Master Mode Baud Rate Divisor"]
pub type MSTRBAUDDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSTRBAUDDIV` writer - Master Mode Baud Rate Divisor"]
pub type MSTRBAUDDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `ENABLEAHBDECODER` reader - Enable Address Decoder"]
pub type ENABLEAHBDECODER_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEAHBDECODER` writer - Enable Address Decoder"]
pub type ENABLEAHBDECODER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENABLEDTRPROTOCOL` reader - Enable DTR Protocol"]
pub type ENABLEDTRPROTOCOL_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEDTRPROTOCOL` writer - Enable DTR Protocol"]
pub type ENABLEDTRPROTOCOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `PIPELINEPHY` reader - Pipeline PHY Mode Enable"]
pub type PIPELINEPHY_R = crate::BitReader<bool>;
#[doc = "Field `PIPELINEPHY` writer - Pipeline PHY Mode Enable"]
pub type PIPELINEPHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `CRCENABLE` reader - CRC Enable Bit"]
pub type CRCENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CRCENABLE` writer - CRC Enable Bit"]
pub type CRCENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `DUALBYTEOPCODEEN` reader - Dual-byte Opcode Mode Enable Bit"]
pub type DUALBYTEOPCODEEN_R = crate::BitReader<bool>;
#[doc = "Field `DUALBYTEOPCODEEN` writer - Dual-byte Opcode Mode Enable Bit"]
pub type DUALBYTEOPCODEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `IDLE` reader - Serial Interface and Low Level SPI Pipeline is IDLE"]
pub type IDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn enbspi(&self) -> ENBSPI_R {
        ENBSPI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity, CPOL"]
    #[inline(always)]
    pub fn selclkpol(&self) -> SELCLKPOL_R {
        SELCLKPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Phase, CPHA"]
    #[inline(always)]
    pub fn selclkphase(&self) -> SELCLKPHASE_R {
        SELCLKPHASE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PHY Mode Enable"]
    #[inline(always)]
    pub fn phymodeenable(&self) -> PHYMODEENABLE_R {
        PHYMODEENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Device Hold"]
    #[inline(always)]
    pub fn enbdevhold(&self) -> ENBDEVHOLD_R {
        ENBDEVHOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Device Reset"]
    #[inline(always)]
    pub fn enbdevrst(&self) -> ENBDEVRST_R {
        ENBDEVRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Device Reset Configuration"]
    #[inline(always)]
    pub fn devrstconfig(&self) -> DEVRSTCONFIG_R {
        DEVRSTCONFIG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn enbdiraccctlr(&self) -> ENBDIRACCCTLR_R {
        ENBDIRACCCTLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn enblegacyipmode(&self) -> ENBLEGACYIPMODE_R {
        ENBLEGACYIPMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Select Decode"]
    #[inline(always)]
    pub fn periphseldec(&self) -> PERIPHSELDEC_R {
        PERIPHSELDEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Peripheral Chip Select Lines"]
    #[inline(always)]
    pub fn periphcslines(&self) -> PERIPHCSLINES_R {
        PERIPHCSLINES_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Write Protect Flash Pin"]
    #[inline(always)]
    pub fn wrprotflash(&self) -> WRPROTFLASH_R {
        WRPROTFLASH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Address Remapping"]
    #[inline(always)]
    pub fn enbahbaddrremap(&self) -> ENBAHBADDRREMAP_R {
        ENBAHBADDRREMAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enter XIP Mode on Next READ"]
    #[inline(always)]
    pub fn enterxipmode(&self) -> ENTERXIPMODE_R {
        ENTERXIPMODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enter XIP Mode Immediately"]
    #[inline(always)]
    pub fn enterxipmodeimm(&self) -> ENTERXIPMODEIMM_R {
        ENTERXIPMODEIMM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Master Mode Baud Rate Divisor"]
    #[inline(always)]
    pub fn mstrbauddiv(&self) -> MSTRBAUDDIV_R {
        MSTRBAUDDIV_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Enable Address Decoder"]
    #[inline(always)]
    pub fn enableahbdecoder(&self) -> ENABLEAHBDECODER_R {
        ENABLEAHBDECODER_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn enabledtrprotocol(&self) -> ENABLEDTRPROTOCOL_R {
        ENABLEDTRPROTOCOL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode Enable"]
    #[inline(always)]
    pub fn pipelinephy(&self) -> PIPELINEPHY_R {
        PIPELINEPHY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - CRC Enable Bit"]
    #[inline(always)]
    pub fn crcenable(&self) -> CRCENABLE_R {
        CRCENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Dual-byte Opcode Mode Enable Bit"]
    #[inline(always)]
    pub fn dualbyteopcodeen(&self) -> DUALBYTEOPCODEEN_R {
        DUALBYTEOPCODEEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Serial Interface and Low Level SPI Pipeline is IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbspi(&mut self) -> ENBSPI_W<0> {
        ENBSPI_W::new(self)
    }
    #[doc = "Bit 1 - Clock Polarity, CPOL"]
    #[inline(always)]
    #[must_use]
    pub fn selclkpol(&mut self) -> SELCLKPOL_W<1> {
        SELCLKPOL_W::new(self)
    }
    #[doc = "Bit 2 - Clock Phase, CPHA"]
    #[inline(always)]
    #[must_use]
    pub fn selclkphase(&mut self) -> SELCLKPHASE_W<2> {
        SELCLKPHASE_W::new(self)
    }
    #[doc = "Bit 3 - PHY Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn phymodeenable(&mut self) -> PHYMODEENABLE_W<3> {
        PHYMODEENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Enable Device Hold"]
    #[inline(always)]
    #[must_use]
    pub fn enbdevhold(&mut self) -> ENBDEVHOLD_W<4> {
        ENBDEVHOLD_W::new(self)
    }
    #[doc = "Bit 5 - Enable Device Reset"]
    #[inline(always)]
    #[must_use]
    pub fn enbdevrst(&mut self) -> ENBDEVRST_W<5> {
        ENBDEVRST_W::new(self)
    }
    #[doc = "Bit 6 - Device Reset Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn devrstconfig(&mut self) -> DEVRSTCONFIG_W<6> {
        DEVRSTCONFIG_W::new(self)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    #[must_use]
    pub fn enbdiraccctlr(&mut self) -> ENBDIRACCCTLR_W<7> {
        ENBDIRACCCTLR_W::new(self)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enblegacyipmode(&mut self) -> ENBLEGACYIPMODE_W<8> {
        ENBLEGACYIPMODE_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral Select Decode"]
    #[inline(always)]
    #[must_use]
    pub fn periphseldec(&mut self) -> PERIPHSELDEC_W<9> {
        PERIPHSELDEC_W::new(self)
    }
    #[doc = "Bits 10:11 - Peripheral Chip Select Lines"]
    #[inline(always)]
    #[must_use]
    pub fn periphcslines(&mut self) -> PERIPHCSLINES_W<10> {
        PERIPHCSLINES_W::new(self)
    }
    #[doc = "Bit 14 - Write Protect Flash Pin"]
    #[inline(always)]
    #[must_use]
    pub fn wrprotflash(&mut self) -> WRPROTFLASH_W<14> {
        WRPROTFLASH_W::new(self)
    }
    #[doc = "Bit 16 - Enable Address Remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enbahbaddrremap(&mut self) -> ENBAHBADDRREMAP_W<16> {
        ENBAHBADDRREMAP_W::new(self)
    }
    #[doc = "Bit 17 - Enter XIP Mode on Next READ"]
    #[inline(always)]
    #[must_use]
    pub fn enterxipmode(&mut self) -> ENTERXIPMODE_W<17> {
        ENTERXIPMODE_W::new(self)
    }
    #[doc = "Bit 18 - Enter XIP Mode Immediately"]
    #[inline(always)]
    #[must_use]
    pub fn enterxipmodeimm(&mut self) -> ENTERXIPMODEIMM_W<18> {
        ENTERXIPMODEIMM_W::new(self)
    }
    #[doc = "Bits 19:22 - Master Mode Baud Rate Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn mstrbauddiv(&mut self) -> MSTRBAUDDIV_W<19> {
        MSTRBAUDDIV_W::new(self)
    }
    #[doc = "Bit 23 - Enable Address Decoder"]
    #[inline(always)]
    #[must_use]
    pub fn enableahbdecoder(&mut self) -> ENABLEAHBDECODER_W<23> {
        ENABLEAHBDECODER_W::new(self)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    #[must_use]
    pub fn enabledtrprotocol(&mut self) -> ENABLEDTRPROTOCOL_W<24> {
        ENABLEDTRPROTOCOL_W::new(self)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pipelinephy(&mut self) -> PIPELINEPHY_W<25> {
        PIPELINEPHY_W::new(self)
    }
    #[doc = "Bit 29 - CRC Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcenable(&mut self) -> CRCENABLE_W<29> {
        CRCENABLE_W::new(self)
    }
    #[doc = "Bit 30 - Dual-byte Opcode Mode Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dualbyteopcodeen(&mut self) -> DUALBYTEOPCODEEN_W<30> {
        DUALBYTEOPCODEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Octal-SPI Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x8078_0081"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x8078_0081;
}
