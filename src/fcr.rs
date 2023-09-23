#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Register 0."]
    pub reg0: REG0,
}
#[doc = "REG0 (rw) register accessor: Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reg0`]
module"]
pub type REG0 = crate::Reg<reg0::REG0_SPEC>;
#[doc = "Register 0."]
pub mod reg0;
