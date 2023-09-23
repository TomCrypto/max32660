#[doc = "Register `INT_FL` reader"]
pub type R = crate::R<INT_FL_SPEC>;
#[doc = "Register `INT_FL` writer"]
pub type W = crate::W<INT_FL_SPEC>;
#[doc = "Field `FRAME` reader - FLAG for RX Frame Error Interrupt."]
pub type FRAME_R = crate::BitReader;
#[doc = "Field `FRAME` writer - FLAG for RX Frame Error Interrupt."]
pub type FRAME_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `PARITY` reader - FLAG for RX Parity Error interrupt."]
pub type PARITY_R = crate::BitReader;
#[doc = "Field `PARITY` writer - FLAG for RX Parity Error interrupt."]
pub type PARITY_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CTS` reader - FLAG for CTS signal change interrupt."]
pub type CTS_R = crate::BitReader;
#[doc = "Field `CTS` writer - FLAG for CTS signal change interrupt."]
pub type CTS_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `RX_OVR` reader - FLAG for RX FIFO Overrun interrupt."]
pub type RX_OVR_R = crate::BitReader;
#[doc = "Field `RX_OVR` writer - FLAG for RX FIFO Overrun interrupt."]
pub type RX_OVR_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `RX_FIFO_LVL` reader - FLAG for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RX_FIFO_LVL_R = crate::BitReader;
#[doc = "Field `RX_FIFO_LVL` writer - FLAG for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RX_FIFO_LVL_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `TX_FIFO_AE` reader - FLAG for interrupt when TX FIFO has only one byte remaining."]
pub type TX_FIFO_AE_R = crate::BitReader;
#[doc = "Field `TX_FIFO_AE` writer - FLAG for interrupt when TX FIFO has only one byte remaining."]
pub type TX_FIFO_AE_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `TX_FIFO_LVL` reader - FLAG for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
pub type TX_FIFO_LVL_R = crate::BitReader;
#[doc = "Field `TX_FIFO_LVL` writer - FLAG for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
pub type TX_FIFO_LVL_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `BREAK` reader - FLAG for received BREAK character interrupt."]
pub type BREAK_R = crate::BitReader;
#[doc = "Field `BREAK` writer - FLAG for received BREAK character interrupt."]
pub type BREAK_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `RX_TO` reader - FLAG for RX Timeout Interrupt."]
pub type RX_TO_R = crate::BitReader;
#[doc = "Field `RX_TO` writer - FLAG for RX Timeout Interrupt."]
pub type RX_TO_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `LAST_BREAK` reader - FLAG for Last break character interrupt."]
pub type LAST_BREAK_R = crate::BitReader;
#[doc = "Field `LAST_BREAK` writer - FLAG for Last break character interrupt."]
pub type LAST_BREAK_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - FLAG for RX Frame Error Interrupt."]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLAG for RX Parity Error interrupt."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLAG for CTS signal change interrupt."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLAG for RX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rx_ovr(&self) -> RX_OVR_R {
        RX_OVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLAG for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_fifo_lvl(&self) -> RX_FIFO_LVL_R {
        RX_FIFO_LVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FLAG for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLAG for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
    #[inline(always)]
    pub fn tx_fifo_lvl(&self) -> TX_FIFO_LVL_R {
        TX_FIFO_LVL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLAG for received BREAK character interrupt."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLAG for RX Timeout Interrupt."]
    #[inline(always)]
    pub fn rx_to(&self) -> RX_TO_R {
        RX_TO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLAG for Last break character interrupt."]
    #[inline(always)]
    pub fn last_break(&self) -> LAST_BREAK_R {
        LAST_BREAK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLAG for RX Frame Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<INT_FL_SPEC, 0> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 1 - FLAG for RX Parity Error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<INT_FL_SPEC, 1> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 2 - FLAG for CTS signal change interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CTS_W<INT_FL_SPEC, 2> {
        CTS_W::new(self)
    }
    #[doc = "Bit 3 - FLAG for RX FIFO Overrun interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ovr(&mut self) -> RX_OVR_W<INT_FL_SPEC, 3> {
        RX_OVR_W::new(self)
    }
    #[doc = "Bit 4 - FLAG for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_lvl(&mut self) -> RX_FIFO_LVL_W<INT_FL_SPEC, 4> {
        RX_FIFO_LVL_W::new(self)
    }
    #[doc = "Bit 5 - FLAG for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W<INT_FL_SPEC, 5> {
        TX_FIFO_AE_W::new(self)
    }
    #[doc = "Bit 6 - FLAG for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_lvl(&mut self) -> TX_FIFO_LVL_W<INT_FL_SPEC, 6> {
        TX_FIFO_LVL_W::new(self)
    }
    #[doc = "Bit 7 - FLAG for received BREAK character interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn break_(&mut self) -> BREAK_W<INT_FL_SPEC, 7> {
        BREAK_W::new(self)
    }
    #[doc = "Bit 8 - FLAG for RX Timeout Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_to(&mut self) -> RX_TO_W<INT_FL_SPEC, 8> {
        RX_TO_W::new(self)
    }
    #[doc = "Bit 9 - FLAG for Last break character interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn last_break(&mut self) -> LAST_BREAK_W<INT_FL_SPEC, 9> {
        LAST_BREAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Status Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_fl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FL_SPEC;
impl crate::RegisterSpec for INT_FL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_fl::R`](R) reader structure"]
impl crate::Readable for INT_FL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_fl::W`](W) writer structure"]
impl crate::Writable for INT_FL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03ff;
}
#[doc = "`reset()` method sets INT_FL to value 0"]
impl crate::Resettable for INT_FL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
