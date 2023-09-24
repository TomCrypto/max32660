#[doc = "Register `LPMEMSD` reader"]
pub type R = crate::R<LPMEMSD_SPEC>;
#[doc = "Register `LPMEMSD` writer"]
pub type W = crate::W<LPMEMSD_SPEC>;
#[doc = "Field `SRAM0_OFF` reader - System RAM block 0 Shut Down."]
pub type SRAM0_OFF_R = crate::BitReader<SRAM0_OFF_A>;
#[doc = "System RAM block 0 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM0_OFF_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM0_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0_OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM0_OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM0_OFF_A {
        match self.bits {
            false => SRAM0_OFF_A::NORMAL,
            true => SRAM0_OFF_A::SHUTDOWN,
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM0_OFF_A::NORMAL
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM0_OFF_A::SHUTDOWN
    }
}
#[doc = "Field `SRAM0_OFF` writer - System RAM block 0 Shut Down."]
pub type SRAM0_OFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRAM0_OFF_A>;
impl<'a, REG, const O: u8> SRAM0_OFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM0_OFF_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM0_OFF_A::SHUTDOWN)
    }
}
#[doc = "Field `SRAM1_OFF` reader - System RAM block 1 Shut Down."]
pub type SRAM1_OFF_R = crate::BitReader<SRAM1_OFF_A>;
#[doc = "System RAM block 1 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1_OFF_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM1_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1_OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM1_OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1_OFF_A {
        match self.bits {
            false => SRAM1_OFF_A::NORMAL,
            true => SRAM1_OFF_A::SHUTDOWN,
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM1_OFF_A::NORMAL
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM1_OFF_A::SHUTDOWN
    }
}
#[doc = "Field `SRAM1_OFF` writer - System RAM block 1 Shut Down."]
pub type SRAM1_OFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRAM1_OFF_A>;
impl<'a, REG, const O: u8> SRAM1_OFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1_OFF_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1_OFF_A::SHUTDOWN)
    }
}
#[doc = "Field `SRAM2_OFF` reader - System RAM block 2 Shut Down."]
pub type SRAM2_OFF_R = crate::BitReader<SRAM2_OFF_A>;
#[doc = "System RAM block 2 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_OFF_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM2_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM2_OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM2_OFF_A {
        match self.bits {
            false => SRAM2_OFF_A::NORMAL,
            true => SRAM2_OFF_A::SHUTDOWN,
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM2_OFF_A::NORMAL
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM2_OFF_A::SHUTDOWN
    }
}
#[doc = "Field `SRAM2_OFF` writer - System RAM block 2 Shut Down."]
pub type SRAM2_OFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRAM2_OFF_A>;
impl<'a, REG, const O: u8> SRAM2_OFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_OFF_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_OFF_A::SHUTDOWN)
    }
}
#[doc = "Field `SRAM3_OFF` reader - System RAM block 3 Shut Down."]
pub type SRAM3_OFF_R = crate::BitReader<SRAM3_OFF_A>;
#[doc = "System RAM block 3 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM3_OFF_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM3_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM3_OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM3_OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM3_OFF_A {
        match self.bits {
            false => SRAM3_OFF_A::NORMAL,
            true => SRAM3_OFF_A::SHUTDOWN,
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM3_OFF_A::NORMAL
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM3_OFF_A::SHUTDOWN
    }
}
#[doc = "Field `SRAM3_OFF` writer - System RAM block 3 Shut Down."]
pub type SRAM3_OFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRAM3_OFF_A>;
impl<'a, REG, const O: u8> SRAM3_OFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM3_OFF_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM3_OFF_A::SHUTDOWN)
    }
}
impl R {
    #[doc = "Bit 0 - System RAM block 0 Shut Down."]
    #[inline(always)]
    pub fn sram0_off(&self) -> SRAM0_OFF_R {
        SRAM0_OFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System RAM block 1 Shut Down."]
    #[inline(always)]
    pub fn sram1_off(&self) -> SRAM1_OFF_R {
        SRAM1_OFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System RAM block 2 Shut Down."]
    #[inline(always)]
    pub fn sram2_off(&self) -> SRAM2_OFF_R {
        SRAM2_OFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System RAM block 3 Shut Down."]
    #[inline(always)]
    pub fn sram3_off(&self) -> SRAM3_OFF_R {
        SRAM3_OFF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM block 0 Shut Down."]
    #[inline(always)]
    #[must_use]
    pub fn sram0_off(&mut self) -> SRAM0_OFF_W<LPMEMSD_SPEC, 0> {
        SRAM0_OFF_W::new(self)
    }
    #[doc = "Bit 1 - System RAM block 1 Shut Down."]
    #[inline(always)]
    #[must_use]
    pub fn sram1_off(&mut self) -> SRAM1_OFF_W<LPMEMSD_SPEC, 1> {
        SRAM1_OFF_W::new(self)
    }
    #[doc = "Bit 2 - System RAM block 2 Shut Down."]
    #[inline(always)]
    #[must_use]
    pub fn sram2_off(&mut self) -> SRAM2_OFF_W<LPMEMSD_SPEC, 2> {
        SRAM2_OFF_W::new(self)
    }
    #[doc = "Bit 3 - System RAM block 3 Shut Down."]
    #[inline(always)]
    #[must_use]
    pub fn sram3_off(&mut self) -> SRAM3_OFF_W<LPMEMSD_SPEC, 3> {
        SRAM3_OFF_W::new(self)
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
#[doc = "Low Power Memory Shutdown Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmemsd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmemsd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMEMSD_SPEC;
impl crate::RegisterSpec for LPMEMSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmemsd::R`](R) reader structure"]
impl crate::Readable for LPMEMSD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpmemsd::W`](W) writer structure"]
impl crate::Writable for LPMEMSD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMEMSD to value 0"]
impl crate::Resettable for LPMEMSD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
