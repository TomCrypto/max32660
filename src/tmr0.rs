#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Counter."]
    pub cnt: CNT,
    #[doc = "0x04 - Timer Compare."]
    pub cmp: CMP,
    #[doc = "0x08 - Timer PWM."]
    pub pwm: PWM,
    #[doc = "0x0c - Timer Interrupt Status."]
    pub intr: INTR,
    #[doc = "0x10 - Timer Control."]
    pub cn: CN,
}
#[doc = "CNT (rw) register accessor: Timer Counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Timer Counter."]
pub mod cnt;
#[doc = "CMP (rw) register accessor: Timer Compare.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp`]
module"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "Timer Compare."]
pub mod cmp;
#[doc = "PWM (rw) register accessor: Timer PWM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwm`]
module"]
pub type PWM = crate::Reg<pwm::PWM_SPEC>;
#[doc = "Timer PWM."]
pub mod pwm;
#[doc = "INTR (rw) register accessor: Timer Interrupt Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Timer Interrupt Status."]
pub mod intr;
#[doc = "CN (rw) register accessor: Timer Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cn`]
module"]
pub type CN = crate::Reg<cn::CN_SPEC>;
#[doc = "Timer Control."]
pub mod cn;
