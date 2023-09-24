#[doc = "Register `MEM_ZCTRL` reader"]
pub type R = crate::R<MEM_ZCTRL_SPEC>;
#[doc = "Register `MEM_ZCTRL` writer"]
pub type W = crate::W<MEM_ZCTRL_SPEC>;
#[doc = "Field `SRAM_ZERO` reader - System RAM Block 0."]
pub type SRAM_ZERO_R = crate::BitReader<SRAM_ZERO_A>;
#[doc = "System RAM Block 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_ZERO_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SRAM_ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_ZERO_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_ZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_ZERO_A {
        match self.bits {
            false => SRAM_ZERO_A::NOP,
            true => SRAM_ZERO_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SRAM_ZERO_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SRAM_ZERO_A::START
    }
}
#[doc = "Field `SRAM_ZERO` writer - System RAM Block 0."]
pub type SRAM_ZERO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRAM_ZERO_A>;
impl<'a, REG, const O: u8> SRAM_ZERO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_ZERO_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_ZERO_A::START)
    }
}
#[doc = "Field `ICACHE_ZERO` reader - Instruction Cache."]
pub type ICACHE_ZERO_R = crate::BitReader<ICACHE_ZERO_A>;
#[doc = "Instruction Cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICACHE_ZERO_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<ICACHE_ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHE_ZERO_A) -> Self {
        variant as u8 != 0
    }
}
impl ICACHE_ZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHE_ZERO_A {
        match self.bits {
            false => ICACHE_ZERO_A::NOP,
            true => ICACHE_ZERO_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ICACHE_ZERO_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ICACHE_ZERO_A::START
    }
}
#[doc = "Field `ICACHE_ZERO` writer - Instruction Cache."]
pub type ICACHE_ZERO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ICACHE_ZERO_A>;
impl<'a, REG, const O: u8> ICACHE_ZERO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(ICACHE_ZERO_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ICACHE_ZERO_A::START)
    }
}
impl R {
    #[doc = "Bit 0 - System RAM Block 0."]
    #[inline(always)]
    pub fn sram_zero(&self) -> SRAM_ZERO_R {
        SRAM_ZERO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Instruction Cache."]
    #[inline(always)]
    pub fn icache_zero(&self) -> ICACHE_ZERO_R {
        ICACHE_ZERO_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM Block 0."]
    #[inline(always)]
    #[must_use]
    pub fn sram_zero(&mut self) -> SRAM_ZERO_W<MEM_ZCTRL_SPEC, 0> {
        SRAM_ZERO_W::new(self)
    }
    #[doc = "Bit 1 - Instruction Cache."]
    #[inline(always)]
    #[must_use]
    pub fn icache_zero(&mut self) -> ICACHE_ZERO_W<MEM_ZCTRL_SPEC, 1> {
        ICACHE_ZERO_W::new(self)
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
#[doc = "Memory Zeroize Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_zctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_zctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_ZCTRL_SPEC;
impl crate::RegisterSpec for MEM_ZCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_zctrl::R`](R) reader structure"]
impl crate::Readable for MEM_ZCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_zctrl::W`](W) writer structure"]
impl crate::Writable for MEM_ZCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_ZCTRL to value 0"]
impl crate::Resettable for MEM_ZCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
