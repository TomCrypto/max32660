#[doc = "Register `INTEN1` reader"]
pub type R = crate::R<INTEN1_SPEC>;
#[doc = "Register `INTEN1` writer"]
pub type W = crate::W<INTEN1_SPEC>;
#[doc = "Field `RXOFIE` reader - Receiver Overflow Interrupt Enable."]
pub type RXOFIE_R = crate::BitReader<RXOFIE_A>;
#[doc = "Receiver Overflow Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOFIE_A {
    #[doc = "0: No Interrupt is Pending."]
    DISABLED = 0,
    #[doc = "1: An interrupt is pending."]
    ENABLED = 1,
}
impl From<RXOFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXOFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOFIE_A {
        match self.bits {
            false => RXOFIE_A::DISABLED,
            true => RXOFIE_A::ENABLED,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXOFIE_A::DISABLED
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXOFIE_A::ENABLED
    }
}
#[doc = "Field `RXOFIE` writer - Receiver Overflow Interrupt Enable."]
pub type RXOFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXOFIE_A>;
impl<'a, REG, const O: u8> RXOFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXOFIE_A::DISABLED)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXOFIE_A::ENABLED)
    }
}
#[doc = "Field `TXUFIE` reader - Transmit Underflow Interrupt Enable."]
pub type TXUFIE_R = crate::BitReader<TXUFIE_A>;
#[doc = "Transmit Underflow Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXUFIE_A {
    #[doc = "0: No Interrupt is Pending."]
    DISABLED = 0,
    #[doc = "1: An interrupt is pending."]
    ENABLED = 1,
}
impl From<TXUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXUFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXUFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUFIE_A {
        match self.bits {
            false => TXUFIE_A::DISABLED,
            true => TXUFIE_A::ENABLED,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXUFIE_A::DISABLED
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXUFIE_A::ENABLED
    }
}
#[doc = "Field `TXUFIE` writer - Transmit Underflow Interrupt Enable."]
pub type TXUFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXUFIE_A>;
impl<'a, REG, const O: u8> TXUFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXUFIE_A::DISABLED)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXUFIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Overflow Interrupt Enable."]
    #[inline(always)]
    pub fn rxofie(&self) -> RXOFIE_R {
        RXOFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt Enable."]
    #[inline(always)]
    pub fn txufie(&self) -> TXUFIE_R {
        TXUFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Overflow Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rxofie(&mut self) -> RXOFIE_W<INTEN1_SPEC, 0> {
        RXOFIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn txufie(&mut self) -> TXUFIE_W<INTEN1_SPEC, 1> {
        TXUFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Status Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN1_SPEC;
impl crate::RegisterSpec for INTEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten1::R`](R) reader structure"]
impl crate::Readable for INTEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten1::W`](W) writer structure"]
impl crate::Writable for INTEN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN1 to value 0"]
impl crate::Resettable for INTEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
