#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `DONE` reader - Flash Done Interrupt."]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "Flash Done Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::INACTIVE,
            true => DONE_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DONE_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DONE_A::PENDING
    }
}
#[doc = "Field `DONE` writer - Flash Done Interrupt."]
pub type DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DONE_A>;
impl<'a, REG, const O: u8> DONE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DONE_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DONE_A::PENDING)
    }
}
#[doc = "Field `ACCESS_FAIL` reader - Flash Access Fail."]
pub type ACCESS_FAIL_R = crate::BitReader<ACCESS_FAIL_A>;
#[doc = "Flash Access Fail.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACCESS_FAIL_A {
    #[doc = "0: No error."]
    NO_ERROR = 0,
    #[doc = "1: Failure occurred."]
    ERROR = 1,
}
impl From<ACCESS_FAIL_A> for bool {
    #[inline(always)]
    fn from(variant: ACCESS_FAIL_A) -> Self {
        variant as u8 != 0
    }
}
impl ACCESS_FAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCESS_FAIL_A {
        match self.bits {
            false => ACCESS_FAIL_A::NO_ERROR,
            true => ACCESS_FAIL_A::ERROR,
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ACCESS_FAIL_A::NO_ERROR
    }
    #[doc = "Failure occurred."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ACCESS_FAIL_A::ERROR
    }
}
#[doc = "Field `ACCESS_FAIL` writer - Flash Access Fail."]
pub type ACCESS_FAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ACCESS_FAIL_A>;
impl<'a, REG, const O: u8> ACCESS_FAIL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(ACCESS_FAIL_A::NO_ERROR)
    }
    #[doc = "Failure occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(ACCESS_FAIL_A::ERROR)
    }
}
#[doc = "Field `DONE_IE` reader - Flash Done Interrupt Enable."]
pub type DONE_IE_R = crate::BitReader<DONE_IE_A>;
#[doc = "Flash Done Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_IE_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<DONE_IE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_IE_A {
        match self.bits {
            false => DONE_IE_A::DISABLED,
            true => DONE_IE_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DONE_IE_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DONE_IE_A::ENABLED
    }
}
#[doc = "Field `DONE_IE` writer - Flash Done Interrupt Enable."]
pub type DONE_IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DONE_IE_A>;
impl<'a, REG, const O: u8> DONE_IE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DONE_IE_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DONE_IE_A::ENABLED)
    }
}
#[doc = "Field `ACCESS_FAIL_IE` reader - Flash Done Interrupt Enable."]
pub use DONE_IE_R as ACCESS_FAIL_IE_R;
#[doc = "Field `ACCESS_FAIL_IE` writer - Flash Done Interrupt Enable."]
pub use DONE_IE_W as ACCESS_FAIL_IE_W;
impl R {
    #[doc = "Bit 0 - Flash Done Interrupt."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Access Fail."]
    #[inline(always)]
    pub fn access_fail(&self) -> ACCESS_FAIL_R {
        ACCESS_FAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn done_ie(&self) -> DONE_IE_R {
        DONE_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn access_fail_ie(&self) -> ACCESS_FAIL_IE_R {
        ACCESS_FAIL_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Done Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INTR_SPEC, 0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - Flash Access Fail."]
    #[inline(always)]
    #[must_use]
    pub fn access_fail(&mut self) -> ACCESS_FAIL_W<INTR_SPEC, 1> {
        ACCESS_FAIL_W::new(self)
    }
    #[doc = "Bit 8 - Flash Done Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn done_ie(&mut self) -> DONE_IE_W<INTR_SPEC, 8> {
        DONE_IE_W::new(self)
    }
    #[doc = "Bit 9 - Flash Done Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn access_fail_ie(&mut self) -> ACCESS_FAIL_IE_W<INTR_SPEC, 9> {
        ACCESS_FAIL_IE_W::new(self)
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
#[doc = "Flash Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
