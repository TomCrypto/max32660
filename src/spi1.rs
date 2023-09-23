#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI 16-bit Data Access."]
    pub data: DATA,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - SPI Control."]
    pub ctrl: CTRL,
    #[doc = "0x08 - SPI Interrupt Flag."]
    pub int_fl: INT_FL,
    #[doc = "0x0c - SPI Mode."]
    pub mode: MODE,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Baud Rate Reload Value."]
    pub brg: BRG,
    #[doc = "0x18 - SPI DMA."]
    pub dma: DMA,
    #[doc = "0x1c - I2S Control."]
    pub i2s_ctrl: I2S_CTRL,
}
#[doc = "DATA (rw) register accessor: SPI 16-bit Data Access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "SPI 16-bit Data Access."]
pub mod data;
#[doc = "CTRL (rw) register accessor: SPI Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI Control."]
pub mod ctrl;
#[doc = "INT_FL (rw) register accessor: SPI Interrupt Flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_fl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_fl`]
module"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "SPI Interrupt Flag."]
pub mod int_fl;
#[doc = "MODE (rw) register accessor: SPI Mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mode`]
module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "SPI Mode."]
pub mod mode;
#[doc = "BRG (rw) register accessor: Baud Rate Reload Value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`brg`]
module"]
pub type BRG = crate::Reg<brg::BRG_SPEC>;
#[doc = "Baud Rate Reload Value."]
pub mod brg;
#[doc = "DMA (rw) register accessor: SPI DMA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "SPI DMA."]
pub mod dma;
#[doc = "I2S_CTRL (rw) register accessor: I2S Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2s_ctrl`]
module"]
pub type I2S_CTRL = crate::Reg<i2s_ctrl::I2S_CTRL_SPEC>;
#[doc = "I2S Control."]
pub mod i2s_ctrl;
