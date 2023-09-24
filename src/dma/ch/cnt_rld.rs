#[doc = "Register `CNT_RLD` reader"]
pub type R = crate::R<CNT_RLD_SPEC>;
#[doc = "Register `CNT_RLD` writer"]
pub type W = crate::W<CNT_RLD_SPEC>;
#[doc = "Field `CNT_RLD` reader - Count Reload Value."]
pub type CNT_RLD_R = crate::FieldReader<u32>;
#[doc = "Field `CNT_RLD` writer - Count Reload Value."]
pub type CNT_RLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `RLDEN` reader - Reload Enable."]
pub type RLDEN_R = crate::BitReader<RLDEN_A>;
#[doc = "Reload Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLDEN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RLDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLDEN_A {
        match self.bits {
            false => RLDEN_A::DISABLED,
            true => RLDEN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RLDEN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RLDEN_A::ENABLED
    }
}
#[doc = "Field `RLDEN` writer - Reload Enable."]
pub type RLDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RLDEN_A>;
impl<'a, REG, const O: u8> RLDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RLDEN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RLDEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:23 - Count Reload Value."]
    #[inline(always)]
    pub fn cnt_rld(&self) -> CNT_RLD_R {
        CNT_RLD_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Reload Enable."]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Count Reload Value."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rld(&mut self) -> CNT_RLD_W<CNT_RLD_SPEC, 0> {
        CNT_RLD_W::new(self)
    }
    #[doc = "Bit 31 - Reload Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rlden(&mut self) -> RLDEN_W<CNT_RLD_SPEC, 31> {
        RLDEN_W::new(self)
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
#[doc = "DMA Channel Count Reload.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt_rld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt_rld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_RLD_SPEC;
impl crate::RegisterSpec for CNT_RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt_rld::R`](R) reader structure"]
impl crate::Readable for CNT_RLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnt_rld::W`](W) writer structure"]
impl crate::Writable for CNT_RLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNT_RLD to value 0"]
impl crate::Resettable for CNT_RLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
