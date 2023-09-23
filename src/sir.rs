#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Initialization Status."]
    pub status: STATUS,
    #[doc = "0x04 - Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory."]
    pub addr: ADDR,
}
#[doc = "STATUS (r) register accessor: System Initialization Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "System Initialization Status."]
pub mod status;
#[doc = "ADDR (r) register accessor: Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory."]
pub mod addr;
