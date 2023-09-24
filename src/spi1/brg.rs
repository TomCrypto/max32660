#[doc = "Register `BRG` reader"]
pub type R = crate::R<BRG_SPEC>;
#[doc = "Register `BRG` writer"]
pub type W = crate::W<BRG_SPEC>;
#[doc = "Field `DIV` reader - Baud Rate Reload Value."]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Baud Rate Reload Value."]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Baud Rate Reload Value."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Reload Value."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<BRG_SPEC, 0> {
        DIV_W::new(self)
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
#[doc = "Baud Rate Reload Value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRG_SPEC;
impl crate::RegisterSpec for BRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brg::R`](R) reader structure"]
impl crate::Readable for BRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brg::W`](W) writer structure"]
impl crate::Writable for BRG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRG to value 0xffff"]
impl crate::Resettable for BRG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
