#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<CTRL0_SPEC>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<CTRL0_SPEC>;
#[doc = "Field `I2CEN` reader - I2C Enable."]
pub type I2CEN_R = crate::BitReader<I2CEN_A>;
#[doc = "I2C Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2CEN_A {
    #[doc = "0: Disable I2C."]
    DISABLED = 0,
    #[doc = "1: enable I2C."]
    ENABLED = 1,
}
impl From<I2CEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2CEN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CEN_A {
        match self.bits {
            false => I2CEN_A::DISABLED,
            true => I2CEN_A::ENABLED,
        }
    }
    #[doc = "Disable I2C."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2CEN_A::DISABLED
    }
    #[doc = "enable I2C."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2CEN_A::ENABLED
    }
}
#[doc = "Field `I2CEN` writer - I2C Enable."]
pub type I2CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2CEN_A>;
impl<'a, REG, const O: u8> I2CEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable I2C."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2CEN_A::DISABLED)
    }
    #[doc = "enable I2C."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2CEN_A::ENABLED)
    }
}
#[doc = "Field `MST` reader - Master Mode Enable."]
pub type MST_R = crate::BitReader<MST_A>;
#[doc = "Master Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_A {
    #[doc = "0: Slave Mode."]
    SLAVE_MODE = 0,
    #[doc = "1: Master Mode."]
    MASTER_MODE = 1,
}
impl From<MST_A> for bool {
    #[inline(always)]
    fn from(variant: MST_A) -> Self {
        variant as u8 != 0
    }
}
impl MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_A {
        match self.bits {
            false => MST_A::SLAVE_MODE,
            true => MST_A::MASTER_MODE,
        }
    }
    #[doc = "Slave Mode."]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        *self == MST_A::SLAVE_MODE
    }
    #[doc = "Master Mode."]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        *self == MST_A::MASTER_MODE
    }
}
#[doc = "Field `MST` writer - Master Mode Enable."]
pub type MST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MST_A>;
impl<'a, REG, const O: u8> MST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave Mode."]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MST_A::SLAVE_MODE)
    }
    #[doc = "Master Mode."]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MST_A::MASTER_MODE)
    }
}
#[doc = "Field `GCEN` reader - General Call Address Enable."]
pub type GCEN_R = crate::BitReader<GCEN_A>;
#[doc = "General Call Address Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCEN_A {
    #[doc = "0: Ignore Gneral Call Address."]
    DISABLED = 0,
    #[doc = "1: Acknowledge general call address."]
    ENABLED = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::DISABLED,
            true => GCEN_A::ENABLED,
        }
    }
    #[doc = "Ignore Gneral Call Address."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCEN_A::DISABLED
    }
    #[doc = "Acknowledge general call address."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCEN_A::ENABLED
    }
}
#[doc = "Field `GCEN` writer - General Call Address Enable."]
pub type GCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GCEN_A>;
impl<'a, REG, const O: u8> GCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore Gneral Call Address."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN_A::DISABLED)
    }
    #[doc = "Acknowledge general call address."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN_A::ENABLED)
    }
}
#[doc = "Field `IRXM` reader - Interactive Receive Mode."]
pub type IRXM_R = crate::BitReader<IRXM_A>;
#[doc = "Interactive Receive Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRXM_A {
    #[doc = "0: Disable Interactive Receive Mode."]
    DISABLED = 0,
    #[doc = "1: Enable Interactive Receive Mode."]
    ENABLED = 1,
}
impl From<IRXM_A> for bool {
    #[inline(always)]
    fn from(variant: IRXM_A) -> Self {
        variant as u8 != 0
    }
}
impl IRXM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRXM_A {
        match self.bits {
            false => IRXM_A::DISABLED,
            true => IRXM_A::ENABLED,
        }
    }
    #[doc = "Disable Interactive Receive Mode."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRXM_A::DISABLED
    }
    #[doc = "Enable Interactive Receive Mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRXM_A::ENABLED
    }
}
#[doc = "Field `IRXM` writer - Interactive Receive Mode."]
pub type IRXM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRXM_A>;
impl<'a, REG, const O: u8> IRXM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Interactive Receive Mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRXM_A::DISABLED)
    }
    #[doc = "Enable Interactive Receive Mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRXM_A::ENABLED)
    }
}
#[doc = "Field `ACK` reader - Data Acknowledge."]
pub type ACK_R = crate::BitReader<ACK_A>;
#[doc = "Data Acknowledge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACK_A {
    #[doc = "0: return ACK (pulling SDA LOW)."]
    ACK = 0,
    #[doc = "1: return NACK (leaving SDA HIGH)."]
    NACK = 1,
}
impl From<ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_A {
        match self.bits {
            false => ACK_A::ACK,
            true => ACK_A::NACK,
        }
    }
    #[doc = "return ACK (pulling SDA LOW)."]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ACK_A::ACK
    }
    #[doc = "return NACK (leaving SDA HIGH)."]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == ACK_A::NACK
    }
}
#[doc = "Field `ACK` writer - Data Acknowledge."]
pub type ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ACK_A>;
impl<'a, REG, const O: u8> ACK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "return ACK (pulling SDA LOW)."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(ACK_A::ACK)
    }
    #[doc = "return NACK (leaving SDA HIGH)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(ACK_A::NACK)
    }
}
#[doc = "Field `SCLO` reader - SCL Output."]
pub type SCLO_R = crate::BitReader<SCLO_A>;
#[doc = "SCL Output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLO_A {
    #[doc = "0: Drive SCL low."]
    DRIVE_SCL_LOW = 0,
    #[doc = "1: Release SCL."]
    RELEASE_SCL = 1,
}
impl From<SCLO_A> for bool {
    #[inline(always)]
    fn from(variant: SCLO_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLO_A {
        match self.bits {
            false => SCLO_A::DRIVE_SCL_LOW,
            true => SCLO_A::RELEASE_SCL,
        }
    }
    #[doc = "Drive SCL low."]
    #[inline(always)]
    pub fn is_drive_scl_low(&self) -> bool {
        *self == SCLO_A::DRIVE_SCL_LOW
    }
    #[doc = "Release SCL."]
    #[inline(always)]
    pub fn is_release_scl(&self) -> bool {
        *self == SCLO_A::RELEASE_SCL
    }
}
#[doc = "Field `SCLO` writer - SCL Output."]
pub type SCLO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCLO_A>;
impl<'a, REG, const O: u8> SCLO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drive SCL low."]
    #[inline(always)]
    pub fn drive_scl_low(self) -> &'a mut crate::W<REG> {
        self.variant(SCLO_A::DRIVE_SCL_LOW)
    }
    #[doc = "Release SCL."]
    #[inline(always)]
    pub fn release_scl(self) -> &'a mut crate::W<REG> {
        self.variant(SCLO_A::RELEASE_SCL)
    }
}
#[doc = "Field `SDAO` reader - SDA Output."]
pub type SDAO_R = crate::BitReader<SDAO_A>;
#[doc = "SDA Output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDAO_A {
    #[doc = "0: Drive SDA low."]
    DRIVE_SDA_LOW = 0,
    #[doc = "1: Release SDA."]
    RELEASE_SDA = 1,
}
impl From<SDAO_A> for bool {
    #[inline(always)]
    fn from(variant: SDAO_A) -> Self {
        variant as u8 != 0
    }
}
impl SDAO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDAO_A {
        match self.bits {
            false => SDAO_A::DRIVE_SDA_LOW,
            true => SDAO_A::RELEASE_SDA,
        }
    }
    #[doc = "Drive SDA low."]
    #[inline(always)]
    pub fn is_drive_sda_low(&self) -> bool {
        *self == SDAO_A::DRIVE_SDA_LOW
    }
    #[doc = "Release SDA."]
    #[inline(always)]
    pub fn is_release_sda(&self) -> bool {
        *self == SDAO_A::RELEASE_SDA
    }
}
#[doc = "Field `SDAO` writer - SDA Output."]
pub type SDAO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SDAO_A>;
impl<'a, REG, const O: u8> SDAO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drive SDA low."]
    #[inline(always)]
    pub fn drive_sda_low(self) -> &'a mut crate::W<REG> {
        self.variant(SDAO_A::DRIVE_SDA_LOW)
    }
    #[doc = "Release SDA."]
    #[inline(always)]
    pub fn release_sda(self) -> &'a mut crate::W<REG> {
        self.variant(SDAO_A::RELEASE_SDA)
    }
}
#[doc = "Field `SCL` reader - SCL status."]
pub type SCL_R = crate::BitReader;
#[doc = "Field `SDA` reader - SDA status."]
pub type SDA_R = crate::BitReader;
#[doc = "Field `SWOE` reader - Software Output Enable."]
pub type SWOE_R = crate::BitReader<SWOE_A>;
#[doc = "Software Output Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWOE_A {
    #[doc = "0: I2C Outputs SCLO and SDAO disabled."]
    OUTPUTS_DISABLE = 0,
    #[doc = "1: I2C Outputs SCLO and SDAO enabled."]
    OUTPUTS_ENABLE = 1,
}
impl From<SWOE_A> for bool {
    #[inline(always)]
    fn from(variant: SWOE_A) -> Self {
        variant as u8 != 0
    }
}
impl SWOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWOE_A {
        match self.bits {
            false => SWOE_A::OUTPUTS_DISABLE,
            true => SWOE_A::OUTPUTS_ENABLE,
        }
    }
    #[doc = "I2C Outputs SCLO and SDAO disabled."]
    #[inline(always)]
    pub fn is_outputs_disable(&self) -> bool {
        *self == SWOE_A::OUTPUTS_DISABLE
    }
    #[doc = "I2C Outputs SCLO and SDAO enabled."]
    #[inline(always)]
    pub fn is_outputs_enable(&self) -> bool {
        *self == SWOE_A::OUTPUTS_ENABLE
    }
}
#[doc = "Field `SWOE` writer - Software Output Enable."]
pub type SWOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWOE_A>;
impl<'a, REG, const O: u8> SWOE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C Outputs SCLO and SDAO disabled."]
    #[inline(always)]
    pub fn outputs_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SWOE_A::OUTPUTS_DISABLE)
    }
    #[doc = "I2C Outputs SCLO and SDAO enabled."]
    #[inline(always)]
    pub fn outputs_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SWOE_A::OUTPUTS_ENABLE)
    }
}
#[doc = "Field `READ` reader - This bit reflects the R/W bit of an address match (AMI = 1) or general call match(GCI = 1)."]
pub type READ_R = crate::BitReader<READ_A>;
#[doc = "This bit reflects the R/W bit of an address match (AMI = 1) or general call match(GCI = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READ_A {
    #[doc = "0: Write."]
    WRITE = 0,
    #[doc = "1: Read."]
    READ = 1,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            false => READ_A::WRITE,
            true => READ_A::READ,
        }
    }
    #[doc = "Write."]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == READ_A::WRITE
    }
    #[doc = "Read."]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == READ_A::READ
    }
}
#[doc = "Field `SCL_STRD` reader - This bit will disable slave clock stretching when set."]
pub type SCL_STRD_R = crate::BitReader<SCL_STRD_A>;
#[doc = "This bit will disable slave clock stretching when set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCL_STRD_A {
    #[doc = "0: Slave clock stretching enabled."]
    ENABLED = 0,
    #[doc = "1: Slave clock stretching disabled."]
    DISABLED = 1,
}
impl From<SCL_STRD_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_STRD_A) -> Self {
        variant as u8 != 0
    }
}
impl SCL_STRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_STRD_A {
        match self.bits {
            false => SCL_STRD_A::ENABLED,
            true => SCL_STRD_A::DISABLED,
        }
    }
    #[doc = "Slave clock stretching enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCL_STRD_A::ENABLED
    }
    #[doc = "Slave clock stretching disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCL_STRD_A::DISABLED
    }
}
#[doc = "Field `SCL_STRD` writer - This bit will disable slave clock stretching when set."]
pub type SCL_STRD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCL_STRD_A>;
impl<'a, REG, const O: u8> SCL_STRD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave clock stretching enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCL_STRD_A::ENABLED)
    }
    #[doc = "Slave clock stretching disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCL_STRD_A::DISABLED)
    }
}
#[doc = "Field `SCL_PPM` reader - SCL Push-Pull Mode."]
pub type SCL_PPM_R = crate::BitReader<SCL_PPM_A>;
#[doc = "SCL Push-Pull Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCL_PPM_A {
    #[doc = "0: Standard open-drain operation: drive low for 0, Hi-Z for 1."]
    DISABLED = 0,
    #[doc = "1: Non-standard push-pull operation: drive low for 0, drive high for 1."]
    ENABLED = 1,
}
impl From<SCL_PPM_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_PPM_A) -> Self {
        variant as u8 != 0
    }
}
impl SCL_PPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_PPM_A {
        match self.bits {
            false => SCL_PPM_A::DISABLED,
            true => SCL_PPM_A::ENABLED,
        }
    }
    #[doc = "Standard open-drain operation: drive low for 0, Hi-Z for 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCL_PPM_A::DISABLED
    }
    #[doc = "Non-standard push-pull operation: drive low for 0, drive high for 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCL_PPM_A::ENABLED
    }
}
#[doc = "Field `SCL_PPM` writer - SCL Push-Pull Mode."]
pub type SCL_PPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCL_PPM_A>;
impl<'a, REG, const O: u8> SCL_PPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard open-drain operation: drive low for 0, Hi-Z for 1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCL_PPM_A::DISABLED)
    }
    #[doc = "Non-standard push-pull operation: drive low for 0, drive high for 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCL_PPM_A::ENABLED)
    }
}
#[doc = "Field `HSMODE` reader - Hs-mode Enable."]
pub type HSMODE_R = crate::BitReader<HSMODE_A>;
#[doc = "Hs-mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSMODE_A {
    #[doc = "0: Hs-mode disabled."]
    DISABLED = 0,
    #[doc = "1: Hs-mode enabled."]
    ENABLED = 1,
}
impl From<HSMODE_A> for bool {
    #[inline(always)]
    fn from(variant: HSMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSMODE_A {
        match self.bits {
            false => HSMODE_A::DISABLED,
            true => HSMODE_A::ENABLED,
        }
    }
    #[doc = "Hs-mode disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSMODE_A::DISABLED
    }
    #[doc = "Hs-mode enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSMODE_A::ENABLED
    }
}
#[doc = "Field `HSMODE` writer - Hs-mode Enable."]
pub type HSMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HSMODE_A>;
impl<'a, REG, const O: u8> HSMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hs-mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSMODE_A::DISABLED)
    }
    #[doc = "Hs-mode enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSMODE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - I2C Enable."]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - General Call Address Enable."]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interactive Receive Mode."]
    #[inline(always)]
    pub fn irxm(&self) -> IRXM_R {
        IRXM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Acknowledge."]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL Output."]
    #[inline(always)]
    pub fn sclo(&self) -> SCLO_R {
        SCLO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SDA Output."]
    #[inline(always)]
    pub fn sdao(&self) -> SDAO_R {
        SDAO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCL status."]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDA status."]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Output Enable."]
    #[inline(always)]
    pub fn swoe(&self) -> SWOE_R {
        SWOE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit reflects the R/W bit of an address match (AMI = 1) or general call match(GCI = 1)."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit will disable slave clock stretching when set."]
    #[inline(always)]
    pub fn scl_strd(&self) -> SCL_STRD_R {
        SCL_STRD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCL Push-Pull Mode."]
    #[inline(always)]
    pub fn scl_ppm(&self) -> SCL_PPM_R {
        SCL_PPM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Hs-mode Enable."]
    #[inline(always)]
    pub fn hsmode(&self) -> HSMODE_R {
        HSMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<CTRL0_SPEC, 0> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mst(&mut self) -> MST_W<CTRL0_SPEC, 1> {
        MST_W::new(self)
    }
    #[doc = "Bit 2 - General Call Address Enable."]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<CTRL0_SPEC, 2> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 3 - Interactive Receive Mode."]
    #[inline(always)]
    #[must_use]
    pub fn irxm(&mut self) -> IRXM_W<CTRL0_SPEC, 3> {
        IRXM_W::new(self)
    }
    #[doc = "Bit 4 - Data Acknowledge."]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<CTRL0_SPEC, 4> {
        ACK_W::new(self)
    }
    #[doc = "Bit 6 - SCL Output."]
    #[inline(always)]
    #[must_use]
    pub fn sclo(&mut self) -> SCLO_W<CTRL0_SPEC, 6> {
        SCLO_W::new(self)
    }
    #[doc = "Bit 7 - SDA Output."]
    #[inline(always)]
    #[must_use]
    pub fn sdao(&mut self) -> SDAO_W<CTRL0_SPEC, 7> {
        SDAO_W::new(self)
    }
    #[doc = "Bit 10 - Software Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn swoe(&mut self) -> SWOE_W<CTRL0_SPEC, 10> {
        SWOE_W::new(self)
    }
    #[doc = "Bit 12 - This bit will disable slave clock stretching when set."]
    #[inline(always)]
    #[must_use]
    pub fn scl_strd(&mut self) -> SCL_STRD_W<CTRL0_SPEC, 12> {
        SCL_STRD_W::new(self)
    }
    #[doc = "Bit 13 - SCL Push-Pull Mode."]
    #[inline(always)]
    #[must_use]
    pub fn scl_ppm(&mut self) -> SCL_PPM_W<CTRL0_SPEC, 13> {
        SCL_PPM_W::new(self)
    }
    #[doc = "Bit 15 - Hs-mode Enable."]
    #[inline(always)]
    #[must_use]
    pub fn hsmode(&mut self) -> HSMODE_W<CTRL0_SPEC, 15> {
        HSMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
