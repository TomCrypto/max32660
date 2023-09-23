#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function Select."]
    pub en0: EN0,
    #[doc = "0x04 - Function Select Set."]
    pub en0_set: EN0_SET,
    #[doc = "0x08 - Function Select Clear."]
    pub en0_clr: EN0_CLR,
    #[doc = "0x0c - Output Enable."]
    pub out_en: OUT_EN,
    #[doc = "0x10 - Output Enable Set."]
    pub out_en_set: OUT_EN_SET,
    #[doc = "0x14 - Output Enable Clear."]
    pub out_en_clr: OUT_EN_CLR,
    #[doc = "0x18 - Output Level."]
    pub out: OUT,
    #[doc = "0x1c - Output Level Set."]
    pub out_set: OUT_SET,
    #[doc = "0x20 - Output Level Clear."]
    pub out_clr: OUT_CLR,
    #[doc = "0x24 - Input Level."]
    pub in_: IN,
    #[doc = "0x28 - Interrupt Mode."]
    pub int_mode: INT_MODE,
    #[doc = "0x2c - Interrupt Polarity."]
    pub int_pol: INT_POL,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - Interrupt Enable."]
    pub int_en: INT_EN,
    #[doc = "0x38 - Interrupt Enable Set."]
    pub int_en_set: INT_EN_SET,
    #[doc = "0x3c - Interrupt Enable Clear."]
    pub int_en_clr: INT_EN_CLR,
    #[doc = "0x40 - Interrupt Status."]
    pub int_stat: INT_STAT,
    _reserved16: [u8; 0x04],
    #[doc = "0x48 - Interrupt Clear."]
    pub int_clr: INT_CLR,
    #[doc = "0x4c - Wakeup Enable."]
    pub wake_en: WAKE_EN,
    #[doc = "0x50 - Wakeup Enable Set."]
    pub wake_en_set: WAKE_EN_SET,
    #[doc = "0x54 - Wakeup Enable Clear."]
    pub wake_en_clr: WAKE_EN_CLR,
    _reserved20: [u8; 0x04],
    #[doc = "0x5c - Interrupt Dual-Edge Mode Select."]
    pub int_dual_edge: INT_DUAL_EDGE,
    #[doc = "0x60 - Pad Configuration."]
    pub pad_cfg: PAD_CFG,
    _reserved22: [u8; 0x04],
    #[doc = "0x68 - Alternate Function Bit 1 Select."]
    pub en1: EN1,
    #[doc = "0x6c - Alternate Function Bit 1 Select Set."]
    pub en1_set: EN1_SET,
    #[doc = "0x70 - Alternate Function Bit 1 Select Clear."]
    pub en1_clr: EN1_CLR,
    #[doc = "0x74 - Alternate Function Bit 2 Select."]
    pub en2: EN2,
    #[doc = "0x78 - Alternate Function Bit 2 Select Set."]
    pub en2_set: EN2_SET,
    #[doc = "0x7c - Alternate Function Bit 2 Select Clear."]
    pub en2_clr: EN2_CLR,
    _reserved28: [u8; 0x28],
    #[doc = "0xa8 - Input Hysteresis Enable."]
    pub is: IS,
    #[doc = "0xac - Slew Rate."]
    pub sr: SR,
    #[doc = "0xb0 - Drive Strength Bit 0 Select."]
    pub ds0: DS0,
    #[doc = "0xb4 - Drive Strength Bit 1 Select."]
    pub ds1: DS1,
    #[doc = "0xb8 - Pull-up/Pull-down Select."]
    pub ps: PS,
}
#[doc = "EN0 (rw) register accessor: Function Select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en0`]
module"]
pub type EN0 = crate::Reg<en0::EN0_SPEC>;
#[doc = "Function Select."]
pub mod en0;
#[doc = "EN0_SET (w) register accessor: Function Select Set.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en0_set`]
module"]
pub type EN0_SET = crate::Reg<en0_set::EN0_SET_SPEC>;
#[doc = "Function Select Set."]
pub mod en0_set;
#[doc = "EN0_CLR (w) register accessor: Function Select Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en0_clr`]
module"]
pub type EN0_CLR = crate::Reg<en0_clr::EN0_CLR_SPEC>;
#[doc = "Function Select Clear."]
pub mod en0_clr;
#[doc = "OUT_EN (rw) register accessor: Output Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_en`]
module"]
pub type OUT_EN = crate::Reg<out_en::OUT_EN_SPEC>;
#[doc = "Output Enable."]
pub mod out_en;
#[doc = "OUT_EN_SET (w) register accessor: Output Enable Set.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_en_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_en_set`]
module"]
pub type OUT_EN_SET = crate::Reg<out_en_set::OUT_EN_SET_SPEC>;
#[doc = "Output Enable Set."]
pub mod out_en_set;
#[doc = "OUT_EN_CLR (w) register accessor: Output Enable Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_en_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_en_clr`]
module"]
pub type OUT_EN_CLR = crate::Reg<out_en_clr::OUT_EN_CLR_SPEC>;
#[doc = "Output Enable Clear."]
pub mod out_en_clr;
#[doc = "OUT (rw) register accessor: Output Level.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out`]
module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Output Level."]
pub mod out;
#[doc = "OUT_SET (w) register accessor: Output Level Set.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_set`]
module"]
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
#[doc = "Output Level Set."]
pub mod out_set;
#[doc = "OUT_CLR (w) register accessor: Output Level Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_clr`]
module"]
pub type OUT_CLR = crate::Reg<out_clr::OUT_CLR_SPEC>;
#[doc = "Output Level Clear."]
pub mod out_clr;
#[doc = "IN (r) register accessor: Input Level.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_`]
module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Input Level."]
pub mod in_;
#[doc = "INT_MODE (rw) register accessor: Interrupt Mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_mode`]
module"]
pub type INT_MODE = crate::Reg<int_mode::INT_MODE_SPEC>;
#[doc = "Interrupt Mode."]
pub mod int_mode;
#[doc = "INT_POL (rw) register accessor: Interrupt Polarity.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_pol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_pol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_pol`]
module"]
pub type INT_POL = crate::Reg<int_pol::INT_POL_SPEC>;
#[doc = "Interrupt Polarity."]
pub mod int_pol;
#[doc = "INT_EN (rw) register accessor: Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_en`]
module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interrupt Enable."]
pub mod int_en;
#[doc = "INT_EN_SET (w) register accessor: Interrupt Enable Set.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_en_set`]
module"]
pub type INT_EN_SET = crate::Reg<int_en_set::INT_EN_SET_SPEC>;
#[doc = "Interrupt Enable Set."]
pub mod int_en_set;
#[doc = "INT_EN_CLR (w) register accessor: Interrupt Enable Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_en_clr`]
module"]
pub type INT_EN_CLR = crate::Reg<int_en_clr::INT_EN_CLR_SPEC>;
#[doc = "Interrupt Enable Clear."]
pub mod int_en_clr;
#[doc = "INT_STAT (r) register accessor: Interrupt Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_stat`]
module"]
pub type INT_STAT = crate::Reg<int_stat::INT_STAT_SPEC>;
#[doc = "Interrupt Status."]
pub mod int_stat;
#[doc = "INT_CLR (w) register accessor: Interrupt Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`]
module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt Clear."]
pub mod int_clr;
#[doc = "WAKE_EN (rw) register accessor: Wakeup Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wake_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wake_en`]
module"]
pub type WAKE_EN = crate::Reg<wake_en::WAKE_EN_SPEC>;
#[doc = "Wakeup Enable."]
pub mod wake_en;
#[doc = "WAKE_EN_SET (w) register accessor: Wakeup Enable Set.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_en_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wake_en_set`]
module"]
pub type WAKE_EN_SET = crate::Reg<wake_en_set::WAKE_EN_SET_SPEC>;
#[doc = "Wakeup Enable Set."]
pub mod wake_en_set;
#[doc = "WAKE_EN_CLR (w) register accessor: Wakeup Enable Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_en_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wake_en_clr`]
module"]
pub type WAKE_EN_CLR = crate::Reg<wake_en_clr::WAKE_EN_CLR_SPEC>;
#[doc = "Wakeup Enable Clear."]
pub mod wake_en_clr;
#[doc = "INT_DUAL_EDGE (rw) register accessor: Interrupt Dual-Edge Mode Select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_dual_edge::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_dual_edge::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_dual_edge`]
module"]
pub type INT_DUAL_EDGE = crate::Reg<int_dual_edge::INT_DUAL_EDGE_SPEC>;
#[doc = "Interrupt Dual-Edge Mode Select."]
pub mod int_dual_edge;
#[doc = "PAD_CFG (rw) register accessor: Pad Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pad_cfg`]
module"]
pub type PAD_CFG = crate::Reg<pad_cfg::PAD_CFG_SPEC>;
#[doc = "Pad Configuration."]
pub mod pad_cfg;
#[doc = "EN1 (rw) register accessor: Alternate Function Bit 1 Select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en1`]
module"]
pub type EN1 = crate::Reg<en1::EN1_SPEC>;
#[doc = "Alternate Function Bit 1 Select."]
pub mod en1;
#[doc = "EN1_SET (w) register accessor: Alternate Function Bit 1 Select Set.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en1_set`]
module"]
pub type EN1_SET = crate::Reg<en1_set::EN1_SET_SPEC>;
#[doc = "Alternate Function Bit 1 Select Set."]
pub mod en1_set;
#[doc = "EN1_CLR (w) register accessor: Alternate Function Bit 1 Select Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en1_clr`]
module"]
pub type EN1_CLR = crate::Reg<en1_clr::EN1_CLR_SPEC>;
#[doc = "Alternate Function Bit 1 Select Clear."]
pub mod en1_clr;
#[doc = "EN2 (rw) register accessor: Alternate Function Bit 2 Select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en2`]
module"]
pub type EN2 = crate::Reg<en2::EN2_SPEC>;
#[doc = "Alternate Function Bit 2 Select."]
pub mod en2;
#[doc = "EN2_SET (w) register accessor: Alternate Function Bit 2 Select Set.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en2_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en2_set`]
module"]
pub type EN2_SET = crate::Reg<en2_set::EN2_SET_SPEC>;
#[doc = "Alternate Function Bit 2 Select Set."]
pub mod en2_set;
#[doc = "EN2_CLR (w) register accessor: Alternate Function Bit 2 Select Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en2_clr`]
module"]
pub type EN2_CLR = crate::Reg<en2_clr::EN2_CLR_SPEC>;
#[doc = "Alternate Function Bit 2 Select Clear."]
pub mod en2_clr;
#[doc = "IS (rw) register accessor: Input Hysteresis Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`is`]
module"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "Input Hysteresis Enable."]
pub mod is;
#[doc = "SR (rw) register accessor: Slew Rate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Slew Rate."]
pub mod sr;
#[doc = "DS0 (rw) register accessor: Drive Strength Bit 0 Select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ds0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ds0`]
module"]
pub type DS0 = crate::Reg<ds0::DS0_SPEC>;
#[doc = "Drive Strength Bit 0 Select."]
pub mod ds0;
#[doc = "DS1 (rw) register accessor: Drive Strength Bit 1 Select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ds1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ds1`]
module"]
pub type DS1 = crate::Reg<ds1::DS1_SPEC>;
#[doc = "Drive Strength Bit 1 Select."]
pub mod ds1;
#[doc = "PS (rw) register accessor: Pull-up/Pull-down Select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ps`]
module"]
pub type PS = crate::Reg<ps::PS_SPEC>;
#[doc = "Pull-up/Pull-down Select."]
pub mod ps;
