#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<CTRL0_SPEC>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<CTRL0_SPEC>;
#[doc = "Field `enabled` reader - UART enabled, to enable UART block, it is used to drive a gated clock in order to save power consumption when UART is not used."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "UART enabled, to enable UART block, it is used to drive a gated clock in order to save power consumption when UART is not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLED_A {
    #[doc = "0: UART disabled."]
    DISABLED = 0,
    #[doc = "1: UART enabled."]
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
    #[doc = "UART disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLED_A::DISABLED
    }
    #[doc = "UART enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLED_A::ENABLED
    }
}
#[doc = "Field `enabled` writer - UART enabled, to enable UART block, it is used to drive a gated clock in order to save power consumption when UART is not used."]
pub type ENABLED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENABLED_A>;
impl<'a, REG, const O: u8> ENABLED_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "UART enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLED_A::ENABLED)
    }
}
#[doc = "Field `PARITY_EN` reader - Enable/disable Parity bit (9th character)."]
pub type PARITY_EN_R = crate::BitReader<PARITY_EN_A>;
#[doc = "Enable/disable Parity bit (9th character).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITY_EN_A {
    #[doc = "0: No Parity."]
    DISABLED = 0,
    #[doc = "1: Parity enabled as 9th bit."]
    ENABLED = 1,
}
impl From<PARITY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITY_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_EN_A {
        match self.bits {
            false => PARITY_EN_A::DISABLED,
            true => PARITY_EN_A::ENABLED,
        }
    }
    #[doc = "No Parity."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PARITY_EN_A::DISABLED
    }
    #[doc = "Parity enabled as 9th bit."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PARITY_EN_A::ENABLED
    }
}
#[doc = "Field `PARITY_EN` writer - Enable/disable Parity bit (9th character)."]
pub type PARITY_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PARITY_EN_A>;
impl<'a, REG, const O: u8> PARITY_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Parity."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_EN_A::DISABLED)
    }
    #[doc = "Parity enabled as 9th bit."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_EN_A::ENABLED)
    }
}
#[doc = "Field `PARITY_MODE` reader - When PARITY_EN=1, selects odd, even, Mark or Space parity."]
pub type PARITY_MODE_R = crate::FieldReader<PARITY_MODE_A>;
#[doc = "When PARITY_EN=1, selects odd, even, Mark or Space parity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARITY_MODE_A {
    #[doc = "0: Even parity selected."]
    EVEN = 0,
    #[doc = "1: Odd parity selected."]
    ODD = 1,
    #[doc = "2: Mark parity selected."]
    MARK = 2,
    #[doc = "3: Space parity selected."]
    SPACE = 3,
}
impl From<PARITY_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITY_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PARITY_MODE_A {
    type Ux = u8;
}
impl PARITY_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_MODE_A {
        match self.bits {
            0 => PARITY_MODE_A::EVEN,
            1 => PARITY_MODE_A::ODD,
            2 => PARITY_MODE_A::MARK,
            3 => PARITY_MODE_A::SPACE,
            _ => unreachable!(),
        }
    }
    #[doc = "Even parity selected."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY_MODE_A::EVEN
    }
    #[doc = "Odd parity selected."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY_MODE_A::ODD
    }
    #[doc = "Mark parity selected."]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == PARITY_MODE_A::MARK
    }
    #[doc = "Space parity selected."]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == PARITY_MODE_A::SPACE
    }
}
#[doc = "Field `PARITY_MODE` writer - When PARITY_EN=1, selects odd, even, Mark or Space parity."]
pub type PARITY_MODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PARITY_MODE_A>;
impl<'a, REG, const O: u8> PARITY_MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Even parity selected."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_MODE_A::EVEN)
    }
    #[doc = "Odd parity selected."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_MODE_A::ODD)
    }
    #[doc = "Mark parity selected."]
    #[inline(always)]
    pub fn mark(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_MODE_A::MARK)
    }
    #[doc = "Space parity selected."]
    #[inline(always)]
    pub fn space(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_MODE_A::SPACE)
    }
}
#[doc = "Field `PARITY_LVL` reader - Selects parity based on 1s or 0s count (when PARITY_EN=1)."]
pub type PARITY_LVL_R = crate::BitReader<PARITY_LVL_A>;
#[doc = "Selects parity based on 1s or 0s count (when PARITY_EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITY_LVL_A {
    #[doc = "0: Parity calculation is based on number of 1s in frame."]
    _1 = 0,
    #[doc = "1: Parity calculation is based on number of 0s in frame."]
    _0 = 1,
}
impl From<PARITY_LVL_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_LVL_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITY_LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_LVL_A {
        match self.bits {
            false => PARITY_LVL_A::_1,
            true => PARITY_LVL_A::_0,
        }
    }
    #[doc = "Parity calculation is based on number of 1s in frame."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PARITY_LVL_A::_1
    }
    #[doc = "Parity calculation is based on number of 0s in frame."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PARITY_LVL_A::_0
    }
}
#[doc = "Field `PARITY_LVL` writer - Selects parity based on 1s or 0s count (when PARITY_EN=1)."]
pub type PARITY_LVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PARITY_LVL_A>;
impl<'a, REG, const O: u8> PARITY_LVL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity calculation is based on number of 1s in frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_LVL_A::_1)
    }
    #[doc = "Parity calculation is based on number of 0s in frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_LVL_A::_0)
    }
}
#[doc = "Field `TXFLUSH` reader - Flushes the TX FIFO buffer."]
pub type TXFLUSH_R = crate::BitReader;
#[doc = "Field `TXFLUSH` writer - Flushes the TX FIFO buffer."]
pub type TXFLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFLUSH` reader - Flushes the RX FIFO buffer."]
pub type RXFLUSH_R = crate::BitReader;
#[doc = "Field `RXFLUSH` writer - Flushes the RX FIFO buffer."]
pub type RXFLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BITACC` reader - Bit accuracy selection."]
pub type BITACC_R = crate::BitReader<BITACC_A>;
#[doc = "Bit accuracy selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BITACC_A {
    #[doc = "0: Frame accuracy."]
    FRAME = 0,
    #[doc = "1: Bit accuracy."]
    BIT = 1,
}
impl From<BITACC_A> for bool {
    #[inline(always)]
    fn from(variant: BITACC_A) -> Self {
        variant as u8 != 0
    }
}
impl BITACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITACC_A {
        match self.bits {
            false => BITACC_A::FRAME,
            true => BITACC_A::BIT,
        }
    }
    #[doc = "Frame accuracy."]
    #[inline(always)]
    pub fn is_frame(&self) -> bool {
        *self == BITACC_A::FRAME
    }
    #[doc = "Bit accuracy."]
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        *self == BITACC_A::BIT
    }
}
#[doc = "Field `BITACC` writer - Bit accuracy selection."]
pub type BITACC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BITACC_A>;
impl<'a, REG, const O: u8> BITACC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frame accuracy."]
    #[inline(always)]
    pub fn frame(self) -> &'a mut crate::W<REG> {
        self.variant(BITACC_A::FRAME)
    }
    #[doc = "Bit accuracy."]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut crate::W<REG> {
        self.variant(BITACC_A::BIT)
    }
}
#[doc = "Field `SIZE` reader - Selects UART character size."]
pub type SIZE_R = crate::FieldReader<SIZE_A>;
#[doc = "Selects UART character size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: 5 bits."]
    _5 = 0,
    #[doc = "1: 6 bits."]
    _6 = 1,
    #[doc = "2: 7 bits."]
    _7 = 2,
    #[doc = "3: 8 bits."]
    _8 = 3,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SIZE_A {
    type Ux = u8;
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIZE_A {
        match self.bits {
            0 => SIZE_A::_5,
            1 => SIZE_A::_6,
            2 => SIZE_A::_7,
            3 => SIZE_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "5 bits."]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == SIZE_A::_5
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == SIZE_A::_6
    }
    #[doc = "7 bits."]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == SIZE_A::_7
    }
    #[doc = "8 bits."]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SIZE_A::_8
    }
}
#[doc = "Field `SIZE` writer - Selects UART character size."]
pub type SIZE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SIZE_A>;
impl<'a, REG, const O: u8> SIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 bits."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::_5)
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::_6)
    }
    #[doc = "7 bits."]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::_7)
    }
    #[doc = "8 bits."]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::_8)
    }
}
#[doc = "Field `STOP` reader - Selects the number of stop bits that will be generated."]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "Selects the number of stop bits that will be generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: 1 stop bit."]
    _1 = 0,
    #[doc = "1: 1."]
    _1_5 = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::_1,
            true => STOP_A::_1_5,
        }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOP_A::_1
    }
    #[doc = "1."]
    #[inline(always)]
    pub fn is_1_5(&self) -> bool {
        *self == STOP_A::_1_5
    }
}
#[doc = "Field `STOP` writer - Selects the number of stop bits that will be generated."]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STOP_A>;
impl<'a, REG, const O: u8> STOP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_1)
    }
    #[doc = "1."]
    #[inline(always)]
    pub fn _1_5(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_1_5)
    }
}
#[doc = "Field `FLOW` reader - Enables/disables hardware flow control."]
pub type FLOW_R = crate::BitReader<FLOW_A>;
#[doc = "Enables/disables hardware flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLOW_A {
    #[doc = "1: HW Flow Control with RTS/CTS enabled."]
    ENABLED = 1,
    #[doc = "0: HW Flow Control disabled."]
    DISABLED = 0,
}
impl From<FLOW_A> for bool {
    #[inline(always)]
    fn from(variant: FLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl FLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLOW_A {
        match self.bits {
            true => FLOW_A::ENABLED,
            false => FLOW_A::DISABLED,
        }
    }
    #[doc = "HW Flow Control with RTS/CTS enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLOW_A::ENABLED
    }
    #[doc = "HW Flow Control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLOW_A::DISABLED
    }
}
#[doc = "Field `FLOW` writer - Enables/disables hardware flow control."]
pub type FLOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLOW_A>;
impl<'a, REG, const O: u8> FLOW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HW Flow Control with RTS/CTS enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLOW_A::ENABLED)
    }
    #[doc = "HW Flow Control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLOW_A::DISABLED)
    }
}
#[doc = "Field `FLOWPOL` reader - RTS/CTS polarity."]
pub type FLOWPOL_R = crate::BitReader<FLOWPOL_A>;
#[doc = "RTS/CTS polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLOWPOL_A {
    #[doc = "0: RTS/CTS asserted is logic 0."]
    _0 = 0,
    #[doc = "1: RTS/CTS asserted is logic 1."]
    _1 = 1,
}
impl From<FLOWPOL_A> for bool {
    #[inline(always)]
    fn from(variant: FLOWPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl FLOWPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLOWPOL_A {
        match self.bits {
            false => FLOWPOL_A::_0,
            true => FLOWPOL_A::_1,
        }
    }
    #[doc = "RTS/CTS asserted is logic 0."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLOWPOL_A::_0
    }
    #[doc = "RTS/CTS asserted is logic 1."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLOWPOL_A::_1
    }
}
#[doc = "Field `FLOWPOL` writer - RTS/CTS polarity."]
pub type FLOWPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLOWPOL_A>;
impl<'a, REG, const O: u8> FLOWPOL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS/CTS asserted is logic 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLOWPOL_A::_0)
    }
    #[doc = "RTS/CTS asserted is logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLOWPOL_A::_1)
    }
}
#[doc = "Field `NULLMOD` reader - NULL Modem Support (RTS/CTS and TXD/RXD swap)."]
pub type NULLMOD_R = crate::BitReader<NULLMOD_A>;
#[doc = "NULL Modem Support (RTS/CTS and TXD/RXD swap).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NULLMOD_A {
    #[doc = "0: Direct convention."]
    DISABLED = 0,
    #[doc = "1: Null Modem Mode."]
    ENABLED = 1,
}
impl From<NULLMOD_A> for bool {
    #[inline(always)]
    fn from(variant: NULLMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl NULLMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NULLMOD_A {
        match self.bits {
            false => NULLMOD_A::DISABLED,
            true => NULLMOD_A::ENABLED,
        }
    }
    #[doc = "Direct convention."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NULLMOD_A::DISABLED
    }
    #[doc = "Null Modem Mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NULLMOD_A::ENABLED
    }
}
#[doc = "Field `NULLMOD` writer - NULL Modem Support (RTS/CTS and TXD/RXD swap)."]
pub type NULLMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NULLMOD_A>;
impl<'a, REG, const O: u8> NULLMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct convention."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NULLMOD_A::DISABLED)
    }
    #[doc = "Null Modem Mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NULLMOD_A::ENABLED)
    }
}
#[doc = "Field `BREAK` reader - Break control bit."]
pub type BREAK_R = crate::BitReader<BREAK_A>;
#[doc = "Break control bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREAK_A {
    #[doc = "0: Break characters are not generated."]
    DISABLED = 0,
    #[doc = "1: Break characters are sent (all the bits are at '0' including start/parity/stop)."]
    ENABLED = 1,
}
impl From<BREAK_A> for bool {
    #[inline(always)]
    fn from(variant: BREAK_A) -> Self {
        variant as u8 != 0
    }
}
impl BREAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREAK_A {
        match self.bits {
            false => BREAK_A::DISABLED,
            true => BREAK_A::ENABLED,
        }
    }
    #[doc = "Break characters are not generated."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BREAK_A::DISABLED
    }
    #[doc = "Break characters are sent (all the bits are at '0' including start/parity/stop)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BREAK_A::ENABLED
    }
}
#[doc = "Field `BREAK` writer - Break control bit."]
pub type BREAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BREAK_A>;
impl<'a, REG, const O: u8> BREAK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break characters are not generated."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BREAK_A::DISABLED)
    }
    #[doc = "Break characters are sent (all the bits are at '0' including start/parity/stop)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BREAK_A::ENABLED)
    }
}
#[doc = "Field `CLK_SEL` reader - Baud Rate Clock Source Select."]
pub type CLK_SEL_R = crate::BitReader<CLK_SEL_A>;
#[doc = "Baud Rate Clock Source Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_SEL_A {
    #[doc = "0: System clock."]
    SYSTEM = 0,
    #[doc = "1: Alternate 7.3727MHz internal clock."]
    ALTERNATE = 1,
}
impl From<CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SEL_A {
        match self.bits {
            false => CLK_SEL_A::SYSTEM,
            true => CLK_SEL_A::ALTERNATE,
        }
    }
    #[doc = "System clock."]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == CLK_SEL_A::SYSTEM
    }
    #[doc = "Alternate 7.3727MHz internal clock."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == CLK_SEL_A::ALTERNATE
    }
}
#[doc = "Field `CLK_SEL` writer - Baud Rate Clock Source Select."]
pub type CLK_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLK_SEL_A>;
impl<'a, REG, const O: u8> CLK_SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System clock."]
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SEL_A::SYSTEM)
    }
    #[doc = "Alternate 7.3727MHz internal clock."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SEL_A::ALTERNATE)
    }
}
#[doc = "Field `TO_CNT` reader - RX Time Out."]
pub type TO_CNT_R = crate::FieldReader;
#[doc = "Field `TO_CNT` writer - RX Time Out."]
pub type TO_CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - UART enabled, to enable UART block, it is used to drive a gated clock in order to save power consumption when UART is not used."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable/disable Parity bit (9th character)."]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - When PARITY_EN=1, selects odd, even, Mark or Space parity."]
    #[inline(always)]
    pub fn parity_mode(&self) -> PARITY_MODE_R {
        PARITY_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Selects parity based on 1s or 0s count (when PARITY_EN=1)."]
    #[inline(always)]
    pub fn parity_lvl(&self) -> PARITY_LVL_R {
        PARITY_LVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flushes the TX FIFO buffer."]
    #[inline(always)]
    pub fn txflush(&self) -> TXFLUSH_R {
        TXFLUSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Flushes the RX FIFO buffer."]
    #[inline(always)]
    pub fn rxflush(&self) -> RXFLUSH_R {
        RXFLUSH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit accuracy selection."]
    #[inline(always)]
    pub fn bitacc(&self) -> BITACC_R {
        BITACC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Selects UART character size."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Selects the number of stop bits that will be generated."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables/disables hardware flow control."]
    #[inline(always)]
    pub fn flow(&self) -> FLOW_R {
        FLOW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RTS/CTS polarity."]
    #[inline(always)]
    pub fn flowpol(&self) -> FLOWPOL_R {
        FLOWPOL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NULL Modem Support (RTS/CTS and TXD/RXD swap)."]
    #[inline(always)]
    pub fn nullmod(&self) -> NULLMOD_R {
        NULLMOD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Break control bit."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Baud Rate Clock Source Select."]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - RX Time Out."]
    #[inline(always)]
    pub fn to_cnt(&self) -> TO_CNT_R {
        TO_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UART enabled, to enable UART block, it is used to drive a gated clock in order to save power consumption when UART is not used."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CTRL0_SPEC, 0> {
        ENABLED_W::new(self)
    }
    #[doc = "Bit 1 - Enable/disable Parity bit (9th character)."]
    #[inline(always)]
    #[must_use]
    pub fn parity_en(&mut self) -> PARITY_EN_W<CTRL0_SPEC, 1> {
        PARITY_EN_W::new(self)
    }
    #[doc = "Bits 2:3 - When PARITY_EN=1, selects odd, even, Mark or Space parity."]
    #[inline(always)]
    #[must_use]
    pub fn parity_mode(&mut self) -> PARITY_MODE_W<CTRL0_SPEC, 2> {
        PARITY_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Selects parity based on 1s or 0s count (when PARITY_EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn parity_lvl(&mut self) -> PARITY_LVL_W<CTRL0_SPEC, 4> {
        PARITY_LVL_W::new(self)
    }
    #[doc = "Bit 5 - Flushes the TX FIFO buffer."]
    #[inline(always)]
    #[must_use]
    pub fn txflush(&mut self) -> TXFLUSH_W<CTRL0_SPEC, 5> {
        TXFLUSH_W::new(self)
    }
    #[doc = "Bit 6 - Flushes the RX FIFO buffer."]
    #[inline(always)]
    #[must_use]
    pub fn rxflush(&mut self) -> RXFLUSH_W<CTRL0_SPEC, 6> {
        RXFLUSH_W::new(self)
    }
    #[doc = "Bit 7 - Bit accuracy selection."]
    #[inline(always)]
    #[must_use]
    pub fn bitacc(&mut self) -> BITACC_W<CTRL0_SPEC, 7> {
        BITACC_W::new(self)
    }
    #[doc = "Bits 8:9 - Selects UART character size."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<CTRL0_SPEC, 8> {
        SIZE_W::new(self)
    }
    #[doc = "Bit 10 - Selects the number of stop bits that will be generated."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CTRL0_SPEC, 10> {
        STOP_W::new(self)
    }
    #[doc = "Bit 11 - Enables/disables hardware flow control."]
    #[inline(always)]
    #[must_use]
    pub fn flow(&mut self) -> FLOW_W<CTRL0_SPEC, 11> {
        FLOW_W::new(self)
    }
    #[doc = "Bit 12 - RTS/CTS polarity."]
    #[inline(always)]
    #[must_use]
    pub fn flowpol(&mut self) -> FLOWPOL_W<CTRL0_SPEC, 12> {
        FLOWPOL_W::new(self)
    }
    #[doc = "Bit 13 - NULL Modem Support (RTS/CTS and TXD/RXD swap)."]
    #[inline(always)]
    #[must_use]
    pub fn nullmod(&mut self) -> NULLMOD_W<CTRL0_SPEC, 13> {
        NULLMOD_W::new(self)
    }
    #[doc = "Bit 14 - Break control bit."]
    #[inline(always)]
    #[must_use]
    pub fn break_(&mut self) -> BREAK_W<CTRL0_SPEC, 14> {
        BREAK_W::new(self)
    }
    #[doc = "Bit 15 - Baud Rate Clock Source Select."]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<CTRL0_SPEC, 15> {
        CLK_SEL_W::new(self)
    }
    #[doc = "Bits 16:23 - RX Time Out."]
    #[inline(always)]
    #[must_use]
    pub fn to_cnt(&mut self) -> TO_CNT_W<CTRL0_SPEC, 16> {
        TO_CNT_W::new(self)
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
#[doc = "Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
