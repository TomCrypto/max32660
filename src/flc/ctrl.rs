#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `WRITE` reader - Write."]
pub type WRITE_R = crate::BitReader<WRITE_A>;
#[doc = "Write.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRITE_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_A {
        match self.bits {
            false => WRITE_A::COMPLETE,
            true => WRITE_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == WRITE_A::COMPLETE
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WRITE_A::START
    }
}
#[doc = "Field `WRITE` writer - Write."]
pub type WRITE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WRITE_A>;
impl<'a, REG, const O: u8> WRITE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(WRITE_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(WRITE_A::START)
    }
}
#[doc = "Field `MASS_ERASE` reader - Mass Erase."]
pub type MASS_ERASE_R = crate::BitReader<MASS_ERASE_A>;
#[doc = "Mass Erase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASS_ERASE_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<MASS_ERASE_A> for bool {
    #[inline(always)]
    fn from(variant: MASS_ERASE_A) -> Self {
        variant as u8 != 0
    }
}
impl MASS_ERASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASS_ERASE_A {
        match self.bits {
            false => MASS_ERASE_A::COMPLETE,
            true => MASS_ERASE_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == MASS_ERASE_A::COMPLETE
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == MASS_ERASE_A::START
    }
}
#[doc = "Field `MASS_ERASE` writer - Mass Erase."]
pub type MASS_ERASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MASS_ERASE_A>;
impl<'a, REG, const O: u8> MASS_ERASE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(MASS_ERASE_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(MASS_ERASE_A::START)
    }
}
#[doc = "Field `PAGE_ERASE` reader - Page Erase."]
pub type PAGE_ERASE_R = crate::BitReader<PAGE_ERASE_A>;
#[doc = "Page Erase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAGE_ERASE_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<PAGE_ERASE_A> for bool {
    #[inline(always)]
    fn from(variant: PAGE_ERASE_A) -> Self {
        variant as u8 != 0
    }
}
impl PAGE_ERASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGE_ERASE_A {
        match self.bits {
            false => PAGE_ERASE_A::COMPLETE,
            true => PAGE_ERASE_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == PAGE_ERASE_A::COMPLETE
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == PAGE_ERASE_A::START
    }
}
#[doc = "Field `PAGE_ERASE` writer - Page Erase."]
pub type PAGE_ERASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PAGE_ERASE_A>;
impl<'a, REG, const O: u8> PAGE_ERASE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(PAGE_ERASE_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(PAGE_ERASE_A::START)
    }
}
#[doc = "Field `WIDTH` reader - Data Width."]
pub type WIDTH_R = crate::BitReader<WIDTH_A>;
#[doc = "Data Width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WIDTH_A {
    #[doc = "0: 128-bit."]
    SIZE128 = 0,
    #[doc = "1: 32-bit."]
    SIZE32 = 1,
}
impl From<WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: WIDTH_A) -> Self {
        variant as u8 != 0
    }
}
impl WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIDTH_A {
        match self.bits {
            false => WIDTH_A::SIZE128,
            true => WIDTH_A::SIZE32,
        }
    }
    #[doc = "128-bit."]
    #[inline(always)]
    pub fn is_size128(&self) -> bool {
        *self == WIDTH_A::SIZE128
    }
    #[doc = "32-bit."]
    #[inline(always)]
    pub fn is_size32(&self) -> bool {
        *self == WIDTH_A::SIZE32
    }
}
#[doc = "Field `WIDTH` writer - Data Width."]
pub type WIDTH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WIDTH_A>;
impl<'a, REG, const O: u8> WIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "128-bit."]
    #[inline(always)]
    pub fn size128(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTH_A::SIZE128)
    }
    #[doc = "32-bit."]
    #[inline(always)]
    pub fn size32(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTH_A::SIZE32)
    }
}
#[doc = "Field `ERASE_CODE` reader - Erase Code."]
pub type ERASE_CODE_R = crate::FieldReader<ERASE_CODE_A>;
#[doc = "Erase Code.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERASE_CODE_A {
    #[doc = "0: No operation."]
    NOP = 0,
    #[doc = "85: Enable page erase."]
    ERASE_PAGE = 85,
    #[doc = "170: Enable mass erase."]
    ERASE_ALL = 170,
}
impl From<ERASE_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERASE_CODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ERASE_CODE_A {
    type Ux = u8;
}
impl ERASE_CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERASE_CODE_A> {
        match self.bits {
            0 => Some(ERASE_CODE_A::NOP),
            85 => Some(ERASE_CODE_A::ERASE_PAGE),
            170 => Some(ERASE_CODE_A::ERASE_ALL),
            _ => None,
        }
    }
    #[doc = "No operation."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ERASE_CODE_A::NOP
    }
    #[doc = "Enable page erase."]
    #[inline(always)]
    pub fn is_erase_page(&self) -> bool {
        *self == ERASE_CODE_A::ERASE_PAGE
    }
    #[doc = "Enable mass erase."]
    #[inline(always)]
    pub fn is_erase_all(&self) -> bool {
        *self == ERASE_CODE_A::ERASE_ALL
    }
}
#[doc = "Field `ERASE_CODE` writer - Erase Code."]
pub type ERASE_CODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, ERASE_CODE_A>;
impl<'a, REG, const O: u8> ERASE_CODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No operation."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(ERASE_CODE_A::NOP)
    }
    #[doc = "Enable page erase."]
    #[inline(always)]
    pub fn erase_page(self) -> &'a mut crate::W<REG> {
        self.variant(ERASE_CODE_A::ERASE_PAGE)
    }
    #[doc = "Enable mass erase."]
    #[inline(always)]
    pub fn erase_all(self) -> &'a mut crate::W<REG> {
        self.variant(ERASE_CODE_A::ERASE_ALL)
    }
}
#[doc = "Field `BUSY` reader - Flash Pending."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Flash Pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Idle."]
    IDLE = 0,
    #[doc = "1: Busy."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `LVE` reader - Low Voltage enable."]
pub type LVE_R = crate::BitReader;
#[doc = "Field `LVE` writer - Low Voltage enable."]
pub type LVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNLOCK_CODE` reader - Flash Unlock."]
pub type UNLOCK_CODE_R = crate::FieldReader<UNLOCK_CODE_A>;
#[doc = "Flash Unlock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UNLOCK_CODE_A {
    #[doc = "2: Flash Unlocked."]
    UNLOCKED = 2,
    #[doc = "3: Flash Locked."]
    LOCKED = 3,
}
impl From<UNLOCK_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UNLOCK_CODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UNLOCK_CODE_A {
    type Ux = u8;
}
impl UNLOCK_CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UNLOCK_CODE_A> {
        match self.bits {
            2 => Some(UNLOCK_CODE_A::UNLOCKED),
            3 => Some(UNLOCK_CODE_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Flash Unlocked."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == UNLOCK_CODE_A::UNLOCKED
    }
    #[doc = "Flash Locked."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == UNLOCK_CODE_A::LOCKED
    }
}
#[doc = "Field `UNLOCK_CODE` writer - Flash Unlock."]
pub type UNLOCK_CODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, UNLOCK_CODE_A>;
impl<'a, REG, const O: u8> UNLOCK_CODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash Unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(UNLOCK_CODE_A::UNLOCKED)
    }
    #[doc = "Flash Locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(UNLOCK_CODE_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - Write."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mass Erase."]
    #[inline(always)]
    pub fn mass_erase(&self) -> MASS_ERASE_R {
        MASS_ERASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Page Erase."]
    #[inline(always)]
    pub fn page_erase(&self) -> PAGE_ERASE_R {
        PAGE_ERASE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Width."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Erase Code."]
    #[inline(always)]
    pub fn erase_code(&self) -> ERASE_CODE_R {
        ERASE_CODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Flash Pending."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low Voltage enable."]
    #[inline(always)]
    pub fn lve(&self) -> LVE_R {
        LVE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Flash Unlock."]
    #[inline(always)]
    pub fn unlock_code(&self) -> UNLOCK_CODE_R {
        UNLOCK_CODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write."]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<CTRL_SPEC, 0> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 1 - Mass Erase."]
    #[inline(always)]
    #[must_use]
    pub fn mass_erase(&mut self) -> MASS_ERASE_W<CTRL_SPEC, 1> {
        MASS_ERASE_W::new(self)
    }
    #[doc = "Bit 2 - Page Erase."]
    #[inline(always)]
    #[must_use]
    pub fn page_erase(&mut self) -> PAGE_ERASE_W<CTRL_SPEC, 2> {
        PAGE_ERASE_W::new(self)
    }
    #[doc = "Bit 4 - Data Width."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<CTRL_SPEC, 4> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 8:15 - Erase Code."]
    #[inline(always)]
    #[must_use]
    pub fn erase_code(&mut self) -> ERASE_CODE_W<CTRL_SPEC, 8> {
        ERASE_CODE_W::new(self)
    }
    #[doc = "Bit 25 - Low Voltage enable."]
    #[inline(always)]
    #[must_use]
    pub fn lve(&mut self) -> LVE_W<CTRL_SPEC, 25> {
        LVE_W::new(self)
    }
    #[doc = "Bits 28:31 - Flash Unlock."]
    #[inline(always)]
    #[must_use]
    pub fn unlock_code(&mut self) -> UNLOCK_CODE_W<CTRL_SPEC, 28> {
        UNLOCK_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
