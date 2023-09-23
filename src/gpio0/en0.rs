#[doc = "Register `EN0` reader"]
pub type R = crate::R<EN0_SPEC>;
#[doc = "Register `EN0` writer"]
pub type W = crate::W<EN0_SPEC>;
#[doc = "Field `p0` reader - Function select for pin 0."]
pub type P0_R = crate::BitReader<P0_A>;
#[doc = "Function select for pin 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_A {
    #[doc = "0: Alternate function selected for pin 0."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 0."]
    GPIO = 1,
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
            false => P0_A::ALTERNATE,
            true => P0_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 0."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P0_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 0."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P0_A::GPIO
    }
}
#[doc = "Field `p0` writer - Function select for pin 0."]
pub type P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P0_A>;
impl<'a, REG, const O: u8> P0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 0."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P0_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 0."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P0_A::GPIO)
    }
}
#[doc = "Field `p1` reader - Function select for pin 1."]
pub type P1_R = crate::BitReader<P1_A>;
#[doc = "Function select for pin 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_A {
    #[doc = "0: Alternate function selected for pin 1."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 1."]
    GPIO = 1,
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
            false => P1_A::ALTERNATE,
            true => P1_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 1."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P1_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 1."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P1_A::GPIO
    }
}
#[doc = "Field `p1` writer - Function select for pin 1."]
pub type P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P1_A>;
impl<'a, REG, const O: u8> P1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 1."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P1_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 1."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P1_A::GPIO)
    }
}
#[doc = "Field `p2` reader - Function select for pin 2."]
pub type P2_R = crate::BitReader<P2_A>;
#[doc = "Function select for pin 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_A {
    #[doc = "0: Alternate function selected for pin 2."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 2."]
    GPIO = 1,
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
            false => P2_A::ALTERNATE,
            true => P2_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 2."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P2_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 2."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P2_A::GPIO
    }
}
#[doc = "Field `p2` writer - Function select for pin 2."]
pub type P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P2_A>;
impl<'a, REG, const O: u8> P2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 2."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P2_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 2."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P2_A::GPIO)
    }
}
#[doc = "Field `p3` reader - Function select for pin 3."]
pub type P3_R = crate::BitReader<P3_A>;
#[doc = "Function select for pin 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3_A {
    #[doc = "0: Alternate function selected for pin 3."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 3."]
    GPIO = 1,
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
            false => P3_A::ALTERNATE,
            true => P3_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 3."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P3_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 3."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P3_A::GPIO
    }
}
#[doc = "Field `p3` writer - Function select for pin 3."]
pub type P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P3_A>;
impl<'a, REG, const O: u8> P3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 3."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P3_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 3."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P3_A::GPIO)
    }
}
#[doc = "Field `p4` reader - Function select for pin 4."]
pub type P4_R = crate::BitReader<P4_A>;
#[doc = "Function select for pin 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4_A {
    #[doc = "0: Alternate function selected for pin 4."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 4."]
    GPIO = 1,
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
            false => P4_A::ALTERNATE,
            true => P4_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 4."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P4_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 4."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P4_A::GPIO
    }
}
#[doc = "Field `p4` writer - Function select for pin 4."]
pub type P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P4_A>;
impl<'a, REG, const O: u8> P4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 4."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P4_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 4."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P4_A::GPIO)
    }
}
#[doc = "Field `p5` reader - Function select for pin 5."]
pub type P5_R = crate::BitReader<P5_A>;
#[doc = "Function select for pin 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5_A {
    #[doc = "0: Alternate function selected for pin 5."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 5."]
    GPIO = 1,
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
            false => P5_A::ALTERNATE,
            true => P5_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 5."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P5_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 5."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P5_A::GPIO
    }
}
#[doc = "Field `p5` writer - Function select for pin 5."]
pub type P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P5_A>;
impl<'a, REG, const O: u8> P5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 5."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P5_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 5."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P5_A::GPIO)
    }
}
#[doc = "Field `p6` reader - Function select for pin 6."]
pub type P6_R = crate::BitReader<P6_A>;
#[doc = "Function select for pin 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6_A {
    #[doc = "0: Alternate function selected for pin 6."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 6."]
    GPIO = 1,
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
            false => P6_A::ALTERNATE,
            true => P6_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 6."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P6_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 6."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P6_A::GPIO
    }
}
#[doc = "Field `p6` writer - Function select for pin 6."]
pub type P6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P6_A>;
impl<'a, REG, const O: u8> P6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 6."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P6_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 6."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P6_A::GPIO)
    }
}
#[doc = "Field `p7` reader - Function select for pin 7."]
pub type P7_R = crate::BitReader<P7_A>;
#[doc = "Function select for pin 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7_A {
    #[doc = "0: Alternate function selected for pin 7."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 7."]
    GPIO = 1,
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
            false => P7_A::ALTERNATE,
            true => P7_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 7."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P7_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 7."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P7_A::GPIO
    }
}
#[doc = "Field `p7` writer - Function select for pin 7."]
pub type P7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P7_A>;
impl<'a, REG, const O: u8> P7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 7."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P7_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 7."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P7_A::GPIO)
    }
}
#[doc = "Field `p8` reader - Function select for pin 8."]
pub type P8_R = crate::BitReader<P8_A>;
#[doc = "Function select for pin 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8_A {
    #[doc = "0: Alternate function selected for pin 8."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 8."]
    GPIO = 1,
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
            false => P8_A::ALTERNATE,
            true => P8_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 8."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P8_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 8."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P8_A::GPIO
    }
}
#[doc = "Field `p8` writer - Function select for pin 8."]
pub type P8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P8_A>;
impl<'a, REG, const O: u8> P8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 8."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P8_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 8."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P8_A::GPIO)
    }
}
#[doc = "Field `p9` reader - Function select for pin 9."]
pub type P9_R = crate::BitReader<P9_A>;
#[doc = "Function select for pin 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9_A {
    #[doc = "0: Alternate function selected for pin 9."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 9."]
    GPIO = 1,
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
            false => P9_A::ALTERNATE,
            true => P9_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 9."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P9_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 9."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P9_A::GPIO
    }
}
#[doc = "Field `p9` writer - Function select for pin 9."]
pub type P9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P9_A>;
impl<'a, REG, const O: u8> P9_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 9."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P9_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 9."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P9_A::GPIO)
    }
}
#[doc = "Field `p10` reader - Function select for pin 10."]
pub type P10_R = crate::BitReader<P10_A>;
#[doc = "Function select for pin 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10_A {
    #[doc = "0: Alternate function selected for pin 10."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 10."]
    GPIO = 1,
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
            false => P10_A::ALTERNATE,
            true => P10_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 10."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P10_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 10."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P10_A::GPIO
    }
}
#[doc = "Field `p10` writer - Function select for pin 10."]
pub type P10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P10_A>;
impl<'a, REG, const O: u8> P10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 10."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P10_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 10."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P10_A::GPIO)
    }
}
#[doc = "Field `p11` reader - Function select for pin 11."]
pub type P11_R = crate::BitReader<P11_A>;
#[doc = "Function select for pin 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11_A {
    #[doc = "0: Alternate function selected for pin 11."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 11."]
    GPIO = 1,
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
            false => P11_A::ALTERNATE,
            true => P11_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 11."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P11_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 11."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P11_A::GPIO
    }
}
#[doc = "Field `p11` writer - Function select for pin 11."]
pub type P11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P11_A>;
impl<'a, REG, const O: u8> P11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 11."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P11_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 11."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P11_A::GPIO)
    }
}
#[doc = "Field `p12` reader - Function select for pin 12."]
pub type P12_R = crate::BitReader<P12_A>;
#[doc = "Function select for pin 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12_A {
    #[doc = "0: Alternate function selected for pin 12."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 12."]
    GPIO = 1,
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
            false => P12_A::ALTERNATE,
            true => P12_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 12."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P12_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 12."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P12_A::GPIO
    }
}
#[doc = "Field `p12` writer - Function select for pin 12."]
pub type P12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P12_A>;
impl<'a, REG, const O: u8> P12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 12."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P12_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 12."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P12_A::GPIO)
    }
}
#[doc = "Field `p13` reader - Function select for pin 13."]
pub type P13_R = crate::BitReader<P13_A>;
#[doc = "Function select for pin 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13_A {
    #[doc = "0: Alternate function selected for pin 13."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function selected for pin 13."]
    GPIO = 1,
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
            false => P13_A::ALTERNATE,
            true => P13_A::GPIO,
        }
    }
    #[doc = "Alternate function selected for pin 13."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == P13_A::ALTERNATE
    }
    #[doc = "GPIO function selected for pin 13."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == P13_A::GPIO
    }
}
#[doc = "Field `p13` writer - Function select for pin 13."]
pub type P13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P13_A>;
impl<'a, REG, const O: u8> P13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alternate function selected for pin 13."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(P13_A::ALTERNATE)
    }
    #[doc = "GPIO function selected for pin 13."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P13_A::GPIO)
    }
}
impl R {
    #[doc = "Bit 0 - Function select for pin 0."]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Function select for pin 1."]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Function select for pin 2."]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Function select for pin 3."]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Function select for pin 4."]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Function select for pin 5."]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Function select for pin 6."]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Function select for pin 7."]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Function select for pin 8."]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Function select for pin 9."]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Function select for pin 10."]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Function select for pin 11."]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Function select for pin 12."]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Function select for pin 13."]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Function select for pin 0."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<EN0_SPEC, 0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - Function select for pin 1."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<EN0_SPEC, 1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - Function select for pin 2."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<EN0_SPEC, 2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - Function select for pin 3."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<EN0_SPEC, 3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - Function select for pin 4."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<EN0_SPEC, 4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - Function select for pin 5."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<EN0_SPEC, 5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - Function select for pin 6."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<EN0_SPEC, 6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - Function select for pin 7."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<EN0_SPEC, 7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - Function select for pin 8."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<EN0_SPEC, 8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - Function select for pin 9."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<EN0_SPEC, 9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - Function select for pin 10."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<EN0_SPEC, 10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - Function select for pin 11."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<EN0_SPEC, 11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - Function select for pin 12."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<EN0_SPEC, 12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - Function select for pin 13."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<EN0_SPEC, 13> {
        P13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Function Select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN0_SPEC;
impl crate::RegisterSpec for EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en0::R`](R) reader structure"]
impl crate::Readable for EN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en0::W`](W) writer structure"]
impl crate::Writable for EN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EN0 to value 0"]
impl crate::Resettable for EN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
