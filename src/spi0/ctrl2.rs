#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CLK_PHA` reader - Clock Phase."]
pub type CLK_PHA_R = crate::BitReader<CLK_PHA_A>;
#[doc = "Clock Phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_PHA_A {
    #[doc = "0: Data Sampled on clock rising edge."]
    RISING_EDGE = 0,
    #[doc = "1: Data Sampled on clock falling edge."]
    FALLING_EDGE = 1,
}
impl From<CLK_PHA_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_PHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_PHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_PHA_A {
        match self.bits {
            false => CLK_PHA_A::RISING_EDGE,
            true => CLK_PHA_A::FALLING_EDGE,
        }
    }
    #[doc = "Data Sampled on clock rising edge."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CLK_PHA_A::RISING_EDGE
    }
    #[doc = "Data Sampled on clock falling edge."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CLK_PHA_A::FALLING_EDGE
    }
}
#[doc = "Field `CLK_PHA` writer - Clock Phase."]
pub type CLK_PHA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLK_PHA_A>;
impl<'a, REG, const O: u8> CLK_PHA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data Sampled on clock rising edge."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_PHA_A::RISING_EDGE)
    }
    #[doc = "Data Sampled on clock falling edge."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_PHA_A::FALLING_EDGE)
    }
}
#[doc = "Field `CLK_POL` reader - Clock Polarity."]
pub type CLK_POL_R = crate::BitReader<CLK_POL_A>;
#[doc = "Clock Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_POL_A {
    #[doc = "0: Normal Clock."]
    NORMAL = 0,
    #[doc = "1: Inverted Clock."]
    INVERTED = 1,
}
impl From<CLK_POL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_POL_A {
        match self.bits {
            false => CLK_POL_A::NORMAL,
            true => CLK_POL_A::INVERTED,
        }
    }
    #[doc = "Normal Clock."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CLK_POL_A::NORMAL
    }
    #[doc = "Inverted Clock."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CLK_POL_A::INVERTED
    }
}
#[doc = "Field `CLK_POL` writer - Clock Polarity."]
pub type CLK_POL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLK_POL_A>;
impl<'a, REG, const O: u8> CLK_POL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Clock."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_POL_A::NORMAL)
    }
    #[doc = "Inverted Clock."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_POL_A::INVERTED)
    }
}
#[doc = "Field `NUM_BITS` reader - Number of Bits per character."]
pub type NUM_BITS_R = crate::FieldReader<NUM_BITS_A>;
#[doc = "Number of Bits per character.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NUM_BITS_A {
    #[doc = "0: 16 bits per character."]
    _0 = 0,
}
impl From<NUM_BITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NUM_BITS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NUM_BITS_A {
    type Ux = u8;
}
impl NUM_BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NUM_BITS_A> {
        match self.bits {
            0 => Some(NUM_BITS_A::_0),
            _ => None,
        }
    }
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NUM_BITS_A::_0
    }
}
#[doc = "Field `NUM_BITS` writer - Number of Bits per character."]
pub type NUM_BITS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, NUM_BITS_A>;
impl<'a, REG, const O: u8> NUM_BITS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NUM_BITS_A::_0)
    }
}
#[doc = "Field `DATA_WIDTH` reader - SPI Data width."]
pub type DATA_WIDTH_R = crate::FieldReader<DATA_WIDTH_A>;
#[doc = "SPI Data width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATA_WIDTH_A {
    #[doc = "0: 1 data pin."]
    MONO = 0,
    #[doc = "1: 2 data pins."]
    DUAL = 1,
    #[doc = "2: 4 data pins."]
    QUAD = 2,
}
impl From<DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATA_WIDTH_A {
    type Ux = u8;
}
impl DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATA_WIDTH_A> {
        match self.bits {
            0 => Some(DATA_WIDTH_A::MONO),
            1 => Some(DATA_WIDTH_A::DUAL),
            2 => Some(DATA_WIDTH_A::QUAD),
            _ => None,
        }
    }
    #[doc = "1 data pin."]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == DATA_WIDTH_A::MONO
    }
    #[doc = "2 data pins."]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DATA_WIDTH_A::DUAL
    }
    #[doc = "4 data pins."]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == DATA_WIDTH_A::QUAD
    }
}
#[doc = "Field `DATA_WIDTH` writer - SPI Data width."]
pub type DATA_WIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DATA_WIDTH_A>;
impl<'a, REG, const O: u8> DATA_WIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data pin."]
    #[inline(always)]
    pub fn mono(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::MONO)
    }
    #[doc = "2 data pins."]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::DUAL)
    }
    #[doc = "4 data pins."]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::QUAD)
    }
}
#[doc = "Field `THREE_WIRE` reader - Three Wire mode."]
pub type THREE_WIRE_R = crate::BitReader<THREE_WIRE_A>;
#[doc = "Three Wire mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THREE_WIRE_A {
    #[doc = "0: Use four wire mode (Mono only)."]
    DISABLED = 0,
    #[doc = "1: Use three wire mode."]
    ENABLED = 1,
}
impl From<THREE_WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: THREE_WIRE_A) -> Self {
        variant as u8 != 0
    }
}
impl THREE_WIRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREE_WIRE_A {
        match self.bits {
            false => THREE_WIRE_A::DISABLED,
            true => THREE_WIRE_A::ENABLED,
        }
    }
    #[doc = "Use four wire mode (Mono only)."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == THREE_WIRE_A::DISABLED
    }
    #[doc = "Use three wire mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == THREE_WIRE_A::ENABLED
    }
}
#[doc = "Field `THREE_WIRE` writer - Three Wire mode."]
pub type THREE_WIRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, THREE_WIRE_A>;
impl<'a, REG, const O: u8> THREE_WIRE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use four wire mode (Mono only)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_WIRE_A::DISABLED)
    }
    #[doc = "Use three wire mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_WIRE_A::ENABLED)
    }
}
#[doc = "Field `SS_POL` reader - Slave Select Polarity, each Slave Select can have unique polarity."]
pub type SS_POL_R = crate::BitReader<SS_POL_A>;
#[doc = "Slave Select Polarity, each Slave Select can have unique polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SS_POL_A {
    #[doc = "0: Active low."]
    LOW = 0,
    #[doc = "1: Active high."]
    HIGH = 1,
}
impl From<SS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: SS_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl SS_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_POL_A {
        match self.bits {
            false => SS_POL_A::LOW,
            true => SS_POL_A::HIGH,
        }
    }
    #[doc = "Active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SS_POL_A::LOW
    }
    #[doc = "Active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SS_POL_A::HIGH
    }
}
#[doc = "Field `SS_POL` writer - Slave Select Polarity, each Slave Select can have unique polarity."]
pub type SS_POL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SS_POL_A>;
impl<'a, REG, const O: u8> SS_POL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SS_POL_A::LOW)
    }
    #[doc = "Active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SS_POL_A::HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    pub fn clk_pha(&self) -> CLK_PHA_R {
        CLK_PHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    pub fn clk_pol(&self) -> CLK_POL_R {
        CLK_POL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    pub fn num_bits(&self) -> NUM_BITS_R {
        NUM_BITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Three Wire mode."]
    #[inline(always)]
    pub fn three_wire(&self) -> THREE_WIRE_R {
        THREE_WIRE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    pub fn ss_pol(&self) -> SS_POL_R {
        SS_POL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    #[must_use]
    pub fn clk_pha(&mut self) -> CLK_PHA_W<CTRL2_SPEC, 0> {
        CLK_PHA_W::new(self)
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn clk_pol(&mut self) -> CLK_POL_W<CTRL2_SPEC, 1> {
        CLK_POL_W::new(self)
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    #[must_use]
    pub fn num_bits(&mut self) -> NUM_BITS_W<CTRL2_SPEC, 8> {
        NUM_BITS_W::new(self)
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DATA_WIDTH_W<CTRL2_SPEC, 12> {
        DATA_WIDTH_W::new(self)
    }
    #[doc = "Bit 15 - Three Wire mode."]
    #[inline(always)]
    #[must_use]
    pub fn three_wire(&mut self) -> THREE_WIRE_W<CTRL2_SPEC, 15> {
        THREE_WIRE_W::new(self)
    }
    #[doc = "Bit 16 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    #[must_use]
    pub fn ss_pol(&mut self) -> SS_POL_W<CTRL2_SPEC, 16> {
        SS_POL_W::new(self)
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
#[doc = "Control Register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
