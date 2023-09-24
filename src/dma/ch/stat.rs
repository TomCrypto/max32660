#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `CH_ST` reader - Channel Status."]
pub type CH_ST_R = crate::BitReader<CH_ST_A>;
#[doc = "Channel Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_ST_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<CH_ST_A> for bool {
    #[inline(always)]
    fn from(variant: CH_ST_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_ST_A {
        match self.bits {
            false => CH_ST_A::DISABLED,
            true => CH_ST_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH_ST_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH_ST_A::ENABLED
    }
}
#[doc = "Field `IPEND` reader - Channel Interrupt."]
pub type IPEND_R = crate::BitReader<IPEND_A>;
#[doc = "Channel Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEND_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IPEND_A> for bool {
    #[inline(always)]
    fn from(variant: IPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEND_A {
        match self.bits {
            false => IPEND_A::INACTIVE,
            true => IPEND_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IPEND_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IPEND_A::PENDING
    }
}
#[doc = "Field `CTZ_ST` reader - Count-to-Zero (CTZ) Status."]
pub type CTZ_ST_R = crate::BitReader<CTZ_ST_A>;
#[doc = "Count-to-Zero (CTZ) Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTZ_ST_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<CTZ_ST_A> for bool {
    #[inline(always)]
    fn from(variant: CTZ_ST_A) -> Self {
        variant as u8 != 0
    }
}
impl CTZ_ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTZ_ST_A {
        match self.bits {
            false => CTZ_ST_A::NO_EVENT,
            true => CTZ_ST_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CTZ_ST_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == CTZ_ST_A::OCCURRED
    }
}
#[doc = "Count-to-Zero (CTZ) Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTZ_ST_AW {
    #[doc = "1: Clear the interrupt flag."]
    CLEAR = 1,
}
impl From<CTZ_ST_AW> for bool {
    #[inline(always)]
    fn from(variant: CTZ_ST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTZ_ST` writer - Count-to-Zero (CTZ) Status."]
pub type CTZ_ST_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, CTZ_ST_AW>;
impl<'a, REG, const O: u8> CTZ_ST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the interrupt flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTZ_ST_AW::CLEAR)
    }
}
#[doc = "Field `RLD_ST` reader - Reload Status."]
pub type RLD_ST_R = crate::BitReader<RLD_ST_A>;
#[doc = "Reload Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLD_ST_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<RLD_ST_A> for bool {
    #[inline(always)]
    fn from(variant: RLD_ST_A) -> Self {
        variant as u8 != 0
    }
}
impl RLD_ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLD_ST_A {
        match self.bits {
            false => RLD_ST_A::NO_EVENT,
            true => RLD_ST_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RLD_ST_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == RLD_ST_A::OCCURRED
    }
}
#[doc = "Reload Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLD_ST_AW {
    #[doc = "1: Clear the interrupt flag."]
    CLEAR = 1,
}
impl From<RLD_ST_AW> for bool {
    #[inline(always)]
    fn from(variant: RLD_ST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLD_ST` writer - Reload Status."]
pub type RLD_ST_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, RLD_ST_AW>;
impl<'a, REG, const O: u8> RLD_ST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the interrupt flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RLD_ST_AW::CLEAR)
    }
}
#[doc = "Field `BUS_ERR` reader - Bus Error."]
pub type BUS_ERR_R = crate::BitReader<BUS_ERR_A>;
#[doc = "Bus Error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUS_ERR_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<BUS_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BUS_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_ERR_A {
        match self.bits {
            false => BUS_ERR_A::NO_EVENT,
            true => BUS_ERR_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == BUS_ERR_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == BUS_ERR_A::OCCURRED
    }
}
#[doc = "Bus Error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUS_ERR_AW {
    #[doc = "1: Clear the interrupt flag."]
    CLEAR = 1,
}
impl From<BUS_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: BUS_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_ERR` writer - Bus Error."]
pub type BUS_ERR_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, BUS_ERR_AW>;
impl<'a, REG, const O: u8> BUS_ERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the interrupt flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BUS_ERR_AW::CLEAR)
    }
}
#[doc = "Field `TO_ST` reader - Time-Out Status."]
pub type TO_ST_R = crate::BitReader<TO_ST_A>;
#[doc = "Time-Out Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TO_ST_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<TO_ST_A> for bool {
    #[inline(always)]
    fn from(variant: TO_ST_A) -> Self {
        variant as u8 != 0
    }
}
impl TO_ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_ST_A {
        match self.bits {
            false => TO_ST_A::NO_EVENT,
            true => TO_ST_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TO_ST_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == TO_ST_A::OCCURRED
    }
}
#[doc = "Time-Out Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TO_ST_AW {
    #[doc = "1: Clear the interrupt flag."]
    CLEAR = 1,
}
impl From<TO_ST_AW> for bool {
    #[inline(always)]
    fn from(variant: TO_ST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TO_ST` writer - Time-Out Status."]
pub type TO_ST_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, TO_ST_AW>;
impl<'a, REG, const O: u8> TO_ST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the interrupt flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TO_ST_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Status."]
    #[inline(always)]
    pub fn ch_st(&self) -> CH_ST_R {
        CH_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Interrupt."]
    #[inline(always)]
    pub fn ipend(&self) -> IPEND_R {
        IPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Count-to-Zero (CTZ) Status."]
    #[inline(always)]
    pub fn ctz_st(&self) -> CTZ_ST_R {
        CTZ_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reload Status."]
    #[inline(always)]
    pub fn rld_st(&self) -> RLD_ST_R {
        RLD_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Error."]
    #[inline(always)]
    pub fn bus_err(&self) -> BUS_ERR_R {
        BUS_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Time-Out Status."]
    #[inline(always)]
    pub fn to_st(&self) -> TO_ST_R {
        TO_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Count-to-Zero (CTZ) Status."]
    #[inline(always)]
    #[must_use]
    pub fn ctz_st(&mut self) -> CTZ_ST_W<STAT_SPEC, 2> {
        CTZ_ST_W::new(self)
    }
    #[doc = "Bit 3 - Reload Status."]
    #[inline(always)]
    #[must_use]
    pub fn rld_st(&mut self) -> RLD_ST_W<STAT_SPEC, 3> {
        RLD_ST_W::new(self)
    }
    #[doc = "Bit 4 - Bus Error."]
    #[inline(always)]
    #[must_use]
    pub fn bus_err(&mut self) -> BUS_ERR_W<STAT_SPEC, 4> {
        BUS_ERR_W::new(self)
    }
    #[doc = "Bit 6 - Time-Out Status."]
    #[inline(always)]
    #[must_use]
    pub fn to_st(&mut self) -> TO_ST_W<STAT_SPEC, 6> {
        TO_ST_W::new(self)
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
#[doc = "DMA Channel Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x5c;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
