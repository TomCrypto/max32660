#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `RX_FIFO_LVL` reader - RX FIFO Threshold Level."]
pub type RX_FIFO_LVL_R = crate::FieldReader;
#[doc = "Field `RX_FIFO_LVL` writer - RX FIFO Threshold Level."]
pub type RX_FIFO_LVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `TX_FIFO_LVL` reader - TX FIFO Threshold Level."]
pub type TX_FIFO_LVL_R = crate::FieldReader;
#[doc = "Field `TX_FIFO_LVL` writer - TX FIFO Threshold Level."]
pub type TX_FIFO_LVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `RTS_FIFO_LVL` reader - RTS threshold control."]
pub type RTS_FIFO_LVL_R = crate::FieldReader;
#[doc = "Field `RTS_FIFO_LVL` writer - RTS threshold control."]
pub type RTS_FIFO_LVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - RX FIFO Threshold Level."]
    #[inline(always)]
    pub fn rx_fifo_lvl(&self) -> RX_FIFO_LVL_R {
        RX_FIFO_LVL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - TX FIFO Threshold Level."]
    #[inline(always)]
    pub fn tx_fifo_lvl(&self) -> TX_FIFO_LVL_R {
        TX_FIFO_LVL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - RTS threshold control."]
    #[inline(always)]
    pub fn rts_fifo_lvl(&self) -> RTS_FIFO_LVL_R {
        RTS_FIFO_LVL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RX FIFO Threshold Level."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_lvl(&mut self) -> RX_FIFO_LVL_W<CTRL1_SPEC, 0> {
        RX_FIFO_LVL_W::new(self)
    }
    #[doc = "Bits 8:13 - TX FIFO Threshold Level."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_lvl(&mut self) -> TX_FIFO_LVL_W<CTRL1_SPEC, 8> {
        TX_FIFO_LVL_W::new(self)
    }
    #[doc = "Bits 16:21 - RTS threshold control."]
    #[inline(always)]
    #[must_use]
    pub fn rts_fifo_lvl(&mut self) -> RTS_FIFO_LVL_W<CTRL1_SPEC, 16> {
        RTS_FIFO_LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Threshold Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
