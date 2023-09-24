#[doc = "Register `INTEN0` reader"]
pub type R = crate::R<INTEN0_SPEC>;
#[doc = "Register `INTEN0` writer"]
pub type W = crate::W<INTEN0_SPEC>;
#[doc = "Field `DONEIE` reader - Transfer Done Interrupt Enable."]
pub type DONEIE_R = crate::BitReader<DONEIE_A>;
#[doc = "Transfer Done Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONEIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled when DONE = 1."]
    ENABLED = 1,
}
impl From<DONEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DONEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONEIE_A {
        match self.bits {
            false => DONEIE_A::DISABLED,
            true => DONEIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DONEIE_A::DISABLED
    }
    #[doc = "Interrupt enabled when DONE = 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DONEIE_A::ENABLED
    }
}
#[doc = "Field `DONEIE` writer - Transfer Done Interrupt Enable."]
pub type DONEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DONEIE_A>;
impl<'a, REG, const O: u8> DONEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DONEIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled when DONE = 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DONEIE_A::ENABLED)
    }
}
#[doc = "Field `IRXMIE` reader - Description not available."]
pub type IRXMIE_R = crate::BitReader<IRXMIE_A>;
#[doc = "Description not available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRXMIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled when RX_MODE = 1."]
    ENABLED = 1,
}
impl From<IRXMIE_A> for bool {
    #[inline(always)]
    fn from(variant: IRXMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl IRXMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRXMIE_A {
        match self.bits {
            false => IRXMIE_A::DISABLED,
            true => IRXMIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRXMIE_A::DISABLED
    }
    #[doc = "Interrupt enabled when RX_MODE = 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRXMIE_A::ENABLED
    }
}
#[doc = "Field `IRXMIE` writer - Description not available."]
pub type IRXMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRXMIE_A>;
impl<'a, REG, const O: u8> IRXMIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRXMIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled when RX_MODE = 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRXMIE_A::ENABLED)
    }
}
#[doc = "Field `GCIE` reader - Slave mode general call address match received input enable."]
pub type GCIE_R = crate::BitReader<GCIE_A>;
#[doc = "Slave mode general call address match received input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled when GEN_CTRL_ADDR = 1."]
    ENABLED = 1,
}
impl From<GCIE_A> for bool {
    #[inline(always)]
    fn from(variant: GCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl GCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCIE_A {
        match self.bits {
            false => GCIE_A::DISABLED,
            true => GCIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCIE_A::DISABLED
    }
    #[doc = "Interrupt enabled when GEN_CTRL_ADDR = 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCIE_A::ENABLED
    }
}
#[doc = "Field `GCIE` writer - Slave mode general call address match received input enable."]
pub type GCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GCIE_A>;
impl<'a, REG, const O: u8> GCIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled when GEN_CTRL_ADDR = 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCIE_A::ENABLED)
    }
}
#[doc = "Field `AMIE` reader - Slave mode incoming address match interrupt."]
pub type AMIE_R = crate::BitReader<AMIE_A>;
#[doc = "Slave mode incoming address match interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled when ADDR_MATCH = 1."]
    ENABLED = 1,
}
impl From<AMIE_A> for bool {
    #[inline(always)]
    fn from(variant: AMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMIE_A {
        match self.bits {
            false => AMIE_A::DISABLED,
            true => AMIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AMIE_A::DISABLED
    }
    #[doc = "Interrupt enabled when ADDR_MATCH = 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AMIE_A::ENABLED
    }
}
#[doc = "Field `AMIE` writer - Slave mode incoming address match interrupt."]
pub type AMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMIE_A>;
impl<'a, REG, const O: u8> AMIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AMIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled when ADDR_MATCH = 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AMIE_A::ENABLED)
    }
}
#[doc = "Field `RXTHIE` reader - RX FIFO Above Treshold Level Interrupt Enable."]
pub type RXTHIE_R = crate::BitReader<RXTHIE_A>;
#[doc = "RX FIFO Above Treshold Level Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXTHIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<RXTHIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXTHIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXTHIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTHIE_A {
        match self.bits {
            false => RXTHIE_A::DISABLED,
            true => RXTHIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXTHIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXTHIE_A::ENABLED
    }
}
#[doc = "Field `RXTHIE` writer - RX FIFO Above Treshold Level Interrupt Enable."]
pub type RXTHIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXTHIE_A>;
impl<'a, REG, const O: u8> RXTHIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXTHIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXTHIE_A::ENABLED)
    }
}
#[doc = "Field `TXTHIE` reader - TX FIFO Below Treshold Level Interrupt Enable."]
pub type TXTHIE_R = crate::BitReader<TXTHIE_A>;
#[doc = "TX FIFO Below Treshold Level Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTHIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<TXTHIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXTHIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXTHIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTHIE_A {
        match self.bits {
            false => TXTHIE_A::DISABLED,
            true => TXTHIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXTHIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXTHIE_A::ENABLED
    }
}
#[doc = "Field `TXTHIE` writer - TX FIFO Below Treshold Level Interrupt Enable."]
pub type TXTHIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXTHIE_A>;
impl<'a, REG, const O: u8> TXTHIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXTHIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXTHIE_A::ENABLED)
    }
}
#[doc = "Field `STOPIE` reader - Stop Interrupt Enable."]
pub type STOPIE_R = crate::BitReader<STOPIE_A>;
#[doc = "Stop Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled when STOP = 1."]
    ENABLED = 1,
}
impl From<STOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPIE_A {
        match self.bits {
            false => STOPIE_A::DISABLED,
            true => STOPIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPIE_A::DISABLED
    }
    #[doc = "Interrupt enabled when STOP = 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPIE_A::ENABLED
    }
}
#[doc = "Field `STOPIE` writer - Stop Interrupt Enable."]
pub type STOPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STOPIE_A>;
impl<'a, REG, const O: u8> STOPIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOPIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled when STOP = 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOPIE_A::ENABLED)
    }
}
#[doc = "Field `ADRACKIE` reader - Received Address ACK from Slave Interrupt."]
pub type ADRACKIE_R = crate::BitReader<ADRACKIE_A>;
#[doc = "Received Address ACK from Slave Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRACKIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ADRACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADRACKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRACKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRACKIE_A {
        match self.bits {
            false => ADRACKIE_A::DISABLED,
            true => ADRACKIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADRACKIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADRACKIE_A::ENABLED
    }
}
#[doc = "Field `ADRACKIE` writer - Received Address ACK from Slave Interrupt."]
pub type ADRACKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADRACKIE_A>;
impl<'a, REG, const O: u8> ADRACKIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRACKIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRACKIE_A::ENABLED)
    }
}
#[doc = "Field `ARBERIE` reader - Master Mode Arbitration Lost Interrupt."]
pub type ARBERIE_R = crate::BitReader<ARBERIE_A>;
#[doc = "Master Mode Arbitration Lost Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBERIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ARBERIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARBERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ARBERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBERIE_A {
        match self.bits {
            false => ARBERIE_A::DISABLED,
            true => ARBERIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARBERIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARBERIE_A::ENABLED
    }
}
#[doc = "Field `ARBERIE` writer - Master Mode Arbitration Lost Interrupt."]
pub type ARBERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ARBERIE_A>;
impl<'a, REG, const O: u8> ARBERIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARBERIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARBERIE_A::ENABLED)
    }
}
#[doc = "Field `TOERIE` reader - Timeout Error Interrupt Enable."]
pub type TOERIE_R = crate::BitReader<TOERIE_A>;
#[doc = "Timeout Error Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOERIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<TOERIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TOERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOERIE_A {
        match self.bits {
            false => TOERIE_A::DISABLED,
            true => TOERIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TOERIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TOERIE_A::ENABLED
    }
}
#[doc = "Field `TOERIE` writer - Timeout Error Interrupt Enable."]
pub type TOERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TOERIE_A>;
impl<'a, REG, const O: u8> TOERIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TOERIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TOERIE_A::ENABLED)
    }
}
#[doc = "Field `ADRERIE` reader - Master Mode Address NACK Received Interrupt."]
pub type ADRERIE_R = crate::BitReader<ADRERIE_A>;
#[doc = "Master Mode Address NACK Received Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRERIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ADRERIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADRERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRERIE_A {
        match self.bits {
            false => ADRERIE_A::DISABLED,
            true => ADRERIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADRERIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADRERIE_A::ENABLED
    }
}
#[doc = "Field `ADRERIE` writer - Master Mode Address NACK Received Interrupt."]
pub type ADRERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADRERIE_A>;
impl<'a, REG, const O: u8> ADRERIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRERIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRERIE_A::ENABLED)
    }
}
#[doc = "Field `DATERIE` reader - Master Mode Data NACK Received Interrupt."]
pub type DATERIE_R = crate::BitReader<DATERIE_A>;
#[doc = "Master Mode Data NACK Received Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATERIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<DATERIE_A> for bool {
    #[inline(always)]
    fn from(variant: DATERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DATERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATERIE_A {
        match self.bits {
            false => DATERIE_A::DISABLED,
            true => DATERIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DATERIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DATERIE_A::ENABLED
    }
}
#[doc = "Field `DATERIE` writer - Master Mode Data NACK Received Interrupt."]
pub type DATERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DATERIE_A>;
impl<'a, REG, const O: u8> DATERIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DATERIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DATERIE_A::ENABLED)
    }
}
#[doc = "Field `DNRERIE` reader - Slave Mode Do Not Respond Interrupt."]
pub type DNRERIE_R = crate::BitReader<DNRERIE_A>;
#[doc = "Slave Mode Do Not Respond Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNRERIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<DNRERIE_A> for bool {
    #[inline(always)]
    fn from(variant: DNRERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DNRERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNRERIE_A {
        match self.bits {
            false => DNRERIE_A::DISABLED,
            true => DNRERIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DNRERIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DNRERIE_A::ENABLED
    }
}
#[doc = "Field `DNRERIE` writer - Slave Mode Do Not Respond Interrupt."]
pub type DNRERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DNRERIE_A>;
impl<'a, REG, const O: u8> DNRERIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DNRERIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DNRERIE_A::ENABLED)
    }
}
#[doc = "Field `STRTERIE` reader - Out of Sequence START condition detected interrupt."]
pub type STRTERIE_R = crate::BitReader<STRTERIE_A>;
#[doc = "Out of Sequence START condition detected interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTERIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<STRTERIE_A> for bool {
    #[inline(always)]
    fn from(variant: STRTERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl STRTERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRTERIE_A {
        match self.bits {
            false => STRTERIE_A::DISABLED,
            true => STRTERIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STRTERIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STRTERIE_A::ENABLED
    }
}
#[doc = "Field `STRTERIE` writer - Out of Sequence START condition detected interrupt."]
pub type STRTERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STRTERIE_A>;
impl<'a, REG, const O: u8> STRTERIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STRTERIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(STRTERIE_A::ENABLED)
    }
}
#[doc = "Field `STOPERIE` reader - Out of Sequence STOP condition detected interrupt."]
pub type STOPERIE_R = crate::BitReader<STOPERIE_A>;
#[doc = "Out of Sequence STOP condition detected interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPERIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<STOPERIE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPERIE_A {
        match self.bits {
            false => STOPERIE_A::DISABLED,
            true => STOPERIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPERIE_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPERIE_A::ENABLED
    }
}
#[doc = "Field `STOPERIE` writer - Out of Sequence STOP condition detected interrupt."]
pub type STOPERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STOPERIE_A>;
impl<'a, REG, const O: u8> STOPERIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOPERIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOPERIE_A::ENABLED)
    }
}
#[doc = "Field `TXLOIE` reader - TX FIFO Locked Out Interrupt."]
pub type TXLOIE_R = crate::BitReader<TXLOIE_A>;
#[doc = "TX FIFO Locked Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXLOIE_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled when TXLOIE = 1."]
    ENABLED = 1,
}
impl From<TXLOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXLOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXLOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXLOIE_A {
        match self.bits {
            false => TXLOIE_A::DISABLED,
            true => TXLOIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXLOIE_A::DISABLED
    }
    #[doc = "Interrupt enabled when TXLOIE = 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXLOIE_A::ENABLED
    }
}
#[doc = "Field `TXLOIE` writer - TX FIFO Locked Out Interrupt."]
pub type TXLOIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXLOIE_A>;
impl<'a, REG, const O: u8> TXLOIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled when TXLOIE = 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOIE_A::ENABLED)
    }
}
#[doc = "Field `MAMIE` reader - Multiple Slave Address Match Interrupt Enable."]
pub type MAMIE_R = crate::FieldReader;
#[doc = "Field `MAMIE` writer - Multiple Slave Address Match Interrupt Enable."]
pub type MAMIE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Transfer Done Interrupt Enable."]
    #[inline(always)]
    pub fn doneie(&self) -> DONEIE_R {
        DONEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Description not available."]
    #[inline(always)]
    pub fn irxmie(&self) -> IRXMIE_R {
        IRXMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave mode general call address match received input enable."]
    #[inline(always)]
    pub fn gcie(&self) -> GCIE_R {
        GCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave mode incoming address match interrupt."]
    #[inline(always)]
    pub fn amie(&self) -> AMIE_R {
        AMIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Above Treshold Level Interrupt Enable."]
    #[inline(always)]
    pub fn rxthie(&self) -> RXTHIE_R {
        RXTHIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO Below Treshold Level Interrupt Enable."]
    #[inline(always)]
    pub fn txthie(&self) -> TXTHIE_R {
        TXTHIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Interrupt Enable."]
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received Address ACK from Slave Interrupt."]
    #[inline(always)]
    pub fn adrackie(&self) -> ADRACKIE_R {
        ADRACKIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Mode Arbitration Lost Interrupt."]
    #[inline(always)]
    pub fn arberie(&self) -> ARBERIE_R {
        ARBERIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timeout Error Interrupt Enable."]
    #[inline(always)]
    pub fn toerie(&self) -> TOERIE_R {
        TOERIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master Mode Address NACK Received Interrupt."]
    #[inline(always)]
    pub fn adrerie(&self) -> ADRERIE_R {
        ADRERIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Mode Data NACK Received Interrupt."]
    #[inline(always)]
    pub fn daterie(&self) -> DATERIE_R {
        DATERIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave Mode Do Not Respond Interrupt."]
    #[inline(always)]
    pub fn dnrerie(&self) -> DNRERIE_R {
        DNRERIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Out of Sequence START condition detected interrupt."]
    #[inline(always)]
    pub fn strterie(&self) -> STRTERIE_R {
        STRTERIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Out of Sequence STOP condition detected interrupt."]
    #[inline(always)]
    pub fn stoperie(&self) -> STOPERIE_R {
        STOPERIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TX FIFO Locked Out Interrupt."]
    #[inline(always)]
    pub fn txloie(&self) -> TXLOIE_R {
        TXLOIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Multiple Slave Address Match Interrupt Enable."]
    #[inline(always)]
    pub fn mamie(&self) -> MAMIE_R {
        MAMIE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Done Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn doneie(&mut self) -> DONEIE_W<INTEN0_SPEC, 0> {
        DONEIE_W::new(self)
    }
    #[doc = "Bit 1 - Description not available."]
    #[inline(always)]
    #[must_use]
    pub fn irxmie(&mut self) -> IRXMIE_W<INTEN0_SPEC, 1> {
        IRXMIE_W::new(self)
    }
    #[doc = "Bit 2 - Slave mode general call address match received input enable."]
    #[inline(always)]
    #[must_use]
    pub fn gcie(&mut self) -> GCIE_W<INTEN0_SPEC, 2> {
        GCIE_W::new(self)
    }
    #[doc = "Bit 3 - Slave mode incoming address match interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn amie(&mut self) -> AMIE_W<INTEN0_SPEC, 3> {
        AMIE_W::new(self)
    }
    #[doc = "Bit 4 - RX FIFO Above Treshold Level Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rxthie(&mut self) -> RXTHIE_W<INTEN0_SPEC, 4> {
        RXTHIE_W::new(self)
    }
    #[doc = "Bit 5 - TX FIFO Below Treshold Level Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn txthie(&mut self) -> TXTHIE_W<INTEN0_SPEC, 5> {
        TXTHIE_W::new(self)
    }
    #[doc = "Bit 6 - Stop Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn stopie(&mut self) -> STOPIE_W<INTEN0_SPEC, 6> {
        STOPIE_W::new(self)
    }
    #[doc = "Bit 7 - Received Address ACK from Slave Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adrackie(&mut self) -> ADRACKIE_W<INTEN0_SPEC, 7> {
        ADRACKIE_W::new(self)
    }
    #[doc = "Bit 8 - Master Mode Arbitration Lost Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arberie(&mut self) -> ARBERIE_W<INTEN0_SPEC, 8> {
        ARBERIE_W::new(self)
    }
    #[doc = "Bit 9 - Timeout Error Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn toerie(&mut self) -> TOERIE_W<INTEN0_SPEC, 9> {
        TOERIE_W::new(self)
    }
    #[doc = "Bit 10 - Master Mode Address NACK Received Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adrerie(&mut self) -> ADRERIE_W<INTEN0_SPEC, 10> {
        ADRERIE_W::new(self)
    }
    #[doc = "Bit 11 - Master Mode Data NACK Received Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn daterie(&mut self) -> DATERIE_W<INTEN0_SPEC, 11> {
        DATERIE_W::new(self)
    }
    #[doc = "Bit 12 - Slave Mode Do Not Respond Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dnrerie(&mut self) -> DNRERIE_W<INTEN0_SPEC, 12> {
        DNRERIE_W::new(self)
    }
    #[doc = "Bit 13 - Out of Sequence START condition detected interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn strterie(&mut self) -> STRTERIE_W<INTEN0_SPEC, 13> {
        STRTERIE_W::new(self)
    }
    #[doc = "Bit 14 - Out of Sequence STOP condition detected interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stoperie(&mut self) -> STOPERIE_W<INTEN0_SPEC, 14> {
        STOPERIE_W::new(self)
    }
    #[doc = "Bit 15 - TX FIFO Locked Out Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txloie(&mut self) -> TXLOIE_W<INTEN0_SPEC, 15> {
        TXLOIE_W::new(self)
    }
    #[doc = "Bits 16:19 - Multiple Slave Address Match Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mamie(&mut self) -> MAMIE_W<INTEN0_SPEC, 16> {
        MAMIE_W::new(self)
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
#[doc = "Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN0_SPEC;
impl crate::RegisterSpec for INTEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten0::R`](R) reader structure"]
impl crate::Readable for INTEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten0::W`](W) writer structure"]
impl crate::Writable for INTEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN0 to value 0"]
impl crate::Resettable for INTEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
