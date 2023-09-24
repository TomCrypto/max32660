#[doc = "Register `EVTEN` reader"]
pub type R = crate::R<EVTEN_SPEC>;
#[doc = "Register `EVTEN` writer"]
pub type W = crate::W<EVTEN_SPEC>;
#[doc = "Field `DMAEVENT` reader - Enable DMA event."]
pub type DMAEVENT_R = crate::BitReader;
#[doc = "Field `DMAEVENT` writer - Enable DMA event."]
pub type DMAEVENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_EVT` reader - Enable RXEV pin event."]
pub type RX_EVT_R = crate::BitReader;
#[doc = "Field `RX_EVT` writer - Enable RXEV pin event."]
pub type RX_EVT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable DMA event."]
    #[inline(always)]
    pub fn dmaevent(&self) -> DMAEVENT_R {
        DMAEVENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RXEV pin event."]
    #[inline(always)]
    pub fn rx_evt(&self) -> RX_EVT_R {
        RX_EVT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA event."]
    #[inline(always)]
    #[must_use]
    pub fn dmaevent(&mut self) -> DMAEVENT_W<EVTEN_SPEC, 0> {
        DMAEVENT_W::new(self)
    }
    #[doc = "Bit 1 - Enable RXEV pin event."]
    #[inline(always)]
    #[must_use]
    pub fn rx_evt(&mut self) -> RX_EVT_W<EVTEN_SPEC, 1> {
        RX_EVT_W::new(self)
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
#[doc = "Event Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVTEN_SPEC;
impl crate::RegisterSpec for EVTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evten::R`](R) reader structure"]
impl crate::Readable for EVTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evten::W`](W) writer structure"]
impl crate::Writable for EVTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTEN to value 0"]
impl crate::Resettable for EVTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
