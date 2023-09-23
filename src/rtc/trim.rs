#[doc = "Register `TRIM` reader"]
pub type R = crate::R<TRIM_SPEC>;
#[doc = "Register `TRIM` writer"]
pub type W = crate::W<TRIM_SPEC>;
#[doc = "Field `TRIM` reader - RTC Trim."]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - RTC Trim."]
pub type TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `VBATTMR` reader - VBAT Timer Value."]
pub type VBATTMR_R = crate::FieldReader<u32>;
#[doc = "Field `VBATTMR` writer - VBAT Timer Value."]
pub type VBATTMR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:7 - RTC Trim."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - VBAT Timer Value."]
    #[inline(always)]
    pub fn vbattmr(&self) -> VBATTMR_R {
        VBATTMR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC Trim."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<TRIM_SPEC, 0> {
        TRIM_W::new(self)
    }
    #[doc = "Bits 8:31 - VBAT Timer Value."]
    #[inline(always)]
    #[must_use]
    pub fn vbattmr(&mut self) -> VBATTMR_W<TRIM_SPEC, 8> {
        VBATTMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC Trim.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIM_SPEC;
impl crate::RegisterSpec for TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trim::R`](R) reader structure"]
impl crate::Readable for TRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trim::W`](W) writer structure"]
impl crate::Writable for TRIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIM to value 0"]
impl crate::Resettable for TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
