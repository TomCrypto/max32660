#[doc = "Register `INT_FL` reader"]
pub type R = crate::R<INT_FL_SPEC>;
#[doc = "Register `INT_FL` writer"]
pub type W = crate::W<INT_FL_SPEC>;
#[doc = "Field `TX_LEVEL` reader - TX FIFO Threshold Crossed."]
pub type TX_LEVEL_R = crate::BitReader<TX_LEVEL_A>;
#[doc = "TX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_LEVEL_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<TX_LEVEL_A> {
        match self.bits {
            true => Some(TX_LEVEL_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_LEVEL_A::CLEAR
    }
}
#[doc = "Field `TX_LEVEL` writer - TX FIFO Threshold Crossed."]
pub type TX_LEVEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_LEVEL_A>;
impl<'a, REG, const O: u8> TX_LEVEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_LEVEL_A::CLEAR)
    }
}
#[doc = "Field `TX_EMPTY` reader - TX FIFO Empty."]
pub type TX_EMPTY_R = crate::BitReader<TX_EMPTY_A>;
#[doc = "TX FIFO Empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EMPTY_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<TX_EMPTY_A> {
        match self.bits {
            true => Some(TX_EMPTY_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_EMPTY_A::CLEAR
    }
}
#[doc = "Field `TX_EMPTY` writer - TX FIFO Empty."]
pub type TX_EMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_EMPTY_A>;
impl<'a, REG, const O: u8> TX_EMPTY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EMPTY_A::CLEAR)
    }
}
#[doc = "Field `RX_LEVEL` reader - RX FIFO Threshold Crossed."]
pub type RX_LEVEL_R = crate::BitReader<RX_LEVEL_A>;
#[doc = "RX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_LEVEL_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<RX_LEVEL_A> {
        match self.bits {
            true => Some(RX_LEVEL_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_LEVEL_A::CLEAR
    }
}
#[doc = "Field `RX_LEVEL` writer - RX FIFO Threshold Crossed."]
pub type RX_LEVEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_LEVEL_A>;
impl<'a, REG, const O: u8> RX_LEVEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_LEVEL_A::CLEAR)
    }
}
#[doc = "Field `RX_FULL` reader - RX FIFO FULL."]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
#[doc = "RX FIFO FULL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FULL_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<RX_FULL_A> {
        match self.bits {
            true => Some(RX_FULL_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_FULL_A::CLEAR
    }
}
#[doc = "Field `RX_FULL` writer - RX FIFO FULL."]
pub type RX_FULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_FULL_A>;
impl<'a, REG, const O: u8> RX_FULL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FULL_A::CLEAR)
    }
}
#[doc = "Field `SSA` reader - Slave Select Asserted."]
pub type SSA_R = crate::BitReader<SSA_A>;
#[doc = "Slave Select Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSA_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<SSA_A> {
        match self.bits {
            true => Some(SSA_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SSA_A::CLEAR
    }
}
#[doc = "Field `SSA` writer - Slave Select Asserted."]
pub type SSA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSA_A>;
impl<'a, REG, const O: u8> SSA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SSA_A::CLEAR)
    }
}
#[doc = "Field `SSD` reader - Slave Select Deasserted."]
pub type SSD_R = crate::BitReader<SSD_A>;
#[doc = "Slave Select Deasserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSD_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<SSD_A> {
        match self.bits {
            true => Some(SSD_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SSD_A::CLEAR
    }
}
#[doc = "Field `SSD` writer - Slave Select Deasserted."]
pub type SSD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSD_A>;
impl<'a, REG, const O: u8> SSD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SSD_A::CLEAR)
    }
}
#[doc = "Field `ABORT` reader - Slave Abort Detected."]
pub type ABORT_R = crate::BitReader<ABORT_A>;
#[doc = "Slave Abort Detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<ABORT_A> {
        match self.bits {
            true => Some(ABORT_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ABORT_A::CLEAR
    }
}
#[doc = "Field `ABORT` writer - Slave Abort Detected."]
pub type ABORT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABORT_A>;
impl<'a, REG, const O: u8> ABORT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT_A::CLEAR)
    }
}
#[doc = "Field `M_DONE` reader - Master Done, set when SPI Master has completed any transactions."]
pub type M_DONE_R = crate::BitReader<M_DONE_A>;
#[doc = "Master Done, set when SPI Master has completed any transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_DONE_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<M_DONE_A> {
        match self.bits {
            true => Some(M_DONE_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == M_DONE_A::CLEAR
    }
}
#[doc = "Field `M_DONE` writer - Master Done, set when SPI Master has completed any transactions."]
pub type M_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, M_DONE_A>;
impl<'a, REG, const O: u8> M_DONE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(M_DONE_A::CLEAR)
    }
}
#[doc = "Field `TX_OVR` reader - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
pub type TX_OVR_R = crate::BitReader<TX_OVR_A>;
#[doc = "Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_OVR_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<TX_OVR_A> {
        match self.bits {
            true => Some(TX_OVR_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_OVR_A::CLEAR
    }
}
#[doc = "Field `TX_OVR` writer - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
pub type TX_OVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_OVR_A>;
impl<'a, REG, const O: u8> TX_OVR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_OVR_A::CLEAR)
    }
}
#[doc = "Field `TX_UND` reader - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
pub type TX_UND_R = crate::BitReader<TX_UND_A>;
#[doc = "Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_UND_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<TX_UND_A> {
        match self.bits {
            true => Some(TX_UND_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_UND_A::CLEAR
    }
}
#[doc = "Field `TX_UND` writer - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
pub type TX_UND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_UND_A>;
impl<'a, REG, const O: u8> TX_UND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_UND_A::CLEAR)
    }
}
#[doc = "Field `RX_OVR` reader - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
pub type RX_OVR_R = crate::BitReader<RX_OVR_A>;
#[doc = "Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_OVR_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<RX_OVR_A> {
        match self.bits {
            true => Some(RX_OVR_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_OVR_A::CLEAR
    }
}
#[doc = "Field `RX_OVR` writer - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
pub type RX_OVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_OVR_A>;
impl<'a, REG, const O: u8> RX_OVR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_OVR_A::CLEAR)
    }
}
#[doc = "Field `RX_UND` reader - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
pub type RX_UND_R = crate::BitReader<RX_UND_A>;
#[doc = "Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_UND_A {
    #[doc = "1: Flag is set when value read is 1."]
    CLEAR = 1,
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
    pub fn variant(&self) -> Option<RX_UND_A> {
        match self.bits {
            true => Some(RX_UND_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_UND_A::CLEAR
    }
}
#[doc = "Field `RX_UND` writer - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
pub type RX_UND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_UND_A>;
impl<'a, REG, const O: u8> RX_UND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_UND_A::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO FULL."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select Asserted."]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deasserted."]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Abort Detected."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Done, set when SPI Master has completed any transactions."]
    #[inline(always)]
    pub fn m_done(&self) -> M_DONE_R {
        M_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
    #[inline(always)]
    pub fn tx_ovr(&self) -> TX_OVR_R {
        TX_OVR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
    #[inline(always)]
    pub fn tx_und(&self) -> TX_UND_R {
        TX_UND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
    #[inline(always)]
    pub fn rx_ovr(&self) -> RX_OVR_R {
        RX_OVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
    #[inline(always)]
    pub fn rx_und(&self) -> RX_UND_R {
        RX_UND_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Threshold Crossed."]
    #[inline(always)]
    #[must_use]
    pub fn tx_level(&mut self) -> TX_LEVEL_W<INT_FL_SPEC, 0> {
        TX_LEVEL_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Empty."]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<INT_FL_SPEC, 1> {
        TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed."]
    #[inline(always)]
    #[must_use]
    pub fn rx_level(&mut self) -> RX_LEVEL_W<INT_FL_SPEC, 2> {
        RX_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO FULL."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<INT_FL_SPEC, 3> {
        RX_FULL_W::new(self)
    }
    #[doc = "Bit 4 - Slave Select Asserted."]
    #[inline(always)]
    #[must_use]
    pub fn ssa(&mut self) -> SSA_W<INT_FL_SPEC, 4> {
        SSA_W::new(self)
    }
    #[doc = "Bit 5 - Slave Select Deasserted."]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<INT_FL_SPEC, 5> {
        SSD_W::new(self)
    }
    #[doc = "Bit 9 - Slave Abort Detected."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<INT_FL_SPEC, 9> {
        ABORT_W::new(self)
    }
    #[doc = "Bit 11 - Master Done, set when SPI Master has completed any transactions."]
    #[inline(always)]
    #[must_use]
    pub fn m_done(&mut self) -> M_DONE_W<INT_FL_SPEC, 11> {
        M_DONE_W::new(self)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ovr(&mut self) -> TX_OVR_W<INT_FL_SPEC, 12> {
        TX_OVR_W::new(self)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_und(&mut self) -> TX_UND_W<INT_FL_SPEC, 13> {
        TX_UND_W::new(self)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ovr(&mut self) -> RX_OVR_W<INT_FL_SPEC, 14> {
        RX_OVR_W::new(self)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_und(&mut self) -> RX_UND_W<INT_FL_SPEC, 15> {
        RX_UND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_fl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FL_SPEC;
impl crate::RegisterSpec for INT_FL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_fl::R`](R) reader structure"]
impl crate::Readable for INT_FL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_fl::W`](W) writer structure"]
impl crate::Writable for INT_FL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_FL to value 0"]
impl crate::Resettable for INT_FL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
