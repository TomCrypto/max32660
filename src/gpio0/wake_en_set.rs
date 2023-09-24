#[doc = "Register `WAKE_EN_SET` writer"]
pub type W = crate::W<WAKE_EN_SET_SPEC>;
#[doc = "Wakeup enable set for pin 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_AW {
    #[doc = "1: Wakeup enabled for pin 0."]
    ENABLED = 1,
}
impl From<P0_AW> for bool {
    #[inline(always)]
    fn from(variant: P0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p0` writer - Wakeup enable set for pin 0."]
pub type P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P0_AW>;
impl<'a, REG, const O: u8> P0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_AW {
    #[doc = "1: Wakeup enabled for pin 1."]
    ENABLED = 1,
}
impl From<P1_AW> for bool {
    #[inline(always)]
    fn from(variant: P1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p1` writer - Wakeup enable set for pin 1."]
pub type P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P1_AW>;
impl<'a, REG, const O: u8> P1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_AW {
    #[doc = "1: Wakeup enabled for pin 2."]
    ENABLED = 1,
}
impl From<P2_AW> for bool {
    #[inline(always)]
    fn from(variant: P2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p2` writer - Wakeup enable set for pin 2."]
pub type P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P2_AW>;
impl<'a, REG, const O: u8> P2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3_AW {
    #[doc = "1: Wakeup enabled for pin 3."]
    ENABLED = 1,
}
impl From<P3_AW> for bool {
    #[inline(always)]
    fn from(variant: P3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p3` writer - Wakeup enable set for pin 3."]
pub type P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P3_AW>;
impl<'a, REG, const O: u8> P3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 3."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P3_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4_AW {
    #[doc = "1: Wakeup enabled for pin 4."]
    ENABLED = 1,
}
impl From<P4_AW> for bool {
    #[inline(always)]
    fn from(variant: P4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p4` writer - Wakeup enable set for pin 4."]
pub type P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P4_AW>;
impl<'a, REG, const O: u8> P4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 4."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P4_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5_AW {
    #[doc = "1: Wakeup enabled for pin 5."]
    ENABLED = 1,
}
impl From<P5_AW> for bool {
    #[inline(always)]
    fn from(variant: P5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p5` writer - Wakeup enable set for pin 5."]
pub type P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P5_AW>;
impl<'a, REG, const O: u8> P5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 5."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P5_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6_AW {
    #[doc = "1: Wakeup enabled for pin 6."]
    ENABLED = 1,
}
impl From<P6_AW> for bool {
    #[inline(always)]
    fn from(variant: P6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p6` writer - Wakeup enable set for pin 6."]
pub type P6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P6_AW>;
impl<'a, REG, const O: u8> P6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 6."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P6_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7_AW {
    #[doc = "1: Wakeup enabled for pin 7."]
    ENABLED = 1,
}
impl From<P7_AW> for bool {
    #[inline(always)]
    fn from(variant: P7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p7` writer - Wakeup enable set for pin 7."]
pub type P7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P7_AW>;
impl<'a, REG, const O: u8> P7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 7."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P7_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8_AW {
    #[doc = "1: Wakeup enabled for pin 8."]
    ENABLED = 1,
}
impl From<P8_AW> for bool {
    #[inline(always)]
    fn from(variant: P8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p8` writer - Wakeup enable set for pin 8."]
pub type P8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P8_AW>;
impl<'a, REG, const O: u8> P8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 8."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P8_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9_AW {
    #[doc = "1: Wakeup enabled for pin 9."]
    ENABLED = 1,
}
impl From<P9_AW> for bool {
    #[inline(always)]
    fn from(variant: P9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p9` writer - Wakeup enable set for pin 9."]
pub type P9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P9_AW>;
impl<'a, REG, const O: u8> P9_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 9."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P9_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10_AW {
    #[doc = "1: Wakeup enabled for pin 10."]
    ENABLED = 1,
}
impl From<P10_AW> for bool {
    #[inline(always)]
    fn from(variant: P10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p10` writer - Wakeup enable set for pin 10."]
pub type P10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P10_AW>;
impl<'a, REG, const O: u8> P10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 10."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P10_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11_AW {
    #[doc = "1: Wakeup enabled for pin 11."]
    ENABLED = 1,
}
impl From<P11_AW> for bool {
    #[inline(always)]
    fn from(variant: P11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p11` writer - Wakeup enable set for pin 11."]
pub type P11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P11_AW>;
impl<'a, REG, const O: u8> P11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 11."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P11_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12_AW {
    #[doc = "1: Wakeup enabled for pin 12."]
    ENABLED = 1,
}
impl From<P12_AW> for bool {
    #[inline(always)]
    fn from(variant: P12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p12` writer - Wakeup enable set for pin 12."]
pub type P12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P12_AW>;
impl<'a, REG, const O: u8> P12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 12."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P12_AW::ENABLED)
    }
}
#[doc = "Wakeup enable set for pin 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13_AW {
    #[doc = "1: Wakeup enabled for pin 13."]
    ENABLED = 1,
}
impl From<P13_AW> for bool {
    #[inline(always)]
    fn from(variant: P13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p13` writer - Wakeup enable set for pin 13."]
pub type P13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P13_AW>;
impl<'a, REG, const O: u8> P13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup enabled for pin 13."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P13_AW::ENABLED)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup enable set for pin 0."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<WAKE_EN_SET_SPEC, 0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup enable set for pin 1."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<WAKE_EN_SET_SPEC, 1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup enable set for pin 2."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<WAKE_EN_SET_SPEC, 2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - Wakeup enable set for pin 3."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<WAKE_EN_SET_SPEC, 3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - Wakeup enable set for pin 4."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<WAKE_EN_SET_SPEC, 4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - Wakeup enable set for pin 5."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<WAKE_EN_SET_SPEC, 5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup enable set for pin 6."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<WAKE_EN_SET_SPEC, 6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - Wakeup enable set for pin 7."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<WAKE_EN_SET_SPEC, 7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - Wakeup enable set for pin 8."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<WAKE_EN_SET_SPEC, 8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - Wakeup enable set for pin 9."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<WAKE_EN_SET_SPEC, 9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup enable set for pin 10."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<WAKE_EN_SET_SPEC, 10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup enable set for pin 11."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<WAKE_EN_SET_SPEC, 11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup enable set for pin 12."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<WAKE_EN_SET_SPEC, 12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - Wakeup enable set for pin 13."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<WAKE_EN_SET_SPEC, 13> {
        P13_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Wakeup Enable Set.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_en_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKE_EN_SET_SPEC;
impl crate::RegisterSpec for WAKE_EN_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wake_en_set::W`](W) writer structure"]
impl crate::Writable for WAKE_EN_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKE_EN_SET to value 0"]
impl crate::Resettable for WAKE_EN_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
