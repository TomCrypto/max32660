#[doc = "Register `SRC_RLD` reader"]
pub type R = crate::R<SRC_RLD_SPEC>;
#[doc = "Register `SRC_RLD` writer"]
pub type W = crate::W<SRC_RLD_SPEC>;
#[doc = "Field `SRC_RLD` reader - Source Address Reload Value."]
pub type SRC_RLD_R = crate::FieldReader<u32>;
#[doc = "Field `SRC_RLD` writer - Source Address Reload Value."]
pub type SRC_RLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
impl R {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    pub fn src_rld(&self) -> SRC_RLD_R {
        SRC_RLD_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    #[must_use]
    pub fn src_rld(&mut self) -> SRC_RLD_W<SRC_RLD_SPEC, 0> {
        SRC_RLD_W::new(self)
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
#[doc = "Source Address Reload Value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_rld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_rld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRC_RLD_SPEC;
impl crate::RegisterSpec for SRC_RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_rld::R`](R) reader structure"]
impl crate::Readable for SRC_RLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`src_rld::W`](W) writer structure"]
impl crate::Writable for SRC_RLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRC_RLD to value 0"]
impl crate::Resettable for SRC_RLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
