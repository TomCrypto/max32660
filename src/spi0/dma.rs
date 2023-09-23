#[doc = "Register `DMA` reader"]
pub type R = crate::R<DMA_SPEC>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DMA_SPEC>;
#[doc = "Field `TX_FIFO_LEVEL` reader - Transmit FIFO level that will trigger a DMA request, also level for threshold status."]
pub type TX_FIFO_LEVEL_R = crate::FieldReader;
#[doc = "Field `TX_FIFO_LEVEL` writer - Transmit FIFO level that will trigger a DMA request, also level for threshold status."]
pub type TX_FIFO_LEVEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `TX_FIFO_EN` reader - Transmit FIFO enabled for SPI transactions."]
pub type TX_FIFO_EN_R = crate::BitReader<TX_FIFO_EN_A>;
#[doc = "Transmit FIFO enabled for SPI transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_FIFO_EN_A {
    #[doc = "0: Transmit FIFO is not enabled."]
    DISABLED = 0,
    #[doc = "1: Transmit FIFO is enabled."]
    ENABLED = 1,
}
impl From<TX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_FIFO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFO_EN_A {
        match self.bits {
            false => TX_FIFO_EN_A::DISABLED,
            true => TX_FIFO_EN_A::ENABLED,
        }
    }
    #[doc = "Transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_FIFO_EN_A::DISABLED
    }
    #[doc = "Transmit FIFO is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_FIFO_EN_A::ENABLED
    }
}
#[doc = "Field `TX_FIFO_EN` writer - Transmit FIFO enabled for SPI transactions."]
pub type TX_FIFO_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_FIFO_EN_A>;
impl<'a, REG, const O: u8> TX_FIFO_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_EN_A::DISABLED)
    }
    #[doc = "Transmit FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_EN_A::ENABLED)
    }
}
#[doc = "Field `TX_FIFO_CLEAR` reader - Clear TX FIFO, clear is accomplished by resetting the read and write pointers."]
pub type TX_FIFO_CLEAR_R = crate::BitReader<TX_FIFO_CLEAR_A>;
#[doc = "Clear TX FIFO, clear is accomplished by resetting the read and write pointers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_FIFO_CLEAR_A {
    #[doc = "1: Clear the Transmit FIFO, clears any pending TX FIFO status."]
    CLEAR = 1,
}
impl From<TX_FIFO_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_FIFO_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_FIFO_CLEAR_A> {
        match self.bits {
            true => Some(TX_FIFO_CLEAR_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Clear the Transmit FIFO, clears any pending TX FIFO status."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_FIFO_CLEAR_A::CLEAR
    }
}
#[doc = "Field `TX_FIFO_CLEAR` writer - Clear TX FIFO, clear is accomplished by resetting the read and write pointers."]
pub type TX_FIFO_CLEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_FIFO_CLEAR_A>;
impl<'a, REG, const O: u8> TX_FIFO_CLEAR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Transmit FIFO, clears any pending TX FIFO status."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `TX_FIFO_CNT` reader - Count of entries in TX FIFO."]
pub type TX_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `TX_DMA_EN` reader - TX DMA Enable."]
pub type TX_DMA_EN_R = crate::BitReader<TX_DMA_EN_A>;
#[doc = "TX DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_DMA_EN_A {
    #[doc = "0: TX DMA requests are disabled, andy pending DMA requests are cleared."]
    DISABLED = 0,
    #[doc = "1: TX DMA requests are enabled."]
    ENABLED = 1,
}
impl From<TX_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_DMA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DMA_EN_A {
        match self.bits {
            false => TX_DMA_EN_A::DISABLED,
            true => TX_DMA_EN_A::ENABLED,
        }
    }
    #[doc = "TX DMA requests are disabled, andy pending DMA requests are cleared."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_DMA_EN_A::DISABLED
    }
    #[doc = "TX DMA requests are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_DMA_EN_A::ENABLED
    }
}
#[doc = "Field `TX_DMA_EN` writer - TX DMA Enable."]
pub type TX_DMA_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_DMA_EN_A>;
impl<'a, REG, const O: u8> TX_DMA_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX DMA requests are disabled, andy pending DMA requests are cleared."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_DMA_EN_A::DISABLED)
    }
    #[doc = "TX DMA requests are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_DMA_EN_A::ENABLED)
    }
}
#[doc = "Field `RX_FIFO_LEVEL` reader - Receive FIFO level that will trigger a DMA request, also level for threshold status."]
pub type RX_FIFO_LEVEL_R = crate::FieldReader;
#[doc = "Field `RX_FIFO_LEVEL` writer - Receive FIFO level that will trigger a DMA request, also level for threshold status."]
pub type RX_FIFO_LEVEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RX_FIFO_EN` reader - Receive FIFO enabled for SPI transactions."]
pub type RX_FIFO_EN_R = crate::BitReader<RX_FIFO_EN_A>;
#[doc = "Receive FIFO enabled for SPI transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_EN_A {
    #[doc = "0: Receive FIFO is not enabled."]
    DISABLED = 0,
    #[doc = "1: Receive FIFO is enabled."]
    ENABLED = 1,
}
impl From<RX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_EN_A {
        match self.bits {
            false => RX_FIFO_EN_A::DISABLED,
            true => RX_FIFO_EN_A::ENABLED,
        }
    }
    #[doc = "Receive FIFO is not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_FIFO_EN_A::DISABLED
    }
    #[doc = "Receive FIFO is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_FIFO_EN_A::ENABLED
    }
}
#[doc = "Field `RX_FIFO_EN` writer - Receive FIFO enabled for SPI transactions."]
pub type RX_FIFO_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_FIFO_EN_A>;
impl<'a, REG, const O: u8> RX_FIFO_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_EN_A::DISABLED)
    }
    #[doc = "Receive FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_EN_A::ENABLED)
    }
}
#[doc = "Field `RX_FIFO_CLEAR` reader - Clear RX FIFO, clear is accomplished by resetting the read and write pointers."]
pub type RX_FIFO_CLEAR_R = crate::BitReader<RX_FIFO_CLEAR_A>;
#[doc = "Clear RX FIFO, clear is accomplished by resetting the read and write pointers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_CLEAR_A {
    #[doc = "1: Clear the Receive FIFO, clears any pending RX FIFO status."]
    CLEAR = 1,
}
impl From<RX_FIFO_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_FIFO_CLEAR_A> {
        match self.bits {
            true => Some(RX_FIFO_CLEAR_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Clear the Receive FIFO, clears any pending RX FIFO status."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_FIFO_CLEAR_A::CLEAR
    }
}
#[doc = "Field `RX_FIFO_CLEAR` writer - Clear RX FIFO, clear is accomplished by resetting the read and write pointers."]
pub type RX_FIFO_CLEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_FIFO_CLEAR_A>;
impl<'a, REG, const O: u8> RX_FIFO_CLEAR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Receive FIFO, clears any pending RX FIFO status."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `RX_FIFO_CNT` reader - Count of entries in RX FIFO."]
pub type RX_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `RX_DMA_EN` reader - RX DMA Enable."]
pub type RX_DMA_EN_R = crate::BitReader<RX_DMA_EN_A>;
#[doc = "RX DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_DMA_EN_A {
    #[doc = "0: RX DMA requests are disabled, any pending DMA requests are cleared."]
    DISABLED = 0,
    #[doc = "1: RX DMA requests are enabled."]
    ENABLED = 1,
}
impl From<RX_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_DMA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DMA_EN_A {
        match self.bits {
            false => RX_DMA_EN_A::DISABLED,
            true => RX_DMA_EN_A::ENABLED,
        }
    }
    #[doc = "RX DMA requests are disabled, any pending DMA requests are cleared."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_DMA_EN_A::DISABLED
    }
    #[doc = "RX DMA requests are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_DMA_EN_A::ENABLED
    }
}
#[doc = "Field `RX_DMA_EN` writer - RX DMA Enable."]
pub type RX_DMA_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_DMA_EN_A>;
impl<'a, REG, const O: u8> RX_DMA_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX DMA requests are disabled, any pending DMA requests are cleared."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DMA_EN_A::DISABLED)
    }
    #[doc = "RX DMA requests are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DMA_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO level that will trigger a DMA request, also level for threshold status."]
    #[inline(always)]
    pub fn tx_fifo_level(&self) -> TX_FIFO_LEVEL_R {
        TX_FIFO_LEVEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Transmit FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TX_FIFO_EN_R {
        TX_FIFO_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear TX FIFO, clear is accomplished by resetting the read and write pointers."]
    #[inline(always)]
    pub fn tx_fifo_clear(&self) -> TX_FIFO_CLEAR_R {
        TX_FIFO_CLEAR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Count of entries in TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - TX DMA Enable."]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Receive FIFO level that will trigger a DMA request, also level for threshold status."]
    #[inline(always)]
    pub fn rx_fifo_level(&self) -> RX_FIFO_LEVEL_R {
        RX_FIFO_LEVEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Receive FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RX_FIFO_EN_R {
        RX_FIFO_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Clear RX FIFO, clear is accomplished by resetting the read and write pointers."]
    #[inline(always)]
    pub fn rx_fifo_clear(&self) -> RX_FIFO_CLEAR_R {
        RX_FIFO_CLEAR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Count of entries in RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - RX DMA Enable."]
    #[inline(always)]
    pub fn rx_dma_en(&self) -> RX_DMA_EN_R {
        RX_DMA_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO level that will trigger a DMA request, also level for threshold status."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_level(&mut self) -> TX_FIFO_LEVEL_W<DMA_SPEC, 0> {
        TX_FIFO_LEVEL_W::new(self)
    }
    #[doc = "Bit 6 - Transmit FIFO enabled for SPI transactions."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_en(&mut self) -> TX_FIFO_EN_W<DMA_SPEC, 6> {
        TX_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 7 - Clear TX FIFO, clear is accomplished by resetting the read and write pointers."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_clear(&mut self) -> TX_FIFO_CLEAR_W<DMA_SPEC, 7> {
        TX_FIFO_CLEAR_W::new(self)
    }
    #[doc = "Bit 15 - TX DMA Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W<DMA_SPEC, 15> {
        TX_DMA_EN_W::new(self)
    }
    #[doc = "Bits 16:20 - Receive FIFO level that will trigger a DMA request, also level for threshold status."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_level(&mut self) -> RX_FIFO_LEVEL_W<DMA_SPEC, 16> {
        RX_FIFO_LEVEL_W::new(self)
    }
    #[doc = "Bit 22 - Receive FIFO enabled for SPI transactions."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_en(&mut self) -> RX_FIFO_EN_W<DMA_SPEC, 22> {
        RX_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 23 - Clear RX FIFO, clear is accomplished by resetting the read and write pointers."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_clear(&mut self) -> RX_FIFO_CLEAR_W<DMA_SPEC, 23> {
        RX_FIFO_CLEAR_W::new(self)
    }
    #[doc = "Bit 31 - RX DMA Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma_en(&mut self) -> RX_DMA_EN_W<DMA_SPEC, 31> {
        RX_DMA_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
