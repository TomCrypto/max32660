#[doc = "Register `SYS_IE` reader"]
pub type R = crate::R<SYS_IE_SPEC>;
#[doc = "Register `SYS_IE` writer"]
pub type W = crate::W<SYS_IE_SPEC>;
#[doc = "Field `ICEULIE` reader - Arm ICE Unlocked Interrupt Enable."]
pub type ICEULIE_R = crate::BitReader;
#[doc = "Field `ICEULIE` writer - Arm ICE Unlocked Interrupt Enable."]
pub type ICEULIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Arm ICE Unlocked Interrupt Enable."]
    #[inline(always)]
    pub fn iceulie(&self) -> ICEULIE_R {
        ICEULIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Arm ICE Unlocked Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn iceulie(&mut self) -> ICEULIE_W<SYS_IE_SPEC, 0> {
        ICEULIE_W::new(self)
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
#[doc = "System Status Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_IE_SPEC;
impl crate::RegisterSpec for SYS_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ie::R`](R) reader structure"]
impl crate::Readable for SYS_IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_ie::W`](W) writer structure"]
impl crate::Writable for SYS_IE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYS_IE to value 0"]
impl crate::Resettable for SYS_IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
