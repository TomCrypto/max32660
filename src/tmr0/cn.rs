#[doc = "Register `CN` reader"]
pub type R = crate::R<CN_SPEC>;
#[doc = "Register `CN` writer"]
pub type W = crate::W<CN_SPEC>;
#[doc = "Field `TMODE` reader - Mode Select."]
pub type TMODE_R = crate::FieldReader<TMODE_A>;
#[doc = "Mode Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMODE_A {
    #[doc = "0: One-Shot Mode."]
    ONE_SHOT = 0,
    #[doc = "1: Continuous Mode."]
    CONTINUOUS = 1,
    #[doc = "2: Counter Mode."]
    COUNTER = 2,
    #[doc = "3: PWM Mode."]
    PWM = 3,
    #[doc = "4: Capture Mode."]
    CAPTURE = 4,
    #[doc = "5: Compare Mode."]
    COMPARE = 5,
    #[doc = "6: Gated Mode."]
    GATED = 6,
    #[doc = "7: Capture/Compare Mode."]
    CAPTURE_COMPARE = 7,
}
impl From<TMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMODE_A {
    type Ux = u8;
}
impl TMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMODE_A {
        match self.bits {
            0 => TMODE_A::ONE_SHOT,
            1 => TMODE_A::CONTINUOUS,
            2 => TMODE_A::COUNTER,
            3 => TMODE_A::PWM,
            4 => TMODE_A::CAPTURE,
            5 => TMODE_A::COMPARE,
            6 => TMODE_A::GATED,
            7 => TMODE_A::CAPTURE_COMPARE,
            _ => unreachable!(),
        }
    }
    #[doc = "One-Shot Mode."]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == TMODE_A::ONE_SHOT
    }
    #[doc = "Continuous Mode."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMODE_A::CONTINUOUS
    }
    #[doc = "Counter Mode."]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == TMODE_A::COUNTER
    }
    #[doc = "PWM Mode."]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == TMODE_A::PWM
    }
    #[doc = "Capture Mode."]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == TMODE_A::CAPTURE
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == TMODE_A::COMPARE
    }
    #[doc = "Gated Mode."]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == TMODE_A::GATED
    }
    #[doc = "Capture/Compare Mode."]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        *self == TMODE_A::CAPTURE_COMPARE
    }
}
#[doc = "Field `TMODE` writer - Mode Select."]
pub type TMODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, TMODE_A>;
impl<'a, REG, const O: u8> TMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One-Shot Mode."]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::ONE_SHOT)
    }
    #[doc = "Continuous Mode."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::CONTINUOUS)
    }
    #[doc = "Counter Mode."]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::COUNTER)
    }
    #[doc = "PWM Mode."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::PWM)
    }
    #[doc = "Capture Mode."]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::CAPTURE)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::COMPARE)
    }
    #[doc = "Gated Mode."]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::GATED)
    }
    #[doc = "Capture/Compare Mode."]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::CAPTURE_COMPARE)
    }
}
#[doc = "Field `PRES` reader - Timer Prescaler Select."]
pub type PRES_R = crate::FieldReader<PRES_A>;
#[doc = "Timer Prescaler Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRES_A {
    #[doc = "0: Divide by 1."]
    DIV_BY_1 = 0,
    #[doc = "1: Divide by 2."]
    DIV_BY_2 = 1,
    #[doc = "2: Divide by 4."]
    DIV_BY_4 = 2,
    #[doc = "3: Divide by 8."]
    DIV_BY_8 = 3,
    #[doc = "4: Divide by 16."]
    DIV_BY_16 = 4,
    #[doc = "5: Divide by 32."]
    DIV_BY_32 = 5,
    #[doc = "6: Divide by 64."]
    DIV_BY_64 = 6,
    #[doc = "7: Divide by 128."]
    DIV_BY_128 = 7,
}
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRES_A {
    type Ux = u8;
}
impl PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRES_A {
        match self.bits {
            0 => PRES_A::DIV_BY_1,
            1 => PRES_A::DIV_BY_2,
            2 => PRES_A::DIV_BY_4,
            3 => PRES_A::DIV_BY_8,
            4 => PRES_A::DIV_BY_16,
            5 => PRES_A::DIV_BY_32,
            6 => PRES_A::DIV_BY_64,
            7 => PRES_A::DIV_BY_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == PRES_A::DIV_BY_1
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == PRES_A::DIV_BY_2
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == PRES_A::DIV_BY_4
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == PRES_A::DIV_BY_8
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == PRES_A::DIV_BY_16
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn is_div_by_32(&self) -> bool {
        *self == PRES_A::DIV_BY_32
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn is_div_by_64(&self) -> bool {
        *self == PRES_A::DIV_BY_64
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn is_div_by_128(&self) -> bool {
        *self == PRES_A::DIV_BY_128
    }
}
#[doc = "Field `PRES` writer - Timer Prescaler Select."]
pub type PRES_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, PRES_A>;
impl<'a, REG, const O: u8> PRES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV_BY_1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV_BY_2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV_BY_4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV_BY_8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV_BY_16)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV_BY_32)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn div_by_64(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV_BY_64)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn div_by_128(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV_BY_128)
    }
}
#[doc = "Field `TPOL` reader - Timer Polarity."]
pub type TPOL_R = crate::BitReader;
#[doc = "Field `TPOL` writer - Timer Polarity."]
pub type TPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEN` reader - Timer Enable."]
pub type TEN_R = crate::BitReader;
#[doc = "Field `TEN` writer - Timer Enable."]
pub type TEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRES3` reader - Timer Prescale Select MSB."]
pub type PRES3_R = crate::BitReader;
#[doc = "Field `PRES3` writer - Timer Prescale Select MSB."]
pub type PRES3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMSYNC` reader - PWM Synchronization Mode."]
pub type PWMSYNC_R = crate::BitReader;
#[doc = "Field `PWMSYNC` writer - PWM Synchronization Mode."]
pub type PWMSYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NOLHPOL` reader - PWM Phase A (Non-Overlapping High) Polarity."]
pub type NOLHPOL_R = crate::BitReader;
#[doc = "Field `NOLHPOL` writer - PWM Phase A (Non-Overlapping High) Polarity."]
pub type NOLHPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NOLLPOL` reader - PWM Phase A-prime (Non-Overlapping Low) Polarity."]
pub type NOLLPOL_R = crate::BitReader;
#[doc = "Field `NOLLPOL` writer - PWM Phase A-prime (Non-Overlapping Low) Polarity."]
pub type NOLLPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMCKBD` reader - PWM Phase A-Prime Output Disable."]
pub type PWMCKBD_R = crate::BitReader;
#[doc = "Field `PWMCKBD` writer - PWM Phase A-Prime Output Disable."]
pub type PWMCKBD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Mode Select."]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Timer Prescaler Select."]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Timer Polarity."]
    #[inline(always)]
    pub fn tpol(&self) -> TPOL_R {
        TPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Enable."]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer Prescale Select MSB."]
    #[inline(always)]
    pub fn pres3(&self) -> PRES3_R {
        PRES3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM Synchronization Mode."]
    #[inline(always)]
    pub fn pwmsync(&self) -> PWMSYNC_R {
        PWMSYNC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM Phase A (Non-Overlapping High) Polarity."]
    #[inline(always)]
    pub fn nolhpol(&self) -> NOLHPOL_R {
        NOLHPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM Phase A-prime (Non-Overlapping Low) Polarity."]
    #[inline(always)]
    pub fn nollpol(&self) -> NOLLPOL_R {
        NOLLPOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM Phase A-Prime Output Disable."]
    #[inline(always)]
    pub fn pwmckbd(&self) -> PWMCKBD_R {
        PWMCKBD_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select."]
    #[inline(always)]
    #[must_use]
    pub fn tmode(&mut self) -> TMODE_W<CN_SPEC, 0> {
        TMODE_W::new(self)
    }
    #[doc = "Bits 3:5 - Timer Prescaler Select."]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PRES_W<CN_SPEC, 3> {
        PRES_W::new(self)
    }
    #[doc = "Bit 6 - Timer Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn tpol(&mut self) -> TPOL_W<CN_SPEC, 6> {
        TPOL_W::new(self)
    }
    #[doc = "Bit 7 - Timer Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<CN_SPEC, 7> {
        TEN_W::new(self)
    }
    #[doc = "Bit 8 - Timer Prescale Select MSB."]
    #[inline(always)]
    #[must_use]
    pub fn pres3(&mut self) -> PRES3_W<CN_SPEC, 8> {
        PRES3_W::new(self)
    }
    #[doc = "Bit 9 - PWM Synchronization Mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwmsync(&mut self) -> PWMSYNC_W<CN_SPEC, 9> {
        PWMSYNC_W::new(self)
    }
    #[doc = "Bit 10 - PWM Phase A (Non-Overlapping High) Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn nolhpol(&mut self) -> NOLHPOL_W<CN_SPEC, 10> {
        NOLHPOL_W::new(self)
    }
    #[doc = "Bit 11 - PWM Phase A-prime (Non-Overlapping Low) Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn nollpol(&mut self) -> NOLLPOL_W<CN_SPEC, 11> {
        NOLLPOL_W::new(self)
    }
    #[doc = "Bit 12 - PWM Phase A-Prime Output Disable."]
    #[inline(always)]
    #[must_use]
    pub fn pwmckbd(&mut self) -> PWMCKBD_W<CN_SPEC, 12> {
        PWMCKBD_W::new(self)
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
#[doc = "Timer Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CN_SPEC;
impl crate::RegisterSpec for CN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cn::R`](R) reader structure"]
impl crate::Readable for CN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cn::W`](W) writer structure"]
impl crate::Writable for CN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CN to value 0"]
impl crate::Resettable for CN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
