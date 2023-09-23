#[doc = "Register `MEM_SIZE` reader"]
pub type R = crate::R<MEM_SIZE_SPEC>;
#[doc = "Field `CCHSZ` reader - Cache Size."]
pub type CCHSZ_R = crate::FieldReader<u16>;
#[doc = "Field `MEMSZ` reader - Main Memory Size."]
pub type MEMSZ_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Cache Size."]
    #[inline(always)]
    pub fn cchsz(&self) -> CCHSZ_R {
        CCHSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Main Memory Size."]
    #[inline(always)]
    pub fn memsz(&self) -> MEMSZ_R {
        MEMSZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Memory Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_SIZE_SPEC;
impl crate::RegisterSpec for MEM_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_size::R`](R) reader structure"]
impl crate::Readable for MEM_SIZE_SPEC {}
#[doc = "`reset()` method sets MEM_SIZE to value 0x0008_0008"]
impl crate::Resettable for MEM_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0008;
}
