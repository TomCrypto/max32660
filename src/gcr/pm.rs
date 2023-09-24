#[doc = "Register `PM` reader"]
pub type R = crate::R<PM_SPEC>;
#[doc = "Register `PM` writer"]
pub type W = crate::W<PM_SPEC>;
#[doc = "Field `MODE` reader - Operating Mode."]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Operating Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Active Mode."]
    ACTIVE = 0,
    #[doc = "3: Shutdown Mode."]
    SHUTDOWN = 3,
    #[doc = "4: Backup Mode."]
    BACKUP = 4,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::ACTIVE),
            3 => Some(MODE_A::SHUTDOWN),
            4 => Some(MODE_A::BACKUP),
            _ => None,
        }
    }
    #[doc = "Active Mode."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MODE_A::ACTIVE
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == MODE_A::SHUTDOWN
    }
    #[doc = "Backup Mode."]
    #[inline(always)]
    pub fn is_backup(&self) -> bool {
        *self == MODE_A::BACKUP
    }
}
#[doc = "Field `MODE` writer - Operating Mode."]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, MODE_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Active Mode."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ACTIVE)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SHUTDOWN)
    }
    #[doc = "Backup Mode."]
    #[inline(always)]
    pub fn backup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::BACKUP)
    }
}
#[doc = "Field `GPIOWK_EN` reader - GPIO Wake Up Enable."]
pub type GPIOWK_EN_R = crate::BitReader<GPIOWK_EN_A>;
#[doc = "GPIO Wake Up Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOWK_EN_A {
    #[doc = "0: Wake Up Disable."]
    DISABLED = 0,
    #[doc = "1: Wake Up Enable."]
    ENABLED = 1,
}
impl From<GPIOWK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOWK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOWK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOWK_EN_A {
        match self.bits {
            false => GPIOWK_EN_A::DISABLED,
            true => GPIOWK_EN_A::ENABLED,
        }
    }
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOWK_EN_A::DISABLED
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOWK_EN_A::ENABLED
    }
}
#[doc = "Field `GPIOWK_EN` writer - GPIO Wake Up Enable."]
pub type GPIOWK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GPIOWK_EN_A>;
impl<'a, REG, const O: u8> GPIOWK_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOWK_EN_A::DISABLED)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOWK_EN_A::ENABLED)
    }
}
#[doc = "Field `RTCWK_EN` reader - RTC Alarm Wake Up Enable."]
pub type RTCWK_EN_R = crate::BitReader<RTCWK_EN_A>;
#[doc = "RTC Alarm Wake Up Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCWK_EN_A {
    #[doc = "0: Wake Up Disable."]
    DISABLED = 0,
    #[doc = "1: Wake Up Enable."]
    ENABLED = 1,
}
impl From<RTCWK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCWK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCWK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCWK_EN_A {
        match self.bits {
            false => RTCWK_EN_A::DISABLED,
            true => RTCWK_EN_A::ENABLED,
        }
    }
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCWK_EN_A::DISABLED
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCWK_EN_A::ENABLED
    }
}
#[doc = "Field `RTCWK_EN` writer - RTC Alarm Wake Up Enable."]
pub type RTCWK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTCWK_EN_A>;
impl<'a, REG, const O: u8> RTCWK_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCWK_EN_A::DISABLED)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCWK_EN_A::ENABLED)
    }
}
#[doc = "Field `HFIOPD` reader - HFIO DEEPSLEEP Auto Off."]
pub type HFIOPD_R = crate::BitReader<HFIOPD_A>;
#[doc = "HFIO DEEPSLEEP Auto Off.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFIOPD_A {
    #[doc = "0: Mode is Active."]
    ACTIVE = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    DEEPSLEEP = 1,
}
impl From<HFIOPD_A> for bool {
    #[inline(always)]
    fn from(variant: HFIOPD_A) -> Self {
        variant as u8 != 0
    }
}
impl HFIOPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFIOPD_A {
        match self.bits {
            false => HFIOPD_A::ACTIVE,
            true => HFIOPD_A::DEEPSLEEP,
        }
    }
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HFIOPD_A::ACTIVE
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == HFIOPD_A::DEEPSLEEP
    }
}
#[doc = "Field `HFIOPD` writer - HFIO DEEPSLEEP Auto Off."]
pub type HFIOPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HFIOPD_A>;
impl<'a, REG, const O: u8> HFIOPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(HFIOPD_A::ACTIVE)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut crate::W<REG> {
        self.variant(HFIOPD_A::DEEPSLEEP)
    }
}
impl R {
    #[doc = "Bits 0:2 - Operating Mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable."]
    #[inline(always)]
    pub fn gpiowk_en(&self) -> GPIOWK_EN_R {
        GPIOWK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable."]
    #[inline(always)]
    pub fn rtcwk_en(&self) -> RTCWK_EN_R {
        RTCWK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - HFIO DEEPSLEEP Auto Off."]
    #[inline(always)]
    pub fn hfiopd(&self) -> HFIOPD_R {
        HFIOPD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Operating Mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<PM_SPEC, 0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn gpiowk_en(&mut self) -> GPIOWK_EN_W<PM_SPEC, 4> {
        GPIOWK_EN_W::new(self)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rtcwk_en(&mut self) -> RTCWK_EN_W<PM_SPEC, 5> {
        RTCWK_EN_W::new(self)
    }
    #[doc = "Bit 15 - HFIO DEEPSLEEP Auto Off."]
    #[inline(always)]
    #[must_use]
    pub fn hfiopd(&mut self) -> HFIOPD_W<PM_SPEC, 15> {
        HFIOPD_W::new(self)
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
#[doc = "Power Management.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PM_SPEC;
impl crate::RegisterSpec for PM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pm::R`](R) reader structure"]
impl crate::Readable for PM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pm::W`](W) writer structure"]
impl crate::Writable for PM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PM to value 0"]
impl crate::Resettable for PM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
