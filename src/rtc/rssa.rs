#[doc = "Register `RSSA` reader"]
pub type R = crate::R<RSSA_SPEC>;
#[doc = "Register `RSSA` writer"]
pub type W = crate::W<RSSA_SPEC>;
#[doc = "Field `RSSA` reader - Reload value for sub-second alarm."]
pub type RSSA_R = crate::FieldReader<u32>;
#[doc = "Field `RSSA` writer - Reload value for sub-second alarm."]
pub type RSSA_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Reload value for sub-second alarm."]
    #[inline(always)]
    pub fn rssa(&self) -> RSSA_R {
        RSSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reload value for sub-second alarm."]
    #[inline(always)]
    #[must_use]
    pub fn rssa(&mut self) -> RSSA_W<RSSA_SPEC, 0> {
        RSSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC sub-second alarm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rssa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rssa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSSA_SPEC;
impl crate::RegisterSpec for RSSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rssa::R`](R) reader structure"]
impl crate::Readable for RSSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rssa::W`](W) writer structure"]
impl crate::Writable for RSSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSSA to value 0"]
impl crate::Resettable for RSSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
