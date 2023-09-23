#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `TX_BUSY` reader - Read-only flag indicating the UART transmit status."]
pub type TX_BUSY_R = crate::BitReader;
#[doc = "Field `RX_BUSY` reader - Read-only flag indicating the UARTreceiver status."]
pub type RX_BUSY_R = crate::BitReader;
#[doc = "Field `PARITY` reader - 9th Received bit state."]
pub type PARITY_R = crate::BitReader;
#[doc = "Field `BREAK` reader - Received BREAK status."]
pub type BREAK_R = crate::BitReader;
#[doc = "Field `RX_EMPTY` reader - Read-only flag indicating the RX FIFO state."]
pub type RX_EMPTY_R = crate::BitReader;
#[doc = "Field `RX_FULL` reader - Read-only flag indicating the RX FIFO state."]
pub type RX_FULL_R = crate::BitReader;
#[doc = "Field `TX_EMPTY` reader - Read-only flag indicating the TX FIFO state."]
pub type TX_EMPTY_R = crate::BitReader;
#[doc = "Field `TX_FULL` reader - Read-only flag indicating the TX FIFO state."]
pub type TX_FULL_R = crate::BitReader;
#[doc = "Field `RX_NUM` reader - Indicates the number of bytes currently in the RX FIFO."]
pub type RX_NUM_R = crate::FieldReader;
#[doc = "Field `TX_NUM` reader - Indicates the number of bytes currently in the TX FIFO."]
pub type TX_NUM_R = crate::FieldReader;
#[doc = "Field `RX_TO` reader - RX Timeout status."]
pub type RX_TO_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read-only flag indicating the UART transmit status."]
    #[inline(always)]
    pub fn tx_busy(&self) -> TX_BUSY_R {
        TX_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read-only flag indicating the UARTreceiver status."]
    #[inline(always)]
    pub fn rx_busy(&self) -> RX_BUSY_R {
        RX_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 9th Received bit state."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Received BREAK status."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read-only flag indicating the RX FIFO state."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read-only flag indicating the RX FIFO state."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read-only flag indicating the TX FIFO state."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read-only flag indicating the TX FIFO state."]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Indicates the number of bytes currently in the RX FIFO."]
    #[inline(always)]
    pub fn rx_num(&self) -> RX_NUM_R {
        RX_NUM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Indicates the number of bytes currently in the TX FIFO."]
    #[inline(always)]
    pub fn tx_num(&self) -> TX_NUM_R {
        TX_NUM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - RX Timeout status."]
    #[inline(always)]
    pub fn rx_to(&self) -> RX_TO_R {
        RX_TO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
