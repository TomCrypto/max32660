#[doc = "Register `PCLK_DIS1` reader"]
pub type R = crate::R<PCLK_DIS1_SPEC>;
#[doc = "Register `PCLK_DIS1` writer"]
pub type W = crate::W<PCLK_DIS1_SPEC>;
#[doc = "Field `FLC` reader - Flash Controller Clock Disable."]
pub type FLC_R = crate::BitReader<FLC_A>;
#[doc = "Flash Controller Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLC_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<FLC_A> for bool {
    #[inline(always)]
    fn from(variant: FLC_A) -> Self {
        variant as u8 != 0
    }
}
impl FLC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLC_A {
        match self.bits {
            false => FLC_A::CLOCK_ENABLED,
            true => FLC_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == FLC_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == FLC_A::CLOCK_DISABLED
    }
}
#[doc = "Field `FLC` writer - Flash Controller Clock Disable."]
pub type FLC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLC_A>;
impl<'a, REG, const O: u8> FLC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLC_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLC_A::CLOCK_DISABLED)
    }
}
#[doc = "Field `ICCD` reader - Instruction Cache Controller Clock Disable."]
pub type ICCD_R = crate::BitReader<ICCD_A>;
#[doc = "Instruction Cache Controller Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICCD_A {
    #[doc = "0: Peripheral clock enabled."]
    CLOCK_ENABLED = 0,
    #[doc = "1: Peripheral clock disabled."]
    CLOCK_DISABLED = 1,
}
impl From<ICCD_A> for bool {
    #[inline(always)]
    fn from(variant: ICCD_A) -> Self {
        variant as u8 != 0
    }
}
impl ICCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICCD_A {
        match self.bits {
            false => ICCD_A::CLOCK_ENABLED,
            true => ICCD_A::CLOCK_DISABLED,
        }
    }
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled(&self) -> bool {
        *self == ICCD_A::CLOCK_ENABLED
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == ICCD_A::CLOCK_DISABLED
    }
}
#[doc = "Field `ICCD` writer - Instruction Cache Controller Clock Disable."]
pub type ICCD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ICCD_A>;
impl<'a, REG, const O: u8> ICCD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock enabled."]
    #[inline(always)]
    pub fn clock_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICCD_A::CLOCK_ENABLED)
    }
    #[doc = "Peripheral clock disabled."]
    #[inline(always)]
    pub fn clock_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICCD_A::CLOCK_DISABLED)
    }
}
impl R {
    #[doc = "Bit 3 - Flash Controller Clock Disable."]
    #[inline(always)]
    pub fn flc(&self) -> FLC_R {
        FLC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Instruction Cache Controller Clock Disable."]
    #[inline(always)]
    pub fn iccd(&self) -> ICCD_R {
        ICCD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Flash Controller Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn flc(&mut self) -> FLC_W<PCLK_DIS1_SPEC, 3> {
        FLC_W::new(self)
    }
    #[doc = "Bit 11 - Instruction Cache Controller Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn iccd(&mut self) -> ICCD_W<PCLK_DIS1_SPEC, 11> {
        ICCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral Clock Disable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclk_dis1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclk_dis1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCLK_DIS1_SPEC;
impl crate::RegisterSpec for PCLK_DIS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclk_dis1::R`](R) reader structure"]
impl crate::Readable for PCLK_DIS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pclk_dis1::W`](W) writer structure"]
impl crate::Writable for PCLK_DIS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLK_DIS1 to value 0"]
impl crate::Resettable for PCLK_DIS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
