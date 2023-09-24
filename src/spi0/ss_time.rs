#[doc = "Register `SS_TIME` reader"]
pub type R = crate::R<SS_TIME_SPEC>;
#[doc = "Register `SS_TIME` writer"]
pub type W = crate::W<SS_TIME_SPEC>;
#[doc = "Field `SSACT1` reader - Slave Select Pre delay."]
pub type SSACT1_R = crate::FieldReader;
#[doc = "Field `SSACT1` writer - Slave Select Pre delay."]
pub type SSACT1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 8, O>;
#[doc = "Field `SSACT2` reader - Slave Select Post delay."]
pub type SSACT2_R = crate::FieldReader;
#[doc = "Field `SSACT2` writer - Slave Select Post delay."]
pub type SSACT2_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 8, O>;
#[doc = "Field `SSINACT` reader - Slave Select Inactive delay."]
pub type SSINACT_R = crate::FieldReader;
#[doc = "Field `SSINACT` writer - Slave Select Inactive delay."]
pub type SSINACT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Slave Select Pre delay."]
    #[inline(always)]
    pub fn ssact1(&self) -> SSACT1_R {
        SSACT1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay."]
    #[inline(always)]
    pub fn ssact2(&self) -> SSACT2_R {
        SSACT2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn ssinact(&self) -> SSINACT_R {
        SSINACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Select Pre delay."]
    #[inline(always)]
    #[must_use]
    pub fn ssact1(&mut self) -> SSACT1_W<SS_TIME_SPEC, 0> {
        SSACT1_W::new(self)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay."]
    #[inline(always)]
    #[must_use]
    pub fn ssact2(&mut self) -> SSACT2_W<SS_TIME_SPEC, 8> {
        SSACT2_W::new(self)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    #[must_use]
    pub fn ssinact(&mut self) -> SSINACT_W<SS_TIME_SPEC, 16> {
        SSINACT_W::new(self)
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
#[doc = "Slave Select Timing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SS_TIME_SPEC;
impl crate::RegisterSpec for SS_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_time::R`](R) reader structure"]
impl crate::Readable for SS_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ss_time::W`](W) writer structure"]
impl crate::Writable for SS_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SS_TIME to value 0"]
impl crate::Resettable for SS_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
