#[doc = "Register `RST1` reader"]
pub type R = crate::R<RST1_SPEC>;
#[doc = "Register `RST1` writer"]
pub type W = crate::W<RST1_SPEC>;
#[doc = "Field `I2C1` reader - I2C1 Reset."]
pub type I2C1_R = crate::BitReader<I2C1_A>;
#[doc = "I2C1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<I2C1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_A {
        match self.bits {
            false => I2C1_A::RESET_DONE,
            true => I2C1_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == I2C1_A::RESET_DONE
    }
    #[doc = "Reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == I2C1_A::BUSY
    }
}
#[doc = "I2C1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_AW {
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<I2C1_AW> for bool {
    #[inline(always)]
    fn from(variant: I2C1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1` writer - I2C1 Reset."]
pub type I2C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C1_AW>;
impl<'a, REG, const O: u8> I2C1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_AW::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<RST1_SPEC, 0> {
        I2C1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reset 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST1_SPEC;
impl crate::RegisterSpec for RST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst1::R`](R) reader structure"]
impl crate::Readable for RST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst1::W`](W) writer structure"]
impl crate::Writable for RST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST1 to value 0"]
impl crate::Resettable for RST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
