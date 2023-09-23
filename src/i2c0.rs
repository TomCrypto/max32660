#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0."]
    pub ctrl0: CTRL0,
    #[doc = "0x04 - Status."]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Status."]
    pub intfl0: INTFL0,
    #[doc = "0x0c - Interrupt Enable."]
    pub inten0: INTEN0,
    #[doc = "0x10 - Interrupt Status Register 1."]
    pub intfl1: INTFL1,
    #[doc = "0x14 - Interrupt Status Register 1."]
    pub inten1: INTEN1,
    #[doc = "0x18 - FIFO Configuration."]
    pub fifolen: FIFOLEN,
    #[doc = "0x1c - Receive Control Register 0."]
    pub rxctrl0: RXCTRL0,
    #[doc = "0x20 - Receive Control Register 1."]
    pub rxctrl1: RXCTRL1,
    #[doc = "0x24 - Transmit Control Register 0."]
    pub txctrl0: TXCTRL0,
    #[doc = "0x28 - Transmit Control Register 1."]
    pub txctrl1: TXCTRL1,
    #[doc = "0x2c - Data."]
    pub fifo: FIFO,
    #[doc = "0x30 - Master Control."]
    pub mstr_mode: MSTR_MODE,
    #[doc = "0x34 - Clock Low."]
    pub clklo: CLKLO,
    #[doc = "0x38 - Clock high."]
    pub clkhi: CLKHI,
    #[doc = "0x3c - HS-Mode Clock Control."]
    pub hs_clk: HS_CLK,
    #[doc = "0x40 - Timeout."]
    pub timeout: TIMEOUT,
    #[doc = "0x44 - Slave Address."]
    pub sladdr: SLADDR,
    #[doc = "0x48 - DMA Control."]
    pub dma: DMA,
}
#[doc = "CTRL0 (rw) register accessor: Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl0`]
module"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Control Register 0."]
pub mod ctrl0;
#[doc = "STATUS (rw) register accessor: Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status."]
pub mod status;
#[doc = "INTFL0 (rw) register accessor: Interrupt Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl0`]
module"]
pub type INTFL0 = crate::Reg<intfl0::INTFL0_SPEC>;
#[doc = "Interrupt Status."]
pub mod intfl0;
#[doc = "INTEN0 (rw) register accessor: Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten0`]
module"]
pub type INTEN0 = crate::Reg<inten0::INTEN0_SPEC>;
#[doc = "Interrupt Enable."]
pub mod inten0;
#[doc = "INTFL1 (rw) register accessor: Interrupt Status Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl1`]
module"]
pub type INTFL1 = crate::Reg<intfl1::INTFL1_SPEC>;
#[doc = "Interrupt Status Register 1."]
pub mod intfl1;
#[doc = "INTEN1 (rw) register accessor: Interrupt Status Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten1`]
module"]
pub type INTEN1 = crate::Reg<inten1::INTEN1_SPEC>;
#[doc = "Interrupt Status Register 1."]
pub mod inten1;
#[doc = "FIFOLEN (rw) register accessor: FIFO Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifolen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifolen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifolen`]
module"]
pub type FIFOLEN = crate::Reg<fifolen::FIFOLEN_SPEC>;
#[doc = "FIFO Configuration."]
pub mod fifolen;
#[doc = "RXCTRL0 (rw) register accessor: Receive Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxctrl0`]
module"]
pub type RXCTRL0 = crate::Reg<rxctrl0::RXCTRL0_SPEC>;
#[doc = "Receive Control Register 0."]
pub mod rxctrl0;
#[doc = "RXCTRL1 (rw) register accessor: Receive Control Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxctrl1`]
module"]
pub type RXCTRL1 = crate::Reg<rxctrl1::RXCTRL1_SPEC>;
#[doc = "Receive Control Register 1."]
pub mod rxctrl1;
#[doc = "TXCTRL0 (rw) register accessor: Transmit Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txctrl0`]
module"]
pub type TXCTRL0 = crate::Reg<txctrl0::TXCTRL0_SPEC>;
#[doc = "Transmit Control Register 0."]
pub mod txctrl0;
#[doc = "TXCTRL1 (rw) register accessor: Transmit Control Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txctrl1`]
module"]
pub type TXCTRL1 = crate::Reg<txctrl1::TXCTRL1_SPEC>;
#[doc = "Transmit Control Register 1."]
pub mod txctrl1;
#[doc = "FIFO (rw) register accessor: Data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Data."]
pub mod fifo;
#[doc = "MSTR_MODE (rw) register accessor: Master Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstr_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstr_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mstr_mode`]
module"]
pub type MSTR_MODE = crate::Reg<mstr_mode::MSTR_MODE_SPEC>;
#[doc = "Master Control."]
pub mod mstr_mode;
#[doc = "CLKLO (rw) register accessor: Clock Low.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clklo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clklo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clklo`]
module"]
pub type CLKLO = crate::Reg<clklo::CLKLO_SPEC>;
#[doc = "Clock Low."]
pub mod clklo;
#[doc = "CLKHI (rw) register accessor: Clock high.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkhi`]
module"]
pub type CLKHI = crate::Reg<clkhi::CLKHI_SPEC>;
#[doc = "Clock high."]
pub mod clkhi;
#[doc = "HS_CLK (rw) register accessor: HS-Mode Clock Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hs_clk`]
module"]
pub type HS_CLK = crate::Reg<hs_clk::HS_CLK_SPEC>;
#[doc = "HS-Mode Clock Control."]
pub mod hs_clk;
#[doc = "TIMEOUT (rw) register accessor: Timeout.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timeout`]
module"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Timeout."]
pub mod timeout;
#[doc = "SLADDR (rw) register accessor: Slave Address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sladdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sladdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sladdr`]
module"]
pub type SLADDR = crate::Reg<sladdr::SLADDR_SPEC>;
#[doc = "Slave Address."]
pub mod sladdr;
#[doc = "DMA (rw) register accessor: DMA Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Control."]
pub mod dma;
