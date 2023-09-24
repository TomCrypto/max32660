#[doc = "Register `TXCTRL1` reader"]
pub type R = crate::R<TXCTRL1_SPEC>;
#[doc = "Register `TXCTRL1` writer"]
pub type W = crate::W<TXCTRL1_SPEC>;
#[doc = "Field `TXRDY` reader - Transmit FIFO Preload Ready."]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` writer - Transmit FIFO Preload Ready."]
pub type TXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXLAST` reader - Transmit Last."]
pub type TXLAST_R = crate::BitReader<TXLAST_A>;
#[doc = "Transmit Last.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXLAST_A {
    #[doc = "0: Hold SCL low on TX_FIFO empty."]
    HOLD_SCL_LOW = 0,
    #[doc = "1: End transaction on TX_FIFO empty."]
    END_TRANSACTION = 1,
}
impl From<TXLAST_A> for bool {
    #[inline(always)]
    fn from(variant: TXLAST_A) -> Self {
        variant as u8 != 0
    }
}
impl TXLAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXLAST_A {
        match self.bits {
            false => TXLAST_A::HOLD_SCL_LOW,
            true => TXLAST_A::END_TRANSACTION,
        }
    }
    #[doc = "Hold SCL low on TX_FIFO empty."]
    #[inline(always)]
    pub fn is_hold_scl_low(&self) -> bool {
        *self == TXLAST_A::HOLD_SCL_LOW
    }
    #[doc = "End transaction on TX_FIFO empty."]
    #[inline(always)]
    pub fn is_end_transaction(&self) -> bool {
        *self == TXLAST_A::END_TRANSACTION
    }
}
#[doc = "Field `TXLAST` writer - Transmit Last."]
pub type TXLAST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXLAST_A>;
impl<'a, REG, const O: u8> TXLAST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hold SCL low on TX_FIFO empty."]
    #[inline(always)]
    pub fn hold_scl_low(self) -> &'a mut crate::W<REG> {
        self.variant(TXLAST_A::HOLD_SCL_LOW)
    }
    #[doc = "End transaction on TX_FIFO empty."]
    #[inline(always)]
    pub fn end_transaction(self) -> &'a mut crate::W<REG> {
        self.variant(TXLAST_A::END_TRANSACTION)
    }
}
#[doc = "Field `FLSH_GCADDR_DIS` reader - TX FIFO Auto Flush Disable on General Call Address Match."]
pub type FLSH_GCADDR_DIS_R = crate::BitReader<FLSH_GCADDR_DIS_A>;
#[doc = "TX FIFO Auto Flush Disable on General Call Address Match.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLSH_GCADDR_DIS_A {
    #[doc = "0: The TX FIFO is automatically flushed on a General Call Address Match."]
    AUTOFLUSH_EN = 0,
    #[doc = "1: The TX FIFO is not flushed on a General Call Address Match."]
    AUTOFLUSH_DIS = 1,
}
impl From<FLSH_GCADDR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: FLSH_GCADDR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FLSH_GCADDR_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLSH_GCADDR_DIS_A {
        match self.bits {
            false => FLSH_GCADDR_DIS_A::AUTOFLUSH_EN,
            true => FLSH_GCADDR_DIS_A::AUTOFLUSH_DIS,
        }
    }
    #[doc = "The TX FIFO is automatically flushed on a General Call Address Match."]
    #[inline(always)]
    pub fn is_autoflush_en(&self) -> bool {
        *self == FLSH_GCADDR_DIS_A::AUTOFLUSH_EN
    }
    #[doc = "The TX FIFO is not flushed on a General Call Address Match."]
    #[inline(always)]
    pub fn is_autoflush_dis(&self) -> bool {
        *self == FLSH_GCADDR_DIS_A::AUTOFLUSH_DIS
    }
}
#[doc = "Field `FLSH_GCADDR_DIS` writer - TX FIFO Auto Flush Disable on General Call Address Match."]
pub type FLSH_GCADDR_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLSH_GCADDR_DIS_A>;
impl<'a, REG, const O: u8> FLSH_GCADDR_DIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TX FIFO is automatically flushed on a General Call Address Match."]
    #[inline(always)]
    pub fn autoflush_en(self) -> &'a mut crate::W<REG> {
        self.variant(FLSH_GCADDR_DIS_A::AUTOFLUSH_EN)
    }
    #[doc = "The TX FIFO is not flushed on a General Call Address Match."]
    #[inline(always)]
    pub fn autoflush_dis(self) -> &'a mut crate::W<REG> {
        self.variant(FLSH_GCADDR_DIS_A::AUTOFLUSH_DIS)
    }
}
#[doc = "Field `FLSH_SLADDR_DIS` reader - TX FIFO Auto Flush Disable for Slave Address Match."]
pub type FLSH_SLADDR_DIS_R = crate::BitReader<FLSH_SLADDR_DIS_A>;
#[doc = "TX FIFO Auto Flush Disable for Slave Address Match.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLSH_SLADDR_DIS_A {
    #[doc = "0: The TX FIFO is automatically flushed on a Slave Address Match."]
    AUTOFLUSH_EN = 0,
    #[doc = "1: The TX FIFO is not flushed on a Slave Address Match."]
    AUTOFLUSH_DIS = 1,
}
impl From<FLSH_SLADDR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: FLSH_SLADDR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FLSH_SLADDR_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLSH_SLADDR_DIS_A {
        match self.bits {
            false => FLSH_SLADDR_DIS_A::AUTOFLUSH_EN,
            true => FLSH_SLADDR_DIS_A::AUTOFLUSH_DIS,
        }
    }
    #[doc = "The TX FIFO is automatically flushed on a Slave Address Match."]
    #[inline(always)]
    pub fn is_autoflush_en(&self) -> bool {
        *self == FLSH_SLADDR_DIS_A::AUTOFLUSH_EN
    }
    #[doc = "The TX FIFO is not flushed on a Slave Address Match."]
    #[inline(always)]
    pub fn is_autoflush_dis(&self) -> bool {
        *self == FLSH_SLADDR_DIS_A::AUTOFLUSH_DIS
    }
}
#[doc = "Field `FLSH_SLADDR_DIS` writer - TX FIFO Auto Flush Disable for Slave Address Match."]
pub type FLSH_SLADDR_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLSH_SLADDR_DIS_A>;
impl<'a, REG, const O: u8> FLSH_SLADDR_DIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TX FIFO is automatically flushed on a Slave Address Match."]
    #[inline(always)]
    pub fn autoflush_en(self) -> &'a mut crate::W<REG> {
        self.variant(FLSH_SLADDR_DIS_A::AUTOFLUSH_EN)
    }
    #[doc = "The TX FIFO is not flushed on a Slave Address Match."]
    #[inline(always)]
    pub fn autoflush_dis(self) -> &'a mut crate::W<REG> {
        self.variant(FLSH_SLADDR_DIS_A::AUTOFLUSH_DIS)
    }
}
#[doc = "Field `FLSH_NACK_DIS` reader - TX FIFO Auto Flush Disable for NACK."]
pub type FLSH_NACK_DIS_R = crate::BitReader<FLSH_NACK_DIS_A>;
#[doc = "TX FIFO Auto Flush Disable for NACK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLSH_NACK_DIS_A {
    #[doc = "0: The TX FIFO is automatically flushed if a NACK is received at the end of a slave transaction."]
    AUTOFLUSH_EN = 0,
    #[doc = "1: The TX FIFO is not flushed when a NACK is received at the end of a slave transaction."]
    AUTOFLUSH_DIS = 1,
}
impl From<FLSH_NACK_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: FLSH_NACK_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FLSH_NACK_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLSH_NACK_DIS_A {
        match self.bits {
            false => FLSH_NACK_DIS_A::AUTOFLUSH_EN,
            true => FLSH_NACK_DIS_A::AUTOFLUSH_DIS,
        }
    }
    #[doc = "The TX FIFO is automatically flushed if a NACK is received at the end of a slave transaction."]
    #[inline(always)]
    pub fn is_autoflush_en(&self) -> bool {
        *self == FLSH_NACK_DIS_A::AUTOFLUSH_EN
    }
    #[doc = "The TX FIFO is not flushed when a NACK is received at the end of a slave transaction."]
    #[inline(always)]
    pub fn is_autoflush_dis(&self) -> bool {
        *self == FLSH_NACK_DIS_A::AUTOFLUSH_DIS
    }
}
#[doc = "Field `FLSH_NACK_DIS` writer - TX FIFO Auto Flush Disable for NACK."]
pub type FLSH_NACK_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLSH_NACK_DIS_A>;
impl<'a, REG, const O: u8> FLSH_NACK_DIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TX FIFO is automatically flushed if a NACK is received at the end of a slave transaction."]
    #[inline(always)]
    pub fn autoflush_en(self) -> &'a mut crate::W<REG> {
        self.variant(FLSH_NACK_DIS_A::AUTOFLUSH_EN)
    }
    #[doc = "The TX FIFO is not flushed when a NACK is received at the end of a slave transaction."]
    #[inline(always)]
    pub fn autoflush_dis(self) -> &'a mut crate::W<REG> {
        self.variant(FLSH_NACK_DIS_A::AUTOFLUSH_DIS)
    }
}
#[doc = "Field `TXFIFO` reader - Transmit FIFO Count."]
pub type TXFIFO_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Last."]
    #[inline(always)]
    pub fn txlast(&self) -> TXLAST_R {
        TXLAST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Auto Flush Disable on General Call Address Match."]
    #[inline(always)]
    pub fn flsh_gcaddr_dis(&self) -> FLSH_GCADDR_DIS_R {
        FLSH_GCADDR_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO Auto Flush Disable for Slave Address Match."]
    #[inline(always)]
    pub fn flsh_sladdr_dis(&self) -> FLSH_SLADDR_DIS_R {
        FLSH_SLADDR_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO Auto Flush Disable for NACK."]
    #[inline(always)]
    pub fn flsh_nack_dis(&self) -> FLSH_NACK_DIS_R {
        FLSH_NACK_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Count."]
    #[inline(always)]
    pub fn txfifo(&self) -> TXFIFO_R {
        TXFIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<TXCTRL1_SPEC, 0> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Last."]
    #[inline(always)]
    #[must_use]
    pub fn txlast(&mut self) -> TXLAST_W<TXCTRL1_SPEC, 1> {
        TXLAST_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO Auto Flush Disable on General Call Address Match."]
    #[inline(always)]
    #[must_use]
    pub fn flsh_gcaddr_dis(&mut self) -> FLSH_GCADDR_DIS_W<TXCTRL1_SPEC, 2> {
        FLSH_GCADDR_DIS_W::new(self)
    }
    #[doc = "Bit 4 - TX FIFO Auto Flush Disable for Slave Address Match."]
    #[inline(always)]
    #[must_use]
    pub fn flsh_sladdr_dis(&mut self) -> FLSH_SLADDR_DIS_W<TXCTRL1_SPEC, 4> {
        FLSH_SLADDR_DIS_W::new(self)
    }
    #[doc = "Bit 5 - TX FIFO Auto Flush Disable for NACK."]
    #[inline(always)]
    #[must_use]
    pub fn flsh_nack_dis(&mut self) -> FLSH_NACK_DIS_W<TXCTRL1_SPEC, 5> {
        FLSH_NACK_DIS_W::new(self)
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
#[doc = "Transmit Control Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCTRL1_SPEC;
impl crate::RegisterSpec for TXCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl1::R`](R) reader structure"]
impl crate::Readable for TXCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txctrl1::W`](W) writer structure"]
impl crate::Writable for TXCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCTRL1 to value 0"]
impl crate::Resettable for TXCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
