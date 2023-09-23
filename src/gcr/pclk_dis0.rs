#[doc = "Register `PCLK_DIS0` reader"]
pub type R = crate::R<PCLK_DIS0_SPEC>;
#[doc = "Register `PCLK_DIS0` writer"]
pub type W = crate::W<PCLK_DIS0_SPEC>;
#[doc = "Field `GPIO0` reader - GPIO0 Clock Disable."]
pub type GPIO0_R = crate::BitReader<GPIO0_A>;
#[doc = "GPIO0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<GPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_A {
        match self.bits {
            false => GPIO0_A::CLOCK_ENABLED,
            true => GPIO0_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == GPIO0_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == GPIO0_A::CLOCK_DISABLED
    }
}
#[doc = "Field `GPIO0` writer - GPIO0 Clock Disable."]
pub type GPIO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GPIO0_A>;
impl<'a, REG, const O: u8> GPIO0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO0_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO0_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `DMA` reader - DMA Clock Disable."]
pub type DMA_R = crate::BitReader<DMA_A>;
#[doc = "DMA Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::CLOCK_ENABLED,
            true => DMA_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == DMA_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == DMA_A::CLOCK_DISABLED
    }
}
#[doc = "Field `DMA` writer - DMA Clock Disable."]
pub type DMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_A>;
impl<'a, REG, const O: u8> DMA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `SPI0` reader - SPI0 Clock Disable."]
pub type SPI0_R = crate::BitReader<SPI0_A>;
#[doc = "SPI0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI0_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::CLOCK_ENABLED,
            true => SPI0_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == SPI0_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == SPI0_A::CLOCK_DISABLED
    }
}
#[doc = "Field `SPI0` writer - SPI0 Clock Disable."]
pub type SPI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPI0_A>;
impl<'a, REG, const O: u8> SPI0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `SPI1` reader - SPI1 Clock Disable."]
pub type SPI1_R = crate::BitReader<SPI1_A>;
#[doc = "SPI1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::CLOCK_ENABLED,
            true => SPI1_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == SPI1_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == SPI1_A::CLOCK_DISABLED
    }
}
#[doc = "Field `SPI1` writer - SPI1 Clock Disable."]
pub type SPI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPI1_A>;
impl<'a, REG, const O: u8> SPI1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `UART0` reader - UART0 Clock Disable."]
pub type UART0_R = crate::BitReader<UART0_A>;
#[doc = "UART0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART0_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
impl UART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::CLOCK_ENABLED,
            true => UART0_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == UART0_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == UART0_A::CLOCK_DISABLED
    }
}
#[doc = "Field `UART0` writer - UART0 Clock Disable."]
pub type UART0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UART0_A>;
impl<'a, REG, const O: u8> UART0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART0_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART0_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `UART1` reader - UART1 Clock Disable."]
pub type UART1_R = crate::BitReader<UART1_A>;
#[doc = "UART1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART1_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
impl UART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::CLOCK_ENABLED,
            true => UART1_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == UART1_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == UART1_A::CLOCK_DISABLED
    }
}
#[doc = "Field `UART1` writer - UART1 Clock Disable."]
pub type UART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UART1_A>;
impl<'a, REG, const O: u8> UART1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART1_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART1_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `I2C0` reader - I2C0 Clock Disable."]
pub type I2C0_R = crate::BitReader<I2C0_A>;
#[doc = "I2C0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<I2C0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::CLOCK_ENABLED,
            true => I2C0_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == I2C0_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == I2C0_A::CLOCK_DISABLED
    }
}
#[doc = "Field `I2C0` writer - I2C0 Clock Disable."]
pub type I2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C0_A>;
impl<'a, REG, const O: u8> I2C0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `TMR0` reader - TMR0 Clock Disable."]
pub type TMR0_R = crate::BitReader<TMR0_A>;
#[doc = "TMR0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR0_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<TMR0_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0_A {
        match self.bits {
            false => TMR0_A::CLOCK_ENABLED,
            true => TMR0_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == TMR0_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == TMR0_A::CLOCK_DISABLED
    }
}
#[doc = "Field `TMR0` writer - TMR0 Clock Disable."]
pub type TMR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMR0_A>;
impl<'a, REG, const O: u8> TMR0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMR0_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMR0_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `TMR1` reader - TMR1 Clock Disable."]
pub type TMR1_R = crate::BitReader<TMR1_A>;
#[doc = "TMR1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR1_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<TMR1_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1_A {
        match self.bits {
            false => TMR1_A::CLOCK_ENABLED,
            true => TMR1_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == TMR1_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == TMR1_A::CLOCK_DISABLED
    }
}
#[doc = "Field `TMR1` writer - TMR1 Clock Disable."]
pub type TMR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMR1_A>;
impl<'a, REG, const O: u8> TMR1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `TMR2` reader - TMR2 Clock Disable."]
pub type TMR2_R = crate::BitReader<TMR2_A>;
#[doc = "TMR2 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR2_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<TMR2_A> for bool {
    #[inline(always)]
    fn from(variant: TMR2_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2_A {
        match self.bits {
            false => TMR2_A::CLOCK_ENABLED,
            true => TMR2_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == TMR2_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == TMR2_A::CLOCK_DISABLED
    }
}
#[doc = "Field `TMR2` writer - TMR2 Clock Disable."]
pub type TMR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMR2_A>;
impl<'a, REG, const O: u8> TMR2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `I2C1D` reader - I2C1 Clock Disable."]
pub type I2C1D_R = crate::BitReader<I2C1D_A>;
#[doc = "I2C1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1D_A {
    #[doc = "0: Peripheral clock enabled."]
    ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    DISABLED = 1,
}
impl From<I2C1D_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1D_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1D_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1D_A {
        match self.bits {
            false => I2C1D_A::ENABLED,
            true => I2C1D_A::DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C1D_A::ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C1D_A::DISABLED
    }
}
#[doc = "Field `I2C1D` writer - I2C1 Clock Disable."]
pub type I2C1D_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C1D_A>;
impl<'a, REG, const O: u8> I2C1D_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1D_A::ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1D_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - GPIO0 Clock Disable."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Clock Disable."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI0 Clock Disable."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI1 Clock Disable."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - UART0 Clock Disable."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART1 Clock Disable."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C0 Clock Disable."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - TMR0 Clock Disable."]
    #[inline(always)]
    pub fn tmr0(&self) -> TMR0_R {
        TMR0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TMR1 Clock Disable."]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TMR2 Clock Disable."]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 28 - I2C1 Clock Disable."]
    #[inline(always)]
    pub fn i2c1d(&self) -> I2C1D_R {
        I2C1D_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO0_W<PCLK_DIS0_SPEC, 0> {
        GPIO0_W::new(self)
    }
    #[doc = "Bit 5 - DMA Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<PCLK_DIS0_SPEC, 5> {
        DMA_W::new(self)
    }
    #[doc = "Bit 6 - SPI0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<PCLK_DIS0_SPEC, 6> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 7 - SPI1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<PCLK_DIS0_SPEC, 7> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 9 - UART0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<PCLK_DIS0_SPEC, 9> {
        UART0_W::new(self)
    }
    #[doc = "Bit 10 - UART1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<PCLK_DIS0_SPEC, 10> {
        UART1_W::new(self)
    }
    #[doc = "Bit 13 - I2C0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<PCLK_DIS0_SPEC, 13> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 15 - TMR0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0(&mut self) -> TMR0_W<PCLK_DIS0_SPEC, 15> {
        TMR0_W::new(self)
    }
    #[doc = "Bit 16 - TMR1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<PCLK_DIS0_SPEC, 16> {
        TMR1_W::new(self)
    }
    #[doc = "Bit 17 - TMR2 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<PCLK_DIS0_SPEC, 17> {
        TMR2_W::new(self)
    }
    #[doc = "Bit 28 - I2C1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1d(&mut self) -> I2C1D_W<PCLK_DIS0_SPEC, 28> {
        I2C1D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral Clock Disable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclk_dis0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclk_dis0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCLK_DIS0_SPEC;
impl crate::RegisterSpec for PCLK_DIS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclk_dis0::R`](R) reader structure"]
impl crate::Readable for PCLK_DIS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pclk_dis0::W`](W) writer structure"]
impl crate::Writable for PCLK_DIS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLK_DIS0 to value 0"]
impl crate::Resettable for PCLK_DIS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
