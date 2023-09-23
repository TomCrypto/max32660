#[doc = "Register `TXFIFO` reader"]
pub type R = crate::R<TXFIFO_SPEC>;
#[doc = "Register `TXFIFO` writer"]
pub type W = crate::W<TXFIFO_SPEC>;
#[doc = "Field `DATA` reader - Reading from this field returns the next character available at the output of the TX FIFO (if one is available, otherwise 00h is returned)."]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - Reading from this field returns the next character available at the output of the TX FIFO (if one is available, otherwise 00h is returned)."]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Reading from this field returns the next character available at the output of the TX FIFO (if one is available, otherwise 00h is returned)."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Reading from this field returns the next character available at the output of the TX FIFO (if one is available, otherwise 00h is returned)."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TXFIFO_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit FIFO Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFIFO_SPEC;
impl crate::RegisterSpec for TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfifo::R`](R) reader structure"]
impl crate::Readable for TXFIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txfifo::W`](W) writer structure"]
impl crate::Writable for TXFIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
