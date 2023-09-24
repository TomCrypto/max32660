#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<CTRL0_SPEC>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<CTRL0_SPEC>;
#[doc = "Field `SPI_EN` reader - SPI Enable."]
pub type SPI_EN_R = crate::BitReader<SPI_EN_A>;
#[doc = "SPI Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI_EN_A {
    #[doc = "0: SPI is disabled."]
    DISABLED = 0,
    #[doc = "1: SPI is enabled."]
    ENABLED = 1,
}
impl From<SPI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI_EN_A {
        match self.bits {
            false => SPI_EN_A::DISABLED,
            true => SPI_EN_A::ENABLED,
        }
    }
    #[doc = "SPI is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI_EN_A::DISABLED
    }
    #[doc = "SPI is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI_EN_A::ENABLED
    }
}
#[doc = "Field `SPI_EN` writer - SPI Enable."]
pub type SPI_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPI_EN_A>;
impl<'a, REG, const O: u8> SPI_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_EN_A::DISABLED)
    }
    #[doc = "SPI is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_EN_A::ENABLED)
    }
}
#[doc = "Field `MM_EN` reader - Master Mode Enable."]
pub type MM_EN_R = crate::BitReader<MM_EN_A>;
#[doc = "Master Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MM_EN_A {
    #[doc = "0: SPI is Slave mode."]
    DISABLED = 0,
    #[doc = "1: SPI is Master mode."]
    ENABLED = 1,
}
impl From<MM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MM_EN_A {
        match self.bits {
            false => MM_EN_A::DISABLED,
            true => MM_EN_A::ENABLED,
        }
    }
    #[doc = "SPI is Slave mode."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MM_EN_A::DISABLED
    }
    #[doc = "SPI is Master mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MM_EN_A::ENABLED
    }
}
#[doc = "Field `MM_EN` writer - Master Mode Enable."]
pub type MM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MM_EN_A>;
impl<'a, REG, const O: u8> MM_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI is Slave mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MM_EN_A::DISABLED)
    }
    #[doc = "SPI is Master mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MM_EN_A::ENABLED)
    }
}
#[doc = "Field `SS_IO` reader - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode."]
pub type SS_IO_R = crate::BitReader<SS_IO_A>;
#[doc = "Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SS_IO_A {
    #[doc = "0: Slave select 0 is output."]
    OUTPUT = 0,
    #[doc = "1: Slave Select 0 is input, only valid if MMEN=1."]
    INPUT = 1,
}
impl From<SS_IO_A> for bool {
    #[inline(always)]
    fn from(variant: SS_IO_A) -> Self {
        variant as u8 != 0
    }
}
impl SS_IO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_IO_A {
        match self.bits {
            false => SS_IO_A::OUTPUT,
            true => SS_IO_A::INPUT,
        }
    }
    #[doc = "Slave select 0 is output."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == SS_IO_A::OUTPUT
    }
    #[doc = "Slave Select 0 is input, only valid if MMEN=1."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == SS_IO_A::INPUT
    }
}
#[doc = "Field `SS_IO` writer - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode."]
pub type SS_IO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SS_IO_A>;
impl<'a, REG, const O: u8> SS_IO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave select 0 is output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(SS_IO_A::OUTPUT)
    }
    #[doc = "Slave Select 0 is input, only valid if MMEN=1."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(SS_IO_A::INPUT)
    }
}
#[doc = "Field `START` reader - Start Transmit."]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Start Transmit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "1: Master Initiates a transaction, this bit is self clearing when transactions are done."]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            true => Some(START_A::START),
            _ => None,
        }
    }
    #[doc = "Master Initiates a transaction, this bit is self clearing when transactions are done."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Field `START` writer - Start Transmit."]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, START_A>;
impl<'a, REG, const O: u8> START_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master Initiates a transaction, this bit is self clearing when transactions are done."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::START)
    }
}
#[doc = "Field `SS_CTRL` reader - Start Select Control."]
pub type SS_CTRL_R = crate::BitReader<SS_CTRL_A>;
#[doc = "Start Select Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SS_CTRL_A {
    #[doc = "0: SPI De-asserts Slave Select at the end of a transaction."]
    DEASSERT = 0,
    #[doc = "1: SPI leaves Slave Select asserted at the end of a transaction."]
    ASSERT = 1,
}
impl From<SS_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: SS_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl SS_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_CTRL_A {
        match self.bits {
            false => SS_CTRL_A::DEASSERT,
            true => SS_CTRL_A::ASSERT,
        }
    }
    #[doc = "SPI De-asserts Slave Select at the end of a transaction."]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == SS_CTRL_A::DEASSERT
    }
    #[doc = "SPI leaves Slave Select asserted at the end of a transaction."]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SS_CTRL_A::ASSERT
    }
}
#[doc = "Field `SS_CTRL` writer - Start Select Control."]
pub type SS_CTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SS_CTRL_A>;
impl<'a, REG, const O: u8> SS_CTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI De-asserts Slave Select at the end of a transaction."]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(SS_CTRL_A::DEASSERT)
    }
    #[doc = "SPI leaves Slave Select asserted at the end of a transaction."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(SS_CTRL_A::ASSERT)
    }
}
#[doc = "Field `SS_SEL` reader - Slave Select, when in Master mode selects which Slave devices are selected."]
pub type SS_SEL_R = crate::FieldReader<SS_SEL_A>;
#[doc = "Slave Select, when in Master mode selects which Slave devices are selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SS_SEL_A {
    #[doc = "1: SS0 is selected."]
    SS0 = 1,
    #[doc = "2: SS1 is selected."]
    SS1 = 2,
    #[doc = "4: SS2 is selected."]
    SS2 = 4,
    #[doc = "8: SS3 is selected."]
    SS3 = 8,
}
impl From<SS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SS_SEL_A {
    type Ux = u8;
}
impl SS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SS_SEL_A> {
        match self.bits {
            1 => Some(SS_SEL_A::SS0),
            2 => Some(SS_SEL_A::SS1),
            4 => Some(SS_SEL_A::SS2),
            8 => Some(SS_SEL_A::SS3),
            _ => None,
        }
    }
    #[doc = "SS0 is selected."]
    #[inline(always)]
    pub fn is_ss0(&self) -> bool {
        *self == SS_SEL_A::SS0
    }
    #[doc = "SS1 is selected."]
    #[inline(always)]
    pub fn is_ss1(&self) -> bool {
        *self == SS_SEL_A::SS1
    }
    #[doc = "SS2 is selected."]
    #[inline(always)]
    pub fn is_ss2(&self) -> bool {
        *self == SS_SEL_A::SS2
    }
    #[doc = "SS3 is selected."]
    #[inline(always)]
    pub fn is_ss3(&self) -> bool {
        *self == SS_SEL_A::SS3
    }
}
#[doc = "Field `SS_SEL` writer - Slave Select, when in Master mode selects which Slave devices are selected."]
pub type SS_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, SS_SEL_A>;
impl<'a, REG, const O: u8> SS_SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SS0 is selected."]
    #[inline(always)]
    pub fn ss0(self) -> &'a mut crate::W<REG> {
        self.variant(SS_SEL_A::SS0)
    }
    #[doc = "SS1 is selected."]
    #[inline(always)]
    pub fn ss1(self) -> &'a mut crate::W<REG> {
        self.variant(SS_SEL_A::SS1)
    }
    #[doc = "SS2 is selected."]
    #[inline(always)]
    pub fn ss2(self) -> &'a mut crate::W<REG> {
        self.variant(SS_SEL_A::SS2)
    }
    #[doc = "SS3 is selected."]
    #[inline(always)]
    pub fn ss3(self) -> &'a mut crate::W<REG> {
        self.variant(SS_SEL_A::SS3)
    }
}
impl R {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn spi_en(&self) -> SPI_EN_R {
        SPI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mm_en(&self) -> MM_EN_R {
        MM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode."]
    #[inline(always)]
    pub fn ss_io(&self) -> SS_IO_R {
        SS_IO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start Transmit."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Start Select Control."]
    #[inline(always)]
    pub fn ss_ctrl(&self) -> SS_CTRL_R {
        SS_CTRL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Slave Select, when in Master mode selects which Slave devices are selected."]
    #[inline(always)]
    pub fn ss_sel(&self) -> SS_SEL_R {
        SS_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_en(&mut self) -> SPI_EN_W<CTRL0_SPEC, 0> {
        SPI_EN_W::new(self)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mm_en(&mut self) -> MM_EN_W<CTRL0_SPEC, 1> {
        MM_EN_W::new(self)
    }
    #[doc = "Bit 4 - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode."]
    #[inline(always)]
    #[must_use]
    pub fn ss_io(&mut self) -> SS_IO_W<CTRL0_SPEC, 4> {
        SS_IO_W::new(self)
    }
    #[doc = "Bit 5 - Start Transmit."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTRL0_SPEC, 5> {
        START_W::new(self)
    }
    #[doc = "Bit 8 - Start Select Control."]
    #[inline(always)]
    #[must_use]
    pub fn ss_ctrl(&mut self) -> SS_CTRL_W<CTRL0_SPEC, 8> {
        SS_CTRL_W::new(self)
    }
    #[doc = "Bits 16:19 - Slave Select, when in Master mode selects which Slave devices are selected."]
    #[inline(always)]
    #[must_use]
    pub fn ss_sel(&mut self) -> SS_SEL_W<CTRL0_SPEC, 16> {
        SS_SEL_W::new(self)
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
#[doc = "Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
