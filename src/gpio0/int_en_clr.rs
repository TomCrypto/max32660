#[doc = "Register `INT_EN_CLR` writer"]
pub type W = crate::W<INT_EN_CLR_SPEC>;
#[doc = "Interrupt enable clear for pin 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_AW {
    #[doc = "1: Interrupt disabled for pin 0."]
    DISABLED = 1,
}
impl From<P0_AW> for bool {
    #[inline(always)]
    fn from(variant: P0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p0` writer - Interrupt enable clear for pin 0."]
pub type P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P0_AW>;
impl<'a, REG, const O: u8> P0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 0."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_AW {
    #[doc = "1: Interrupt disabled for pin 1."]
    DISABLED = 1,
}
impl From<P1_AW> for bool {
    #[inline(always)]
    fn from(variant: P1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p1` writer - Interrupt enable clear for pin 1."]
pub type P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P1_AW>;
impl<'a, REG, const O: u8> P1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_AW {
    #[doc = "1: Interrupt disabled for pin 2."]
    DISABLED = 1,
}
impl From<P2_AW> for bool {
    #[inline(always)]
    fn from(variant: P2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p2` writer - Interrupt enable clear for pin 2."]
pub type P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P2_AW>;
impl<'a, REG, const O: u8> P2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 2."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3_AW {
    #[doc = "1: Interrupt disabled for pin 3."]
    DISABLED = 1,
}
impl From<P3_AW> for bool {
    #[inline(always)]
    fn from(variant: P3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p3` writer - Interrupt enable clear for pin 3."]
pub type P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P3_AW>;
impl<'a, REG, const O: u8> P3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 3."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P3_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4_AW {
    #[doc = "1: Interrupt disabled for pin 4."]
    DISABLED = 1,
}
impl From<P4_AW> for bool {
    #[inline(always)]
    fn from(variant: P4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p4` writer - Interrupt enable clear for pin 4."]
pub type P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P4_AW>;
impl<'a, REG, const O: u8> P4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 4."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P4_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5_AW {
    #[doc = "1: Interrupt disabled for pin 5."]
    DISABLED = 1,
}
impl From<P5_AW> for bool {
    #[inline(always)]
    fn from(variant: P5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p5` writer - Interrupt enable clear for pin 5."]
pub type P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P5_AW>;
impl<'a, REG, const O: u8> P5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 5."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P5_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6_AW {
    #[doc = "1: Interrupt disabled for pin 6."]
    DISABLED = 1,
}
impl From<P6_AW> for bool {
    #[inline(always)]
    fn from(variant: P6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p6` writer - Interrupt enable clear for pin 6."]
pub type P6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P6_AW>;
impl<'a, REG, const O: u8> P6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 6."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P6_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7_AW {
    #[doc = "1: Interrupt disabled for pin 7."]
    DISABLED = 1,
}
impl From<P7_AW> for bool {
    #[inline(always)]
    fn from(variant: P7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p7` writer - Interrupt enable clear for pin 7."]
pub type P7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P7_AW>;
impl<'a, REG, const O: u8> P7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 7."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P7_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8_AW {
    #[doc = "1: Interrupt disabled for pin 8."]
    DISABLED = 1,
}
impl From<P8_AW> for bool {
    #[inline(always)]
    fn from(variant: P8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p8` writer - Interrupt enable clear for pin 8."]
pub type P8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P8_AW>;
impl<'a, REG, const O: u8> P8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 8."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P8_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9_AW {
    #[doc = "1: Interrupt disabled for pin 9."]
    DISABLED = 1,
}
impl From<P9_AW> for bool {
    #[inline(always)]
    fn from(variant: P9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p9` writer - Interrupt enable clear for pin 9."]
pub type P9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P9_AW>;
impl<'a, REG, const O: u8> P9_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 9."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P9_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10_AW {
    #[doc = "1: Interrupt disabled for pin 10."]
    DISABLED = 1,
}
impl From<P10_AW> for bool {
    #[inline(always)]
    fn from(variant: P10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p10` writer - Interrupt enable clear for pin 10."]
pub type P10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P10_AW>;
impl<'a, REG, const O: u8> P10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 10."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P10_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11_AW {
    #[doc = "1: Interrupt disabled for pin 11."]
    DISABLED = 1,
}
impl From<P11_AW> for bool {
    #[inline(always)]
    fn from(variant: P11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p11` writer - Interrupt enable clear for pin 11."]
pub type P11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P11_AW>;
impl<'a, REG, const O: u8> P11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 11."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P11_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12_AW {
    #[doc = "1: Interrupt disabled for pin 12."]
    DISABLED = 1,
}
impl From<P12_AW> for bool {
    #[inline(always)]
    fn from(variant: P12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p12` writer - Interrupt enable clear for pin 12."]
pub type P12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P12_AW>;
impl<'a, REG, const O: u8> P12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 12."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P12_AW::DISABLED)
    }
}
#[doc = "Interrupt enable clear for pin 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13_AW {
    #[doc = "1: Interrupt disabled for pin 13."]
    DISABLED = 1,
}
impl From<P13_AW> for bool {
    #[inline(always)]
    fn from(variant: P13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `p13` writer - Interrupt enable clear for pin 13."]
pub type P13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, P13_AW>;
impl<'a, REG, const O: u8> P13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for pin 13."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P13_AW::DISABLED)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable clear for pin 0."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<INT_EN_CLR_SPEC, 0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable clear for pin 1."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<INT_EN_CLR_SPEC, 1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable clear for pin 2."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<INT_EN_CLR_SPEC, 2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable clear for pin 3."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<INT_EN_CLR_SPEC, 3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable clear for pin 4."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<INT_EN_CLR_SPEC, 4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable clear for pin 5."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<INT_EN_CLR_SPEC, 5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt enable clear for pin 6."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<INT_EN_CLR_SPEC, 6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable clear for pin 7."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<INT_EN_CLR_SPEC, 7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable clear for pin 8."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<INT_EN_CLR_SPEC, 8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt enable clear for pin 9."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<INT_EN_CLR_SPEC, 9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt enable clear for pin 10."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<INT_EN_CLR_SPEC, 10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt enable clear for pin 11."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<INT_EN_CLR_SPEC, 11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt enable clear for pin 12."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<INT_EN_CLR_SPEC, 12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt enable clear for pin 13."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<INT_EN_CLR_SPEC, 13> {
        P13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_CLR_SPEC;
impl crate::RegisterSpec for INT_EN_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_en_clr::W`](W) writer structure"]
impl crate::Writable for INT_EN_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_EN_CLR to value 0"]
impl crate::Resettable for INT_EN_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
