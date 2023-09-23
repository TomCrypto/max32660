#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control."]
    pub ctrl0: CTRL0,
    #[doc = "0x04 - Threshold Control."]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Status."]
    pub stat: STAT,
    #[doc = "0x0c - Interrupt Enable."]
    pub int_en: INT_EN,
    #[doc = "0x10 - Interrupt Status Flags."]
    pub int_fl: INT_FL,
    #[doc = "0x14 - Baud rate."]
    pub baud0: BAUD0,
    #[doc = "0x18 - Baud rate."]
    pub baud1: BAUD1,
    #[doc = "0x1c - FIFO Data buffer."]
    pub fifo: FIFO,
    #[doc = "0x20 - DMA Configuration."]
    pub dma: DMA,
    #[doc = "0x24 - Transmit FIFO Status."]
    pub txfifo: TXFIFO,
}
#[doc = "CTRL0 (rw) register accessor: Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl0`]
module"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Control."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: Threshold Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Threshold Control."]
pub mod ctrl1;
#[doc = "STAT (r) register accessor: Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status."]
pub mod stat;
#[doc = "INT_EN (rw) register accessor: Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_en`]
module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interrupt Enable."]
pub mod int_en;
#[doc = "INT_FL (rw) register accessor: Interrupt Status Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_fl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_fl`]
module"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "Interrupt Status Flags."]
pub mod int_fl;
#[doc = "BAUD0 (rw) register accessor: Baud rate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`baud0`]
module"]
pub type BAUD0 = crate::Reg<baud0::BAUD0_SPEC>;
#[doc = "Baud rate."]
pub mod baud0;
#[doc = "BAUD1 (rw) register accessor: Baud rate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`baud1`]
module"]
pub type BAUD1 = crate::Reg<baud1::BAUD1_SPEC>;
#[doc = "Baud rate."]
pub mod baud1;
#[doc = "FIFO (rw) register accessor: FIFO Data buffer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Data buffer."]
pub mod fifo;
#[doc = "DMA (rw) register accessor: DMA Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Configuration."]
pub mod dma;
#[doc = "TXFIFO (rw) register accessor: Transmit FIFO Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txfifo`]
module"]
pub type TXFIFO = crate::Reg<txfifo::TXFIFO_SPEC>;
#[doc = "Transmit FIFO Status."]
pub mod txfifo;
