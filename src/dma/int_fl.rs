#[doc = "Register `INT_FL` reader"]
pub type R = crate::R<INT_FL_SPEC>;
#[doc = "Field `IPEND0` reader - Channel 0 Interrupt Pending."]
pub type IPEND0_R = crate::BitReader<IPEND0_A>;
#[doc = "Channel 0 Interrupt Pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEND0_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IPEND0_A> for bool {
    #[inline(always)]
    fn from(variant: IPEND0_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEND0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEND0_A {
        match self.bits {
            false => IPEND0_A::INACTIVE,
            true => IPEND0_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IPEND0_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IPEND0_A::PENDING
    }
}
#[doc = "Field `IPEND1` reader - Channel 1 Interrupt Pending."]
pub type IPEND1_R = crate::BitReader<IPEND1_A>;
#[doc = "Channel 1 Interrupt Pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEND1_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IPEND1_A> for bool {
    #[inline(always)]
    fn from(variant: IPEND1_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEND1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEND1_A {
        match self.bits {
            false => IPEND1_A::INACTIVE,
            true => IPEND1_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IPEND1_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IPEND1_A::PENDING
    }
}
#[doc = "Field `IPEND2` reader - Channel 2 Interrupt Pending."]
pub type IPEND2_R = crate::BitReader<IPEND2_A>;
#[doc = "Channel 2 Interrupt Pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEND2_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IPEND2_A> for bool {
    #[inline(always)]
    fn from(variant: IPEND2_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEND2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEND2_A {
        match self.bits {
            false => IPEND2_A::INACTIVE,
            true => IPEND2_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IPEND2_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IPEND2_A::PENDING
    }
}
#[doc = "Field `IPEND3` reader - Channel 3 Interrupt Pending."]
pub type IPEND3_R = crate::BitReader<IPEND3_A>;
#[doc = "Channel 3 Interrupt Pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEND3_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IPEND3_A> for bool {
    #[inline(always)]
    fn from(variant: IPEND3_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEND3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEND3_A {
        match self.bits {
            false => IPEND3_A::INACTIVE,
            true => IPEND3_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IPEND3_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IPEND3_A::PENDING
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Interrupt Pending."]
    #[inline(always)]
    pub fn ipend0(&self) -> IPEND0_R {
        IPEND0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Pending."]
    #[inline(always)]
    pub fn ipend1(&self) -> IPEND1_R {
        IPEND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Pending."]
    #[inline(always)]
    pub fn ipend2(&self) -> IPEND2_R {
        IPEND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Pending."]
    #[inline(always)]
    pub fn ipend3(&self) -> IPEND3_R {
        IPEND3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DMA Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FL_SPEC;
impl crate::RegisterSpec for INT_FL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_fl::R`](R) reader structure"]
impl crate::Readable for INT_FL_SPEC {}
#[doc = "`reset()` method sets INT_FL to value 0"]
impl crate::Resettable for INT_FL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
