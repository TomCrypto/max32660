#[doc = "Register `LP_WAKEFL` reader"]
pub type R = crate::R<LP_WAKEFL_SPEC>;
#[doc = "Register `LP_WAKEFL` writer"]
pub type W = crate::W<LP_WAKEFL_SPEC>;
#[doc = "Field `WAKEST` reader - Wakeup IRQ flags (write ones to clear)."]
pub type WAKEST_R = crate::FieldReader<u16>;
#[doc = "Field `WAKEST` writer - Wakeup IRQ flags (write ones to clear)."]
pub type WAKEST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Wakeup IRQ flags (write ones to clear)."]
    #[inline(always)]
    pub fn wakest(&self) -> WAKEST_R {
        WAKEST_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Wakeup IRQ flags (write ones to clear)."]
    #[inline(always)]
    #[must_use]
    pub fn wakest(&mut self) -> WAKEST_W<LP_WAKEFL_SPEC, 0> {
        WAKEST_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Low Power Mode Wakeup Flags for GPIO0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_wakefl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_wakefl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_WAKEFL_SPEC;
impl crate::RegisterSpec for LP_WAKEFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_wakefl::R`](R) reader structure"]
impl crate::Readable for LP_WAKEFL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_wakefl::W`](W) writer structure"]
impl crate::Writable for LP_WAKEFL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_WAKEFL to value 0"]
impl crate::Resettable for LP_WAKEFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
