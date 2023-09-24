#[doc = "Register `SYS_STAT` reader"]
pub type R = crate::R<SYS_STAT_SPEC>;
#[doc = "Register `SYS_STAT` writer"]
pub type W = crate::W<SYS_STAT_SPEC>;
#[doc = "Field `ICECLOCK` reader - ARM ICE Lock Status."]
pub type ICECLOCK_R = crate::BitReader<ICECLOCK_A>;
#[doc = "ARM ICE Lock Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICECLOCK_A {
    #[doc = "0: ICE is unlocked."]
    UNLOCKED = 0,
    #[doc = "1: ICE is locked."]
    LOCKED = 1,
}
impl From<ICECLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: ICECLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl ICECLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICECLOCK_A {
        match self.bits {
            false => ICECLOCK_A::UNLOCKED,
            true => ICECLOCK_A::LOCKED,
        }
    }
    #[doc = "ICE is unlocked."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == ICECLOCK_A::UNLOCKED
    }
    #[doc = "ICE is locked."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == ICECLOCK_A::LOCKED
    }
}
#[doc = "Field `ICECLOCK` writer - ARM ICE Lock Status."]
pub type ICECLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ICECLOCK_A>;
impl<'a, REG, const O: u8> ICECLOCK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ICE is unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(ICECLOCK_A::UNLOCKED)
    }
    #[doc = "ICE is locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(ICECLOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    pub fn iceclock(&self) -> ICECLOCK_R {
        ICECLOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    #[must_use]
    pub fn iceclock(&mut self) -> ICECLOCK_W<SYS_STAT_SPEC, 0> {
        ICECLOCK_W::new(self)
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
#[doc = "System Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_STAT_SPEC;
impl crate::RegisterSpec for SYS_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_stat::R`](R) reader structure"]
impl crate::Readable for SYS_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_stat::W`](W) writer structure"]
impl crate::Writable for SYS_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYS_STAT to value 0"]
impl crate::Resettable for SYS_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
