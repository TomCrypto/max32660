#[doc = "Register `TXCTRL0` reader"]
pub type R = crate::R<TXCTRL0_SPEC>;
#[doc = "Register `TXCTRL0` writer"]
pub type W = crate::W<TXCTRL0_SPEC>;
#[doc = "Field `TXPRELD` reader - Transmit FIFO Preaload Mode."]
pub type TXPRELD_R = crate::BitReader;
#[doc = "Field `TXPRELD` writer - Transmit FIFO Preaload Mode."]
pub type TXPRELD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_READY_MODE` reader - Transmit FIFO Ready Manual Mode."]
pub type TX_READY_MODE_R = crate::BitReader<TX_READY_MODE_A>;
#[doc = "Transmit FIFO Ready Manual Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_READY_MODE_A {
    #[doc = "0: HW control of I2CTXRDY enabled."]
    ENABLED = 0,
    #[doc = "1: HW control of I2CTXRDY disabled."]
    DISABLED = 1,
}
impl From<TX_READY_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_READY_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_READY_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_READY_MODE_A {
        match self.bits {
            false => TX_READY_MODE_A::ENABLED,
            true => TX_READY_MODE_A::DISABLED,
        }
    }
    #[doc = "HW control of I2CTXRDY enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_READY_MODE_A::ENABLED
    }
    #[doc = "HW control of I2CTXRDY disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_READY_MODE_A::DISABLED
    }
}
#[doc = "Field `TX_READY_MODE` writer - Transmit FIFO Ready Manual Mode."]
pub type TX_READY_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_READY_MODE_A>;
impl<'a, REG, const O: u8> TX_READY_MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HW control of I2CTXRDY enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_READY_MODE_A::ENABLED)
    }
    #[doc = "HW control of I2CTXRDY disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_READY_MODE_A::DISABLED)
    }
}
#[doc = "Field `TXFSH` reader - Transmit FIFO Flush."]
pub type TXFSH_R = crate::BitReader<TXFSH_A>;
#[doc = "Transmit FIFO Flush.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFSH_A {
    #[doc = "0: FIFO not flushed."]
    NOT_FLUSHED = 0,
    #[doc = "1: Flush TX_FIFO."]
    FLUSH = 1,
}
impl From<TXFSH_A> for bool {
    #[inline(always)]
    fn from(variant: TXFSH_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFSH_A {
        match self.bits {
            false => TXFSH_A::NOT_FLUSHED,
            true => TXFSH_A::FLUSH,
        }
    }
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn is_not_flushed(&self) -> bool {
        *self == TXFSH_A::NOT_FLUSHED
    }
    #[doc = "Flush TX_FIFO."]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == TXFSH_A::FLUSH
    }
}
#[doc = "Field `TXFSH` writer - Transmit FIFO Flush."]
pub type TXFSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXFSH_A>;
impl<'a, REG, const O: u8> TXFSH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn not_flushed(self) -> &'a mut crate::W<REG> {
        self.variant(TXFSH_A::NOT_FLUSHED)
    }
    #[doc = "Flush TX_FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(TXFSH_A::FLUSH)
    }
}
#[doc = "Field `TXTH` reader - Transmit FIFO Threshold."]
pub type TXTH_R = crate::FieldReader;
#[doc = "Field `TXTH` writer - Transmit FIFO Threshold."]
pub type TXTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode."]
    #[inline(always)]
    pub fn txpreld(&self) -> TXPRELD_R {
        TXPRELD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    pub fn tx_ready_mode(&self) -> TX_READY_MODE_R {
        TX_READY_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Flush."]
    #[inline(always)]
    pub fn txfsh(&self) -> TXFSH_R {
        TXFSH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold."]
    #[inline(always)]
    pub fn txth(&self) -> TXTH_R {
        TXTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode."]
    #[inline(always)]
    #[must_use]
    pub fn txpreld(&mut self) -> TXPRELD_W<TXCTRL0_SPEC, 0> {
        TXPRELD_W::new(self)
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ready_mode(&mut self) -> TX_READY_MODE_W<TXCTRL0_SPEC, 1> {
        TX_READY_MODE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO Flush."]
    #[inline(always)]
    #[must_use]
    pub fn txfsh(&mut self) -> TXFSH_W<TXCTRL0_SPEC, 7> {
        TXFSH_W::new(self)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold."]
    #[inline(always)]
    #[must_use]
    pub fn txth(&mut self) -> TXTH_W<TXCTRL0_SPEC, 8> {
        TXTH_W::new(self)
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
#[doc = "Transmit Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCTRL0_SPEC;
impl crate::RegisterSpec for TXCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl0::R`](R) reader structure"]
impl crate::Readable for TXCTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txctrl0::W`](W) writer structure"]
impl crate::Writable for TXCTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCTRL0 to value 0"]
impl crate::Resettable for TXCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
