#[doc = "Register `FIFOLEN` reader"]
pub type R = crate::R<FIFOLEN_SPEC>;
#[doc = "Register `FIFOLEN` writer"]
pub type W = crate::W<FIFOLEN_SPEC>;
#[doc = "Field `RXLEN` reader - Receive FIFO Length."]
pub type RXLEN_R = crate::FieldReader;
#[doc = "Field `TXLEN` reader - Transmit FIFO Length."]
pub type TXLEN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Length."]
    #[inline(always)]
    pub fn rxlen(&self) -> RXLEN_R {
        RXLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit FIFO Length."]
    #[inline(always)]
    pub fn txlen(&self) -> TXLEN_R {
        TXLEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FIFO Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifolen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifolen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOLEN_SPEC;
impl crate::RegisterSpec for FIFOLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifolen::R`](R) reader structure"]
impl crate::Readable for FIFOLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifolen::W`](W) writer structure"]
impl crate::Writable for FIFOLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOLEN to value 0"]
impl crate::Resettable for FIFOLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
