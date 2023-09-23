#[doc = "Register `INT_STAT` reader"]
pub type R = crate::R<INT_STAT_SPEC>;
#[doc = "Field `p0` reader - Interrupt status for pin 0."]
pub type P0_R = crate::BitReader<P0_A>;
#[doc = "Interrupt status for pin 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_A {
    #[doc = "0: Interrupt not pending for pin 0."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 0."]
    PENDING = 1,
}
impl From<P0_A> for bool {
    #[inline(always)]
    fn from(variant: P0_A) -> Self {
        variant as u8 != 0
    }
}
impl P0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_A {
        match self.bits {
            false => P0_A::NOT_PENDING,
            true => P0_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 0."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P0_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 0."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P0_A::PENDING
    }
}
#[doc = "Field `p1` reader - Interrupt status for pin 1."]
pub type P1_R = crate::BitReader<P1_A>;
#[doc = "Interrupt status for pin 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_A {
    #[doc = "0: Interrupt not pending for pin 1."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 1."]
    PENDING = 1,
}
impl From<P1_A> for bool {
    #[inline(always)]
    fn from(variant: P1_A) -> Self {
        variant as u8 != 0
    }
}
impl P1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_A {
        match self.bits {
            false => P1_A::NOT_PENDING,
            true => P1_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 1."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P1_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 1."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P1_A::PENDING
    }
}
#[doc = "Field `p2` reader - Interrupt status for pin 2."]
pub type P2_R = crate::BitReader<P2_A>;
#[doc = "Interrupt status for pin 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_A {
    #[doc = "0: Interrupt not pending for pin 2."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 2."]
    PENDING = 1,
}
impl From<P2_A> for bool {
    #[inline(always)]
    fn from(variant: P2_A) -> Self {
        variant as u8 != 0
    }
}
impl P2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_A {
        match self.bits {
            false => P2_A::NOT_PENDING,
            true => P2_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 2."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P2_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 2."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P2_A::PENDING
    }
}
#[doc = "Field `p3` reader - Interrupt status for pin 3."]
pub type P3_R = crate::BitReader<P3_A>;
#[doc = "Interrupt status for pin 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3_A {
    #[doc = "0: Interrupt not pending for pin 3."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 3."]
    PENDING = 1,
}
impl From<P3_A> for bool {
    #[inline(always)]
    fn from(variant: P3_A) -> Self {
        variant as u8 != 0
    }
}
impl P3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P3_A {
        match self.bits {
            false => P3_A::NOT_PENDING,
            true => P3_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 3."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P3_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 3."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P3_A::PENDING
    }
}
#[doc = "Field `p4` reader - Interrupt status for pin 4."]
pub type P4_R = crate::BitReader<P4_A>;
#[doc = "Interrupt status for pin 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4_A {
    #[doc = "0: Interrupt not pending for pin 4."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 4."]
    PENDING = 1,
}
impl From<P4_A> for bool {
    #[inline(always)]
    fn from(variant: P4_A) -> Self {
        variant as u8 != 0
    }
}
impl P4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P4_A {
        match self.bits {
            false => P4_A::NOT_PENDING,
            true => P4_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 4."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P4_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 4."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P4_A::PENDING
    }
}
#[doc = "Field `p5` reader - Interrupt status for pin 5."]
pub type P5_R = crate::BitReader<P5_A>;
#[doc = "Interrupt status for pin 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5_A {
    #[doc = "0: Interrupt not pending for pin 5."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 5."]
    PENDING = 1,
}
impl From<P5_A> for bool {
    #[inline(always)]
    fn from(variant: P5_A) -> Self {
        variant as u8 != 0
    }
}
impl P5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P5_A {
        match self.bits {
            false => P5_A::NOT_PENDING,
            true => P5_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 5."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P5_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 5."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P5_A::PENDING
    }
}
#[doc = "Field `p6` reader - Interrupt status for pin 6."]
pub type P6_R = crate::BitReader<P6_A>;
#[doc = "Interrupt status for pin 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6_A {
    #[doc = "0: Interrupt not pending for pin 6."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 6."]
    PENDING = 1,
}
impl From<P6_A> for bool {
    #[inline(always)]
    fn from(variant: P6_A) -> Self {
        variant as u8 != 0
    }
}
impl P6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P6_A {
        match self.bits {
            false => P6_A::NOT_PENDING,
            true => P6_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 6."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P6_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 6."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P6_A::PENDING
    }
}
#[doc = "Field `p7` reader - Interrupt status for pin 7."]
pub type P7_R = crate::BitReader<P7_A>;
#[doc = "Interrupt status for pin 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7_A {
    #[doc = "0: Interrupt not pending for pin 7."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 7."]
    PENDING = 1,
}
impl From<P7_A> for bool {
    #[inline(always)]
    fn from(variant: P7_A) -> Self {
        variant as u8 != 0
    }
}
impl P7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P7_A {
        match self.bits {
            false => P7_A::NOT_PENDING,
            true => P7_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 7."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P7_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 7."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P7_A::PENDING
    }
}
#[doc = "Field `p8` reader - Interrupt status for pin 8."]
pub type P8_R = crate::BitReader<P8_A>;
#[doc = "Interrupt status for pin 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8_A {
    #[doc = "0: Interrupt not pending for pin 8."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 8."]
    PENDING = 1,
}
impl From<P8_A> for bool {
    #[inline(always)]
    fn from(variant: P8_A) -> Self {
        variant as u8 != 0
    }
}
impl P8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P8_A {
        match self.bits {
            false => P8_A::NOT_PENDING,
            true => P8_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 8."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P8_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 8."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P8_A::PENDING
    }
}
#[doc = "Field `p9` reader - Interrupt status for pin 9."]
pub type P9_R = crate::BitReader<P9_A>;
#[doc = "Interrupt status for pin 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9_A {
    #[doc = "0: Interrupt not pending for pin 9."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 9."]
    PENDING = 1,
}
impl From<P9_A> for bool {
    #[inline(always)]
    fn from(variant: P9_A) -> Self {
        variant as u8 != 0
    }
}
impl P9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P9_A {
        match self.bits {
            false => P9_A::NOT_PENDING,
            true => P9_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 9."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P9_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 9."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P9_A::PENDING
    }
}
#[doc = "Field `p10` reader - Interrupt status for pin 10."]
pub type P10_R = crate::BitReader<P10_A>;
#[doc = "Interrupt status for pin 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10_A {
    #[doc = "0: Interrupt not pending for pin 10."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 10."]
    PENDING = 1,
}
impl From<P10_A> for bool {
    #[inline(always)]
    fn from(variant: P10_A) -> Self {
        variant as u8 != 0
    }
}
impl P10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P10_A {
        match self.bits {
            false => P10_A::NOT_PENDING,
            true => P10_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 10."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P10_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 10."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P10_A::PENDING
    }
}
#[doc = "Field `p11` reader - Interrupt status for pin 11."]
pub type P11_R = crate::BitReader<P11_A>;
#[doc = "Interrupt status for pin 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11_A {
    #[doc = "0: Interrupt not pending for pin 11."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 11."]
    PENDING = 1,
}
impl From<P11_A> for bool {
    #[inline(always)]
    fn from(variant: P11_A) -> Self {
        variant as u8 != 0
    }
}
impl P11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P11_A {
        match self.bits {
            false => P11_A::NOT_PENDING,
            true => P11_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 11."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P11_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 11."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P11_A::PENDING
    }
}
#[doc = "Field `p12` reader - Interrupt status for pin 12."]
pub type P12_R = crate::BitReader<P12_A>;
#[doc = "Interrupt status for pin 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12_A {
    #[doc = "0: Interrupt not pending for pin 12."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 12."]
    PENDING = 1,
}
impl From<P12_A> for bool {
    #[inline(always)]
    fn from(variant: P12_A) -> Self {
        variant as u8 != 0
    }
}
impl P12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P12_A {
        match self.bits {
            false => P12_A::NOT_PENDING,
            true => P12_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 12."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P12_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 12."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P12_A::PENDING
    }
}
#[doc = "Field `p13` reader - Interrupt status for pin 13."]
pub type P13_R = crate::BitReader<P13_A>;
#[doc = "Interrupt status for pin 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13_A {
    #[doc = "0: Interrupt not pending for pin 13."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending for pin 13."]
    PENDING = 1,
}
impl From<P13_A> for bool {
    #[inline(always)]
    fn from(variant: P13_A) -> Self {
        variant as u8 != 0
    }
}
impl P13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P13_A {
        match self.bits {
            false => P13_A::NOT_PENDING,
            true => P13_A::PENDING,
        }
    }
    #[doc = "Interrupt not pending for pin 13."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P13_A::NOT_PENDING
    }
    #[doc = "Interrupt pending for pin 13."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P13_A::PENDING
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt status for pin 0."]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for pin 1."]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for pin 2."]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for pin 3."]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for pin 4."]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status for pin 5."]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status for pin 6."]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt status for pin 7."]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt status for pin 8."]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt status for pin 9."]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt status for pin 10."]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt status for pin 11."]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status for pin 12."]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt status for pin 13."]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Interrupt Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STAT_SPEC;
impl crate::RegisterSpec for INT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_stat::R`](R) reader structure"]
impl crate::Readable for INT_STAT_SPEC {}
#[doc = "`reset()` method sets INT_STAT to value 0"]
impl crate::Resettable for INT_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
