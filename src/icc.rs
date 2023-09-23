#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache ID."]
    pub cache_id: CACHE_ID,
    #[doc = "0x04 - Memory Configuration."]
    pub mem_size: MEM_SIZE,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - Cache Control and Status."]
    pub cache_ctrl: CACHE_CTRL,
    _reserved3: [u8; 0x05fc],
    #[doc = "0x700 - Invalidate all registers."]
    pub invalidate: INVALIDATE,
}
#[doc = "CACHE_ID (r) register accessor: Cache ID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_id`]
module"]
pub type CACHE_ID = crate::Reg<cache_id::CACHE_ID_SPEC>;
#[doc = "Cache ID."]
pub mod cache_id;
#[doc = "MEM_SIZE (r) register accessor: Memory Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_size`]
module"]
pub type MEM_SIZE = crate::Reg<mem_size::MEM_SIZE_SPEC>;
#[doc = "Memory Configuration."]
pub mod mem_size;
#[doc = "CACHE_CTRL (rw) register accessor: Cache Control and Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_ctrl`]
module"]
pub type CACHE_CTRL = crate::Reg<cache_ctrl::CACHE_CTRL_SPEC>;
#[doc = "Cache Control and Status."]
pub mod cache_ctrl;
#[doc = "INVALIDATE (rw) register accessor: Invalidate all registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`invalidate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`invalidate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`invalidate`]
module"]
pub type INVALIDATE = crate::Reg<invalidate::INVALIDATE_SPEC>;
#[doc = "Invalidate all registers."]
pub mod invalidate;
