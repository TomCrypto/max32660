#[doc = "Register `OSCCTRL` reader"]
pub type R = crate::R<OSCCTRL_SPEC>;
#[doc = "Register `OSCCTRL` writer"]
pub type W = crate::W<OSCCTRL_SPEC>;
#[doc = "Field `FILTER_EN` reader - RTC Oscillator Filter Enable."]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - RTC Oscillator Filter Enable."]
pub type FILTER_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IBIAS_SEL` reader - RTC Oscillator 4X Bias Current Select."]
pub type IBIAS_SEL_R = crate::BitReader<IBIAS_SEL_A>;
#[doc = "RTC Oscillator 4X Bias Current Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIAS_SEL_A {
    #[doc = "0: Selects 2X bias current for RTC oscillator."]
    _2X = 0,
    #[doc = "1: Selects 4X bias current for RTC oscillator."]
    _4X = 1,
}
impl From<IBIAS_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: IBIAS_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IBIAS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBIAS_SEL_A {
        match self.bits {
            false => IBIAS_SEL_A::_2X,
            true => IBIAS_SEL_A::_4X,
        }
    }
    #[doc = "Selects 2X bias current for RTC oscillator."]
    #[inline(always)]
    pub fn is_2x(&self) -> bool {
        *self == IBIAS_SEL_A::_2X
    }
    #[doc = "Selects 4X bias current for RTC oscillator."]
    #[inline(always)]
    pub fn is_4x(&self) -> bool {
        *self == IBIAS_SEL_A::_4X
    }
}
#[doc = "Field `IBIAS_SEL` writer - RTC Oscillator 4X Bias Current Select."]
pub type IBIAS_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IBIAS_SEL_A>;
impl<'a, REG, const O: u8> IBIAS_SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects 2X bias current for RTC oscillator."]
    #[inline(always)]
    pub fn _2x(self) -> &'a mut crate::W<REG> {
        self.variant(IBIAS_SEL_A::_2X)
    }
    #[doc = "Selects 4X bias current for RTC oscillator."]
    #[inline(always)]
    pub fn _4x(self) -> &'a mut crate::W<REG> {
        self.variant(IBIAS_SEL_A::_4X)
    }
}
#[doc = "Field `HYST_EN` reader - RTC Oscillator Hysteresis Buffer Enable."]
pub type HYST_EN_R = crate::BitReader;
#[doc = "Field `HYST_EN` writer - RTC Oscillator Hysteresis Buffer Enable."]
pub type HYST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IBIAS_EN` reader - RTC Oscillator Bias Current Enable."]
pub type IBIAS_EN_R = crate::BitReader;
#[doc = "Field `IBIAS_EN` writer - RTC Oscillator Bias Current Enable."]
pub type IBIAS_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BYPASS` reader - RTC Crystal Bypass."]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - RTC Crystal Bypass."]
pub type BYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUT32K` reader - RTC 32kHz Square Wave Output."]
pub type OUT32K_R = crate::BitReader;
#[doc = "Field `OUT32K` writer - RTC 32kHz Square Wave Output."]
pub type OUT32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RTC Oscillator Filter Enable."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Oscillator 4X Bias Current Select."]
    #[inline(always)]
    pub fn ibias_sel(&self) -> IBIAS_SEL_R {
        IBIAS_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Oscillator Hysteresis Buffer Enable."]
    #[inline(always)]
    pub fn hyst_en(&self) -> HYST_EN_R {
        HYST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Oscillator Bias Current Enable."]
    #[inline(always)]
    pub fn ibias_en(&self) -> IBIAS_EN_R {
        IBIAS_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output."]
    #[inline(always)]
    pub fn out32k(&self) -> OUT32K_R {
        OUT32K_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Oscillator Filter Enable."]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<OSCCTRL_SPEC, 0> {
        FILTER_EN_W::new(self)
    }
    #[doc = "Bit 1 - RTC Oscillator 4X Bias Current Select."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_sel(&mut self) -> IBIAS_SEL_W<OSCCTRL_SPEC, 1> {
        IBIAS_SEL_W::new(self)
    }
    #[doc = "Bit 2 - RTC Oscillator Hysteresis Buffer Enable."]
    #[inline(always)]
    #[must_use]
    pub fn hyst_en(&mut self) -> HYST_EN_W<OSCCTRL_SPEC, 2> {
        HYST_EN_W::new(self)
    }
    #[doc = "Bit 3 - RTC Oscillator Bias Current Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_en(&mut self) -> IBIAS_EN_W<OSCCTRL_SPEC, 3> {
        IBIAS_EN_W::new(self)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<OSCCTRL_SPEC, 4> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output."]
    #[inline(always)]
    #[must_use]
    pub fn out32k(&mut self) -> OUT32K_W<OSCCTRL_SPEC, 5> {
        OUT32K_W::new(self)
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
#[doc = "RTC Oscillator Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCCTRL_SPEC;
impl crate::RegisterSpec for OSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscctrl::R`](R) reader structure"]
impl crate::Readable for OSCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oscctrl::W`](W) writer structure"]
impl crate::Writable for OSCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCTRL to value 0"]
impl crate::Resettable for OSCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
