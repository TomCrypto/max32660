#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    pub data: DATA,
    #[doc = "0x04 - Control Register 0."]
    pub ctrl0: CTRL0,
    #[doc = "0x08 - Control Register 1."]
    pub ctrl1: CTRL1,
    #[doc = "0x0c - Control Register 2."]
    pub ctrl2: CTRL2,
    #[doc = "0x10 - Slave Select Timing."]
    pub ss_time: SS_TIME,
    #[doc = "0x14 - Clock Configuration."]
    pub clk_cfg: CLK_CFG,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DMA Control."]
    pub dma: DMA,
    #[doc = "0x20 - Interrupt Flags."]
    pub int_fl: INT_FL,
    #[doc = "0x24 - Interrupt Enable."]
    pub int_en: INT_EN,
    #[doc = "0x28 - Wakeup Flags."]
    pub wake_fl: WAKE_FL,
    #[doc = "0x2c - Wakeup Enable."]
    pub wake_en: WAKE_EN,
    #[doc = "0x30 - SPI Status."]
    pub stat: STAT,
}
#[doc = "DATA (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod data;
#[doc = "CTRL0 (rw) register accessor: Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl0`]
module"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Control Register 0."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: Control Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control Register 1."]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Control Register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control Register 2."]
pub mod ctrl2;
#[doc = "SS_TIME (rw) register accessor: Slave Select Timing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ss_time`]
module"]
pub type SS_TIME = crate::Reg<ss_time::SS_TIME_SPEC>;
#[doc = "Slave Select Timing."]
pub mod ss_time;
#[doc = "CLK_CFG (rw) register accessor: Clock Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_cfg`]
module"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = "Clock Configuration."]
pub mod clk_cfg;
#[doc = "DMA (rw) register accessor: DMA Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Control."]
pub mod dma;
#[doc = "INT_FL (rw) register accessor: Interrupt Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_fl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_fl`]
module"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "Interrupt Flags."]
pub mod int_fl;
#[doc = "INT_EN (rw) register accessor: Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_en`]
module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interrupt Enable."]
pub mod int_en;
#[doc = "WAKE_FL (rw) register accessor: Wakeup Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wake_fl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_fl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wake_fl`]
module"]
pub type WAKE_FL = crate::Reg<wake_fl::WAKE_FL_SPEC>;
#[doc = "Wakeup Flags."]
pub mod wake_fl;
#[doc = "WAKE_EN (rw) register accessor: Wakeup Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wake_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wake_en`]
module"]
pub type WAKE_EN = crate::Reg<wake_en::WAKE_EN_SPEC>;
#[doc = "Wakeup Enable."]
pub mod wake_en;
#[doc = "STAT (r) register accessor: SPI Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SPI Status."]
pub mod stat;
