#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `BUSY` reader - SPI active status."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "SPI active status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: SPI not active."]
    NOT = 0,
    #[doc = "1: SPI active."]
    ACTIVE = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NOT,
            true => BUSY_A::ACTIVE,
        }
    }
    #[doc = "SPI not active."]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == BUSY_A::NOT
    }
    #[doc = "SPI active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BUSY_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - SPI active status."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SPI Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
