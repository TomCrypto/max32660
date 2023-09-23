#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Second Counter."]
    pub sec: SEC,
    #[doc = "0x04 - RTC Sub-second Counter."]
    pub ssec: SSEC,
    #[doc = "0x08 - Time-of-day Alarm."]
    pub ras: RAS,
    #[doc = "0x0c - RTC sub-second alarm."]
    pub rssa: RSSA,
    #[doc = "0x10 - RTC Control."]
    pub ctrl: CTRL,
    #[doc = "0x14 - RTC Trim."]
    pub trim: TRIM,
    #[doc = "0x18 - RTC Oscillator Control."]
    pub oscctrl: OSCCTRL,
}
#[doc = "SEC (rw) register accessor: RTC Second Counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sec`]
module"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "RTC Second Counter."]
pub mod sec;
#[doc = "SSEC (rw) register accessor: RTC Sub-second Counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ssec`]
module"]
pub type SSEC = crate::Reg<ssec::SSEC_SPEC>;
#[doc = "RTC Sub-second Counter."]
pub mod ssec;
#[doc = "RAS (rw) register accessor: Time-of-day Alarm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ras::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ras::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ras`]
module"]
pub type RAS = crate::Reg<ras::RAS_SPEC>;
#[doc = "Time-of-day Alarm."]
pub mod ras;
#[doc = "RSSA (rw) register accessor: RTC sub-second alarm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rssa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rssa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rssa`]
module"]
pub type RSSA = crate::Reg<rssa::RSSA_SPEC>;
#[doc = "RTC sub-second alarm."]
pub mod rssa;
#[doc = "CTRL (rw) register accessor: RTC Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC Control."]
pub mod ctrl;
#[doc = "TRIM (rw) register accessor: RTC Trim.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`trim`]
module"]
pub type TRIM = crate::Reg<trim::TRIM_SPEC>;
#[doc = "RTC Trim."]
pub mod trim;
#[doc = "OSCCTRL (rw) register accessor: RTC Oscillator Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oscctrl`]
module"]
pub type OSCCTRL = crate::Reg<oscctrl::OSCCTRL_SPEC>;
#[doc = "RTC Oscillator Control."]
pub mod oscctrl;
