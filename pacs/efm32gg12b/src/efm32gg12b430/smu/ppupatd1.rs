#[doc = "Register `PPUPATD1` reader"]
pub struct R(crate::R<PPUPATD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUPATD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUPATD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUPATD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPUPATD1` writer"]
pub struct W(crate::W<PPUPATD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPUPATD1_SPEC>;
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
impl From<crate::W<PPUPATD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPUPATD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCNT0` reader - Pulse Counter 0 access control bit"]
pub type PCNT0_R = crate::BitReader<bool>;
#[doc = "Field `PCNT0` writer - Pulse Counter 0 access control bit"]
pub type PCNT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `PCNT1` reader - Pulse Counter 1 access control bit"]
pub type PCNT1_R = crate::BitReader<bool>;
#[doc = "Field `PCNT1` writer - Pulse Counter 1 access control bit"]
pub type PCNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `PCNT2` reader - Pulse Counter 2 access control bit"]
pub type PCNT2_R = crate::BitReader<bool>;
#[doc = "Field `PCNT2` writer - Pulse Counter 2 access control bit"]
pub type PCNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `PDM` reader - PDM Interface access control bit"]
pub type PDM_R = crate::BitReader<bool>;
#[doc = "Field `PDM` writer - PDM Interface access control bit"]
pub type PDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `QSPI0` reader - Quad-SPI access control bit"]
pub type QSPI0_R = crate::BitReader<bool>;
#[doc = "Field `QSPI0` writer - Quad-SPI access control bit"]
pub type QSPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `RMU` reader - Reset Management Unit access control bit"]
pub type RMU_R = crate::BitReader<bool>;
#[doc = "Field `RMU` writer - Reset Management Unit access control bit"]
pub type RMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `RTC` reader - Real-Time Counter access control bit"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - Real-Time Counter access control bit"]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar access control bit"]
pub type RTCC_R = crate::BitReader<bool>;
#[doc = "Field `RTCC` writer - Real-Time Counter and Calendar access control bit"]
pub type RTCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `SDIO` reader - SDIO Controller access control bit"]
pub type SDIO_R = crate::BitReader<bool>;
#[doc = "Field `SDIO` writer - SDIO Controller access control bit"]
pub type SDIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `SMU` reader - Security Management Unit access control bit"]
pub type SMU_R = crate::BitReader<bool>;
#[doc = "Field `SMU` writer - Security Management Unit access control bit"]
pub type SMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `TIMER0` reader - Timer 0 access control bit"]
pub type TIMER0_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0` writer - Timer 0 access control bit"]
pub type TIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `TIMER1` reader - Timer 1 access control bit"]
pub type TIMER1_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1` writer - Timer 1 access control bit"]
pub type TIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `TIMER2` reader - Timer 2 access control bit"]
pub type TIMER2_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2` writer - Timer 2 access control bit"]
pub type TIMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `TIMER3` reader - Timer 3 access control bit"]
pub type TIMER3_R = crate::BitReader<bool>;
#[doc = "Field `TIMER3` writer - Timer 3 access control bit"]
pub type TIMER3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `TRNG0` reader - True Random Number Generator 0 access control bit"]
pub type TRNG0_R = crate::BitReader<bool>;
#[doc = "Field `TRNG0` writer - True Random Number Generator 0 access control bit"]
pub type TRNG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `UART0` reader - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `UART1` reader - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
pub type USART0_R = crate::BitReader<bool>;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
pub type USART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
pub type USART1_R = crate::BitReader<bool>;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
pub type USART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USART2` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
pub type USART2_R = crate::BitReader<bool>;
#[doc = "Field `USART2` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
pub type USART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USART3` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type USART3_R = crate::BitReader<bool>;
#[doc = "Field `USART3` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type USART3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USART4` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
pub type USART4_R = crate::BitReader<bool>;
#[doc = "Field `USART4` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
pub type USART4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USB` reader - Universal Serial Bus Interface access control bit"]
pub type USB_R = crate::BitReader<bool>;
#[doc = "Field `USB` writer - Universal Serial Bus Interface access control bit"]
pub type USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `WDOG0` reader - Watchdog access control bit"]
pub type WDOG0_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0` writer - Watchdog access control bit"]
pub type WDOG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `WDOG1` reader - Watchdog access control bit"]
pub type WDOG1_R = crate::BitReader<bool>;
#[doc = "Field `WDOG1` writer - Watchdog access control bit"]
pub type WDOG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `WTIMER0` reader - Wide Timer 0 access control bit"]
pub type WTIMER0_R = crate::BitReader<bool>;
#[doc = "Field `WTIMER0` writer - Wide Timer 0 access control bit"]
pub type WTIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `WTIMER1` reader - Wide Timer 0 access control bit"]
pub type WTIMER1_R = crate::BitReader<bool>;
#[doc = "Field `WTIMER1` writer - Wide Timer 0 access control bit"]
pub type WTIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    pub fn pcnt0(&self) -> PCNT0_R {
        PCNT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    pub fn pcnt1(&self) -> PCNT1_R {
        PCNT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    pub fn pcnt2(&self) -> PCNT2_R {
        PCNT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDM Interface access control bit"]
    #[inline(always)]
    pub fn pdm(&self) -> PDM_R {
        PDM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Quad-SPI access control bit"]
    #[inline(always)]
    pub fn qspi0(&self) -> QSPI0_R {
        QSPI0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset Management Unit access control bit"]
    #[inline(always)]
    pub fn rmu(&self) -> RMU_R {
        RMU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real-Time Counter access control bit"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO Controller access control bit"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security Management Unit access control bit"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer 0 access control bit"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 1 access control bit"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer 2 access control bit"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer 3 access control bit"]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    pub fn trng0(&self) -> TRNG0_R {
        TRNG0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Universal Serial Bus Interface access control bit"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog1(&self) -> WDOG1_R {
        WDOG1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&self) -> WTIMER0_R {
        WTIMER0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&self) -> WTIMER1_R {
        WTIMER1_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0(&mut self) -> PCNT0_W<0> {
        PCNT0_W::new(self)
    }
    #[doc = "Bit 1 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt1(&mut self) -> PCNT1_W<1> {
        PCNT1_W::new(self)
    }
    #[doc = "Bit 2 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt2(&mut self) -> PCNT2_W<2> {
        PCNT2_W::new(self)
    }
    #[doc = "Bit 3 - PDM Interface access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pdm(&mut self) -> PDM_W<3> {
        PDM_W::new(self)
    }
    #[doc = "Bit 4 - Quad-SPI access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn qspi0(&mut self) -> QSPI0_W<4> {
        QSPI0_W::new(self)
    }
    #[doc = "Bit 5 - Reset Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn rmu(&mut self) -> RMU_W<5> {
        RMU_W::new(self)
    }
    #[doc = "Bit 6 - Real-Time Counter access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<6> {
        RTC_W::new(self)
    }
    #[doc = "Bit 7 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn rtcc(&mut self) -> RTCC_W<7> {
        RTCC_W::new(self)
    }
    #[doc = "Bit 8 - SDIO Controller access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn sdio(&mut self) -> SDIO_W<8> {
        SDIO_W::new(self)
    }
    #[doc = "Bit 9 - Security Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn smu(&mut self) -> SMU_W<9> {
        SMU_W::new(self)
    }
    #[doc = "Bit 10 - Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> TIMER0_W<10> {
        TIMER0_W::new(self)
    }
    #[doc = "Bit 11 - Timer 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> TIMER1_W<11> {
        TIMER1_W::new(self)
    }
    #[doc = "Bit 12 - Timer 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer2(&mut self) -> TIMER2_W<12> {
        TIMER2_W::new(self)
    }
    #[doc = "Bit 13 - Timer 3 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer3(&mut self) -> TIMER3_W<13> {
        TIMER3_W::new(self)
    }
    #[doc = "Bit 14 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn trng0(&mut self) -> TRNG0_W<14> {
        TRNG0_W::new(self)
    }
    #[doc = "Bit 15 - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<15> {
        UART0_W::new(self)
    }
    #[doc = "Bit 16 - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<16> {
        UART1_W::new(self)
    }
    #[doc = "Bit 17 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<17> {
        USART0_W::new(self)
    }
    #[doc = "Bit 18 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<18> {
        USART1_W::new(self)
    }
    #[doc = "Bit 19 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart2(&mut self) -> USART2_W<19> {
        USART2_W::new(self)
    }
    #[doc = "Bit 20 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart3(&mut self) -> USART3_W<20> {
        USART3_W::new(self)
    }
    #[doc = "Bit 21 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart4(&mut self) -> USART4_W<21> {
        USART4_W::new(self)
    }
    #[doc = "Bit 22 - Universal Serial Bus Interface access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<22> {
        USB_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0(&mut self) -> WDOG0_W<23> {
        WDOG0_W::new(self)
    }
    #[doc = "Bit 24 - Watchdog access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdog1(&mut self) -> WDOG1_W<24> {
        WDOG1_W::new(self)
    }
    #[doc = "Bit 25 - Wide Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer0(&mut self) -> WTIMER0_W<25> {
        WTIMER0_W::new(self)
    }
    #[doc = "Bit 26 - Wide Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer1(&mut self) -> WTIMER1_W<26> {
        WTIMER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPU Privilege Access Type Descriptor 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd1](index.html) module"]
pub struct PPUPATD1_SPEC;
impl crate::RegisterSpec for PPUPATD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppupatd1::R](R) reader structure"]
impl crate::Readable for PPUPATD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppupatd1::W](W) writer structure"]
impl crate::Writable for PPUPATD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPUPATD1 to value 0"]
impl crate::Resettable for PPUPATD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
