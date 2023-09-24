#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `enabled` reader - SPI Enable."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "SPI Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DISABLED,
            true => ENABLED_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLED_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLED_A::ENABLED
    }
}
#[doc = "Field `enabled` writer - SPI Enable."]
pub type ENABLED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENABLED_A>;
impl<'a, REG, const O: u8> ENABLED_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLED_A::ENABLED)
    }
}
#[doc = "Field `MMEN` reader - SPI Master Mode Enable."]
pub type MMEN_R = crate::BitReader<MMEN_A>;
#[doc = "SPI Master Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMEN_A {
    #[doc = "0: `0`"]
    SLAVE = 0,
    #[doc = "1: `1`"]
    MASTER = 1,
}
impl From<MMEN_A> for bool {
    #[inline(always)]
    fn from(variant: MMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMEN_A {
        match self.bits {
            false => MMEN_A::SLAVE,
            true => MMEN_A::MASTER,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MMEN_A::SLAVE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MMEN_A::MASTER
    }
}
#[doc = "Field `MMEN` writer - SPI Master Mode Enable."]
pub type MMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MMEN_A>;
impl<'a, REG, const O: u8> MMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MMEN_A::SLAVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(MMEN_A::MASTER)
    }
}
#[doc = "Field `WOR` reader - Wired OR (open drain) Enable."]
pub type WOR_R = crate::BitReader<WOR_A>;
#[doc = "Wired OR (open drain) Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<WOR_A> for bool {
    #[inline(always)]
    fn from(variant: WOR_A) -> Self {
        variant as u8 != 0
    }
}
impl WOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WOR_A {
        match self.bits {
            false => WOR_A::DISABLED,
            true => WOR_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WOR_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WOR_A::ENABLED
    }
}
#[doc = "Field `WOR` writer - Wired OR (open drain) Enable."]
pub type WOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WOR_A>;
impl<'a, REG, const O: u8> WOR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WOR_A::ENABLED)
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity."]
pub type CLKPOL_R = crate::BitReader<CLKPOL_A>;
#[doc = "Clock Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPOL_A {
    #[doc = "0: SCLK idles Low (0) after character transmission/reception."]
    IDLE_LOW = 0,
    #[doc = "1: SCLK idles High (1) after character transmission/reception."]
    IDLE_HIGH = 1,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKPOL_A {
        match self.bits {
            false => CLKPOL_A::IDLE_LOW,
            true => CLKPOL_A::IDLE_HIGH,
        }
    }
    #[doc = "SCLK idles Low (0) after character transmission/reception."]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CLKPOL_A::IDLE_LOW
    }
    #[doc = "SCLK idles High (1) after character transmission/reception."]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CLKPOL_A::IDLE_HIGH
    }
}
#[doc = "Field `CLKPOL` writer - Clock Polarity."]
pub type CLKPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLKPOL_A>;
impl<'a, REG, const O: u8> CLKPOL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCLK idles Low (0) after character transmission/reception."]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::IDLE_LOW)
    }
    #[doc = "SCLK idles High (1) after character transmission/reception."]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::IDLE_HIGH)
    }
}
#[doc = "Field `PHASE` reader - Phase Select."]
pub type PHASE_R = crate::BitReader<PHASE_A>;
#[doc = "Phase Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHASE_A {
    #[doc = "0: Transmit on active edge of SCLK."]
    ACTIVE_EDGE = 0,
    #[doc = "1: Transmit on inactive edge of SCLK."]
    INACTIVE_EDGE = 1,
}
impl From<PHASE_A> for bool {
    #[inline(always)]
    fn from(variant: PHASE_A) -> Self {
        variant as u8 != 0
    }
}
impl PHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHASE_A {
        match self.bits {
            false => PHASE_A::ACTIVE_EDGE,
            true => PHASE_A::INACTIVE_EDGE,
        }
    }
    #[doc = "Transmit on active edge of SCLK."]
    #[inline(always)]
    pub fn is_active_edge(&self) -> bool {
        *self == PHASE_A::ACTIVE_EDGE
    }
    #[doc = "Transmit on inactive edge of SCLK."]
    #[inline(always)]
    pub fn is_inactive_edge(&self) -> bool {
        *self == PHASE_A::INACTIVE_EDGE
    }
}
#[doc = "Field `PHASE` writer - Phase Select."]
pub type PHASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PHASE_A>;
impl<'a, REG, const O: u8> PHASE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit on active edge of SCLK."]
    #[inline(always)]
    pub fn active_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PHASE_A::ACTIVE_EDGE)
    }
    #[doc = "Transmit on inactive edge of SCLK."]
    #[inline(always)]
    pub fn inactive_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PHASE_A::INACTIVE_EDGE)
    }
}
#[doc = "Field `BIRQ` reader - Baud Rate Generator Timer Interrupt Request."]
pub type BIRQ_R = crate::BitReader<BIRQ_A>;
#[doc = "Baud Rate Generator Timer Interrupt Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIRQ_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<BIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: BIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl BIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIRQ_A {
        match self.bits {
            false => BIRQ_A::DISABLED,
            true => BIRQ_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BIRQ_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BIRQ_A::ENABLED
    }
}
#[doc = "Field `BIRQ` writer - Baud Rate Generator Timer Interrupt Request."]
pub type BIRQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BIRQ_A>;
impl<'a, REG, const O: u8> BIRQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BIRQ_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BIRQ_A::ENABLED)
    }
}
#[doc = "Field `STR` reader - Start SPI Interrupt."]
pub type STR_R = crate::BitReader<STR_A>;
#[doc = "Start SPI Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STR_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<STR_A> for bool {
    #[inline(always)]
    fn from(variant: STR_A) -> Self {
        variant as u8 != 0
    }
}
impl STR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STR_A {
        match self.bits {
            false => STR_A::COMPLETE,
            true => STR_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == STR_A::COMPLETE
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STR_A::START
    }
}
#[doc = "Field `STR` writer - Start SPI Interrupt."]
pub type STR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STR_A>;
impl<'a, REG, const O: u8> STR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(STR_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STR_A::START)
    }
}
#[doc = "Field `IRQE` reader - Interrupt Request Enable."]
pub type IRQE_R = crate::BitReader<IRQE_A>;
#[doc = "Interrupt Request Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<IRQE_A> for bool {
    #[inline(always)]
    fn from(variant: IRQE_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQE_A {
        match self.bits {
            false => IRQE_A::DISABLED,
            true => IRQE_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRQE_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRQE_A::ENABLED
    }
}
#[doc = "Field `IRQE` writer - Interrupt Request Enable."]
pub type IRQE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRQE_A>;
impl<'a, REG, const O: u8> IRQE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRQE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRQE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Master Mode Enable."]
    #[inline(always)]
    pub fn mmen(&self) -> MMEN_R {
        MMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wired OR (open drain) Enable."]
    #[inline(always)]
    pub fn wor(&self) -> WOR_R {
        WOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Polarity."]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Phase Select."]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Baud Rate Generator Timer Interrupt Request."]
    #[inline(always)]
    pub fn birq(&self) -> BIRQ_R {
        BIRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start SPI Interrupt."]
    #[inline(always)]
    pub fn str(&self) -> STR_R {
        STR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Request Enable."]
    #[inline(always)]
    pub fn irqe(&self) -> IRQE_R {
        IRQE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CTRL_SPEC, 0> {
        ENABLED_W::new(self)
    }
    #[doc = "Bit 1 - SPI Master Mode Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mmen(&mut self) -> MMEN_W<CTRL_SPEC, 1> {
        MMEN_W::new(self)
    }
    #[doc = "Bit 2 - Wired OR (open drain) Enable."]
    #[inline(always)]
    #[must_use]
    pub fn wor(&mut self) -> WOR_W<CTRL_SPEC, 2> {
        WOR_W::new(self)
    }
    #[doc = "Bit 3 - Clock Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<CTRL_SPEC, 3> {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 4 - Phase Select."]
    #[inline(always)]
    #[must_use]
    pub fn phase(&mut self) -> PHASE_W<CTRL_SPEC, 4> {
        PHASE_W::new(self)
    }
    #[doc = "Bit 5 - Baud Rate Generator Timer Interrupt Request."]
    #[inline(always)]
    #[must_use]
    pub fn birq(&mut self) -> BIRQ_W<CTRL_SPEC, 5> {
        BIRQ_W::new(self)
    }
    #[doc = "Bit 6 - Start SPI Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn str(&mut self) -> STR_W<CTRL_SPEC, 6> {
        STR_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Request Enable."]
    #[inline(always)]
    #[must_use]
    pub fn irqe(&mut self) -> IRQE_W<CTRL_SPEC, 7> {
        IRQE_W::new(self)
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
#[doc = "SPI Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
