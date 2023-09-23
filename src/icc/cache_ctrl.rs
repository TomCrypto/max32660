#[doc = "Register `CACHE_CTRL` reader"]
pub type R = crate::R<CACHE_CTRL_SPEC>;
#[doc = "Register `CACHE_CTRL` writer"]
pub type W = crate::W<CACHE_CTRL_SPEC>;
#[doc = "Field `enabled` reader - Cache Enable."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "Cache Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLED_A {
    #[doc = "0: Cache Bypassed."]
    DISABLED = 0,
    #[doc = "1: Cache Enabled."]
    ENABLED = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DISABLED,
            true => ENABLED_A::ENABLED,
        }
    }
    #[doc = "Cache Bypassed."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLED_A::DISABLED
    }
    #[doc = "Cache Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLED_A::ENABLED
    }
}
#[doc = "Field `enabled` writer - Cache Enable."]
pub type ENABLED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENABLED_A>;
impl<'a, REG, const O: u8> ENABLED_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cache Bypassed."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "Cache Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLED_A::ENABLED)
    }
}
#[doc = "Field `READY` reader - Cache Ready flag."]
pub type READY_R = crate::BitReader<READY_A>;
#[doc = "Cache Ready flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY_A {
    #[doc = "0: Not Ready."]
    NOT_READY = 0,
    #[doc = "1: Ready."]
    READY = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::NOT_READY,
            true => READY_A::READY,
        }
    }
    #[doc = "Not Ready."]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == READY_A::NOT_READY
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READY_A::READY
    }
}
impl R {
    #[doc = "Bit 0 - Cache Enable."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Cache Ready flag."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CACHE_CTRL_SPEC, 0> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cache Control and Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_CTRL to value 0"]
impl crate::Resettable for CACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
