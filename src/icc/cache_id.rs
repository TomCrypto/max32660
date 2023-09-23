#[doc = "Register `CACHE_ID` reader"]
pub type R = crate::R<CACHE_ID_SPEC>;
#[doc = "Field `RELNUM` reader - Release Number."]
pub type RELNUM_R = crate::FieldReader;
#[doc = "Field `PARTNUM` reader - Part Number."]
pub type PARTNUM_R = crate::FieldReader;
#[doc = "Field `CCHID` reader - Cache ID."]
pub type CCHID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Release Number."]
    #[inline(always)]
    pub fn relnum(&self) -> RELNUM_R {
        RELNUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - Part Number."]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:15 - Cache ID."]
    #[inline(always)]
    pub fn cchid(&self) -> CCHID_R {
        CCHID_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
#[doc = "Cache ID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ID_SPEC;
impl crate::RegisterSpec for CACHE_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_id::R`](R) reader structure"]
impl crate::Readable for CACHE_ID_SPEC {}
#[doc = "`reset()` method sets CACHE_ID to value 0"]
impl crate::Resettable for CACHE_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
