#[doc = "Register `LPWK_EN` reader"]
pub type R = crate::R<LPWK_EN_SPEC>;
#[doc = "Register `LPWK_EN` writer"]
pub type W = crate::W<LPWK_EN_SPEC>;
#[doc = "Field `WAKEEN` reader - Enable wakeup."]
pub type WAKEEN_R = crate::FieldReader<u16>;
#[doc = "Field `WAKEEN` writer - Enable wakeup."]
pub type WAKEEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Enable wakeup."]
    #[inline(always)]
    pub fn wakeen(&self) -> WAKEEN_R {
        WAKEEN_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Enable wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn wakeen(&mut self) -> WAKEEN_W<LPWK_EN_SPEC, 0> {
        WAKEEN_W::new(self)
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
#[doc = "Low Power I/O Wakeup Enable Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpwk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpwk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPWK_EN_SPEC;
impl crate::RegisterSpec for LPWK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpwk_en::R`](R) reader structure"]
impl crate::Readable for LPWK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpwk_en::W`](W) writer structure"]
impl crate::Writable for LPWK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPWK_EN to value 0"]
impl crate::Resettable for LPWK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
