#[doc = "Register `WAKE_EN` reader"]
pub type R = crate::R<WAKE_EN_SPEC>;
#[doc = "Register `WAKE_EN` writer"]
pub type W = crate::W<WAKE_EN_SPEC>;
#[doc = "Field `TX_LEVEL` reader - Wake on TX FIFO Threshold Crossed Enable."]
pub type TX_LEVEL_R = crate::BitReader<TX_LEVEL_A>;
#[doc = "Wake on TX FIFO Threshold Crossed Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_LEVEL_A {
    #[doc = "0: Wakeup source disabled."]
    DISABLED = 0,
    #[doc = "1: Wakeup source enabled."]
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
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_LEVEL_A::DISABLED
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_LEVEL_A::ENABLED
    }
}
#[doc = "Field `TX_LEVEL` writer - Wake on TX FIFO Threshold Crossed Enable."]
pub type TX_LEVEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_LEVEL_A>;
impl<'a, REG, const O: u8> TX_LEVEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_LEVEL_A::DISABLED)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_LEVEL_A::ENABLED)
    }
}
#[doc = "Field `TX_EMPTY` reader - Wake on TX FIFO Empty Enable."]
pub type TX_EMPTY_R = crate::BitReader<TX_EMPTY_A>;
#[doc = "Wake on TX FIFO Empty Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EMPTY_A {
    #[doc = "0: Wakeup source disabled."]
    DISABLED = 0,
    #[doc = "1: Wakeup source enabled."]
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
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_EMPTY_A::DISABLED
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_EMPTY_A::ENABLED
    }
}
#[doc = "Field `TX_EMPTY` writer - Wake on TX FIFO Empty Enable."]
pub type TX_EMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_EMPTY_A>;
impl<'a, REG, const O: u8> TX_EMPTY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EMPTY_A::DISABLED)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EMPTY_A::ENABLED)
    }
}
#[doc = "Field `RX_LEVEL` reader - Wake on RX FIFO Threshold Crossed Enable."]
pub type RX_LEVEL_R = crate::BitReader<RX_LEVEL_A>;
#[doc = "Wake on RX FIFO Threshold Crossed Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_LEVEL_A {
    #[doc = "0: Wakeup source disabled."]
    DISABLED = 0,
    #[doc = "1: Wakeup source enabled."]
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
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_LEVEL_A::DISABLED
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_LEVEL_A::ENABLED
    }
}
#[doc = "Field `RX_LEVEL` writer - Wake on RX FIFO Threshold Crossed Enable."]
pub type RX_LEVEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_LEVEL_A>;
impl<'a, REG, const O: u8> RX_LEVEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_LEVEL_A::DISABLED)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_LEVEL_A::ENABLED)
    }
}
#[doc = "Field `RX_FULL` reader - Wake on RX FIFO Full Enable."]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
#[doc = "Wake on RX FIFO Full Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FULL_A {
    #[doc = "0: Wakeup source disabled."]
    DISABLED = 0,
    #[doc = "1: Wakeup source enabled."]
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
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_FULL_A::DISABLED
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_FULL_A::ENABLED
    }
}
#[doc = "Field `RX_FULL` writer - Wake on RX FIFO Full Enable."]
pub type RX_FULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_FULL_A>;
impl<'a, REG, const O: u8> RX_FULL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FULL_A::DISABLED)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FULL_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty Enable."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full Enable."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_level(&mut self) -> TX_LEVEL_W<WAKE_EN_SPEC, 0> {
        TX_LEVEL_W::new(self)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<WAKE_EN_SPEC, 1> {
        TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_level(&mut self) -> RX_LEVEL_W<WAKE_EN_SPEC, 2> {
        RX_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<WAKE_EN_SPEC, 3> {
        RX_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Wakeup Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wake_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKE_EN_SPEC;
impl crate::RegisterSpec for WAKE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wake_en::R`](R) reader structure"]
impl crate::Readable for WAKE_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wake_en::W`](W) writer structure"]
impl crate::Writable for WAKE_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKE_EN to value 0"]
impl crate::Resettable for WAKE_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
