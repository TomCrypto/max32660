#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<INT_EN_SPEC>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<INT_EN_SPEC>;
#[doc = "Field `RX_FRAME_ERROR` reader - Enable for RX Frame Error Interrupt."]
pub type RX_FRAME_ERROR_R = crate::BitReader;
#[doc = "Field `RX_FRAME_ERROR` writer - Enable for RX Frame Error Interrupt."]
pub type RX_FRAME_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_PARITY_ERROR` reader - Enable for RX Parity Error interrupt."]
pub type RX_PARITY_ERROR_R = crate::BitReader;
#[doc = "Field `RX_PARITY_ERROR` writer - Enable for RX Parity Error interrupt."]
pub type RX_PARITY_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTS` reader - Enable for CTS signal change interrupt."]
pub type CTS_R = crate::BitReader;
#[doc = "Field `CTS` writer - Enable for CTS signal change interrupt."]
pub type CTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_OVERRUN` reader - Enable for RX FIFO OVerrun interrupt."]
pub type RX_OVERRUN_R = crate::BitReader;
#[doc = "Field `RX_OVERRUN` writer - Enable for RX FIFO OVerrun interrupt."]
pub type RX_OVERRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_FIFO_LVL` reader - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RX_FIFO_LVL_R = crate::BitReader;
#[doc = "Field `RX_FIFO_LVL` writer - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RX_FIFO_LVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_FIFO_AE` reader - Enable for interrupt when TX FIFO has only one byte remaining."]
pub type TX_FIFO_AE_R = crate::BitReader;
#[doc = "Field `TX_FIFO_AE` writer - Enable for interrupt when TX FIFO has only one byte remaining."]
pub type TX_FIFO_AE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_FIFO_LVL` reader - Enable for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
pub type TX_FIFO_LVL_R = crate::BitReader;
#[doc = "Field `TX_FIFO_LVL` writer - Enable for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
pub type TX_FIFO_LVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BREAK` reader - Enable for received BREAK character interrupt."]
pub type BREAK_R = crate::BitReader;
#[doc = "Field `BREAK` writer - Enable for received BREAK character interrupt."]
pub type BREAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_TO` reader - Enable for RX Timeout Interrupt."]
pub type RX_TO_R = crate::BitReader;
#[doc = "Field `RX_TO` writer - Enable for RX Timeout Interrupt."]
pub type RX_TO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LAST_BREAK` reader - Enable for Last break character interrupt."]
pub type LAST_BREAK_R = crate::BitReader;
#[doc = "Field `LAST_BREAK` writer - Enable for Last break character interrupt."]
pub type LAST_BREAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable for RX Frame Error Interrupt."]
    #[inline(always)]
    pub fn rx_frame_error(&self) -> RX_FRAME_ERROR_R {
        RX_FRAME_ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable for RX Parity Error interrupt."]
    #[inline(always)]
    pub fn rx_parity_error(&self) -> RX_PARITY_ERROR_R {
        RX_PARITY_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable for CTS signal change interrupt."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for RX FIFO OVerrun interrupt."]
    #[inline(always)]
    pub fn rx_overrun(&self) -> RX_OVERRUN_R {
        RX_OVERRUN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_fifo_lvl(&self) -> RX_FIFO_LVL_R {
        RX_FIFO_LVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
    #[inline(always)]
    pub fn tx_fifo_lvl(&self) -> TX_FIFO_LVL_R {
        TX_FIFO_LVL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable for received BREAK character interrupt."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable for RX Timeout Interrupt."]
    #[inline(always)]
    pub fn rx_to(&self) -> RX_TO_R {
        RX_TO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable for Last break character interrupt."]
    #[inline(always)]
    pub fn last_break(&self) -> LAST_BREAK_R {
        LAST_BREAK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for RX Frame Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_frame_error(&mut self) -> RX_FRAME_ERROR_W<INT_EN_SPEC, 0> {
        RX_FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 1 - Enable for RX Parity Error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_parity_error(&mut self) -> RX_PARITY_ERROR_W<INT_EN_SPEC, 1> {
        RX_PARITY_ERROR_W::new(self)
    }
    #[doc = "Bit 2 - Enable for CTS signal change interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CTS_W<INT_EN_SPEC, 2> {
        CTS_W::new(self)
    }
    #[doc = "Bit 3 - Enable for RX FIFO OVerrun interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_overrun(&mut self) -> RX_OVERRUN_W<INT_EN_SPEC, 3> {
        RX_OVERRUN_W::new(self)
    }
    #[doc = "Bit 4 - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_lvl(&mut self) -> RX_FIFO_LVL_W<INT_EN_SPEC, 4> {
        RX_FIFO_LVL_W::new(self)
    }
    #[doc = "Bit 5 - Enable for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W<INT_EN_SPEC, 5> {
        TX_FIFO_AE_W::new(self)
    }
    #[doc = "Bit 6 - Enable for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_lvl(&mut self) -> TX_FIFO_LVL_W<INT_EN_SPEC, 6> {
        TX_FIFO_LVL_W::new(self)
    }
    #[doc = "Bit 7 - Enable for received BREAK character interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn break_(&mut self) -> BREAK_W<INT_EN_SPEC, 7> {
        BREAK_W::new(self)
    }
    #[doc = "Bit 8 - Enable for RX Timeout Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_to(&mut self) -> RX_TO_W<INT_EN_SPEC, 8> {
        RX_TO_W::new(self)
    }
    #[doc = "Bit 9 - Enable for Last break character interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn last_break(&mut self) -> LAST_BREAK_W<INT_EN_SPEC, 9> {
        LAST_BREAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
