#[doc = "Register `REV` reader"]
pub type R = crate::R<REV_SPEC>;
#[doc = "Field `REVISION` reader - Manufacturer Chip Revision."]
pub type REVISION_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Manufacturer Chip Revision."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Revision.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rev::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REV_SPEC;
impl crate::RegisterSpec for REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rev::R`](R) reader structure"]
impl crate::Readable for REV_SPEC {}
#[doc = "`reset()` method sets REV to value 0"]
impl crate::Resettable for REV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
