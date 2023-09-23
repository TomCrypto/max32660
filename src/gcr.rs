#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control."]
    pub scon: SCON,
    #[doc = "0x04 - Reset."]
    pub rst0: RST0,
    #[doc = "0x08 - Clock Control."]
    pub clk_ctrl: CLK_CTRL,
    #[doc = "0x0c - Power Management."]
    pub pm: PM,
    _reserved4: [u8; 0x14],
    #[doc = "0x24 - Peripheral Clock Disable."]
    pub pclk_dis0: PCLK_DIS0,
    #[doc = "0x28 - Memory Clock Control."]
    pub mem_ctrl: MEM_CTRL,
    #[doc = "0x2c - Memory Zeroize Control."]
    pub mem_zctrl: MEM_ZCTRL,
    _reserved7: [u8; 0x10],
    #[doc = "0x40 - System Status."]
    pub sys_stat: SYS_STAT,
    #[doc = "0x44 - Reset 1."]
    pub rst1: RST1,
    #[doc = "0x48 - Peripheral Clock Disable."]
    pub pclk_dis1: PCLK_DIS1,
    #[doc = "0x4c - Event Enable."]
    pub evten: EVTEN,
    #[doc = "0x50 - Revision."]
    pub rev: REV,
    #[doc = "0x54 - System Status Interrupt Enable."]
    pub sys_ie: SYS_IE,
}
#[doc = "SCON (rw) register accessor: System Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scon`]
module"]
pub type SCON = crate::Reg<scon::SCON_SPEC>;
#[doc = "System Control."]
pub mod scon;
#[doc = "RST0 (rw) register accessor: Reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rst0`]
module"]
pub type RST0 = crate::Reg<rst0::RST0_SPEC>;
#[doc = "Reset."]
pub mod rst0;
#[doc = "CLK_CTRL (rw) register accessor: Clock Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_ctrl`]
module"]
pub type CLK_CTRL = crate::Reg<clk_ctrl::CLK_CTRL_SPEC>;
#[doc = "Clock Control."]
pub mod clk_ctrl;
#[doc = "PM (rw) register accessor: Power Management.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pm`]
module"]
pub type PM = crate::Reg<pm::PM_SPEC>;
#[doc = "Power Management."]
pub mod pm;
#[doc = "PCLK_DIS0 (rw) register accessor: Peripheral Clock Disable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclk_dis0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclk_dis0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pclk_dis0`]
module"]
pub type PCLK_DIS0 = crate::Reg<pclk_dis0::PCLK_DIS0_SPEC>;
#[doc = "Peripheral Clock Disable."]
pub mod pclk_dis0;
#[doc = "MEM_CTRL (rw) register accessor: Memory Clock Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_ctrl`]
module"]
pub type MEM_CTRL = crate::Reg<mem_ctrl::MEM_CTRL_SPEC>;
#[doc = "Memory Clock Control."]
pub mod mem_ctrl;
#[doc = "MEM_ZCTRL (rw) register accessor: Memory Zeroize Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_zctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_zctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_zctrl`]
module"]
pub type MEM_ZCTRL = crate::Reg<mem_zctrl::MEM_ZCTRL_SPEC>;
#[doc = "Memory Zeroize Control."]
pub mod mem_zctrl;
#[doc = "SYS_STAT (rw) register accessor: System Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sys_stat`]
module"]
pub type SYS_STAT = crate::Reg<sys_stat::SYS_STAT_SPEC>;
#[doc = "System Status."]
pub mod sys_stat;
#[doc = "RST1 (rw) register accessor: Reset 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rst1`]
module"]
pub type RST1 = crate::Reg<rst1::RST1_SPEC>;
#[doc = "Reset 1."]
pub mod rst1;
#[doc = "PCLK_DIS1 (rw) register accessor: Peripheral Clock Disable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclk_dis1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclk_dis1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pclk_dis1`]
module"]
pub type PCLK_DIS1 = crate::Reg<pclk_dis1::PCLK_DIS1_SPEC>;
#[doc = "Peripheral Clock Disable."]
pub mod pclk_dis1;
#[doc = "EVTEN (rw) register accessor: Event Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`evten`]
module"]
pub type EVTEN = crate::Reg<evten::EVTEN_SPEC>;
#[doc = "Event Enable."]
pub mod evten;
#[doc = "REV (r) register accessor: Revision.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rev::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rev`]
module"]
pub type REV = crate::Reg<rev::REV_SPEC>;
#[doc = "Revision."]
pub mod rev;
#[doc = "SYS_IE (rw) register accessor: System Status Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sys_ie`]
module"]
pub type SYS_IE = crate::Reg<sys_ie::SYS_IE_SPEC>;
#[doc = "System Status Interrupt Enable."]
pub mod sys_ie;
