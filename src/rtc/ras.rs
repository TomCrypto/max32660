#[doc = "Register `RAS` reader"]
pub type R = crate::R<RAS_SPEC>;
#[doc = "Register `RAS` writer"]
pub type W = crate::W<RAS_SPEC>;
#[doc = "Field `RAS` reader - Time-of-day Alarm."]
pub type RAS_R = crate::FieldReader<u32>;
#[doc = "Field `RAS` writer - Time-of-day Alarm."]
pub type RAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    pub fn ras(&self) -> RAS_R {
        RAS_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    #[must_use]
    pub fn ras(&mut self) -> RAS_W<RAS_SPEC, 0> {
        RAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Time-of-day Alarm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ras::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ras::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAS_SPEC;
impl crate::RegisterSpec for RAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ras::R`](R) reader structure"]
impl crate::Readable for RAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ras::W`](W) writer structure"]
impl crate::Writable for RAS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAS to value 0"]
impl crate::Resettable for RAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
