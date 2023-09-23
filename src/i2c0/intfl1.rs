#[doc = "Register `INTFL1` reader"]
pub type R = crate::R<INTFL1_SPEC>;
#[doc = "Register `INTFL1` writer"]
pub type W = crate::W<INTFL1_SPEC>;
#[doc = "Field `RXOFI` reader - Receiver Overflow Interrupt."]
pub type RXOFI_R = crate::BitReader<RXOFI_A>;
#[doc = "Receiver Overflow Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOFI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<RXOFI_A> for bool {
    #[inline(always)]
    fn from(variant: RXOFI_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOFI_A {
        match self.bits {
            false => RXOFI_A::INACTIVE,
            true => RXOFI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RXOFI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXOFI_A::PENDING
    }
}
#[doc = "Field `RXOFI` writer - Receiver Overflow Interrupt."]
pub type RXOFI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXOFI_A>;
impl<'a, REG, const O: u8> RXOFI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(RXOFI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RXOFI_A::PENDING)
    }
}
#[doc = "Field `TXUFI` reader - Transmit Underflow Interrupt."]
pub type TXUFI_R = crate::BitReader<TXUFI_A>;
#[doc = "Transmit Underflow Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXUFI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<TXUFI_A> for bool {
    #[inline(always)]
    fn from(variant: TXUFI_A) -> Self {
        variant as u8 != 0
    }
}
impl TXUFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUFI_A {
        match self.bits {
            false => TXUFI_A::INACTIVE,
            true => TXUFI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TXUFI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TXUFI_A::PENDING
    }
}
#[doc = "Field `TXUFI` writer - Transmit Underflow Interrupt."]
pub type TXUFI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXUFI_A>;
impl<'a, REG, const O: u8> TXUFI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(TXUFI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TXUFI_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Overflow Interrupt."]
    #[inline(always)]
    pub fn rxofi(&self) -> RXOFI_R {
        RXOFI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt."]
    #[inline(always)]
    pub fn txufi(&self) -> TXUFI_R {
        TXUFI_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Overflow Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxofi(&mut self) -> RXOFI_W<INTFL1_SPEC, 0> {
        RXOFI_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txufi(&mut self) -> TXUFI_W<INTFL1_SPEC, 1> {
        TXUFI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Status Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFL1_SPEC;
impl crate::RegisterSpec for INTFL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl1::R`](R) reader structure"]
impl crate::Readable for INTFL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intfl1::W`](W) writer structure"]
impl crate::Writable for INTFL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFL1 to value 0"]
impl crate::Resettable for INTFL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
