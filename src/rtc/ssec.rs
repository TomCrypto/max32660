#[doc = "Register `SSEC` reader"]
pub type R = crate::R<SSEC_SPEC>;
#[doc = "Register `SSEC` writer"]
pub type W = crate::W<SSEC_SPEC>;
#[doc = "Field `RTSS` reader - RTC Sub-second Counter."]
pub type RTSS_R = crate::FieldReader;
#[doc = "Field `RTSS` writer - RTC Sub-second Counter."]
pub type RTSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - RTC Sub-second Counter."]
    #[inline(always)]
    pub fn rtss(&self) -> RTSS_R {
        RTSS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC Sub-second Counter."]
    #[inline(always)]
    #[must_use]
    pub fn rtss(&mut self) -> RTSS_W<SSEC_SPEC, 0> {
        RTSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC Sub-second Counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSEC_SPEC;
impl crate::RegisterSpec for SSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssec::R`](R) reader structure"]
impl crate::Readable for SSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssec::W`](W) writer structure"]
impl crate::Writable for SSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSEC to value 0"]
impl crate::Resettable for SSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
