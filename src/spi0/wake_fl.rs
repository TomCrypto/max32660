#[doc = "Register `WAKE_FL` reader"]
pub type R = crate::R<WAKE_FL_SPEC>;
#[doc = "Register `WAKE_FL` writer"]
pub type W = crate::W<WAKE_FL_SPEC>;
#[doc = "Field `TX_LEVEL` reader - Wake on TX FIFO Threshold Crossed."]
pub type TX_LEVEL_R = crate::BitReader<TX_LEVEL_A>;
#[doc = "Wake on TX FIFO Threshold Crossed.\n\nValue on reset: 0"]
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
#[doc = "Field `TX_LEVEL` writer - Wake on TX FIFO Threshold Crossed."]
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
#[doc = "Field `TX_EMPTY` reader - Wake on TX FIFO Empty."]
pub type TX_EMPTY_R = crate::BitReader<TX_EMPTY_A>;
#[doc = "Wake on TX FIFO Empty.\n\nValue on reset: 0"]
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
#[doc = "Field `TX_EMPTY` writer - Wake on TX FIFO Empty."]
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
#[doc = "Field `RX_LEVEL` reader - Wake on RX FIFO Threshold Crossed."]
pub type RX_LEVEL_R = crate::BitReader<RX_LEVEL_A>;
#[doc = "Wake on RX FIFO Threshold Crossed.\n\nValue on reset: 0"]
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
#[doc = "Field `RX_LEVEL` writer - Wake on RX FIFO Threshold Crossed."]
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
#[doc = "Field `RX_FULL` reader - Wake on RX FIFO Full."]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
#[doc = "Wake on RX FIFO Full.\n\nValue on reset: 0"]
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
#[doc = "Field `RX_FULL` writer - Wake on RX FIFO Full."]
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
impl R {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed."]
    #[inline(always)]
    #[must_use]
    pub fn tx_level(&mut self) -> TX_LEVEL_W<WAKE_FL_SPEC, 0> {
        TX_LEVEL_W::new(self)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty."]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<WAKE_FL_SPEC, 1> {
        TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed."]
    #[inline(always)]
    #[must_use]
    pub fn rx_level(&mut self) -> RX_LEVEL_W<WAKE_FL_SPEC, 2> {
        RX_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<WAKE_FL_SPEC, 3> {
        RX_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Wakeup Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wake_fl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_fl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKE_FL_SPEC;
impl crate::RegisterSpec for WAKE_FL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wake_fl::R`](R) reader structure"]
impl crate::Readable for WAKE_FL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wake_fl::W`](W) writer structure"]
impl crate::Writable for WAKE_FL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKE_FL to value 0"]
impl crate::Resettable for WAKE_FL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
