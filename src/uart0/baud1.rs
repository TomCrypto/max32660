#[doc = "Register `BAUD1` reader"]
pub type R = crate::R<BAUD1_SPEC>;
#[doc = "Register `BAUD1` writer"]
pub type W = crate::W<BAUD1_SPEC>;
#[doc = "Field `DBAUD` reader - Decimal portion of baud rate divisor value."]
pub type DBAUD_R = crate::FieldReader<u16>;
#[doc = "Field `DBAUD` writer - Decimal portion of baud rate divisor value."]
pub type DBAUD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Decimal portion of baud rate divisor value."]
    #[inline(always)]
    pub fn dbaud(&self) -> DBAUD_R {
        DBAUD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Decimal portion of baud rate divisor value."]
    #[inline(always)]
    #[must_use]
    pub fn dbaud(&mut self) -> DBAUD_W<BAUD1_SPEC, 0> {
        DBAUD_W::new(self)
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
#[doc = "Baud rate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUD1_SPEC;
impl crate::RegisterSpec for BAUD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baud1::R`](R) reader structure"]
impl crate::Readable for BAUD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baud1::W`](W) writer structure"]
impl crate::Writable for BAUD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUD1 to value 0"]
impl crate::Resettable for BAUD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
