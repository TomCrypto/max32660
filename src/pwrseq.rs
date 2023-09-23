#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Power Control."]
    pub lp_ctrl: LP_CTRL,
    #[doc = "0x04 - Low Power Mode Wakeup Flags for GPIO0."]
    pub lp_wakefl: LP_WAKEFL,
    #[doc = "0x08 - Low Power I/O Wakeup Enable Register 0."]
    pub lpwk_en: LPWK_EN,
    _reserved3: [u8; 0x34],
    #[doc = "0x40 - Low Power Memory Shutdown Control."]
    pub lpmemsd: LPMEMSD,
}
#[doc = "LP_CTRL (rw) register accessor: Low Power Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_ctrl`]
module"]
pub type LP_CTRL = crate::Reg<lp_ctrl::LP_CTRL_SPEC>;
#[doc = "Low Power Control."]
pub mod lp_ctrl;
#[doc = "LP_WAKEFL (rw) register accessor: Low Power Mode Wakeup Flags for GPIO0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_wakefl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_wakefl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_wakefl`]
module"]
pub type LP_WAKEFL = crate::Reg<lp_wakefl::LP_WAKEFL_SPEC>;
#[doc = "Low Power Mode Wakeup Flags for GPIO0."]
pub mod lp_wakefl;
#[doc = "LPWK_EN (rw) register accessor: Low Power I/O Wakeup Enable Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpwk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpwk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpwk_en`]
module"]
pub type LPWK_EN = crate::Reg<lpwk_en::LPWK_EN_SPEC>;
#[doc = "Low Power I/O Wakeup Enable Register 0."]
pub mod lpwk_en;
#[doc = "LPMEMSD (rw) register accessor: Low Power Memory Shutdown Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmemsd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmemsd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpmemsd`]
module"]
pub type LPMEMSD = crate::Reg<lpmemsd::LPMEMSD_SPEC>;
#[doc = "Low Power Memory Shutdown Control."]
pub mod lpmemsd;
