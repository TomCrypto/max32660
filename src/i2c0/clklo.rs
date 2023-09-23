#[doc = "Register `CLKLO` reader"]
pub type R = crate::R<CLKLO_SPEC>;
#[doc = "Register `CLKLO` writer"]
pub type W = crate::W<CLKLO_SPEC>;
#[doc = "Field `SCL_LO` reader - SCL low period."]
pub type SCL_LO_R = crate::FieldReader<u16>;
#[doc = "Field `SCL_LO` writer - SCL low period."]
pub type SCL_LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - SCL low period."]
    #[inline(always)]
    pub fn scl_lo(&self) -> SCL_LO_R {
        SCL_LO_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - SCL low period."]
    #[inline(always)]
    #[must_use]
    pub fn scl_lo(&mut self) -> SCL_LO_W<CLKLO_SPEC, 0> {
        SCL_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Low.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clklo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clklo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKLO_SPEC;
impl crate::RegisterSpec for CLKLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clklo::R`](R) reader structure"]
impl crate::Readable for CLKLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clklo::W`](W) writer structure"]
impl crate::Writable for CLKLO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKLO to value 0"]
impl crate::Resettable for CLKLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
