#[doc = "Register `TIMEOUT` reader"]
pub type R = crate::R<TIMEOUT_SPEC>;
#[doc = "Register `TIMEOUT` writer"]
pub type W = crate::W<TIMEOUT_SPEC>;
#[doc = "Field `TO` reader - Timeout."]
pub type TO_R = crate::FieldReader<u16>;
#[doc = "Field `TO` writer - Timeout."]
pub type TO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout."]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<TIMEOUT_SPEC, 0> {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timeout.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_SPEC;
impl crate::RegisterSpec for TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeout::W`](W) writer structure"]
impl crate::Writable for TIMEOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
