#[doc = "Register `DMA` reader"]
pub type R = crate::R<DMA_SPEC>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DMA_SPEC>;
#[doc = "Field `TXDMA_EN` reader - TX DMA channel enable."]
pub type TXDMA_EN_R = crate::BitReader<TXDMA_EN_A>;
#[doc = "TX DMA channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMA_EN_A {
    #[doc = "0: DMA is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA is enabled."]
    ENABLED = 1,
}
impl From<TXDMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDMA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMA_EN_A {
        match self.bits {
            false => TXDMA_EN_A::DISABLED,
            true => TXDMA_EN_A::ENABLED,
        }
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMA_EN_A::DISABLED
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMA_EN_A::ENABLED
    }
}
#[doc = "Field `TXDMA_EN` writer - TX DMA channel enable."]
pub type TXDMA_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXDMA_EN_A>;
impl<'a, REG, const O: u8> TXDMA_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMA_EN_A::DISABLED)
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMA_EN_A::ENABLED)
    }
}
#[doc = "Field `RXDMA_EN` reader - RX DMA channel enable."]
pub type RXDMA_EN_R = crate::BitReader<RXDMA_EN_A>;
#[doc = "RX DMA channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMA_EN_A {
    #[doc = "0: DMA is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA is enabled."]
    ENABLED = 1,
}
impl From<RXDMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMA_EN_A {
        match self.bits {
            false => RXDMA_EN_A::DISABLED,
            true => RXDMA_EN_A::ENABLED,
        }
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMA_EN_A::DISABLED
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMA_EN_A::ENABLED
    }
}
#[doc = "Field `RXDMA_EN` writer - RX DMA channel enable."]
pub type RXDMA_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXDMA_EN_A>;
impl<'a, REG, const O: u8> RXDMA_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMA_EN_A::DISABLED)
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMA_EN_A::ENABLED)
    }
}
#[doc = "Field `TXDMA_LVL` reader - TX threshold for DMA transmission."]
pub type TXDMA_LVL_R = crate::FieldReader;
#[doc = "Field `TXDMA_LVL` writer - TX threshold for DMA transmission."]
pub type TXDMA_LVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `RXDMA_LVL` reader - RX threshold for DMA transmission."]
pub type RXDMA_LVL_R = crate::FieldReader;
#[doc = "Field `RXDMA_LVL` writer - RX threshold for DMA transmission."]
pub type RXDMA_LVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bit 0 - TX DMA channel enable."]
    #[inline(always)]
    pub fn txdma_en(&self) -> TXDMA_EN_R {
        TXDMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX DMA channel enable."]
    #[inline(always)]
    pub fn rxdma_en(&self) -> RXDMA_EN_R {
        RXDMA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:13 - TX threshold for DMA transmission."]
    #[inline(always)]
    pub fn txdma_lvl(&self) -> TXDMA_LVL_R {
        TXDMA_LVL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - RX threshold for DMA transmission."]
    #[inline(always)]
    pub fn rxdma_lvl(&self) -> RXDMA_LVL_R {
        RXDMA_LVL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX DMA channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn txdma_en(&mut self) -> TXDMA_EN_W<DMA_SPEC, 0> {
        TXDMA_EN_W::new(self)
    }
    #[doc = "Bit 1 - RX DMA channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn rxdma_en(&mut self) -> RXDMA_EN_W<DMA_SPEC, 1> {
        RXDMA_EN_W::new(self)
    }
    #[doc = "Bits 8:13 - TX threshold for DMA transmission."]
    #[inline(always)]
    #[must_use]
    pub fn txdma_lvl(&mut self) -> TXDMA_LVL_W<DMA_SPEC, 8> {
        TXDMA_LVL_W::new(self)
    }
    #[doc = "Bits 16:21 - RX threshold for DMA transmission."]
    #[inline(always)]
    #[must_use]
    pub fn rxdma_lvl(&mut self) -> RXDMA_LVL_W<DMA_SPEC, 16> {
        RXDMA_LVL_W::new(self)
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
#[doc = "DMA Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
