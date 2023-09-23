#[doc = "Register `SS_TIME` reader"]
pub type R = crate::R<SS_TIME_SPEC>;
#[doc = "Register `SS_TIME` writer"]
pub type W = crate::W<SS_TIME_SPEC>;
#[doc = "Field `SSACT1` reader - Slave Select Pre delay 1."]
pub type SSACT1_R = crate::FieldReader<SSACT1_A>;
#[doc = "Slave Select Pre delay 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSACT1_A {
    #[doc = "0: 256 system clocks between SS active and first serial clock edge."]
    _256 = 0,
}
impl From<SSACT1_A> for u8 {
    #[inline(always)]
    fn from(variant: SSACT1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SSACT1_A {
    type Ux = u8;
}
impl SSACT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSACT1_A> {
        match self.bits {
            0 => Some(SSACT1_A::_256),
            _ => None,
        }
    }
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SSACT1_A::_256
    }
}
#[doc = "Field `SSACT1` writer - Slave Select Pre delay 1."]
pub type SSACT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, SSACT1_A>;
impl<'a, REG, const O: u8> SSACT1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(SSACT1_A::_256)
    }
}
#[doc = "Field `SSACT2` reader - Slave Select Post delay 2."]
pub type SSACT2_R = crate::FieldReader<SSACT2_A>;
#[doc = "Slave Select Post delay 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSACT2_A {
    #[doc = "0: 256 system clocks between last serial clock edge and SS inactive."]
    _256 = 0,
}
impl From<SSACT2_A> for u8 {
    #[inline(always)]
    fn from(variant: SSACT2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SSACT2_A {
    type Ux = u8;
}
impl SSACT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSACT2_A> {
        match self.bits {
            0 => Some(SSACT2_A::_256),
            _ => None,
        }
    }
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SSACT2_A::_256
    }
}
#[doc = "Field `SSACT2` writer - Slave Select Post delay 2."]
pub type SSACT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, SSACT2_A>;
impl<'a, REG, const O: u8> SSACT2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(SSACT2_A::_256)
    }
}
#[doc = "Field `SSINACT` reader - Slave Select Inactive delay."]
pub type SSINACT_R = crate::FieldReader<SSINACT_A>;
#[doc = "Slave Select Inactive delay.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSINACT_A {
    #[doc = "0: 256 system clocks between transactions."]
    _256 = 0,
}
impl From<SSINACT_A> for u8 {
    #[inline(always)]
    fn from(variant: SSINACT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SSINACT_A {
    type Ux = u8;
}
impl SSINACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSINACT_A> {
        match self.bits {
            0 => Some(SSINACT_A::_256),
            _ => None,
        }
    }
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SSINACT_A::_256
    }
}
#[doc = "Field `SSINACT` writer - Slave Select Inactive delay."]
pub type SSINACT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, SSINACT_A>;
impl<'a, REG, const O: u8> SSINACT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(SSINACT_A::_256)
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    pub fn ssact1(&self) -> SSACT1_R {
        SSACT1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    pub fn ssact2(&self) -> SSACT2_R {
        SSACT2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn ssinact(&self) -> SSINACT_R {
        SSINACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    #[must_use]
    pub fn ssact1(&mut self) -> SSACT1_W<SS_TIME_SPEC, 0> {
        SSACT1_W::new(self)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    #[must_use]
    pub fn ssact2(&mut self) -> SSACT2_W<SS_TIME_SPEC, 8> {
        SSACT2_W::new(self)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    #[must_use]
    pub fn ssinact(&mut self) -> SSINACT_W<SS_TIME_SPEC, 16> {
        SSINACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Slave Select Timing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SS_TIME_SPEC;
impl crate::RegisterSpec for SS_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_time::R`](R) reader structure"]
impl crate::Readable for SS_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ss_time::W`](W) writer structure"]
impl crate::Writable for SS_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SS_TIME to value 0"]
impl crate::Resettable for SS_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
