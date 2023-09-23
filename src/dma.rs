#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control."]
    pub int_en: INT_EN,
    #[doc = "0x04 - DMA Interrupt."]
    pub int_fl: INT_FL,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100..0x180 - DMA Channel registers."]
    pub ch: [CH; 4],
}
#[doc = "INT_EN (rw) register accessor: DMA Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_en`]
module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "DMA Control."]
pub mod int_en;
#[doc = "INT_FL (r) register accessor: DMA Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_fl`]
module"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "DMA Interrupt."]
pub mod int_fl;
#[doc = "DMA Channel registers."]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "DMA Channel registers."]
pub mod ch;
