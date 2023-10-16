#[doc = "Register `RXCTRL0` reader"]
pub type R = crate::R<RXCTRL0_SPEC>;
#[doc = "Register `RXCTRL0` writer"]
pub type W = crate::W<RXCTRL0_SPEC>;
#[doc = "Field `DNR` reader - Do Not Respond."]
pub type DNR_R = crate::BitReader<DNR_A>;
#[doc = "Do Not Respond.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNR_A {
    #[doc = "0: Always respond to address match."]
    RESPOND = 0,
    #[doc = "1: Do not respond to address match when RX_FIFO is not empty."]
    NOT_RESPOND_RX_FIFO_EMPTY = 1,
}
impl From<DNR_A> for bool {
    #[inline(always)]
    fn from(variant: DNR_A) -> Self {
        variant as u8 != 0
    }
}
impl DNR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNR_A {
        match self.bits {
            false => DNR_A::RESPOND,
            true => DNR_A::NOT_RESPOND_RX_FIFO_EMPTY,
        }
    }
    #[doc = "Always respond to address match."]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        *self == DNR_A::RESPOND
    }
    #[doc = "Do not respond to address match when RX_FIFO is not empty."]
    #[inline(always)]
    pub fn is_not_respond_rx_fifo_empty(&self) -> bool {
        *self == DNR_A::NOT_RESPOND_RX_FIFO_EMPTY
    }
}
#[doc = "Field `DNR` writer - Do Not Respond."]
pub type DNR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DNR_A>;
impl<'a, REG, const O: u8> DNR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Always respond to address match."]
    #[inline(always)]
    pub fn respond(self) -> &'a mut crate::W<REG> {
        self.variant(DNR_A::RESPOND)
    }
    #[doc = "Do not respond to address match when RX_FIFO is not empty."]
    #[inline(always)]
    pub fn not_respond_rx_fifo_empty(self) -> &'a mut crate::W<REG> {
        self.variant(DNR_A::NOT_RESPOND_RX_FIFO_EMPTY)
    }
}
#[doc = "Field `RXFSH` reader - Receive FIFO Flush."]
pub type RXFSH_R = crate::BitReader<RXFSH_A>;
#[doc = "Receive FIFO Flush.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFSH_A {
    #[doc = "0: FIFO not flushed."]
    NOT_FLUSHED = 0,
    #[doc = "1: Flush RX_FIFO."]
    FLUSH = 1,
}
impl From<RXFSH_A> for bool {
    #[inline(always)]
    fn from(variant: RXFSH_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFSH_A {
        match self.bits {
            false => RXFSH_A::NOT_FLUSHED,
            true => RXFSH_A::FLUSH,
        }
    }
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn is_not_flushed(&self) -> bool {
        *self == RXFSH_A::NOT_FLUSHED
    }
    #[doc = "Flush RX_FIFO."]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == RXFSH_A::FLUSH
    }
}
#[doc = "Field `RXFSH` writer - Receive FIFO Flush."]
pub type RXFSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXFSH_A>;
impl<'a, REG, const O: u8> RXFSH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn not_flushed(self) -> &'a mut crate::W<REG> {
        self.variant(RXFSH_A::NOT_FLUSHED)
    }
    #[doc = "Flush RX_FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(RXFSH_A::FLUSH)
    }
}
#[doc = "Field `RXTH` reader - Receive FIFO Threshold."]
pub type RXTH_R = crate::FieldReader;
#[doc = "Field `RXTH` writer - Receive FIFO Threshold."]
pub type RXTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Do Not Respond."]
    #[inline(always)]
    pub fn dnr(&self) -> DNR_R {
        DNR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Flush."]
    #[inline(always)]
    pub fn rxfsh(&self) -> RXFSH_R {
        RXFSH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Receive FIFO Threshold."]
    #[inline(always)]
    pub fn rxth(&self) -> RXTH_R {
        RXTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Do Not Respond."]
    #[inline(always)]
    #[must_use]
    pub fn dnr(&mut self) -> DNR_W<RXCTRL0_SPEC, 0> {
        DNR_W::new(self)
    }
    #[doc = "Bit 7 - Receive FIFO Flush."]
    #[inline(always)]
    #[must_use]
    pub fn rxfsh(&mut self) -> RXFSH_W<RXCTRL0_SPEC, 7> {
        RXFSH_W::new(self)
    }
    #[doc = "Bits 8:11 - Receive FIFO Threshold."]
    #[inline(always)]
    #[must_use]
    pub fn rxth(&mut self) -> RXTH_W<RXCTRL0_SPEC, 8> {
        RXTH_W::new(self)
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
#[doc = "Receive Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCTRL0_SPEC;
impl crate::RegisterSpec for RXCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl0::R`](R) reader structure"]
impl crate::Readable for RXCTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxctrl0::W`](W) writer structure"]
impl crate::Writable for RXCTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCTRL0 to value 0"]
impl crate::Resettable for RXCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
