#[doc = "Register `RXCTRL1` reader"]
pub type R = crate::R<RXCTRL1_SPEC>;
#[doc = "Register `RXCTRL1` writer"]
pub type W = crate::W<RXCTRL1_SPEC>;
#[doc = "Field `RXCNT` reader - Receive Count Bits."]
pub type RXCNT_R = crate::FieldReader;
#[doc = "Field `RXCNT` writer - Receive Count Bits."]
pub type RXCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RXFIFO` reader - Receive FIFO Count."]
pub type RXFIFO_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive Count Bits."]
    #[inline(always)]
    pub fn rxcnt(&self) -> RXCNT_R {
        RXCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Receive FIFO Count."]
    #[inline(always)]
    pub fn rxfifo(&self) -> RXFIFO_R {
        RXFIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Count Bits."]
    #[inline(always)]
    #[must_use]
    pub fn rxcnt(&mut self) -> RXCNT_W<RXCTRL1_SPEC, 0> {
        RXCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Control Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCTRL1_SPEC;
impl crate::RegisterSpec for RXCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl1::R`](R) reader structure"]
impl crate::Readable for RXCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxctrl1::W`](W) writer structure"]
impl crate::Writable for RXCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCTRL1 to value 0"]
impl crate::Resettable for RXCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
