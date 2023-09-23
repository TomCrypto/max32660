#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `CFG_VALID` reader - Configuration Valid Flag."]
pub type CFG_VALID_R = crate::BitReader<CFG_VALID_A>;
#[doc = "Configuration Valid Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFG_VALID_A {
    #[doc = "0: config invalid."]
    INVALID = 0,
    #[doc = "1: config valid."]
    VALID = 1,
}
impl From<CFG_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: CFG_VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl CFG_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG_VALID_A {
        match self.bits {
            false => CFG_VALID_A::INVALID,
            true => CFG_VALID_A::VALID,
        }
    }
    #[doc = "config invalid."]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == CFG_VALID_A::INVALID
    }
    #[doc = "config valid."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == CFG_VALID_A::VALID
    }
}
#[doc = "Field `CFG_ERR` reader - Configuration Error Flag."]
pub type CFG_ERR_R = crate::BitReader<CFG_ERR_A>;
#[doc = "Configuration Error Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFG_ERR_A {
    #[doc = "0: config valid."]
    VALID = 0,
    #[doc = "1: config invalid."]
    INVALID = 1,
}
impl From<CFG_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CFG_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl CFG_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG_ERR_A {
        match self.bits {
            false => CFG_ERR_A::VALID,
            true => CFG_ERR_A::INVALID,
        }
    }
    #[doc = "config valid."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == CFG_ERR_A::VALID
    }
    #[doc = "config invalid."]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == CFG_ERR_A::INVALID
    }
}
impl R {
    #[doc = "Bit 0 - Configuration Valid Flag."]
    #[inline(always)]
    pub fn cfg_valid(&self) -> CFG_VALID_R {
        CFG_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Error Flag."]
    #[inline(always)]
    pub fn cfg_err(&self) -> CFG_ERR_R {
        CFG_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "System Initialization Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
