#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `BUSY` reader - Bus Status."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Bus Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: I2C Bus Idle."]
    IDLE = 0,
    #[doc = "1: I2C Bus Busy."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "I2C Bus Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "I2C Bus Busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `RXE` reader - RX empty."]
pub type RXE_R = crate::BitReader<RXE_A>;
#[doc = "RX empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXE_A {
    #[doc = "0: Not Empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Empty."]
    EMPTY = 1,
}
impl From<RXE_A> for bool {
    #[inline(always)]
    fn from(variant: RXE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXE_A {
        match self.bits {
            false => RXE_A::NOT_EMPTY,
            true => RXE_A::EMPTY,
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXE_A::NOT_EMPTY
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXE_A::EMPTY
    }
}
#[doc = "Field `RXF` reader - RX Full."]
pub type RXF_R = crate::BitReader<RXF_A>;
#[doc = "RX Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXF_A {
    #[doc = "0: Not Full."]
    NOT_FULL = 0,
    #[doc = "1: Full."]
    FULL = 1,
}
impl From<RXF_A> for bool {
    #[inline(always)]
    fn from(variant: RXF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXF_A {
        match self.bits {
            false => RXF_A::NOT_FULL,
            true => RXF_A::FULL,
        }
    }
    #[doc = "Not Full."]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXF_A::NOT_FULL
    }
    #[doc = "Full."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXF_A::FULL
    }
}
#[doc = "Field `TXE` reader - TX Empty."]
pub type TXE_R = crate::BitReader<TXE_A>;
#[doc = "TX Empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE_A {
    #[doc = "0: Not Empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Empty."]
    EMPTY = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NOT_EMPTY,
            true => TXE_A::EMPTY,
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NOT_EMPTY
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE_A::EMPTY
    }
}
#[doc = "Field `TXE` writer - TX Empty."]
pub type TXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXE_A>;
impl<'a, REG, const O: u8> TXE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(TXE_A::NOT_EMPTY)
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TXE_A::EMPTY)
    }
}
#[doc = "Field `TXF` reader - TX Full."]
pub type TXF_R = crate::BitReader<TXF_A>;
#[doc = "TX Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXF_A {
    #[doc = "0: Not Empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Empty."]
    EMPTY = 1,
}
impl From<TXF_A> for bool {
    #[inline(always)]
    fn from(variant: TXF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXF_A {
        match self.bits {
            false => TXF_A::NOT_EMPTY,
            true => TXF_A::EMPTY,
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXF_A::NOT_EMPTY
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXF_A::EMPTY
    }
}
#[doc = "Field `TXF` writer - TX Full."]
pub type TXF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXF_A>;
impl<'a, REG, const O: u8> TXF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(TXF_A::NOT_EMPTY)
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TXF_A::EMPTY)
    }
}
#[doc = "Field `CKMD` reader - Clock Mode."]
pub type CKMD_R = crate::BitReader<CKMD_A>;
#[doc = "Clock Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKMD_A {
    #[doc = "0: Device not actively driving SCL clock cycles."]
    NOT_ACTIVELY_DRIVING_SCL_CLOCK = 0,
    #[doc = "1: Device operating as master and actively driving SCL clock cycles."]
    ACTIVELY_DRIVING_SCL_CLOCK = 1,
}
impl From<CKMD_A> for bool {
    #[inline(always)]
    fn from(variant: CKMD_A) -> Self {
        variant as u8 != 0
    }
}
impl CKMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMD_A {
        match self.bits {
            false => CKMD_A::NOT_ACTIVELY_DRIVING_SCL_CLOCK,
            true => CKMD_A::ACTIVELY_DRIVING_SCL_CLOCK,
        }
    }
    #[doc = "Device not actively driving SCL clock cycles."]
    #[inline(always)]
    pub fn is_not_actively_driving_scl_clock(&self) -> bool {
        *self == CKMD_A::NOT_ACTIVELY_DRIVING_SCL_CLOCK
    }
    #[doc = "Device operating as master and actively driving SCL clock cycles."]
    #[inline(always)]
    pub fn is_actively_driving_scl_clock(&self) -> bool {
        *self == CKMD_A::ACTIVELY_DRIVING_SCL_CLOCK
    }
}
#[doc = "Field `STAT` reader - Controller Status."]
pub type STAT_R = crate::FieldReader<STAT_A>;
#[doc = "Controller Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STAT_A {
    #[doc = "0: Controller Idle."]
    IDLE = 0,
    #[doc = "1: master Transmit address."]
    MTX_ADDR = 1,
    #[doc = "2: Master Receive address ACK."]
    MRX_ADDR_ACK = 2,
    #[doc = "3: Master Transmit extended address."]
    MTX_EX_ADDR = 3,
    #[doc = "4: Master Receive extended address ACK."]
    MRX_EX_ADDR = 4,
    #[doc = "5: Slave Receive address."]
    SRX_ADDR = 5,
    #[doc = "6: Slave Transmit address ACK."]
    STX_ADDR_ACK = 6,
    #[doc = "7: Slave Receive extended address."]
    SRX_EX_ADDR = 7,
    #[doc = "8: Slave Transmit extended address ACK."]
    STX_EX_ADDR_ACK = 8,
    #[doc = "9: Transmit data (master or slave)."]
    TX = 9,
    #[doc = "10: Receive data ACK (master or slave)."]
    RX_ACK = 10,
    #[doc = "11: Receive data (master or slave)."]
    RX = 11,
    #[doc = "12: Transmit data ACK (master or slave)."]
    TX_ACK = 12,
    #[doc = "13: NACK stage (master or slave)."]
    NACK = 13,
    #[doc = "15: Bystander state (ongoing transaction but not participant - another master addressing another slave)."]
    BY_ST = 15,
}
impl From<STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STAT_A {
    type Ux = u8;
}
impl STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STAT_A> {
        match self.bits {
            0 => Some(STAT_A::IDLE),
            1 => Some(STAT_A::MTX_ADDR),
            2 => Some(STAT_A::MRX_ADDR_ACK),
            3 => Some(STAT_A::MTX_EX_ADDR),
            4 => Some(STAT_A::MRX_EX_ADDR),
            5 => Some(STAT_A::SRX_ADDR),
            6 => Some(STAT_A::STX_ADDR_ACK),
            7 => Some(STAT_A::SRX_EX_ADDR),
            8 => Some(STAT_A::STX_EX_ADDR_ACK),
            9 => Some(STAT_A::TX),
            10 => Some(STAT_A::RX_ACK),
            11 => Some(STAT_A::RX),
            12 => Some(STAT_A::TX_ACK),
            13 => Some(STAT_A::NACK),
            15 => Some(STAT_A::BY_ST),
            _ => None,
        }
    }
    #[doc = "Controller Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STAT_A::IDLE
    }
    #[doc = "master Transmit address."]
    #[inline(always)]
    pub fn is_mtx_addr(&self) -> bool {
        *self == STAT_A::MTX_ADDR
    }
    #[doc = "Master Receive address ACK."]
    #[inline(always)]
    pub fn is_mrx_addr_ack(&self) -> bool {
        *self == STAT_A::MRX_ADDR_ACK
    }
    #[doc = "Master Transmit extended address."]
    #[inline(always)]
    pub fn is_mtx_ex_addr(&self) -> bool {
        *self == STAT_A::MTX_EX_ADDR
    }
    #[doc = "Master Receive extended address ACK."]
    #[inline(always)]
    pub fn is_mrx_ex_addr(&self) -> bool {
        *self == STAT_A::MRX_EX_ADDR
    }
    #[doc = "Slave Receive address."]
    #[inline(always)]
    pub fn is_srx_addr(&self) -> bool {
        *self == STAT_A::SRX_ADDR
    }
    #[doc = "Slave Transmit address ACK."]
    #[inline(always)]
    pub fn is_stx_addr_ack(&self) -> bool {
        *self == STAT_A::STX_ADDR_ACK
    }
    #[doc = "Slave Receive extended address."]
    #[inline(always)]
    pub fn is_srx_ex_addr(&self) -> bool {
        *self == STAT_A::SRX_EX_ADDR
    }
    #[doc = "Slave Transmit extended address ACK."]
    #[inline(always)]
    pub fn is_stx_ex_addr_ack(&self) -> bool {
        *self == STAT_A::STX_EX_ADDR_ACK
    }
    #[doc = "Transmit data (master or slave)."]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == STAT_A::TX
    }
    #[doc = "Receive data ACK (master or slave)."]
    #[inline(always)]
    pub fn is_rx_ack(&self) -> bool {
        *self == STAT_A::RX_ACK
    }
    #[doc = "Receive data (master or slave)."]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == STAT_A::RX
    }
    #[doc = "Transmit data ACK (master or slave)."]
    #[inline(always)]
    pub fn is_tx_ack(&self) -> bool {
        *self == STAT_A::TX_ACK
    }
    #[doc = "NACK stage (master or slave)."]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == STAT_A::NACK
    }
    #[doc = "Bystander state (ongoing transaction but not participant - another master addressing another slave)."]
    #[inline(always)]
    pub fn is_by_st(&self) -> bool {
        *self == STAT_A::BY_ST
    }
}
#[doc = "Field `STAT` writer - Controller Status."]
pub type STAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, STAT_A>;
impl<'a, REG, const O: u8> STAT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Controller Idle."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::IDLE)
    }
    #[doc = "master Transmit address."]
    #[inline(always)]
    pub fn mtx_addr(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::MTX_ADDR)
    }
    #[doc = "Master Receive address ACK."]
    #[inline(always)]
    pub fn mrx_addr_ack(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::MRX_ADDR_ACK)
    }
    #[doc = "Master Transmit extended address."]
    #[inline(always)]
    pub fn mtx_ex_addr(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::MTX_EX_ADDR)
    }
    #[doc = "Master Receive extended address ACK."]
    #[inline(always)]
    pub fn mrx_ex_addr(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::MRX_EX_ADDR)
    }
    #[doc = "Slave Receive address."]
    #[inline(always)]
    pub fn srx_addr(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::SRX_ADDR)
    }
    #[doc = "Slave Transmit address ACK."]
    #[inline(always)]
    pub fn stx_addr_ack(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::STX_ADDR_ACK)
    }
    #[doc = "Slave Receive extended address."]
    #[inline(always)]
    pub fn srx_ex_addr(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::SRX_EX_ADDR)
    }
    #[doc = "Slave Transmit extended address ACK."]
    #[inline(always)]
    pub fn stx_ex_addr_ack(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::STX_EX_ADDR_ACK)
    }
    #[doc = "Transmit data (master or slave)."]
    #[inline(always)]
    pub fn tx(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::TX)
    }
    #[doc = "Receive data ACK (master or slave)."]
    #[inline(always)]
    pub fn rx_ack(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::RX_ACK)
    }
    #[doc = "Receive data (master or slave)."]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::RX)
    }
    #[doc = "Transmit data ACK (master or slave)."]
    #[inline(always)]
    pub fn tx_ack(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::TX_ACK)
    }
    #[doc = "NACK stage (master or slave)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::NACK)
    }
    #[doc = "Bystander state (ongoing transaction but not participant - another master addressing another slave)."]
    #[inline(always)]
    pub fn by_st(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_A::BY_ST)
    }
}
impl R {
    #[doc = "Bit 0 - Bus Status."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX empty."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Full."]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Empty."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX Full."]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Mode."]
    #[inline(always)]
    pub fn ckmd(&self) -> CKMD_R {
        CKMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Controller Status."]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - TX Empty."]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<STATUS_SPEC, 3> {
        TXE_W::new(self)
    }
    #[doc = "Bit 4 - TX Full."]
    #[inline(always)]
    #[must_use]
    pub fn txf(&mut self) -> TXF_W<STATUS_SPEC, 4> {
        TXF_W::new(self)
    }
    #[doc = "Bits 8:11 - Controller Status."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> STAT_W<STATUS_SPEC, 8> {
        STAT_W::new(self)
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
#[doc = "Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
