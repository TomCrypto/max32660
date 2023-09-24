#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<INT_EN_SPEC>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<INT_EN_SPEC>;
#[doc = "Field `TX_LEVEL` reader - TX FIFO Threshold interrupt enable."]
pub type TX_LEVEL_R = crate::BitReader<TX_LEVEL_A>;
#[doc = "TX FIFO Threshold interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_LEVEL_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<TX_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_LEVEL_A {
        match self.bits {
            false => TX_LEVEL_A::DISABLED,
            true => TX_LEVEL_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_LEVEL_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_LEVEL_A::ENABLED
    }
}
#[doc = "Field `TX_LEVEL` writer - TX FIFO Threshold interrupt enable."]
pub type TX_LEVEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_LEVEL_A>;
impl<'a, REG, const O: u8> TX_LEVEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_LEVEL_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_LEVEL_A::ENABLED)
    }
}
#[doc = "Field `TX_EMPTY` reader - TX FIFO Empty interrupt enable."]
pub type TX_EMPTY_R = crate::BitReader<TX_EMPTY_A>;
#[doc = "TX FIFO Empty interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EMPTY_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<TX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EMPTY_A {
        match self.bits {
            false => TX_EMPTY_A::DISABLED,
            true => TX_EMPTY_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_EMPTY_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_EMPTY_A::ENABLED
    }
}
#[doc = "Field `TX_EMPTY` writer - TX FIFO Empty interrupt enable."]
pub type TX_EMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_EMPTY_A>;
impl<'a, REG, const O: u8> TX_EMPTY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EMPTY_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EMPTY_A::ENABLED)
    }
}
#[doc = "Field `RX_LEVEL` reader - RX FIFO Threshold Crossed interrupt enable."]
pub type RX_LEVEL_R = crate::BitReader<RX_LEVEL_A>;
#[doc = "RX FIFO Threshold Crossed interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_LEVEL_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<RX_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_LEVEL_A {
        match self.bits {
            false => RX_LEVEL_A::DISABLED,
            true => RX_LEVEL_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_LEVEL_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_LEVEL_A::ENABLED
    }
}
#[doc = "Field `RX_LEVEL` writer - RX FIFO Threshold Crossed interrupt enable."]
pub type RX_LEVEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_LEVEL_A>;
impl<'a, REG, const O: u8> RX_LEVEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_LEVEL_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_LEVEL_A::ENABLED)
    }
}
#[doc = "Field `RX_FULL` reader - RX FIFO FULL interrupt enable."]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
#[doc = "RX FIFO FULL interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FULL_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FULL_A {
        match self.bits {
            false => RX_FULL_A::DISABLED,
            true => RX_FULL_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_FULL_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_FULL_A::ENABLED
    }
}
#[doc = "Field `RX_FULL` writer - RX FIFO FULL interrupt enable."]
pub type RX_FULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_FULL_A>;
impl<'a, REG, const O: u8> RX_FULL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FULL_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FULL_A::ENABLED)
    }
}
#[doc = "Field `SSA` reader - Slave Select Asserted interrupt enable."]
pub type SSA_R = crate::BitReader<SSA_A>;
#[doc = "Slave Select Asserted interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSA_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<SSA_A> for bool {
    #[inline(always)]
    fn from(variant: SSA_A) -> Self {
        variant as u8 != 0
    }
}
impl SSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSA_A {
        match self.bits {
            false => SSA_A::DISABLED,
            true => SSA_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSA_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSA_A::ENABLED
    }
}
#[doc = "Field `SSA` writer - Slave Select Asserted interrupt enable."]
pub type SSA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSA_A>;
impl<'a, REG, const O: u8> SSA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSA_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSA_A::ENABLED)
    }
}
#[doc = "Field `SSD` reader - Slave Select Deasserted interrupt enable."]
pub type SSD_R = crate::BitReader<SSD_A>;
#[doc = "Slave Select Deasserted interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSD_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<SSD_A> for bool {
    #[inline(always)]
    fn from(variant: SSD_A) -> Self {
        variant as u8 != 0
    }
}
impl SSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSD_A {
        match self.bits {
            false => SSD_A::DISABLED,
            true => SSD_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSD_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSD_A::ENABLED
    }
}
#[doc = "Field `SSD` writer - Slave Select Deasserted interrupt enable."]
pub type SSD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSD_A>;
impl<'a, REG, const O: u8> SSD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSD_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSD_A::ENABLED)
    }
}
#[doc = "Field `FAULT` reader - Multi-Master Mode Fault interrupt enable."]
pub type FAULT_R = crate::BitReader<FAULT_A>;
#[doc = "Multi-Master Mode Fault interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULT_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT_A {
        match self.bits {
            false => FAULT_A::DISABLED,
            true => FAULT_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAULT_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FAULT_A::ENABLED
    }
}
#[doc = "Field `FAULT` writer - Multi-Master Mode Fault interrupt enable."]
pub type FAULT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FAULT_A>;
impl<'a, REG, const O: u8> FAULT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT_A::ENABLED)
    }
}
#[doc = "Field `ABORT` reader - Slave Abort Detected interrupt enable."]
pub type ABORT_R = crate::BitReader<ABORT_A>;
#[doc = "Slave Abort Detected interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
impl ABORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABORT_A {
        match self.bits {
            false => ABORT_A::DISABLED,
            true => ABORT_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABORT_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABORT_A::ENABLED
    }
}
#[doc = "Field `ABORT` writer - Slave Abort Detected interrupt enable."]
pub type ABORT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABORT_A>;
impl<'a, REG, const O: u8> ABORT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT_A::ENABLED)
    }
}
#[doc = "Field `M_DONE` reader - Master Done interrupt enable."]
pub type M_DONE_R = crate::BitReader<M_DONE_A>;
#[doc = "Master Done interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_DONE_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<M_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: M_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl M_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_DONE_A {
        match self.bits {
            false => M_DONE_A::DISABLED,
            true => M_DONE_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_DONE_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_DONE_A::ENABLED
    }
}
#[doc = "Field `M_DONE` writer - Master Done interrupt enable."]
pub type M_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, M_DONE_A>;
impl<'a, REG, const O: u8> M_DONE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_DONE_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_DONE_A::ENABLED)
    }
}
#[doc = "Field `TX_OVR` reader - Transmit FIFO Overrun interrupt enable."]
pub type TX_OVR_R = crate::BitReader<TX_OVR_A>;
#[doc = "Transmit FIFO Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_OVR_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<TX_OVR_A> for bool {
    #[inline(always)]
    fn from(variant: TX_OVR_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_OVR_A {
        match self.bits {
            false => TX_OVR_A::DISABLED,
            true => TX_OVR_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_OVR_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_OVR_A::ENABLED
    }
}
#[doc = "Field `TX_OVR` writer - Transmit FIFO Overrun interrupt enable."]
pub type TX_OVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_OVR_A>;
impl<'a, REG, const O: u8> TX_OVR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_OVR_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_OVR_A::ENABLED)
    }
}
#[doc = "Field `TX_UND` reader - Transmit FIFO Underrun interrupt enable."]
pub type TX_UND_R = crate::BitReader<TX_UND_A>;
#[doc = "Transmit FIFO Underrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_UND_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<TX_UND_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UND_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_UND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_UND_A {
        match self.bits {
            false => TX_UND_A::DISABLED,
            true => TX_UND_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_UND_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_UND_A::ENABLED
    }
}
#[doc = "Field `TX_UND` writer - Transmit FIFO Underrun interrupt enable."]
pub type TX_UND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_UND_A>;
impl<'a, REG, const O: u8> TX_UND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_UND_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_UND_A::ENABLED)
    }
}
#[doc = "Field `RX_OVR` reader - Receive FIFO Overrun interrupt enable."]
pub type RX_OVR_R = crate::BitReader<RX_OVR_A>;
#[doc = "Receive FIFO Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_OVR_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<RX_OVR_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OVR_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OVR_A {
        match self.bits {
            false => RX_OVR_A::DISABLED,
            true => RX_OVR_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_OVR_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_OVR_A::ENABLED
    }
}
#[doc = "Field `RX_OVR` writer - Receive FIFO Overrun interrupt enable."]
pub type RX_OVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_OVR_A>;
impl<'a, REG, const O: u8> RX_OVR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_OVR_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_OVR_A::ENABLED)
    }
}
#[doc = "Field `RX_UND` reader - Receive FIFO Underrun interrupt enable."]
pub type RX_UND_R = crate::BitReader<RX_UND_A>;
#[doc = "Receive FIFO Underrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_UND_A {
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled."]
    ENABLED = 1,
}
impl From<RX_UND_A> for bool {
    #[inline(always)]
    fn from(variant: RX_UND_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_UND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_UND_A {
        match self.bits {
            false => RX_UND_A::DISABLED,
            true => RX_UND_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_UND_A::DISABLED
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_UND_A::ENABLED
    }
}
#[doc = "Field `RX_UND` writer - Receive FIFO Underrun interrupt enable."]
pub type RX_UND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_UND_A>;
impl<'a, REG, const O: u8> RX_UND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_UND_A::DISABLED)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_UND_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - TX FIFO Threshold interrupt enable."]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty interrupt enable."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed interrupt enable."]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO FULL interrupt enable."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select Asserted interrupt enable."]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deasserted interrupt enable."]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault interrupt enable."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Abort Detected interrupt enable."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Done interrupt enable."]
    #[inline(always)]
    pub fn m_done(&self) -> M_DONE_R {
        M_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn tx_ovr(&self) -> TX_OVR_R {
        TX_OVR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn tx_und(&self) -> TX_UND_R {
        TX_UND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn rx_ovr(&self) -> RX_OVR_R {
        RX_OVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn rx_und(&self) -> RX_UND_R {
        RX_UND_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Threshold interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_level(&mut self) -> TX_LEVEL_W<INT_EN_SPEC, 0> {
        TX_LEVEL_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Empty interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<INT_EN_SPEC, 1> {
        TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_level(&mut self) -> RX_LEVEL_W<INT_EN_SPEC, 2> {
        RX_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO FULL interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<INT_EN_SPEC, 3> {
        RX_FULL_W::new(self)
    }
    #[doc = "Bit 4 - Slave Select Asserted interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ssa(&mut self) -> SSA_W<INT_EN_SPEC, 4> {
        SSA_W::new(self)
    }
    #[doc = "Bit 5 - Slave Select Deasserted interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<INT_EN_SPEC, 5> {
        SSD_W::new(self)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FAULT_W<INT_EN_SPEC, 8> {
        FAULT_W::new(self)
    }
    #[doc = "Bit 9 - Slave Abort Detected interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<INT_EN_SPEC, 9> {
        ABORT_W::new(self)
    }
    #[doc = "Bit 11 - Master Done interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn m_done(&mut self) -> M_DONE_W<INT_EN_SPEC, 11> {
        M_DONE_W::new(self)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ovr(&mut self) -> TX_OVR_W<INT_EN_SPEC, 12> {
        TX_OVR_W::new(self)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_und(&mut self) -> TX_UND_W<INT_EN_SPEC, 13> {
        TX_UND_W::new(self)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ovr(&mut self) -> RX_OVR_W<INT_EN_SPEC, 14> {
        RX_OVR_W::new(self)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_und(&mut self) -> RX_UND_W<INT_EN_SPEC, 15> {
        RX_UND_W::new(self)
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
#[doc = "Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
