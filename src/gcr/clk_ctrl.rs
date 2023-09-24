#[doc = "Register `CLK_CTRL` reader"]
pub type R = crate::R<CLK_CTRL_SPEC>;
#[doc = "Register `CLK_CTRL` writer"]
pub type W = crate::W<CLK_CTRL_SPEC>;
#[doc = "Field `PSC` reader - Prescaler Select."]
pub type PSC_R = crate::FieldReader<PSC_A>;
#[doc = "Prescaler Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Divide by 1."]
    DIV1 = 0,
    #[doc = "1: Divide by 2."]
    DIV2 = 1,
    #[doc = "2: Divide by 4."]
    DIV4 = 2,
    #[doc = "3: Divide by 8."]
    DIV8 = 3,
    #[doc = "4: Divide by 16."]
    DIV16 = 4,
    #[doc = "5: Divide by 32."]
    DIV32 = 5,
    #[doc = "6: Divide by 64."]
    DIV64 = 6,
    #[doc = "7: Divide by 128."]
    DIV128 = 7,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSC_A {
    type Ux = u8;
}
impl PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSC_A {
        match self.bits {
            0 => PSC_A::DIV1,
            1 => PSC_A::DIV2,
            2 => PSC_A::DIV4,
            3 => PSC_A::DIV8,
            4 => PSC_A::DIV16,
            5 => PSC_A::DIV32,
            6 => PSC_A::DIV64,
            7 => PSC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PSC_A::DIV1
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PSC_A::DIV2
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSC_A::DIV4
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSC_A::DIV8
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PSC_A::DIV16
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PSC_A::DIV32
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PSC_A::DIV64
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PSC_A::DIV128
    }
}
#[doc = "Field `PSC` writer - Prescaler Select."]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, PSC_A>;
impl<'a, REG, const O: u8> PSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV16)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV32)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV64)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV128)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Source Select."]
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
#[doc = "Clock Source Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: The internal 96 MHz oscillator is used for the system clock."]
    HIRC = 0,
    #[doc = "3: The nano-ring output is used for the system clock."]
    NANO_RING = 3,
    #[doc = "6: X32K is used for the system clock."]
    X32K = 6,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL_A {
    type Ux = u8;
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::HIRC),
            3 => Some(CLKSEL_A::NANO_RING),
            6 => Some(CLKSEL_A::X32K),
            _ => None,
        }
    }
    #[doc = "The internal 96 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn is_hirc(&self) -> bool {
        *self == CLKSEL_A::HIRC
    }
    #[doc = "The nano-ring output is used for the system clock."]
    #[inline(always)]
    pub fn is_nano_ring(&self) -> bool {
        *self == CLKSEL_A::NANO_RING
    }
    #[doc = "X32K is used for the system clock."]
    #[inline(always)]
    pub fn is_x32k(&self) -> bool {
        *self == CLKSEL_A::X32K
    }
}
#[doc = "Field `CLKSEL` writer - Clock Source Select."]
pub type CLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CLKSEL_A>;
impl<'a, REG, const O: u8> CLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The internal 96 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn hirc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::HIRC)
    }
    #[doc = "The nano-ring output is used for the system clock."]
    #[inline(always)]
    pub fn nano_ring(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::NANO_RING)
    }
    #[doc = "X32K is used for the system clock."]
    #[inline(always)]
    pub fn x32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::X32K)
    }
}
#[doc = "Field `CLKRDY` reader - Clock Ready."]
pub type CLKRDY_R = crate::BitReader<CLKRDY_A>;
#[doc = "Clock Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKRDY_A {
    #[doc = "0: Switchover to the new clock source (as selected by CLKSEL) has not yet occurred."]
    BUSY = 0,
    #[doc = "1: System clock running from CLKSEL clock source."]
    READY = 1,
}
impl From<CLKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CLKRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKRDY_A {
        match self.bits {
            false => CLKRDY_A::BUSY,
            true => CLKRDY_A::READY,
        }
    }
    #[doc = "Switchover to the new clock source (as selected by CLKSEL) has not yet occurred."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == CLKRDY_A::BUSY
    }
    #[doc = "System clock running from CLKSEL clock source."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CLKRDY_A::READY
    }
}
#[doc = "Field `X32K_EN` reader - 32kHz Crystal Oscillator Enable."]
pub type X32K_EN_R = crate::BitReader<X32K_EN_A>;
#[doc = "32kHz Crystal Oscillator Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X32K_EN_A {
    #[doc = "0: Is Disabled."]
    DISABLED = 0,
    #[doc = "1: Is Enabled."]
    ENABLED = 1,
}
impl From<X32K_EN_A> for bool {
    #[inline(always)]
    fn from(variant: X32K_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl X32K_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X32K_EN_A {
        match self.bits {
            false => X32K_EN_A::DISABLED,
            true => X32K_EN_A::ENABLED,
        }
    }
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == X32K_EN_A::DISABLED
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == X32K_EN_A::ENABLED
    }
}
#[doc = "Field `X32K_EN` writer - 32kHz Crystal Oscillator Enable."]
pub type X32K_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, X32K_EN_A>;
impl<'a, REG, const O: u8> X32K_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(X32K_EN_A::DISABLED)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(X32K_EN_A::ENABLED)
    }
}
#[doc = "Field `HIRC_EN` reader - 60MHz High Frequency Internal Reference Clock Enable."]
pub use X32K_EN_R as HIRC_EN_R;
#[doc = "Field `HIRC_EN` writer - 60MHz High Frequency Internal Reference Clock Enable."]
pub use X32K_EN_W as HIRC_EN_W;
#[doc = "Field `X32K_RDY` reader - 32kHz Crystal Oscillator Ready."]
pub type X32K_RDY_R = crate::BitReader<X32K_RDY_A>;
#[doc = "32kHz Crystal Oscillator Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X32K_RDY_A {
    #[doc = "0: Is not Ready."]
    NOT = 0,
    #[doc = "1: Is Ready."]
    READY = 1,
}
impl From<X32K_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: X32K_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl X32K_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X32K_RDY_A {
        match self.bits {
            false => X32K_RDY_A::NOT,
            true => X32K_RDY_A::READY,
        }
    }
    #[doc = "Is not Ready."]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == X32K_RDY_A::NOT
    }
    #[doc = "Is Ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == X32K_RDY_A::READY
    }
}
#[doc = "Field `HIRC_RDY` reader - 60MHz HIRC Ready."]
pub use X32K_RDY_R as HIRC_RDY_R;
#[doc = "Field `LIRC8K_RDY` reader - 8kHz Low Frequency Reference Clock Ready."]
pub use X32K_RDY_R as LIRC8K_RDY_R;
impl R {
    #[doc = "Bits 6:8 - Prescaler Select."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Clock Source Select."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 13 - Clock Ready."]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - 32kHz Crystal Oscillator Enable."]
    #[inline(always)]
    pub fn x32k_en(&self) -> X32K_EN_R {
        X32K_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 60MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn hirc_en(&self) -> HIRC_EN_R {
        HIRC_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 25 - 32kHz Crystal Oscillator Ready."]
    #[inline(always)]
    pub fn x32k_rdy(&self) -> X32K_RDY_R {
        X32K_RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 60MHz HIRC Ready."]
    #[inline(always)]
    pub fn hirc_rdy(&self) -> HIRC_RDY_R {
        HIRC_RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - 8kHz Low Frequency Reference Clock Ready."]
    #[inline(always)]
    pub fn lirc8k_rdy(&self) -> LIRC8K_RDY_R {
        LIRC8K_RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:8 - Prescaler Select."]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<CLK_CTRL_SPEC, 6> {
        PSC_W::new(self)
    }
    #[doc = "Bits 9:11 - Clock Source Select."]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<CLK_CTRL_SPEC, 9> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 17 - 32kHz Crystal Oscillator Enable."]
    #[inline(always)]
    #[must_use]
    pub fn x32k_en(&mut self) -> X32K_EN_W<CLK_CTRL_SPEC, 17> {
        X32K_EN_W::new(self)
    }
    #[doc = "Bit 18 - 60MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn hirc_en(&mut self) -> HIRC_EN_W<CLK_CTRL_SPEC, 18> {
        HIRC_EN_W::new(self)
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
#[doc = "Clock Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CTRL_SPEC;
impl crate::RegisterSpec for CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ctrl::R`](R) reader structure"]
impl crate::Readable for CLK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_ctrl::W`](W) writer structure"]
impl crate::Writable for CLK_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CTRL to value 0x08"]
impl crate::Resettable for CLK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
