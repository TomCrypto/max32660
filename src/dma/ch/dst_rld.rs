#[doc = "Register `DST_RLD` reader"]
pub type R = crate::R<DST_RLD_SPEC>;
#[doc = "Register `DST_RLD` writer"]
pub type W = crate::W<DST_RLD_SPEC>;
#[doc = "Field `DST_RLD` reader - Destination Address Reload Value."]
pub type DST_RLD_R = crate::FieldReader<u32>;
#[doc = "Field `DST_RLD` writer - Destination Address Reload Value."]
pub type DST_RLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
impl R {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
    #[inline(always)]
    pub fn dst_rld(&self) -> DST_RLD_R {
        DST_RLD_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
    #[inline(always)]
    #[must_use]
    pub fn dst_rld(&mut self) -> DST_RLD_W<DST_RLD_SPEC, 0> {
        DST_RLD_W::new(self)
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
#[doc = "Destination Address Reload Value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_rld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_rld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DST_RLD_SPEC;
impl crate::RegisterSpec for DST_RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_rld::R`](R) reader structure"]
impl crate::Readable for DST_RLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dst_rld::W`](W) writer structure"]
impl crate::Writable for DST_RLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DST_RLD to value 0"]
impl crate::Resettable for DST_RLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
