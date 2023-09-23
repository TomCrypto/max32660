#[doc = "Register `DMA` reader"]
pub type R = crate::R<DMA_SPEC>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DMA_SPEC>;
#[doc = "Field `TX_FIFO_LVL` reader - Transmit FIFO Level."]
pub type TX_FIFO_LVL_R = crate::FieldReader<TX_FIFO_LVL_A>;
#[doc = "Transmit FIFO Level.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_FIFO_LVL_A {
    #[doc = "0: `0`"]
    ENTRY1 = 0,
    #[doc = "1: `1`"]
    ENTRIES2 = 1,
    #[doc = "2: `10`"]
    ENTRIES3 = 2,
    #[doc = "3: `11`"]
    ENTRIES4 = 3,
    #[doc = "4: `100`"]
    ENTRIES5 = 4,
    #[doc = "5: `101`"]
    ENTRIES6 = 5,
    #[doc = "6: `110`"]
    ENTRIES7 = 6,
    #[doc = "7: `111`"]
    ENTRIES8 = 7,
}
impl From<TX_FIFO_LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_FIFO_LVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_FIFO_LVL_A {
    type Ux = u8;
}
impl TX_FIFO_LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFO_LVL_A {
        match self.bits {
            0 => TX_FIFO_LVL_A::ENTRY1,
            1 => TX_FIFO_LVL_A::ENTRIES2,
            2 => TX_FIFO_LVL_A::ENTRIES3,
            3 => TX_FIFO_LVL_A::ENTRIES4,
            4 => TX_FIFO_LVL_A::ENTRIES5,
            5 => TX_FIFO_LVL_A::ENTRIES6,
            6 => TX_FIFO_LVL_A::ENTRIES7,
            7 => TX_FIFO_LVL_A::ENTRIES8,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_entry1(&self) -> bool {
        *self == TX_FIFO_LVL_A::ENTRY1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_entries2(&self) -> bool {
        *self == TX_FIFO_LVL_A::ENTRIES2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_entries3(&self) -> bool {
        *self == TX_FIFO_LVL_A::ENTRIES3
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_entries4(&self) -> bool {
        *self == TX_FIFO_LVL_A::ENTRIES4
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_entries5(&self) -> bool {
        *self == TX_FIFO_LVL_A::ENTRIES5
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_entries6(&self) -> bool {
        *self == TX_FIFO_LVL_A::ENTRIES6
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_entries7(&self) -> bool {
        *self == TX_FIFO_LVL_A::ENTRIES7
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_entries8(&self) -> bool {
        *self == TX_FIFO_LVL_A::ENTRIES8
    }
}
#[doc = "Field `TX_FIFO_LVL` writer - Transmit FIFO Level."]
pub type TX_FIFO_LVL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, TX_FIFO_LVL_A>;
impl<'a, REG, const O: u8> TX_FIFO_LVL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn entry1(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_LVL_A::ENTRY1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn entries2(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_LVL_A::ENTRIES2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn entries3(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_LVL_A::ENTRIES3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn entries4(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_LVL_A::ENTRIES4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn entries5(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_LVL_A::ENTRIES5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn entries6(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_LVL_A::ENTRIES6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn entries7(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_LVL_A::ENTRIES7)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn entries8(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_LVL_A::ENTRIES8)
    }
}
#[doc = "Transmit FIFO Clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_FIFO_CLR_AW {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<TX_FIFO_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_CLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_CLR` writer - Transmit FIFO Clear."]
pub type TX_FIFO_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_FIFO_CLR_AW>;
impl<'a, REG, const O: u8> TX_FIFO_CLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_CLR_AW::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_CLR_AW::START)
    }
}
#[doc = "Field `TX_FIFO_CNT` reader - Transmit FIFO Count."]
pub type TX_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `TX_DMA_EN` reader - Transmit DMA Enable."]
pub type TX_DMA_EN_R = crate::BitReader<TX_DMA_EN_A>;
#[doc = "Transmit DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_DMA_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
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
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_DMA_EN_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_DMA_EN_A::ENABLED
    }
}
#[doc = "Field `TX_DMA_EN` writer - Transmit DMA Enable."]
pub type TX_DMA_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_DMA_EN_A>;
impl<'a, REG, const O: u8> TX_DMA_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_DMA_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_DMA_EN_A::ENABLED)
    }
}
#[doc = "Field `RX_FIFO_LVL` reader - Receive FIFO Level."]
pub type RX_FIFO_LVL_R = crate::FieldReader<RX_FIFO_LVL_A>;
#[doc = "Receive FIFO Level.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_FIFO_LVL_A {
    #[doc = "0: `0`"]
    ENTRY1 = 0,
    #[doc = "1: `1`"]
    ENTRIES2 = 1,
    #[doc = "2: `10`"]
    ENTRIES3 = 2,
    #[doc = "3: `11`"]
    ENTRIES4 = 3,
    #[doc = "4: `100`"]
    ENTRIES5 = 4,
    #[doc = "5: `101`"]
    ENTRIES6 = 5,
    #[doc = "6: `110`"]
    ENTRIES7 = 6,
    #[doc = "7: `111`"]
    ENTRIES8 = 7,
}
impl From<RX_FIFO_LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FIFO_LVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_FIFO_LVL_A {
    type Ux = u8;
}
impl RX_FIFO_LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_LVL_A {
        match self.bits {
            0 => RX_FIFO_LVL_A::ENTRY1,
            1 => RX_FIFO_LVL_A::ENTRIES2,
            2 => RX_FIFO_LVL_A::ENTRIES3,
            3 => RX_FIFO_LVL_A::ENTRIES4,
            4 => RX_FIFO_LVL_A::ENTRIES5,
            5 => RX_FIFO_LVL_A::ENTRIES6,
            6 => RX_FIFO_LVL_A::ENTRIES7,
            7 => RX_FIFO_LVL_A::ENTRIES8,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_entry1(&self) -> bool {
        *self == RX_FIFO_LVL_A::ENTRY1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_entries2(&self) -> bool {
        *self == RX_FIFO_LVL_A::ENTRIES2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_entries3(&self) -> bool {
        *self == RX_FIFO_LVL_A::ENTRIES3
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_entries4(&self) -> bool {
        *self == RX_FIFO_LVL_A::ENTRIES4
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_entries5(&self) -> bool {
        *self == RX_FIFO_LVL_A::ENTRIES5
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_entries6(&self) -> bool {
        *self == RX_FIFO_LVL_A::ENTRIES6
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_entries7(&self) -> bool {
        *self == RX_FIFO_LVL_A::ENTRIES7
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_entries8(&self) -> bool {
        *self == RX_FIFO_LVL_A::ENTRIES8
    }
}
#[doc = "Field `RX_FIFO_LVL` writer - Receive FIFO Level."]
pub type RX_FIFO_LVL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, RX_FIFO_LVL_A>;
impl<'a, REG, const O: u8> RX_FIFO_LVL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn entry1(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_LVL_A::ENTRY1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn entries2(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_LVL_A::ENTRIES2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn entries3(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_LVL_A::ENTRIES3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn entries4(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_LVL_A::ENTRIES4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn entries5(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_LVL_A::ENTRIES5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn entries6(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_LVL_A::ENTRIES6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn entries7(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_LVL_A::ENTRIES7)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn entries8(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_LVL_A::ENTRIES8)
    }
}
#[doc = "Field `RX_FIFO_CLR` reader - Receive FIFO Clear."]
pub type RX_FIFO_CLR_R = crate::BitReader<RX_FIFO_CLR_A>;
#[doc = "Receive FIFO Clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_CLR_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<RX_FIFO_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_CLR_A {
        match self.bits {
            false => RX_FIFO_CLR_A::COMPLETE,
            true => RX_FIFO_CLR_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == RX_FIFO_CLR_A::COMPLETE
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RX_FIFO_CLR_A::START
    }
}
#[doc = "Field `RX_FIFO_CLR` writer - Receive FIFO Clear."]
pub type RX_FIFO_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_FIFO_CLR_A>;
impl<'a, REG, const O: u8> RX_FIFO_CLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_CLR_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_CLR_A::START)
    }
}
#[doc = "Field `RX_FIFO_CNT` reader - Receive FIFO Count."]
pub type RX_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `RX_DMA_EN` reader - Receive DMA Enable."]
pub type RX_DMA_EN_R = crate::BitReader<RX_DMA_EN_A>;
#[doc = "Receive DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_DMA_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
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
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_DMA_EN_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_DMA_EN_A::ENABLED
    }
}
#[doc = "Field `RX_DMA_EN` writer - Receive DMA Enable."]
pub type RX_DMA_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RX_DMA_EN_A>;
impl<'a, REG, const O: u8> RX_DMA_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DMA_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DMA_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit FIFO Level."]
    #[inline(always)]
    pub fn tx_fifo_lvl(&self) -> TX_FIFO_LVL_R {
        TX_FIFO_LVL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Count."]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Transmit DMA Enable."]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Receive FIFO Level."]
    #[inline(always)]
    pub fn rx_fifo_lvl(&self) -> RX_FIFO_LVL_R {
        RX_FIFO_LVL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Receive FIFO Clear."]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RX_FIFO_CLR_R {
        RX_FIFO_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Receive FIFO Count."]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Receive DMA Enable."]
    #[inline(always)]
    pub fn rx_dma_en(&self) -> RX_DMA_EN_R {
        RX_DMA_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit FIFO Level."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_lvl(&mut self) -> TX_FIFO_LVL_W<DMA_SPEC, 0> {
        TX_FIFO_LVL_W::new(self)
    }
    #[doc = "Bit 4 - Transmit FIFO Clear."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_clr(&mut self) -> TX_FIFO_CLR_W<DMA_SPEC, 4> {
        TX_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Transmit DMA Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W<DMA_SPEC, 15> {
        TX_DMA_EN_W::new(self)
    }
    #[doc = "Bits 16:18 - Receive FIFO Level."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_lvl(&mut self) -> RX_FIFO_LVL_W<DMA_SPEC, 16> {
        RX_FIFO_LVL_W::new(self)
    }
    #[doc = "Bit 20 - Receive FIFO Clear."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_clr(&mut self) -> RX_FIFO_CLR_W<DMA_SPEC, 20> {
        RX_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 31 - Receive DMA Enable."]
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
#[doc = "SPI DMA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets DMA to value 0x0007_0007"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0007;
}
