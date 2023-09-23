#[doc = "Register `IN` reader"]
pub type R = crate::R<IN_SPEC>;
#[doc = "Field `p0` reader - Input level for pin 0."]
pub type P0_R = crate::BitReader<P0_A>;
#[doc = "Input level for pin 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_A {
    #[doc = "0: Low for pin 0."]
    LOW = 0,
    #[doc = "1: High for pin 0."]
    HIGH = 1,
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
            false => P0_A::LOW,
            true => P0_A::HIGH,
        }
    }
    #[doc = "Low for pin 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P0_A::LOW
    }
    #[doc = "High for pin 0."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P0_A::HIGH
    }
}
#[doc = "Field `p1` reader - Input level for pin 1."]
pub type P1_R = crate::BitReader<P1_A>;
#[doc = "Input level for pin 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_A {
    #[doc = "0: Low for pin 1."]
    LOW = 0,
    #[doc = "1: High for pin 1."]
    HIGH = 1,
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
            false => P1_A::LOW,
            true => P1_A::HIGH,
        }
    }
    #[doc = "Low for pin 1."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P1_A::LOW
    }
    #[doc = "High for pin 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P1_A::HIGH
    }
}
#[doc = "Field `p2` reader - Input level for pin 2."]
pub type P2_R = crate::BitReader<P2_A>;
#[doc = "Input level for pin 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_A {
    #[doc = "0: Low for pin 2."]
    LOW = 0,
    #[doc = "1: High for pin 2."]
    HIGH = 1,
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
            false => P2_A::LOW,
            true => P2_A::HIGH,
        }
    }
    #[doc = "Low for pin 2."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P2_A::LOW
    }
    #[doc = "High for pin 2."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P2_A::HIGH
    }
}
#[doc = "Field `p3` reader - Input level for pin 3."]
pub type P3_R = crate::BitReader<P3_A>;
#[doc = "Input level for pin 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3_A {
    #[doc = "0: Low for pin 3."]
    LOW = 0,
    #[doc = "1: High for pin 3."]
    HIGH = 1,
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
            false => P3_A::LOW,
            true => P3_A::HIGH,
        }
    }
    #[doc = "Low for pin 3."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P3_A::LOW
    }
    #[doc = "High for pin 3."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P3_A::HIGH
    }
}
#[doc = "Field `p4` reader - Input level for pin 4."]
pub type P4_R = crate::BitReader<P4_A>;
#[doc = "Input level for pin 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4_A {
    #[doc = "0: Low for pin 4."]
    LOW = 0,
    #[doc = "1: High for pin 4."]
    HIGH = 1,
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
            false => P4_A::LOW,
            true => P4_A::HIGH,
        }
    }
    #[doc = "Low for pin 4."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P4_A::LOW
    }
    #[doc = "High for pin 4."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P4_A::HIGH
    }
}
#[doc = "Field `p5` reader - Input level for pin 5."]
pub type P5_R = crate::BitReader<P5_A>;
#[doc = "Input level for pin 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5_A {
    #[doc = "0: Low for pin 5."]
    LOW = 0,
    #[doc = "1: High for pin 5."]
    HIGH = 1,
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
            false => P5_A::LOW,
            true => P5_A::HIGH,
        }
    }
    #[doc = "Low for pin 5."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P5_A::LOW
    }
    #[doc = "High for pin 5."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P5_A::HIGH
    }
}
#[doc = "Field `p6` reader - Input level for pin 6."]
pub type P6_R = crate::BitReader<P6_A>;
#[doc = "Input level for pin 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6_A {
    #[doc = "0: Low for pin 6."]
    LOW = 0,
    #[doc = "1: High for pin 6."]
    HIGH = 1,
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
            false => P6_A::LOW,
            true => P6_A::HIGH,
        }
    }
    #[doc = "Low for pin 6."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P6_A::LOW
    }
    #[doc = "High for pin 6."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P6_A::HIGH
    }
}
#[doc = "Field `p7` reader - Input level for pin 7."]
pub type P7_R = crate::BitReader<P7_A>;
#[doc = "Input level for pin 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7_A {
    #[doc = "0: Low for pin 7."]
    LOW = 0,
    #[doc = "1: High for pin 7."]
    HIGH = 1,
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
            false => P7_A::LOW,
            true => P7_A::HIGH,
        }
    }
    #[doc = "Low for pin 7."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P7_A::LOW
    }
    #[doc = "High for pin 7."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P7_A::HIGH
    }
}
#[doc = "Field `p8` reader - Input level for pin 8."]
pub type P8_R = crate::BitReader<P8_A>;
#[doc = "Input level for pin 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8_A {
    #[doc = "0: Low for pin 8."]
    LOW = 0,
    #[doc = "1: High for pin 8."]
    HIGH = 1,
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
            false => P8_A::LOW,
            true => P8_A::HIGH,
        }
    }
    #[doc = "Low for pin 8."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P8_A::LOW
    }
    #[doc = "High for pin 8."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P8_A::HIGH
    }
}
#[doc = "Field `p9` reader - Input level for pin 9."]
pub type P9_R = crate::BitReader<P9_A>;
#[doc = "Input level for pin 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9_A {
    #[doc = "0: Low for pin 9."]
    LOW = 0,
    #[doc = "1: High for pin 9."]
    HIGH = 1,
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
            false => P9_A::LOW,
            true => P9_A::HIGH,
        }
    }
    #[doc = "Low for pin 9."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P9_A::LOW
    }
    #[doc = "High for pin 9."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P9_A::HIGH
    }
}
#[doc = "Field `p10` reader - Input level for pin 10."]
pub type P10_R = crate::BitReader<P10_A>;
#[doc = "Input level for pin 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10_A {
    #[doc = "0: Low for pin 10."]
    LOW = 0,
    #[doc = "1: High for pin 10."]
    HIGH = 1,
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
            false => P10_A::LOW,
            true => P10_A::HIGH,
        }
    }
    #[doc = "Low for pin 10."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P10_A::LOW
    }
    #[doc = "High for pin 10."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P10_A::HIGH
    }
}
#[doc = "Field `p11` reader - Input level for pin 11."]
pub type P11_R = crate::BitReader<P11_A>;
#[doc = "Input level for pin 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11_A {
    #[doc = "0: Low for pin 11."]
    LOW = 0,
    #[doc = "1: High for pin 11."]
    HIGH = 1,
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
            false => P11_A::LOW,
            true => P11_A::HIGH,
        }
    }
    #[doc = "Low for pin 11."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P11_A::LOW
    }
    #[doc = "High for pin 11."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P11_A::HIGH
    }
}
#[doc = "Field `p12` reader - Input level for pin 12."]
pub type P12_R = crate::BitReader<P12_A>;
#[doc = "Input level for pin 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12_A {
    #[doc = "0: Low for pin 12."]
    LOW = 0,
    #[doc = "1: High for pin 12."]
    HIGH = 1,
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
            false => P12_A::LOW,
            true => P12_A::HIGH,
        }
    }
    #[doc = "Low for pin 12."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P12_A::LOW
    }
    #[doc = "High for pin 12."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P12_A::HIGH
    }
}
#[doc = "Field `p13` reader - Input level for pin 13."]
pub type P13_R = crate::BitReader<P13_A>;
#[doc = "Input level for pin 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13_A {
    #[doc = "0: Low for pin 13."]
    LOW = 0,
    #[doc = "1: High for pin 13."]
    HIGH = 1,
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
            false => P13_A::LOW,
            true => P13_A::HIGH,
        }
    }
    #[doc = "Low for pin 13."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == P13_A::LOW
    }
    #[doc = "High for pin 13."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == P13_A::HIGH
    }
}
impl R {
    #[doc = "Bit 0 - Input level for pin 0."]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input level for pin 1."]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input level for pin 2."]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input level for pin 3."]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input level for pin 4."]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input level for pin 5."]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input level for pin 6."]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input level for pin 7."]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input level for pin 8."]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input level for pin 9."]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input level for pin 10."]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input level for pin 11."]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input level for pin 12."]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input level for pin 13."]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Input Level.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for IN_SPEC {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
