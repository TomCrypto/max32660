#[doc = "Register `I2S_CTRL` reader"]
pub type R = crate::R<I2S_CTRL_SPEC>;
#[doc = "Register `I2S_CTRL` writer"]
pub type W = crate::W<I2S_CTRL_SPEC>;
#[doc = "Field `I2S_EN` reader - I2S Mode Enable."]
pub type I2S_EN_R = crate::BitReader<I2S_EN_A>;
#[doc = "I2S Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<I2S_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_EN_A {
        match self.bits {
            false => I2S_EN_A::DISABLED,
            true => I2S_EN_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2S_EN_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2S_EN_A::ENABLED
    }
}
#[doc = "Field `I2S_EN` writer - I2S Mode Enable."]
pub type I2S_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2S_EN_A>;
impl<'a, REG, const O: u8> I2S_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_EN_A::ENABLED)
    }
}
#[doc = "Field `I2S_MUTE` reader - I2S Mute transmit."]
pub type I2S_MUTE_R = crate::BitReader<I2S_MUTE_A>;
#[doc = "I2S Mute transmit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_MUTE_A {
    #[doc = "0: Normal Transmit."]
    NORMAL = 0,
    #[doc = "1: Transmit data is replaced with 0."]
    REPLACED = 1,
}
impl From<I2S_MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_MUTE_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_MUTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_MUTE_A {
        match self.bits {
            false => I2S_MUTE_A::NORMAL,
            true => I2S_MUTE_A::REPLACED,
        }
    }
    #[doc = "Normal Transmit."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == I2S_MUTE_A::NORMAL
    }
    #[doc = "Transmit data is replaced with 0."]
    #[inline(always)]
    pub fn is_replaced(&self) -> bool {
        *self == I2S_MUTE_A::REPLACED
    }
}
#[doc = "Field `I2S_MUTE` writer - I2S Mute transmit."]
pub type I2S_MUTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2S_MUTE_A>;
impl<'a, REG, const O: u8> I2S_MUTE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Transmit."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_MUTE_A::NORMAL)
    }
    #[doc = "Transmit data is replaced with 0."]
    #[inline(always)]
    pub fn replaced(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_MUTE_A::REPLACED)
    }
}
#[doc = "Field `I2S_PAUSE` reader - I2S Pause transmit/receive."]
pub type I2S_PAUSE_R = crate::BitReader<I2S_PAUSE_A>;
#[doc = "I2S Pause transmit/receive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_PAUSE_A {
    #[doc = "0: Normal Transmit."]
    NORMAL = 0,
    #[doc = "1: Halt transmit and receive FIFO and DMA access, transmit 0's."]
    HALT = 1,
}
impl From<I2S_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_PAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_PAUSE_A {
        match self.bits {
            false => I2S_PAUSE_A::NORMAL,
            true => I2S_PAUSE_A::HALT,
        }
    }
    #[doc = "Normal Transmit."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == I2S_PAUSE_A::NORMAL
    }
    #[doc = "Halt transmit and receive FIFO and DMA access, transmit 0's."]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == I2S_PAUSE_A::HALT
    }
}
#[doc = "Field `I2S_PAUSE` writer - I2S Pause transmit/receive."]
pub type I2S_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2S_PAUSE_A>;
impl<'a, REG, const O: u8> I2S_PAUSE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Transmit."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_PAUSE_A::NORMAL)
    }
    #[doc = "Halt transmit and receive FIFO and DMA access, transmit 0's."]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_PAUSE_A::HALT)
    }
}
#[doc = "Field `I2S_MONO` reader - I2S Monophonic Audio Mode."]
pub type I2S_MONO_R = crate::BitReader<I2S_MONO_A>;
#[doc = "I2S Monophonic Audio Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_MONO_A {
    #[doc = "0: Stereophonic audio."]
    STEREOPHONIC = 0,
    #[doc = "1: Monophonic audio format."]
    MONOPHONIC = 1,
}
impl From<I2S_MONO_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_MONO_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_MONO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_MONO_A {
        match self.bits {
            false => I2S_MONO_A::STEREOPHONIC,
            true => I2S_MONO_A::MONOPHONIC,
        }
    }
    #[doc = "Stereophonic audio."]
    #[inline(always)]
    pub fn is_stereophonic(&self) -> bool {
        *self == I2S_MONO_A::STEREOPHONIC
    }
    #[doc = "Monophonic audio format."]
    #[inline(always)]
    pub fn is_monophonic(&self) -> bool {
        *self == I2S_MONO_A::MONOPHONIC
    }
}
#[doc = "Field `I2S_MONO` writer - I2S Monophonic Audio Mode."]
pub type I2S_MONO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2S_MONO_A>;
impl<'a, REG, const O: u8> I2S_MONO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stereophonic audio."]
    #[inline(always)]
    pub fn stereophonic(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_MONO_A::STEREOPHONIC)
    }
    #[doc = "Monophonic audio format."]
    #[inline(always)]
    pub fn monophonic(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_MONO_A::MONOPHONIC)
    }
}
#[doc = "Field `I2S_LJ` reader - I2S Left Justify."]
pub type I2S_LJ_R = crate::BitReader<I2S_LJ_A>;
#[doc = "I2S Left Justify.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_LJ_A {
    #[doc = "0: Normal I2S audio protocol."]
    NORMAL = 0,
    #[doc = "1: Audio data is synchronized with SSEL."]
    REPLACED = 1,
}
impl From<I2S_LJ_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_LJ_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_LJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_LJ_A {
        match self.bits {
            false => I2S_LJ_A::NORMAL,
            true => I2S_LJ_A::REPLACED,
        }
    }
    #[doc = "Normal I2S audio protocol."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == I2S_LJ_A::NORMAL
    }
    #[doc = "Audio data is synchronized with SSEL."]
    #[inline(always)]
    pub fn is_replaced(&self) -> bool {
        *self == I2S_LJ_A::REPLACED
    }
}
#[doc = "Field `I2S_LJ` writer - I2S Left Justify."]
pub type I2S_LJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2S_LJ_A>;
impl<'a, REG, const O: u8> I2S_LJ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal I2S audio protocol."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_LJ_A::NORMAL)
    }
    #[doc = "Audio data is synchronized with SSEL."]
    #[inline(always)]
    pub fn replaced(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_LJ_A::REPLACED)
    }
}
impl R {
    #[doc = "Bit 0 - I2S Mode Enable."]
    #[inline(always)]
    pub fn i2s_en(&self) -> I2S_EN_R {
        I2S_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2S Mute transmit."]
    #[inline(always)]
    pub fn i2s_mute(&self) -> I2S_MUTE_R {
        I2S_MUTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2S Pause transmit/receive."]
    #[inline(always)]
    pub fn i2s_pause(&self) -> I2S_PAUSE_R {
        I2S_PAUSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2S Monophonic Audio Mode."]
    #[inline(always)]
    pub fn i2s_mono(&self) -> I2S_MONO_R {
        I2S_MONO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2S Left Justify."]
    #[inline(always)]
    pub fn i2s_lj(&self) -> I2S_LJ_R {
        I2S_LJ_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S Mode Enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_en(&mut self) -> I2S_EN_W<I2S_CTRL_SPEC, 0> {
        I2S_EN_W::new(self)
    }
    #[doc = "Bit 1 - I2S Mute transmit."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_mute(&mut self) -> I2S_MUTE_W<I2S_CTRL_SPEC, 1> {
        I2S_MUTE_W::new(self)
    }
    #[doc = "Bit 2 - I2S Pause transmit/receive."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_pause(&mut self) -> I2S_PAUSE_W<I2S_CTRL_SPEC, 2> {
        I2S_PAUSE_W::new(self)
    }
    #[doc = "Bit 3 - I2S Monophonic Audio Mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_mono(&mut self) -> I2S_MONO_W<I2S_CTRL_SPEC, 3> {
        I2S_MONO_W::new(self)
    }
    #[doc = "Bit 4 - I2S Left Justify."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_lj(&mut self) -> I2S_LJ_W<I2S_CTRL_SPEC, 4> {
        I2S_LJ_W::new(self)
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
#[doc = "I2S Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_CTRL_SPEC;
impl crate::RegisterSpec for I2S_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_ctrl::R`](R) reader structure"]
impl crate::Readable for I2S_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_ctrl::W`](W) writer structure"]
impl crate::Writable for I2S_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_CTRL to value 0"]
impl crate::Resettable for I2S_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
