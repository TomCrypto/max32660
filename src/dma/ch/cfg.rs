#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `CHEN` reader - Channel Enable."]
pub type CHEN_R = crate::BitReader<CHEN_A>;
#[doc = "Channel Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHEN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<CHEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN_A {
        match self.bits {
            false => CHEN_A::DISABLED,
            true => CHEN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHEN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHEN_A::ENABLED
    }
}
#[doc = "Field `CHEN` writer - Channel Enable."]
pub type CHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHEN_A>;
impl<'a, REG, const O: u8> CHEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHEN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHEN_A::ENABLED)
    }
}
#[doc = "Field `RLDEN` reader - Reload Enable."]
pub type RLDEN_R = crate::BitReader<RLDEN_A>;
#[doc = "Reload Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLDEN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RLDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLDEN_A {
        match self.bits {
            false => RLDEN_A::DISABLED,
            true => RLDEN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RLDEN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RLDEN_A::ENABLED
    }
}
#[doc = "Field `RLDEN` writer - Reload Enable."]
pub type RLDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RLDEN_A>;
impl<'a, REG, const O: u8> RLDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RLDEN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RLDEN_A::ENABLED)
    }
}
#[doc = "Field `PRI` reader - DMA Priority."]
pub type PRI_R = crate::FieldReader<PRI_A>;
#[doc = "DMA Priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRI_A {
    #[doc = "0: High priority."]
    HIGH = 0,
    #[doc = "1: Medium-high priority."]
    MEDIUM_HIGH = 1,
    #[doc = "2: Medium-low priority."]
    MEDIUM_LOW = 2,
    #[doc = "3: Low priority."]
    LOW = 3,
}
impl From<PRI_A> for u8 {
    #[inline(always)]
    fn from(variant: PRI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRI_A {
    type Ux = u8;
}
impl PRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRI_A {
        match self.bits {
            0 => PRI_A::HIGH,
            1 => PRI_A::MEDIUM_HIGH,
            2 => PRI_A::MEDIUM_LOW,
            3 => PRI_A::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "High priority."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PRI_A::HIGH
    }
    #[doc = "Medium-high priority."]
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == PRI_A::MEDIUM_HIGH
    }
    #[doc = "Medium-low priority."]
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == PRI_A::MEDIUM_LOW
    }
    #[doc = "Low priority."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PRI_A::LOW
    }
}
#[doc = "Field `PRI` writer - DMA Priority."]
pub type PRI_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PRI_A>;
impl<'a, REG, const O: u8> PRI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High priority."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::HIGH)
    }
    #[doc = "Medium-high priority."]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::MEDIUM_HIGH)
    }
    #[doc = "Medium-low priority."]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::MEDIUM_LOW)
    }
    #[doc = "Low priority."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::LOW)
    }
}
#[doc = "Field `REQSEL` reader - Request Select."]
pub type REQSEL_R = crate::FieldReader<REQSEL_A>;
#[doc = "Request Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REQSEL_A {
    #[doc = "0: Memory-to-memory."]
    MEM_TO_MEM = 0,
    #[doc = "1: SPI0 RX."]
    SPI0_RX = 1,
    #[doc = "2: SPI1 RX."]
    SPI1_RX = 2,
    #[doc = "4: UART0 RX."]
    UART0_RX = 4,
    #[doc = "5: UART1 RX."]
    UART1_RX = 5,
    #[doc = "7: I2C0 RX."]
    I2C0_RX = 7,
    #[doc = "8: I2C1 RX."]
    I2C1_RX = 8,
    #[doc = "33: SPI0 TX."]
    SPI0_TX = 33,
    #[doc = "34: SPI1 TX."]
    SPI1_TX = 34,
    #[doc = "36: UART0 TX."]
    UART0_TX = 36,
    #[doc = "37: UART1 TX."]
    UART1_TX = 37,
    #[doc = "39: I2C0 TX."]
    I2C0_TX = 39,
    #[doc = "40: I2C1 TX."]
    I2C1_TX = 40,
}
impl From<REQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REQSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REQSEL_A {
    type Ux = u8;
}
impl REQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REQSEL_A> {
        match self.bits {
            0 => Some(REQSEL_A::MEM_TO_MEM),
            1 => Some(REQSEL_A::SPI0_RX),
            2 => Some(REQSEL_A::SPI1_RX),
            4 => Some(REQSEL_A::UART0_RX),
            5 => Some(REQSEL_A::UART1_RX),
            7 => Some(REQSEL_A::I2C0_RX),
            8 => Some(REQSEL_A::I2C1_RX),
            33 => Some(REQSEL_A::SPI0_TX),
            34 => Some(REQSEL_A::SPI1_TX),
            36 => Some(REQSEL_A::UART0_TX),
            37 => Some(REQSEL_A::UART1_TX),
            39 => Some(REQSEL_A::I2C0_TX),
            40 => Some(REQSEL_A::I2C1_TX),
            _ => None,
        }
    }
    #[doc = "Memory-to-memory."]
    #[inline(always)]
    pub fn is_mem_to_mem(&self) -> bool {
        *self == REQSEL_A::MEM_TO_MEM
    }
    #[doc = "SPI0 RX."]
    #[inline(always)]
    pub fn is_spi0_rx(&self) -> bool {
        *self == REQSEL_A::SPI0_RX
    }
    #[doc = "SPI1 RX."]
    #[inline(always)]
    pub fn is_spi1_rx(&self) -> bool {
        *self == REQSEL_A::SPI1_RX
    }
    #[doc = "UART0 RX."]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == REQSEL_A::UART0_RX
    }
    #[doc = "UART1 RX."]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == REQSEL_A::UART1_RX
    }
    #[doc = "I2C0 RX."]
    #[inline(always)]
    pub fn is_i2c0_rx(&self) -> bool {
        *self == REQSEL_A::I2C0_RX
    }
    #[doc = "I2C1 RX."]
    #[inline(always)]
    pub fn is_i2c1_rx(&self) -> bool {
        *self == REQSEL_A::I2C1_RX
    }
    #[doc = "SPI0 TX."]
    #[inline(always)]
    pub fn is_spi0_tx(&self) -> bool {
        *self == REQSEL_A::SPI0_TX
    }
    #[doc = "SPI1 TX."]
    #[inline(always)]
    pub fn is_spi1_tx(&self) -> bool {
        *self == REQSEL_A::SPI1_TX
    }
    #[doc = "UART0 TX."]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == REQSEL_A::UART0_TX
    }
    #[doc = "UART1 TX."]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == REQSEL_A::UART1_TX
    }
    #[doc = "I2C0 TX."]
    #[inline(always)]
    pub fn is_i2c0_tx(&self) -> bool {
        *self == REQSEL_A::I2C0_TX
    }
    #[doc = "I2C1 TX."]
    #[inline(always)]
    pub fn is_i2c1_tx(&self) -> bool {
        *self == REQSEL_A::I2C1_TX
    }
}
#[doc = "Field `REQSEL` writer - Request Select."]
pub type REQSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, REQSEL_A>;
impl<'a, REG, const O: u8> REQSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory-to-memory."]
    #[inline(always)]
    pub fn mem_to_mem(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::MEM_TO_MEM)
    }
    #[doc = "SPI0 RX."]
    #[inline(always)]
    pub fn spi0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::SPI0_RX)
    }
    #[doc = "SPI1 RX."]
    #[inline(always)]
    pub fn spi1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::SPI1_RX)
    }
    #[doc = "UART0 RX."]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::UART0_RX)
    }
    #[doc = "UART1 RX."]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::UART1_RX)
    }
    #[doc = "I2C0 RX."]
    #[inline(always)]
    pub fn i2c0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::I2C0_RX)
    }
    #[doc = "I2C1 RX."]
    #[inline(always)]
    pub fn i2c1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::I2C1_RX)
    }
    #[doc = "SPI0 TX."]
    #[inline(always)]
    pub fn spi0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::SPI0_TX)
    }
    #[doc = "SPI1 TX."]
    #[inline(always)]
    pub fn spi1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::SPI1_TX)
    }
    #[doc = "UART0 TX."]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::UART0_TX)
    }
    #[doc = "UART1 TX."]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::UART1_TX)
    }
    #[doc = "I2C0 TX."]
    #[inline(always)]
    pub fn i2c0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::I2C0_TX)
    }
    #[doc = "I2C1 TX."]
    #[inline(always)]
    pub fn i2c1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL_A::I2C1_TX)
    }
}
#[doc = "Field `REQWAIT` reader - Request Wait Enable."]
pub type REQWAIT_R = crate::BitReader<REQWAIT_A>;
#[doc = "Request Wait Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQWAIT_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<REQWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: REQWAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl REQWAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQWAIT_A {
        match self.bits {
            false => REQWAIT_A::DISABLED,
            true => REQWAIT_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REQWAIT_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REQWAIT_A::ENABLED
    }
}
#[doc = "Field `REQWAIT` writer - Request Wait Enable."]
pub type REQWAIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, REQWAIT_A>;
impl<'a, REG, const O: u8> REQWAIT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REQWAIT_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REQWAIT_A::ENABLED)
    }
}
#[doc = "Field `TOSEL` reader - Time-Out Select."]
pub type TOSEL_R = crate::FieldReader<TOSEL_A>;
#[doc = "Time-Out Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOSEL_A {
    #[doc = "0: Timeout of 3 to 4 prescale clocks."]
    TO4 = 0,
    #[doc = "1: Timeout of 7 to 8 prescale clocks."]
    TO8 = 1,
    #[doc = "2: Timeout of 15 to 16 prescale clocks."]
    TO16 = 2,
    #[doc = "3: Timeout of 31 to 32 prescale clocks."]
    TO32 = 3,
    #[doc = "4: Timeout of 63 to 64 prescale clocks."]
    TO64 = 4,
    #[doc = "5: Timeout of 127 to 128 prescale clocks."]
    TO128 = 5,
    #[doc = "6: Timeout of 255 to 256 prescale clocks."]
    TO256 = 6,
    #[doc = "7: Timeout of 511 to 512 prescale clocks."]
    TO512 = 7,
}
impl From<TOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TOSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TOSEL_A {
    type Ux = u8;
}
impl TOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOSEL_A {
        match self.bits {
            0 => TOSEL_A::TO4,
            1 => TOSEL_A::TO8,
            2 => TOSEL_A::TO16,
            3 => TOSEL_A::TO32,
            4 => TOSEL_A::TO64,
            5 => TOSEL_A::TO128,
            6 => TOSEL_A::TO256,
            7 => TOSEL_A::TO512,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout of 3 to 4 prescale clocks."]
    #[inline(always)]
    pub fn is_to4(&self) -> bool {
        *self == TOSEL_A::TO4
    }
    #[doc = "Timeout of 7 to 8 prescale clocks."]
    #[inline(always)]
    pub fn is_to8(&self) -> bool {
        *self == TOSEL_A::TO8
    }
    #[doc = "Timeout of 15 to 16 prescale clocks."]
    #[inline(always)]
    pub fn is_to16(&self) -> bool {
        *self == TOSEL_A::TO16
    }
    #[doc = "Timeout of 31 to 32 prescale clocks."]
    #[inline(always)]
    pub fn is_to32(&self) -> bool {
        *self == TOSEL_A::TO32
    }
    #[doc = "Timeout of 63 to 64 prescale clocks."]
    #[inline(always)]
    pub fn is_to64(&self) -> bool {
        *self == TOSEL_A::TO64
    }
    #[doc = "Timeout of 127 to 128 prescale clocks."]
    #[inline(always)]
    pub fn is_to128(&self) -> bool {
        *self == TOSEL_A::TO128
    }
    #[doc = "Timeout of 255 to 256 prescale clocks."]
    #[inline(always)]
    pub fn is_to256(&self) -> bool {
        *self == TOSEL_A::TO256
    }
    #[doc = "Timeout of 511 to 512 prescale clocks."]
    #[inline(always)]
    pub fn is_to512(&self) -> bool {
        *self == TOSEL_A::TO512
    }
}
#[doc = "Field `TOSEL` writer - Time-Out Select."]
pub type TOSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, TOSEL_A>;
impl<'a, REG, const O: u8> TOSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout of 3 to 4 prescale clocks."]
    #[inline(always)]
    pub fn to4(self) -> &'a mut crate::W<REG> {
        self.variant(TOSEL_A::TO4)
    }
    #[doc = "Timeout of 7 to 8 prescale clocks."]
    #[inline(always)]
    pub fn to8(self) -> &'a mut crate::W<REG> {
        self.variant(TOSEL_A::TO8)
    }
    #[doc = "Timeout of 15 to 16 prescale clocks."]
    #[inline(always)]
    pub fn to16(self) -> &'a mut crate::W<REG> {
        self.variant(TOSEL_A::TO16)
    }
    #[doc = "Timeout of 31 to 32 prescale clocks."]
    #[inline(always)]
    pub fn to32(self) -> &'a mut crate::W<REG> {
        self.variant(TOSEL_A::TO32)
    }
    #[doc = "Timeout of 63 to 64 prescale clocks."]
    #[inline(always)]
    pub fn to64(self) -> &'a mut crate::W<REG> {
        self.variant(TOSEL_A::TO64)
    }
    #[doc = "Timeout of 127 to 128 prescale clocks."]
    #[inline(always)]
    pub fn to128(self) -> &'a mut crate::W<REG> {
        self.variant(TOSEL_A::TO128)
    }
    #[doc = "Timeout of 255 to 256 prescale clocks."]
    #[inline(always)]
    pub fn to256(self) -> &'a mut crate::W<REG> {
        self.variant(TOSEL_A::TO256)
    }
    #[doc = "Timeout of 511 to 512 prescale clocks."]
    #[inline(always)]
    pub fn to512(self) -> &'a mut crate::W<REG> {
        self.variant(TOSEL_A::TO512)
    }
}
#[doc = "Field `PSSEL` reader - Pre-Scale Select."]
pub type PSSEL_R = crate::FieldReader<PSSEL_A>;
#[doc = "Pre-Scale Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSSEL_A {
    #[doc = "0: Disable timer."]
    DISABLED = 0,
    #[doc = "1: Divide by 256."]
    DIV_256 = 1,
    #[doc = "2: Divide by 64K."]
    DIV_64K = 2,
    #[doc = "3: Divide by 16M."]
    DIV_16M = 3,
}
impl From<PSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSSEL_A {
    type Ux = u8;
}
impl PSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSSEL_A {
        match self.bits {
            0 => PSSEL_A::DISABLED,
            1 => PSSEL_A::DIV_256,
            2 => PSSEL_A::DIV_64K,
            3 => PSSEL_A::DIV_16M,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable timer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PSSEL_A::DISABLED
    }
    #[doc = "Divide by 256."]
    #[inline(always)]
    pub fn is_div_256(&self) -> bool {
        *self == PSSEL_A::DIV_256
    }
    #[doc = "Divide by 64K."]
    #[inline(always)]
    pub fn is_div_64k(&self) -> bool {
        *self == PSSEL_A::DIV_64K
    }
    #[doc = "Divide by 16M."]
    #[inline(always)]
    pub fn is_div_16m(&self) -> bool {
        *self == PSSEL_A::DIV_16M
    }
}
#[doc = "Field `PSSEL` writer - Pre-Scale Select."]
pub type PSSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PSSEL_A>;
impl<'a, REG, const O: u8> PSSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable timer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSSEL_A::DISABLED)
    }
    #[doc = "Divide by 256."]
    #[inline(always)]
    pub fn div_256(self) -> &'a mut crate::W<REG> {
        self.variant(PSSEL_A::DIV_256)
    }
    #[doc = "Divide by 64K."]
    #[inline(always)]
    pub fn div_64k(self) -> &'a mut crate::W<REG> {
        self.variant(PSSEL_A::DIV_64K)
    }
    #[doc = "Divide by 16M."]
    #[inline(always)]
    pub fn div_16m(self) -> &'a mut crate::W<REG> {
        self.variant(PSSEL_A::DIV_16M)
    }
}
#[doc = "Field `SRCWD` reader - Source Width."]
pub type SRCWD_R = crate::FieldReader<SRCWD_A>;
#[doc = "Source Width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Half-word."]
    HALF_WORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<SRCWD_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCWD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRCWD_A {
    type Ux = u8;
}
impl SRCWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRCWD_A> {
        match self.bits {
            0 => Some(SRCWD_A::BYTE),
            1 => Some(SRCWD_A::HALF_WORD),
            2 => Some(SRCWD_A::WORD),
            _ => None,
        }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SRCWD_A::BYTE
    }
    #[doc = "Half-word."]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SRCWD_A::HALF_WORD
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SRCWD_A::WORD
    }
}
#[doc = "Field `SRCWD` writer - Source Width."]
pub type SRCWD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SRCWD_A>;
impl<'a, REG, const O: u8> SRCWD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(SRCWD_A::BYTE)
    }
    #[doc = "Half-word."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(SRCWD_A::HALF_WORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(SRCWD_A::WORD)
    }
}
#[doc = "Field `SRCINC` reader - Source Increment Enable."]
pub type SRCINC_R = crate::BitReader<SRCINC_A>;
#[doc = "Source Increment Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCINC_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<SRCINC_A> for bool {
    #[inline(always)]
    fn from(variant: SRCINC_A) -> Self {
        variant as u8 != 0
    }
}
impl SRCINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCINC_A {
        match self.bits {
            false => SRCINC_A::DISABLED,
            true => SRCINC_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRCINC_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRCINC_A::ENABLED
    }
}
#[doc = "Field `SRCINC` writer - Source Increment Enable."]
pub type SRCINC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRCINC_A>;
impl<'a, REG, const O: u8> SRCINC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC_A::ENABLED)
    }
}
#[doc = "Field `DSTWD` reader - Destination Width."]
pub type DSTWD_R = crate::FieldReader<DSTWD_A>;
#[doc = "Destination Width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSTWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Half-word."]
    HALF_WORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<DSTWD_A> for u8 {
    #[inline(always)]
    fn from(variant: DSTWD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSTWD_A {
    type Ux = u8;
}
impl DSTWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSTWD_A> {
        match self.bits {
            0 => Some(DSTWD_A::BYTE),
            1 => Some(DSTWD_A::HALF_WORD),
            2 => Some(DSTWD_A::WORD),
            _ => None,
        }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DSTWD_A::BYTE
    }
    #[doc = "Half-word."]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DSTWD_A::HALF_WORD
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DSTWD_A::WORD
    }
}
#[doc = "Field `DSTWD` writer - Destination Width."]
pub type DSTWD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DSTWD_A>;
impl<'a, REG, const O: u8> DSTWD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(DSTWD_A::BYTE)
    }
    #[doc = "Half-word."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(DSTWD_A::HALF_WORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(DSTWD_A::WORD)
    }
}
#[doc = "Field `DSTINC` reader - Destination Increment Enable."]
pub type DSTINC_R = crate::BitReader<DSTINC_A>;
#[doc = "Destination Increment Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSTINC_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<DSTINC_A> for bool {
    #[inline(always)]
    fn from(variant: DSTINC_A) -> Self {
        variant as u8 != 0
    }
}
impl DSTINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTINC_A {
        match self.bits {
            false => DSTINC_A::DISABLED,
            true => DSTINC_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSTINC_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSTINC_A::ENABLED
    }
}
#[doc = "Field `DSTINC` writer - Destination Increment Enable."]
pub type DSTINC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DSTINC_A>;
impl<'a, REG, const O: u8> DSTINC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC_A::ENABLED)
    }
}
#[doc = "Field `BRST` reader - Burst Size."]
pub type BRST_R = crate::FieldReader;
#[doc = "Field `BRST` writer - Burst Size."]
pub type BRST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CHDIEN` reader - Channel Disable Interrupt Enable."]
pub type CHDIEN_R = crate::BitReader<CHDIEN_A>;
#[doc = "Channel Disable Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHDIEN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<CHDIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHDIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHDIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHDIEN_A {
        match self.bits {
            false => CHDIEN_A::DISABLED,
            true => CHDIEN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHDIEN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHDIEN_A::ENABLED
    }
}
#[doc = "Field `CHDIEN` writer - Channel Disable Interrupt Enable."]
pub type CHDIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHDIEN_A>;
impl<'a, REG, const O: u8> CHDIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHDIEN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHDIEN_A::ENABLED)
    }
}
#[doc = "Field `CTZIEN` reader - Count-to-zero Interrupts Enable."]
pub type CTZIEN_R = crate::BitReader<CTZIEN_A>;
#[doc = "Count-to-zero Interrupts Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTZIEN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<CTZIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTZIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTZIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTZIEN_A {
        match self.bits {
            false => CTZIEN_A::DISABLED,
            true => CTZIEN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTZIEN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTZIEN_A::ENABLED
    }
}
#[doc = "Field `CTZIEN` writer - Count-to-zero Interrupts Enable."]
pub type CTZIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTZIEN_A>;
impl<'a, REG, const O: u8> CTZIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTZIEN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTZIEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Enable."]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reload Enable."]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:9 - Request Select."]
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - Request Wait Enable."]
    #[inline(always)]
    pub fn reqwait(&self) -> REQWAIT_R {
        REQWAIT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Time-Out Select."]
    #[inline(always)]
    pub fn tosel(&self) -> TOSEL_R {
        TOSEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select."]
    #[inline(always)]
    pub fn pssel(&self) -> PSSEL_R {
        PSSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Source Width."]
    #[inline(always)]
    pub fn srcwd(&self) -> SRCWD_R {
        SRCWD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Source Increment Enable."]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Destination Width."]
    #[inline(always)]
    pub fn dstwd(&self) -> DSTWD_R {
        DSTWD_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Destination Increment Enable."]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Burst Size."]
    #[inline(always)]
    pub fn brst(&self) -> BRST_R {
        BRST_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable."]
    #[inline(always)]
    pub fn chdien(&self) -> CHDIEN_R {
        CHDIEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable."]
    #[inline(always)]
    pub fn ctzien(&self) -> CTZIEN_R {
        CTZIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable."]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<CFG_SPEC, 0> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 1 - Reload Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rlden(&mut self) -> RLDEN_W<CFG_SPEC, 1> {
        RLDEN_W::new(self)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PRI_W<CFG_SPEC, 2> {
        PRI_W::new(self)
    }
    #[doc = "Bits 4:9 - Request Select."]
    #[inline(always)]
    #[must_use]
    pub fn reqsel(&mut self) -> REQSEL_W<CFG_SPEC, 4> {
        REQSEL_W::new(self)
    }
    #[doc = "Bit 10 - Request Wait Enable."]
    #[inline(always)]
    #[must_use]
    pub fn reqwait(&mut self) -> REQWAIT_W<CFG_SPEC, 10> {
        REQWAIT_W::new(self)
    }
    #[doc = "Bits 11:13 - Time-Out Select."]
    #[inline(always)]
    #[must_use]
    pub fn tosel(&mut self) -> TOSEL_W<CFG_SPEC, 11> {
        TOSEL_W::new(self)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select."]
    #[inline(always)]
    #[must_use]
    pub fn pssel(&mut self) -> PSSEL_W<CFG_SPEC, 14> {
        PSSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Source Width."]
    #[inline(always)]
    #[must_use]
    pub fn srcwd(&mut self) -> SRCWD_W<CFG_SPEC, 16> {
        SRCWD_W::new(self)
    }
    #[doc = "Bit 18 - Source Increment Enable."]
    #[inline(always)]
    #[must_use]
    pub fn srcinc(&mut self) -> SRCINC_W<CFG_SPEC, 18> {
        SRCINC_W::new(self)
    }
    #[doc = "Bits 20:21 - Destination Width."]
    #[inline(always)]
    #[must_use]
    pub fn dstwd(&mut self) -> DSTWD_W<CFG_SPEC, 20> {
        DSTWD_W::new(self)
    }
    #[doc = "Bit 22 - Destination Increment Enable."]
    #[inline(always)]
    #[must_use]
    pub fn dstinc(&mut self) -> DSTINC_W<CFG_SPEC, 22> {
        DSTINC_W::new(self)
    }
    #[doc = "Bits 24:28 - Burst Size."]
    #[inline(always)]
    #[must_use]
    pub fn brst(&mut self) -> BRST_W<CFG_SPEC, 24> {
        BRST_W::new(self)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn chdien(&mut self) -> CHDIEN_W<CFG_SPEC, 30> {
        CHDIEN_W::new(self)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ctzien(&mut self) -> CTZIEN_W<CFG_SPEC, 31> {
        CTZIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Channel Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
