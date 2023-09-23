#[doc = "Register `BAUD0` reader"]
pub type R = crate::R<BAUD0_SPEC>;
#[doc = "Register `BAUD0` writer"]
pub type W = crate::W<BAUD0_SPEC>;
#[doc = "Field `IBAUD` reader - Integer portion of baud rate divisor value."]
pub type IBAUD_R = crate::FieldReader<u16>;
#[doc = "Field `IBAUD` writer - Integer portion of baud rate divisor value."]
pub type IBAUD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `CLKDIV` reader - FACTOR must be chosen to have IDIV>0."]
pub type CLKDIV_R = crate::FieldReader<CLKDIV_A>;
#[doc = "FACTOR must be chosen to have IDIV>0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV_A {
    #[doc = "0: Baud Factor 128."]
    _128 = 0,
    #[doc = "1: Baud Factor 64."]
    _64 = 1,
    #[doc = "2: Baud Factor 32."]
    _32 = 2,
    #[doc = "3: Baud Factor 16."]
    _16 = 3,
    #[doc = "4: Baud Factor 8."]
    _8 = 4,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKDIV_A {
    type Ux = u8;
}
impl CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKDIV_A> {
        match self.bits {
            0 => Some(CLKDIV_A::_128),
            1 => Some(CLKDIV_A::_64),
            2 => Some(CLKDIV_A::_32),
            3 => Some(CLKDIV_A::_16),
            4 => Some(CLKDIV_A::_8),
            _ => None,
        }
    }
    #[doc = "Baud Factor 128."]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == CLKDIV_A::_128
    }
    #[doc = "Baud Factor 64."]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == CLKDIV_A::_64
    }
    #[doc = "Baud Factor 32."]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == CLKDIV_A::_32
    }
    #[doc = "Baud Factor 16."]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CLKDIV_A::_16
    }
    #[doc = "Baud Factor 8."]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CLKDIV_A::_8
    }
}
#[doc = "Field `CLKDIV` writer - FACTOR must be chosen to have IDIV>0."]
pub type CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CLKDIV_A>;
impl<'a, REG, const O: u8> CLKDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Baud Factor 128."]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::_128)
    }
    #[doc = "Baud Factor 64."]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::_64)
    }
    #[doc = "Baud Factor 32."]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::_32)
    }
    #[doc = "Baud Factor 16."]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::_16)
    }
    #[doc = "Baud Factor 8."]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::_8)
    }
}
impl R {
    #[doc = "Bits 0:11 - Integer portion of baud rate divisor value."]
    #[inline(always)]
    pub fn ibaud(&self) -> IBAUD_R {
        IBAUD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - FACTOR must be chosen to have IDIV>0."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Integer portion of baud rate divisor value."]
    #[inline(always)]
    #[must_use]
    pub fn ibaud(&mut self) -> IBAUD_W<BAUD0_SPEC, 0> {
        IBAUD_W::new(self)
    }
    #[doc = "Bits 16:18 - FACTOR must be chosen to have IDIV>0."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<BAUD0_SPEC, 16> {
        CLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Baud rate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUD0_SPEC;
impl crate::RegisterSpec for BAUD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baud0::R`](R) reader structure"]
impl crate::Readable for BAUD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baud0::W`](W) writer structure"]
impl crate::Writable for BAUD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUD0 to value 0"]
impl crate::Resettable for BAUD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
