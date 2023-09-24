#[doc = "Register `RST0` reader"]
pub type R = crate::R<RST0_SPEC>;
#[doc = "Register `RST0` writer"]
pub type W = crate::W<RST0_SPEC>;
#[doc = "Field `DMA` reader - DMA Reset."]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - DMA Reset."]
pub type DMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT0` reader - Watchdog Timer Reset."]
pub type WDT0_R = crate::BitReader;
#[doc = "Field `WDT0` writer - Watchdog Timer Reset."]
pub type WDT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO0` reader - GPIO0 Reset."]
pub type GPIO0_R = crate::BitReader;
#[doc = "Field `GPIO0` writer - GPIO0 Reset."]
pub type GPIO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR0` reader - TMR0 Reset."]
pub type TMR0_R = crate::BitReader;
#[doc = "Field `TMR0` writer - TMR0 Reset."]
pub type TMR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR1` reader - TMR1 Reset."]
pub type TMR1_R = crate::BitReader;
#[doc = "Field `TMR1` writer - TMR1 Reset."]
pub type TMR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR2` reader - TMR2 Reset."]
pub type TMR2_R = crate::BitReader;
#[doc = "Field `TMR2` writer - TMR2 Reset."]
pub type TMR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART0` reader - UART0 Reset."]
pub type UART0_R = crate::BitReader;
#[doc = "Field `UART0` writer - UART0 Reset."]
pub type UART0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART1` reader - UART1 Reset."]
pub type UART1_R = crate::BitReader;
#[doc = "Field `UART1` writer - UART1 Reset."]
pub type UART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI0` reader - SPI0 Reset."]
pub type SPI0_R = crate::BitReader;
#[doc = "Field `SPI0` writer - SPI0 Reset."]
pub type SPI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1` reader - SPI1 Reset."]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 Reset."]
pub type SPI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C0` reader - I2C0 Reset."]
pub type I2C0_R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C0 Reset."]
pub type I2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC` reader - Real Time Clock Reset."]
pub type RTC_R = crate::BitReader;
#[doc = "Field `RTC` writer - Real Time Clock Reset."]
pub type RTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFT` reader - Soft Reset."]
pub type SOFT_R = crate::BitReader;
#[doc = "Field `SOFT` writer - Soft Reset."]
pub type SOFT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERIPH` reader - Peripheral Reset."]
pub type PERIPH_R = crate::BitReader;
#[doc = "Field `PERIPH` writer - Peripheral Reset."]
pub type PERIPH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSTEM` reader - System Reset."]
pub type SYSTEM_R = crate::BitReader;
#[doc = "Field `SYSTEM` writer - System Reset."]
pub type SYSTEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset."]
    #[inline(always)]
    pub fn wdt0(&self) -> WDT0_R {
        WDT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO0 Reset."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TMR0 Reset."]
    #[inline(always)]
    pub fn tmr0(&self) -> TMR0_R {
        TMR0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TMR1 Reset."]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TMR2 Reset."]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - UART0 Reset."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART1 Reset."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI0 Reset."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 Reset."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C0 Reset."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 29 - Soft Reset."]
    #[inline(always)]
    pub fn soft(&self) -> SOFT_R {
        SOFT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral Reset."]
    #[inline(always)]
    pub fn periph(&self) -> PERIPH_R {
        PERIPH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - System Reset."]
    #[inline(always)]
    pub fn system(&self) -> SYSTEM_R {
        SYSTEM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<RST0_SPEC, 0> {
        DMA_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdt0(&mut self) -> WDT0_W<RST0_SPEC, 1> {
        WDT0_W::new(self)
    }
    #[doc = "Bit 2 - GPIO0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO0_W<RST0_SPEC, 2> {
        GPIO0_W::new(self)
    }
    #[doc = "Bit 5 - TMR0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0(&mut self) -> TMR0_W<RST0_SPEC, 5> {
        TMR0_W::new(self)
    }
    #[doc = "Bit 6 - TMR1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<RST0_SPEC, 6> {
        TMR1_W::new(self)
    }
    #[doc = "Bit 7 - TMR2 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<RST0_SPEC, 7> {
        TMR2_W::new(self)
    }
    #[doc = "Bit 11 - UART0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<RST0_SPEC, 11> {
        UART0_W::new(self)
    }
    #[doc = "Bit 12 - UART1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<RST0_SPEC, 12> {
        UART1_W::new(self)
    }
    #[doc = "Bit 13 - SPI0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<RST0_SPEC, 13> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 14 - SPI1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<RST0_SPEC, 14> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 16 - I2C0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<RST0_SPEC, 16> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<RST0_SPEC, 17> {
        RTC_W::new(self)
    }
    #[doc = "Bit 29 - Soft Reset."]
    #[inline(always)]
    #[must_use]
    pub fn soft(&mut self) -> SOFT_W<RST0_SPEC, 29> {
        SOFT_W::new(self)
    }
    #[doc = "Bit 30 - Peripheral Reset."]
    #[inline(always)]
    #[must_use]
    pub fn periph(&mut self) -> PERIPH_W<RST0_SPEC, 30> {
        PERIPH_W::new(self)
    }
    #[doc = "Bit 31 - System Reset."]
    #[inline(always)]
    #[must_use]
    pub fn system(&mut self) -> SYSTEM_W<RST0_SPEC, 31> {
        SYSTEM_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST0_SPEC;
impl crate::RegisterSpec for RST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst0::R`](R) reader structure"]
impl crate::Readable for RST0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst0::W`](W) writer structure"]
impl crate::Writable for RST0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST0 to value 0"]
impl crate::Resettable for RST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
