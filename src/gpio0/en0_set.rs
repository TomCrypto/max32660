#[doc = "Register `EN0_SET` writer"]
pub type W = crate::W<EN0_SET_SPEC>;
#[doc = "Function select set for pin 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_AW {
    #[doc = "1: GPIO function selected for pin 0."]
    GPIO = 1,
}
impl From<P0_AW> for bool {
    #[inline(always)]
    fn from(variant: P0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p0` writer - Function select set for pin 0."]
pub type P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P0_AW>;
impl<'a, REG, const O: u8> P0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 0."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P0_AW::GPIO)
    }
}
#[doc = "Function select set for pin 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_AW {
    #[doc = "1: GPIO function selected for pin 1."]
    GPIO = 1,
}
impl From<P1_AW> for bool {
    #[inline(always)]
    fn from(variant: P1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p1` writer - Function select set for pin 1."]
pub type P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P1_AW>;
impl<'a, REG, const O: u8> P1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 1."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P1_AW::GPIO)
    }
}
#[doc = "Function select set for pin 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_AW {
    #[doc = "1: GPIO function selected for pin 2."]
    GPIO = 1,
}
impl From<P2_AW> for bool {
    #[inline(always)]
    fn from(variant: P2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p2` writer - Function select set for pin 2."]
pub type P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P2_AW>;
impl<'a, REG, const O: u8> P2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 2."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P2_AW::GPIO)
    }
}
#[doc = "Function select set for pin 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3_AW {
    #[doc = "1: GPIO function selected for pin 3."]
    GPIO = 1,
}
impl From<P3_AW> for bool {
    #[inline(always)]
    fn from(variant: P3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p3` writer - Function select set for pin 3."]
pub type P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P3_AW>;
impl<'a, REG, const O: u8> P3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 3."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P3_AW::GPIO)
    }
}
#[doc = "Function select set for pin 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4_AW {
    #[doc = "1: GPIO function selected for pin 4."]
    GPIO = 1,
}
impl From<P4_AW> for bool {
    #[inline(always)]
    fn from(variant: P4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p4` writer - Function select set for pin 4."]
pub type P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P4_AW>;
impl<'a, REG, const O: u8> P4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 4."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P4_AW::GPIO)
    }
}
#[doc = "Function select set for pin 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5_AW {
    #[doc = "1: GPIO function selected for pin 5."]
    GPIO = 1,
}
impl From<P5_AW> for bool {
    #[inline(always)]
    fn from(variant: P5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p5` writer - Function select set for pin 5."]
pub type P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P5_AW>;
impl<'a, REG, const O: u8> P5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 5."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P5_AW::GPIO)
    }
}
#[doc = "Function select set for pin 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6_AW {
    #[doc = "1: GPIO function selected for pin 6."]
    GPIO = 1,
}
impl From<P6_AW> for bool {
    #[inline(always)]
    fn from(variant: P6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p6` writer - Function select set for pin 6."]
pub type P6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P6_AW>;
impl<'a, REG, const O: u8> P6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 6."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P6_AW::GPIO)
    }
}
#[doc = "Function select set for pin 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7_AW {
    #[doc = "1: GPIO function selected for pin 7."]
    GPIO = 1,
}
impl From<P7_AW> for bool {
    #[inline(always)]
    fn from(variant: P7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p7` writer - Function select set for pin 7."]
pub type P7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P7_AW>;
impl<'a, REG, const O: u8> P7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 7."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P7_AW::GPIO)
    }
}
#[doc = "Function select set for pin 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8_AW {
    #[doc = "1: GPIO function selected for pin 8."]
    GPIO = 1,
}
impl From<P8_AW> for bool {
    #[inline(always)]
    fn from(variant: P8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p8` writer - Function select set for pin 8."]
pub type P8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P8_AW>;
impl<'a, REG, const O: u8> P8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 8."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P8_AW::GPIO)
    }
}
#[doc = "Function select set for pin 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9_AW {
    #[doc = "1: GPIO function selected for pin 9."]
    GPIO = 1,
}
impl From<P9_AW> for bool {
    #[inline(always)]
    fn from(variant: P9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p9` writer - Function select set for pin 9."]
pub type P9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P9_AW>;
impl<'a, REG, const O: u8> P9_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 9."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P9_AW::GPIO)
    }
}
#[doc = "Function select set for pin 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10_AW {
    #[doc = "1: GPIO function selected for pin 10."]
    GPIO = 1,
}
impl From<P10_AW> for bool {
    #[inline(always)]
    fn from(variant: P10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p10` writer - Function select set for pin 10."]
pub type P10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P10_AW>;
impl<'a, REG, const O: u8> P10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 10."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P10_AW::GPIO)
    }
}
#[doc = "Function select set for pin 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11_AW {
    #[doc = "1: GPIO function selected for pin 11."]
    GPIO = 1,
}
impl From<P11_AW> for bool {
    #[inline(always)]
    fn from(variant: P11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p11` writer - Function select set for pin 11."]
pub type P11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P11_AW>;
impl<'a, REG, const O: u8> P11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 11."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P11_AW::GPIO)
    }
}
#[doc = "Function select set for pin 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12_AW {
    #[doc = "1: GPIO function selected for pin 12."]
    GPIO = 1,
}
impl From<P12_AW> for bool {
    #[inline(always)]
    fn from(variant: P12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p12` writer - Function select set for pin 12."]
pub type P12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P12_AW>;
impl<'a, REG, const O: u8> P12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 12."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P12_AW::GPIO)
    }
}
#[doc = "Function select set for pin 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13_AW {
    #[doc = "1: GPIO function selected for pin 13."]
    GPIO = 1,
}
impl From<P13_AW> for bool {
    #[inline(always)]
    fn from(variant: P13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p13` writer - Function select set for pin 13."]
pub type P13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P13_AW>;
impl<'a, REG, const O: u8> P13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO function selected for pin 13."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(P13_AW::GPIO)
    }
}
impl W {
    #[doc = "Bit 0 - Function select set for pin 0."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<EN0_SET_SPEC, 0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - Function select set for pin 1."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<EN0_SET_SPEC, 1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - Function select set for pin 2."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<EN0_SET_SPEC, 2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - Function select set for pin 3."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<EN0_SET_SPEC, 3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - Function select set for pin 4."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<EN0_SET_SPEC, 4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - Function select set for pin 5."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<EN0_SET_SPEC, 5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - Function select set for pin 6."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<EN0_SET_SPEC, 6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - Function select set for pin 7."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<EN0_SET_SPEC, 7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - Function select set for pin 8."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<EN0_SET_SPEC, 8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - Function select set for pin 9."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<EN0_SET_SPEC, 9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - Function select set for pin 10."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<EN0_SET_SPEC, 10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - Function select set for pin 11."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<EN0_SET_SPEC, 11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - Function select set for pin 12."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<EN0_SET_SPEC, 12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - Function select set for pin 13."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<EN0_SET_SPEC, 13> {
        P13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Function Select Set.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN0_SET_SPEC;
impl crate::RegisterSpec for EN0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`en0_set::W`](W) writer structure"]
impl crate::Writable for EN0_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EN0_SET to value 0"]
impl crate::Resettable for EN0_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
