#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<INT_EN_SPEC>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<INT_EN_SPEC>;
#[doc = "Field `CHIEN0` reader - Channel 0 Interrupt Enable."]
pub type CHIEN0_R = crate::BitReader<CHIEN0_A>;
#[doc = "Channel 0 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHIEN0_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<CHIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CHIEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHIEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIEN0_A {
        match self.bits {
            false => CHIEN0_A::DISABLED,
            true => CHIEN0_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHIEN0_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHIEN0_A::ENABLED
    }
}
#[doc = "Field `CHIEN0` writer - Channel 0 Interrupt Enable."]
pub type CHIEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHIEN0_A>;
impl<'a, REG, const O: u8> CHIEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHIEN0_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHIEN0_A::ENABLED)
    }
}
#[doc = "Field `CHIEN1` reader - Channel 1 Interrupt Enable."]
pub type CHIEN1_R = crate::BitReader<CHIEN1_A>;
#[doc = "Channel 1 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHIEN1_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<CHIEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CHIEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHIEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIEN1_A {
        match self.bits {
            false => CHIEN1_A::DISABLED,
            true => CHIEN1_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHIEN1_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHIEN1_A::ENABLED
    }
}
#[doc = "Field `CHIEN1` writer - Channel 1 Interrupt Enable."]
pub type CHIEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHIEN1_A>;
impl<'a, REG, const O: u8> CHIEN1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHIEN1_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHIEN1_A::ENABLED)
    }
}
#[doc = "Field `CHIEN2` reader - Channel 2 Interrupt Enable."]
pub type CHIEN2_R = crate::BitReader<CHIEN2_A>;
#[doc = "Channel 2 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHIEN2_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<CHIEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CHIEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHIEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIEN2_A {
        match self.bits {
            false => CHIEN2_A::DISABLED,
            true => CHIEN2_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHIEN2_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHIEN2_A::ENABLED
    }
}
#[doc = "Field `CHIEN2` writer - Channel 2 Interrupt Enable."]
pub type CHIEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHIEN2_A>;
impl<'a, REG, const O: u8> CHIEN2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHIEN2_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHIEN2_A::ENABLED)
    }
}
#[doc = "Field `CHIEN3` reader - Channel 3 Interrupt Enable."]
pub type CHIEN3_R = crate::BitReader<CHIEN3_A>;
#[doc = "Channel 3 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHIEN3_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<CHIEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CHIEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl CHIEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIEN3_A {
        match self.bits {
            false => CHIEN3_A::DISABLED,
            true => CHIEN3_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHIEN3_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHIEN3_A::ENABLED
    }
}
#[doc = "Field `CHIEN3` writer - Channel 3 Interrupt Enable."]
pub type CHIEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHIEN3_A>;
impl<'a, REG, const O: u8> CHIEN3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHIEN3_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHIEN3_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    pub fn chien0(&self) -> CHIEN0_R {
        CHIEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    pub fn chien1(&self) -> CHIEN1_R {
        CHIEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    pub fn chien2(&self) -> CHIEN2_R {
        CHIEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    pub fn chien3(&self) -> CHIEN3_R {
        CHIEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn chien0(&mut self) -> CHIEN0_W<INT_EN_SPEC, 0> {
        CHIEN0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn chien1(&mut self) -> CHIEN1_W<INT_EN_SPEC, 1> {
        CHIEN1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn chien2(&mut self) -> CHIEN2_W<INT_EN_SPEC, 2> {
        CHIEN2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn chien3(&mut self) -> CHIEN3_W<INT_EN_SPEC, 3> {
        CHIEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
