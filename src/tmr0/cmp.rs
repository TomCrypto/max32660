#[doc = "Register `CMP` reader"]
pub type R = crate::R<CMP_SPEC>;
#[doc = "Register `CMP` writer"]
pub type W = crate::W<CMP_SPEC>;
#[doc = "Field `COMPARE` reader - Compare value."]
pub type COMPARE_R = crate::FieldReader<u32>;
#[doc = "Field `COMPARE` writer - Compare value."]
pub type COMPARE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare value."]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare value."]
    #[inline(always)]
    #[must_use]
    pub fn compare(&mut self) -> COMPARE_W<CMP_SPEC, 0> {
        COMPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer Compare.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP_SPEC;
impl crate::RegisterSpec for CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp::R`](R) reader structure"]
impl crate::Readable for CMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp::W`](W) writer structure"]
impl crate::Writable for CMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP to value 0"]
impl crate::Resettable for CMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
