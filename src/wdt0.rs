#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Watchdog Timer Reset."]
    pub rst: RST,
}
#[doc = "CTRL (rw) register accessor: Watchdog Timer Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Watchdog Timer Control."]
pub mod ctrl;
#[doc = "RST (w) register accessor: Watchdog Timer Reset.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rst`]
module"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "Watchdog Timer Reset."]
pub mod rst;
