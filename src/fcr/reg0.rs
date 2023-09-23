#[doc = "Register `REG0` reader"]
pub type R = crate::R<REG0_SPEC>;
#[doc = "Register `REG0` writer"]
pub type W = crate::W<REG0_SPEC>;
#[doc = "Field `I2C0_SDA_FILTER_EN` reader - I2C0 SDA Filter Enable."]
pub type I2C0_SDA_FILTER_EN_R = crate::BitReader<I2C0_SDA_FILTER_EN_A>;
#[doc = "I2C0 SDA Filter Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0_SDA_FILTER_EN_A {
    #[doc = "0: Filter disabled."]
    DISABLED = 0,
    #[doc = "1: Filter enabled."]
    ENABLED = 1,
}
impl From<I2C0_SDA_FILTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_SDA_FILTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0_SDA_FILTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_SDA_FILTER_EN_A {
        match self.bits {
            false => I2C0_SDA_FILTER_EN_A::DISABLED,
            true => I2C0_SDA_FILTER_EN_A::ENABLED,
        }
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C0_SDA_FILTER_EN_A::DISABLED
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C0_SDA_FILTER_EN_A::ENABLED
    }
}
#[doc = "Field `I2C0_SDA_FILTER_EN` writer - I2C0 SDA Filter Enable."]
pub type I2C0_SDA_FILTER_EN_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, I2C0_SDA_FILTER_EN_A>;
impl<'a, REG, const O: u8> I2C0_SDA_FILTER_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0_SDA_FILTER_EN_A::DISABLED)
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0_SDA_FILTER_EN_A::ENABLED)
    }
}
#[doc = "Field `I2C0_SCL_FILTER_EN` reader - I2C0 SCL Filter Enable."]
pub type I2C0_SCL_FILTER_EN_R = crate::BitReader<I2C0_SCL_FILTER_EN_A>;
#[doc = "I2C0 SCL Filter Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0_SCL_FILTER_EN_A {
    #[doc = "0: Filter disabled."]
    DISABLED = 0,
    #[doc = "1: Filter enabled."]
    ENABLED = 1,
}
impl From<I2C0_SCL_FILTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_SCL_FILTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0_SCL_FILTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_SCL_FILTER_EN_A {
        match self.bits {
            false => I2C0_SCL_FILTER_EN_A::DISABLED,
            true => I2C0_SCL_FILTER_EN_A::ENABLED,
        }
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C0_SCL_FILTER_EN_A::DISABLED
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C0_SCL_FILTER_EN_A::ENABLED
    }
}
#[doc = "Field `I2C0_SCL_FILTER_EN` writer - I2C0 SCL Filter Enable."]
pub type I2C0_SCL_FILTER_EN_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, I2C0_SCL_FILTER_EN_A>;
impl<'a, REG, const O: u8> I2C0_SCL_FILTER_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0_SCL_FILTER_EN_A::DISABLED)
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0_SCL_FILTER_EN_A::ENABLED)
    }
}
#[doc = "Field `I2C1_SDA_FILTER_EN` reader - I2C1 SDA Filter Enable."]
pub type I2C1_SDA_FILTER_EN_R = crate::BitReader<I2C1_SDA_FILTER_EN_A>;
#[doc = "I2C1 SDA Filter Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_SDA_FILTER_EN_A {
    #[doc = "0: Filter disabled."]
    DISABLED = 0,
    #[doc = "1: Filter enabled."]
    ENABLED = 1,
}
impl From<I2C1_SDA_FILTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_SDA_FILTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_SDA_FILTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_SDA_FILTER_EN_A {
        match self.bits {
            false => I2C1_SDA_FILTER_EN_A::DISABLED,
            true => I2C1_SDA_FILTER_EN_A::ENABLED,
        }
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C1_SDA_FILTER_EN_A::DISABLED
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C1_SDA_FILTER_EN_A::ENABLED
    }
}
#[doc = "Field `I2C1_SDA_FILTER_EN` writer - I2C1 SDA Filter Enable."]
pub type I2C1_SDA_FILTER_EN_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, I2C1_SDA_FILTER_EN_A>;
impl<'a, REG, const O: u8> I2C1_SDA_FILTER_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_SDA_FILTER_EN_A::DISABLED)
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_SDA_FILTER_EN_A::ENABLED)
    }
}
#[doc = "Field `I2C1_SCL_FILTER_EN` reader - I2C1 SCL Filter Enable."]
pub type I2C1_SCL_FILTER_EN_R = crate::BitReader<I2C1_SCL_FILTER_EN_A>;
#[doc = "I2C1 SCL Filter Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_SCL_FILTER_EN_A {
    #[doc = "0: Filter disabled."]
    DISABLED = 0,
    #[doc = "1: Filter enabled."]
    ENABLED = 1,
}
impl From<I2C1_SCL_FILTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_SCL_FILTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_SCL_FILTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_SCL_FILTER_EN_A {
        match self.bits {
            false => I2C1_SCL_FILTER_EN_A::DISABLED,
            true => I2C1_SCL_FILTER_EN_A::ENABLED,
        }
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C1_SCL_FILTER_EN_A::DISABLED
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C1_SCL_FILTER_EN_A::ENABLED
    }
}
#[doc = "Field `I2C1_SCL_FILTER_EN` writer - I2C1 SCL Filter Enable."]
pub type I2C1_SCL_FILTER_EN_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, I2C1_SCL_FILTER_EN_A>;
impl<'a, REG, const O: u8> I2C1_SCL_FILTER_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_SCL_FILTER_EN_A::DISABLED)
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_SCL_FILTER_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 20 - I2C0 SDA Filter Enable."]
    #[inline(always)]
    pub fn i2c0_sda_filter_en(&self) -> I2C0_SDA_FILTER_EN_R {
        I2C0_SDA_FILTER_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 SCL Filter Enable."]
    #[inline(always)]
    pub fn i2c0_scl_filter_en(&self) -> I2C0_SCL_FILTER_EN_R {
        I2C0_SCL_FILTER_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 SDA Filter Enable."]
    #[inline(always)]
    pub fn i2c1_sda_filter_en(&self) -> I2C1_SDA_FILTER_EN_R {
        I2C1_SDA_FILTER_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C1 SCL Filter Enable."]
    #[inline(always)]
    pub fn i2c1_scl_filter_en(&self) -> I2C1_SCL_FILTER_EN_R {
        I2C1_SCL_FILTER_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - I2C0 SDA Filter Enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_sda_filter_en(&mut self) -> I2C0_SDA_FILTER_EN_W<REG0_SPEC, 20> {
        I2C0_SDA_FILTER_EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C0 SCL Filter Enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_scl_filter_en(&mut self) -> I2C0_SCL_FILTER_EN_W<REG0_SPEC, 21> {
        I2C0_SCL_FILTER_EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C1 SDA Filter Enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_sda_filter_en(&mut self) -> I2C1_SDA_FILTER_EN_W<REG0_SPEC, 22> {
        I2C1_SDA_FILTER_EN_W::new(self)
    }
    #[doc = "Bit 23 - I2C1 SCL Filter Enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_scl_filter_en(&mut self) -> I2C1_SCL_FILTER_EN_W<REG0_SPEC, 23> {
        I2C1_SCL_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG0_SPEC;
impl crate::RegisterSpec for REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg0::R`](R) reader structure"]
impl crate::Readable for REG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg0::W`](W) writer structure"]
impl crate::Writable for REG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG0 to value 0"]
impl crate::Resettable for REG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
