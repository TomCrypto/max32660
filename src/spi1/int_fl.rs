#[doc = "Register `INT_FL` reader"]
pub type R = crate::R<INT_FL_SPEC>;
#[doc = "Register `INT_FL` writer"]
pub type W = crate::W<INT_FL_SPEC>;
#[doc = "Field `SLAS` reader - Slave Select."]
pub type SLAS_R = crate::BitReader<SLAS_A>;
#[doc = "Slave Select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAS_A {
    #[doc = "0: `0`"]
    SELECTED = 0,
    #[doc = "1: `1`"]
    NOT_SELECTED = 1,
}
impl From<SLAS_A> for bool {
    #[inline(always)]
    fn from(variant: SLAS_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAS_A {
        match self.bits {
            false => SLAS_A::SELECTED,
            true => SLAS_A::NOT_SELECTED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == SLAS_A::SELECTED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == SLAS_A::NOT_SELECTED
    }
}
#[doc = "Field `TXST` reader - Transmit Status."]
pub type TXST_R = crate::BitReader<TXST_A>;
#[doc = "Transmit Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXST_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<TXST_A> for bool {
    #[inline(always)]
    fn from(variant: TXST_A) -> Self {
        variant as u8 != 0
    }
}
impl TXST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXST_A {
        match self.bits {
            false => TXST_A::IDLE,
            true => TXST_A::BUSY,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TXST_A::IDLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TXST_A::BUSY
    }
}
#[doc = "Field `TUND` reader - Transmit Underrun."]
pub type TUND_R = crate::BitReader<TUND_A>;
#[doc = "Transmit Underrun.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUND_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<TUND_A> for bool {
    #[inline(always)]
    fn from(variant: TUND_A) -> Self {
        variant as u8 != 0
    }
}
impl TUND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TUND_A {
        match self.bits {
            false => TUND_A::NO_EVENT,
            true => TUND_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TUND_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == TUND_A::OCCURRED
    }
}
#[doc = "Field `TUND` writer - Transmit Underrun."]
pub type TUND_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, TUND_A>;
impl<'a, REG, const O: u8> TUND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(TUND_A::NO_EVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(TUND_A::OCCURRED)
    }
}
#[doc = "Field `ROVR` reader - Receive Overrun."]
pub type ROVR_R = crate::BitReader<ROVR_A>;
#[doc = "Receive Overrun.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVR_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<ROVR_A> for bool {
    #[inline(always)]
    fn from(variant: ROVR_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVR_A {
        match self.bits {
            false => ROVR_A::NO_EVENT,
            true => ROVR_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ROVR_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == ROVR_A::OCCURRED
    }
}
#[doc = "Field `ROVR` writer - Receive Overrun."]
pub type ROVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ROVR_A>;
impl<'a, REG, const O: u8> ROVR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(ROVR_A::NO_EVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(ROVR_A::OCCURRED)
    }
}
#[doc = "Field `ABT` reader - Slave Mode Transaction Abort."]
pub type ABT_R = crate::BitReader<ABT_A>;
#[doc = "Slave Mode Transaction Abort.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABT_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<ABT_A> for bool {
    #[inline(always)]
    fn from(variant: ABT_A) -> Self {
        variant as u8 != 0
    }
}
impl ABT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABT_A {
        match self.bits {
            false => ABT_A::NO_EVENT,
            true => ABT_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ABT_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == ABT_A::OCCURRED
    }
}
#[doc = "Field `ABT` writer - Slave Mode Transaction Abort."]
pub type ABT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABT_A>;
impl<'a, REG, const O: u8> ABT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(ABT_A::NO_EVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(ABT_A::OCCURRED)
    }
}
#[doc = "Field `COL` reader - Collision."]
pub type COL_R = crate::BitReader<COL_A>;
#[doc = "Collision.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COL_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<COL_A> for bool {
    #[inline(always)]
    fn from(variant: COL_A) -> Self {
        variant as u8 != 0
    }
}
impl COL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COL_A {
        match self.bits {
            false => COL_A::NO_EVENT,
            true => COL_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == COL_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == COL_A::OCCURRED
    }
}
#[doc = "Field `COL` writer - Collision."]
pub type COL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, COL_A>;
impl<'a, REG, const O: u8> COL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(COL_A::NO_EVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(COL_A::OCCURRED)
    }
}
#[doc = "Field `TOVR` reader - Transmit Overrun."]
pub type TOVR_R = crate::BitReader<TOVR_A>;
#[doc = "Transmit Overrun.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOVR_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<TOVR_A> for bool {
    #[inline(always)]
    fn from(variant: TOVR_A) -> Self {
        variant as u8 != 0
    }
}
impl TOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOVR_A {
        match self.bits {
            false => TOVR_A::NO_EVENT,
            true => TOVR_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TOVR_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == TOVR_A::OCCURRED
    }
}
#[doc = "Field `TOVR` writer - Transmit Overrun."]
pub type TOVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TOVR_A>;
impl<'a, REG, const O: u8> TOVR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(TOVR_A::NO_EVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(TOVR_A::OCCURRED)
    }
}
#[doc = "Field `IRQ` reader - SPI Interrupt Request."]
pub type IRQ_R = crate::BitReader<IRQ_A>;
#[doc = "SPI Interrupt Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQ_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ_A {
        match self.bits {
            false => IRQ_A::INACTIVE,
            true => IRQ_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IRQ_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IRQ_A::PENDING
    }
}
#[doc = "Field `IRQ` writer - SPI Interrupt Request."]
pub type IRQ_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, IRQ_A>;
impl<'a, REG, const O: u8> IRQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IRQ_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(IRQ_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Select."]
    #[inline(always)]
    pub fn slas(&self) -> SLAS_R {
        SLAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Status."]
    #[inline(always)]
    pub fn txst(&self) -> TXST_R {
        TXST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Underrun."]
    #[inline(always)]
    pub fn tund(&self) -> TUND_R {
        TUND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Overrun."]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Mode Transaction Abort."]
    #[inline(always)]
    pub fn abt(&self) -> ABT_R {
        ABT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Collision."]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Overrun."]
    #[inline(always)]
    pub fn tovr(&self) -> TOVR_R {
        TOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Request."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Transmit Underrun."]
    #[inline(always)]
    #[must_use]
    pub fn tund(&mut self) -> TUND_W<INT_FL_SPEC, 2> {
        TUND_W::new(self)
    }
    #[doc = "Bit 3 - Receive Overrun."]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> ROVR_W<INT_FL_SPEC, 3> {
        ROVR_W::new(self)
    }
    #[doc = "Bit 4 - Slave Mode Transaction Abort."]
    #[inline(always)]
    #[must_use]
    pub fn abt(&mut self) -> ABT_W<INT_FL_SPEC, 4> {
        ABT_W::new(self)
    }
    #[doc = "Bit 5 - Collision."]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<INT_FL_SPEC, 5> {
        COL_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Overrun."]
    #[inline(always)]
    #[must_use]
    pub fn tovr(&mut self) -> TOVR_W<INT_FL_SPEC, 6> {
        TOVR_W::new(self)
    }
    #[doc = "Bit 7 - SPI Interrupt Request."]
    #[inline(always)]
    #[must_use]
    pub fn irq(&mut self) -> IRQ_W<INT_FL_SPEC, 7> {
        IRQ_W::new(self)
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
#[doc = "SPI Interrupt Flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_fl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FL_SPEC;
impl crate::RegisterSpec for INT_FL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_fl::R`](R) reader structure"]
impl crate::Readable for INT_FL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_fl::W`](W) writer structure"]
impl crate::Writable for INT_FL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x84;
}
#[doc = "`reset()` method sets INT_FL to value 0x01"]
impl crate::Resettable for INT_FL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
