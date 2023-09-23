#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA Channel Configuration."]
    pub cfg: CFG,
    #[doc = "0x04 - DMA Channel Status."]
    pub stat: STAT,
    #[doc = "0x08 - Source Device Address."]
    pub src: SRC,
    #[doc = "0x0c - Destination Device Address."]
    pub dst: DST,
    #[doc = "0x10 - DMA Counter."]
    pub cnt: CNT,
    #[doc = "0x14 - Source Address Reload Value."]
    pub src_rld: SRC_RLD,
    #[doc = "0x18 - Destination Address Reload Value."]
    pub dst_rld: DST_RLD,
    #[doc = "0x1c - DMA Channel Count Reload."]
    pub cnt_rld: CNT_RLD,
}
#[doc = "CFG (rw) register accessor: DMA Channel Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "DMA Channel Configuration."]
pub mod cfg;
#[doc = "STAT (rw) register accessor: DMA Channel Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "DMA Channel Status."]
pub mod stat;
#[doc = "SRC (rw) register accessor: Source Device Address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`src`]
module"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "Source Device Address."]
pub mod src;
#[doc = "DST (rw) register accessor: Destination Device Address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dst`]
module"]
pub type DST = crate::Reg<dst::DST_SPEC>;
#[doc = "Destination Device Address."]
pub mod dst;
#[doc = "CNT (rw) register accessor: DMA Counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "DMA Counter."]
pub mod cnt;
#[doc = "SRC_RLD (rw) register accessor: Source Address Reload Value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_rld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_rld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`src_rld`]
module"]
pub type SRC_RLD = crate::Reg<src_rld::SRC_RLD_SPEC>;
#[doc = "Source Address Reload Value."]
pub mod src_rld;
#[doc = "DST_RLD (rw) register accessor: Destination Address Reload Value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_rld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_rld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dst_rld`]
module"]
pub type DST_RLD = crate::Reg<dst_rld::DST_RLD_SPEC>;
#[doc = "Destination Address Reload Value."]
pub mod dst_rld;
#[doc = "CNT_RLD (rw) register accessor: DMA Channel Count Reload.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt_rld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt_rld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnt_rld`]
module"]
pub type CNT_RLD = crate::Reg<cnt_rld::CNT_RLD_SPEC>;
#[doc = "DMA Channel Count Reload."]
pub mod cnt_rld;
