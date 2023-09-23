#[doc = "Register `MEM_CTRL` reader"]
pub type R = crate::R<MEM_CTRL_SPEC>;
#[doc = "Register `MEM_CTRL` writer"]
pub type W = crate::W<MEM_CTRL_SPEC>;
#[doc = "Field `FWS` reader - Flash Wait State."]
pub type FWS_R = crate::FieldReader;
#[doc = "Field `FWS` writer - Flash Wait State."]
pub type FWS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RAM0_LS` reader - System RAM 0 Light Sleep Mode."]
pub type RAM0_LS_R = crate::BitReader<RAM0_LS_A>;
#[doc = "System RAM 0 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM0_LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<RAM0_LS_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_LS_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM0_LS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_LS_A {
        match self.bits {
            false => RAM0_LS_A::ACTIVE,
            true => RAM0_LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RAM0_LS_A::ACTIVE
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == RAM0_LS_A::LIGHT_SLEEP
    }
}
#[doc = "Field `RAM0_LS` writer - System RAM 0 Light Sleep Mode."]
pub type RAM0_LS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAM0_LS_A>;
impl<'a, REG, const O: u8> RAM0_LS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(RAM0_LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(RAM0_LS_A::LIGHT_SLEEP)
    }
}
#[doc = "Field `RAM1_LS` reader - System RAM 1 Light Sleep Mode."]
pub type RAM1_LS_R = crate::BitReader<RAM1_LS_A>;
#[doc = "System RAM 1 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM1_LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<RAM1_LS_A> for bool {
    #[inline(always)]
    fn from(variant: RAM1_LS_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM1_LS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM1_LS_A {
        match self.bits {
            false => RAM1_LS_A::ACTIVE,
            true => RAM1_LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RAM1_LS_A::ACTIVE
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == RAM1_LS_A::LIGHT_SLEEP
    }
}
#[doc = "Field `RAM1_LS` writer - System RAM 1 Light Sleep Mode."]
pub type RAM1_LS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAM1_LS_A>;
impl<'a, REG, const O: u8> RAM1_LS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(RAM1_LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(RAM1_LS_A::LIGHT_SLEEP)
    }
}
#[doc = "Field `RAM2_LS` reader - System RAM 2 Light Sleep Mode."]
pub type RAM2_LS_R = crate::BitReader<RAM2_LS_A>;
#[doc = "System RAM 2 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM2_LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<RAM2_LS_A> for bool {
    #[inline(always)]
    fn from(variant: RAM2_LS_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM2_LS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM2_LS_A {
        match self.bits {
            false => RAM2_LS_A::ACTIVE,
            true => RAM2_LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RAM2_LS_A::ACTIVE
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == RAM2_LS_A::LIGHT_SLEEP
    }
}
#[doc = "Field `RAM2_LS` writer - System RAM 2 Light Sleep Mode."]
pub type RAM2_LS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAM2_LS_A>;
impl<'a, REG, const O: u8> RAM2_LS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(RAM2_LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(RAM2_LS_A::LIGHT_SLEEP)
    }
}
#[doc = "Field `RAM3_LS` reader - System RAM 3 Light Sleep Mode."]
pub type RAM3_LS_R = crate::BitReader<RAM3_LS_A>;
#[doc = "System RAM 3 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM3_LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<RAM3_LS_A> for bool {
    #[inline(always)]
    fn from(variant: RAM3_LS_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM3_LS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM3_LS_A {
        match self.bits {
            false => RAM3_LS_A::ACTIVE,
            true => RAM3_LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RAM3_LS_A::ACTIVE
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == RAM3_LS_A::LIGHT_SLEEP
    }
}
#[doc = "Field `RAM3_LS` writer - System RAM 3 Light Sleep Mode."]
pub type RAM3_LS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAM3_LS_A>;
impl<'a, REG, const O: u8> RAM3_LS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(RAM3_LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(RAM3_LS_A::LIGHT_SLEEP)
    }
}
#[doc = "Field `ICACHE_RET` reader - ICache RAM Light Sleep Mode."]
pub type ICACHE_RET_R = crate::BitReader<ICACHE_RET_A>;
#[doc = "ICache RAM Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICACHE_RET_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<ICACHE_RET_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHE_RET_A) -> Self {
        variant as u8 != 0
    }
}
impl ICACHE_RET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHE_RET_A {
        match self.bits {
            false => ICACHE_RET_A::ACTIVE,
            true => ICACHE_RET_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ICACHE_RET_A::ACTIVE
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == ICACHE_RET_A::LIGHT_SLEEP
    }
}
#[doc = "Field `ICACHE_RET` writer - ICache RAM Light Sleep Mode."]
pub type ICACHE_RET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ICACHE_RET_A>;
impl<'a, REG, const O: u8> ICACHE_RET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(ICACHE_RET_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(ICACHE_RET_A::LIGHT_SLEEP)
    }
}
impl R {
    #[doc = "Bits 0:2 - Flash Wait State."]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - System RAM 0 Light Sleep Mode."]
    #[inline(always)]
    pub fn ram0_ls(&self) -> RAM0_LS_R {
        RAM0_LS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - System RAM 1 Light Sleep Mode."]
    #[inline(always)]
    pub fn ram1_ls(&self) -> RAM1_LS_R {
        RAM1_LS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - System RAM 2 Light Sleep Mode."]
    #[inline(always)]
    pub fn ram2_ls(&self) -> RAM2_LS_R {
        RAM2_LS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - System RAM 3 Light Sleep Mode."]
    #[inline(always)]
    pub fn ram3_ls(&self) -> RAM3_LS_R {
        RAM3_LS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ICache RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn icache_ret(&self) -> ICACHE_RET_R {
        ICACHE_RET_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flash Wait State."]
    #[inline(always)]
    #[must_use]
    pub fn fws(&mut self) -> FWS_W<MEM_CTRL_SPEC, 0> {
        FWS_W::new(self)
    }
    #[doc = "Bit 8 - System RAM 0 Light Sleep Mode."]
    #[inline(always)]
    #[must_use]
    pub fn ram0_ls(&mut self) -> RAM0_LS_W<MEM_CTRL_SPEC, 8> {
        RAM0_LS_W::new(self)
    }
    #[doc = "Bit 9 - System RAM 1 Light Sleep Mode."]
    #[inline(always)]
    #[must_use]
    pub fn ram1_ls(&mut self) -> RAM1_LS_W<MEM_CTRL_SPEC, 9> {
        RAM1_LS_W::new(self)
    }
    #[doc = "Bit 10 - System RAM 2 Light Sleep Mode."]
    #[inline(always)]
    #[must_use]
    pub fn ram2_ls(&mut self) -> RAM2_LS_W<MEM_CTRL_SPEC, 10> {
        RAM2_LS_W::new(self)
    }
    #[doc = "Bit 11 - System RAM 3 Light Sleep Mode."]
    #[inline(always)]
    #[must_use]
    pub fn ram3_ls(&mut self) -> RAM3_LS_W<MEM_CTRL_SPEC, 11> {
        RAM3_LS_W::new(self)
    }
    #[doc = "Bit 12 - ICache RAM Light Sleep Mode."]
    #[inline(always)]
    #[must_use]
    pub fn icache_ret(&mut self) -> ICACHE_RET_W<MEM_CTRL_SPEC, 12> {
        ICACHE_RET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Memory Clock Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CTRL_SPEC;
impl crate::RegisterSpec for MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_CTRL to value 0"]
impl crate::Resettable for MEM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
