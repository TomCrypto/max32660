#[doc = "Register `LP_CTRL` reader"]
pub type R = crate::R<LP_CTRL_SPEC>;
#[doc = "Register `LP_CTRL` writer"]
pub type W = crate::W<LP_CTRL_SPEC>;
#[doc = "Field `RAMRET_SEL0` reader - System RAM 0 Data retention in BACKUP mode."]
pub type RAMRET_SEL0_R = crate::BitReader<RAMRET_SEL0_A>;
#[doc = "System RAM 0 Data retention in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRET_SEL0_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RAMRET_SEL0_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET_SEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRET_SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRET_SEL0_A {
        match self.bits {
            false => RAMRET_SEL0_A::DISABLED,
            true => RAMRET_SEL0_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAMRET_SEL0_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAMRET_SEL0_A::ENABLED
    }
}
#[doc = "Field `RAMRET_SEL0` writer - System RAM 0 Data retention in BACKUP mode."]
pub type RAMRET_SEL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAMRET_SEL0_A>;
impl<'a, REG, const O: u8> RAMRET_SEL0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET_SEL0_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET_SEL0_A::ENABLED)
    }
}
#[doc = "Field `RAMRET_SEL1` reader - System RAM 1 Data retention in BACKUP mode."]
pub type RAMRET_SEL1_R = crate::BitReader<RAMRET_SEL1_A>;
#[doc = "System RAM 1 Data retention in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRET_SEL1_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RAMRET_SEL1_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET_SEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRET_SEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRET_SEL1_A {
        match self.bits {
            false => RAMRET_SEL1_A::DISABLED,
            true => RAMRET_SEL1_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAMRET_SEL1_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAMRET_SEL1_A::ENABLED
    }
}
#[doc = "Field `RAMRET_SEL1` writer - System RAM 1 Data retention in BACKUP mode."]
pub type RAMRET_SEL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAMRET_SEL1_A>;
impl<'a, REG, const O: u8> RAMRET_SEL1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET_SEL1_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET_SEL1_A::ENABLED)
    }
}
#[doc = "Field `RAMRET_SEL2` reader - System RAM 2 Data retention in BACKUP mode."]
pub type RAMRET_SEL2_R = crate::BitReader<RAMRET_SEL2_A>;
#[doc = "System RAM 2 Data retention in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRET_SEL2_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RAMRET_SEL2_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET_SEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRET_SEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRET_SEL2_A {
        match self.bits {
            false => RAMRET_SEL2_A::DISABLED,
            true => RAMRET_SEL2_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAMRET_SEL2_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAMRET_SEL2_A::ENABLED
    }
}
#[doc = "Field `RAMRET_SEL2` writer - System RAM 2 Data retention in BACKUP mode."]
pub type RAMRET_SEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAMRET_SEL2_A>;
impl<'a, REG, const O: u8> RAMRET_SEL2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET_SEL2_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET_SEL2_A::ENABLED)
    }
}
#[doc = "Field `RAMRET_SEL3` reader - System RAM 3 Data retention in BACKUP mode."]
pub type RAMRET_SEL3_R = crate::BitReader<RAMRET_SEL3_A>;
#[doc = "System RAM 3 Data retention in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRET_SEL3_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RAMRET_SEL3_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET_SEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRET_SEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRET_SEL3_A {
        match self.bits {
            false => RAMRET_SEL3_A::DISABLED,
            true => RAMRET_SEL3_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAMRET_SEL3_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAMRET_SEL3_A::ENABLED
    }
}
#[doc = "Field `RAMRET_SEL3` writer - System RAM 3 Data retention in BACKUP mode."]
pub type RAMRET_SEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAMRET_SEL3_A>;
impl<'a, REG, const O: u8> RAMRET_SEL3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET_SEL3_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET_SEL3_A::ENABLED)
    }
}
#[doc = "Field `OVR` reader - Operating Voltage Range."]
pub type OVR_R = crate::FieldReader<OVR_A>;
#[doc = "Operating Voltage Range.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVR_A {
    #[doc = "0: 0."]
    _0_9V = 0,
    #[doc = "1: 1."]
    _1_0V = 1,
    #[doc = "2: 1."]
    _1_1V = 2,
}
impl From<OVR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVR_A {
    type Ux = u8;
}
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVR_A> {
        match self.bits {
            0 => Some(OVR_A::_0_9V),
            1 => Some(OVR_A::_1_0V),
            2 => Some(OVR_A::_1_1V),
            _ => None,
        }
    }
    #[doc = "0."]
    #[inline(always)]
    pub fn is_0_9v(&self) -> bool {
        *self == OVR_A::_0_9V
    }
    #[doc = "1."]
    #[inline(always)]
    pub fn is_1_0v(&self) -> bool {
        *self == OVR_A::_1_0V
    }
    #[doc = "1."]
    #[inline(always)]
    pub fn is_1_1v(&self) -> bool {
        *self == OVR_A::_1_1V
    }
}
#[doc = "Field `OVR` writer - Operating Voltage Range."]
pub type OVR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, OVR_A>;
impl<'a, REG, const O: u8> OVR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0."]
    #[inline(always)]
    pub fn _0_9v(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_A::_0_9V)
    }
    #[doc = "1."]
    #[inline(always)]
    pub fn _1_0v(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_A::_1_0V)
    }
    #[doc = "1."]
    #[inline(always)]
    pub fn _1_1v(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_A::_1_1V)
    }
}
#[doc = "Field `VCORE_DET_BYPASS` reader - Bypass V CORE External Supply Detection."]
pub type VCORE_DET_BYPASS_R = crate::BitReader<VCORE_DET_BYPASS_A>;
#[doc = "Bypass V CORE External Supply Detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCORE_DET_BYPASS_A {
    #[doc = "0: Enabled."]
    ENABLED = 0,
    #[doc = "1: Disabled."]
    DISABLED = 1,
}
impl From<VCORE_DET_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: VCORE_DET_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl VCORE_DET_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORE_DET_BYPASS_A {
        match self.bits {
            false => VCORE_DET_BYPASS_A::ENABLED,
            true => VCORE_DET_BYPASS_A::DISABLED,
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VCORE_DET_BYPASS_A::ENABLED
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VCORE_DET_BYPASS_A::DISABLED
    }
}
#[doc = "Field `VCORE_DET_BYPASS` writer - Bypass V CORE External Supply Detection."]
pub type VCORE_DET_BYPASS_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, VCORE_DET_BYPASS_A>;
impl<'a, REG, const O: u8> VCORE_DET_BYPASS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VCORE_DET_BYPASS_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VCORE_DET_BYPASS_A::DISABLED)
    }
}
#[doc = "Field `RETREG_EN` reader - Retention Regulator Enable."]
pub type RETREG_EN_R = crate::BitReader<RETREG_EN_A>;
#[doc = "Retention Regulator Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RETREG_EN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RETREG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RETREG_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RETREG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETREG_EN_A {
        match self.bits {
            false => RETREG_EN_A::DISABLED,
            true => RETREG_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RETREG_EN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RETREG_EN_A::ENABLED
    }
}
#[doc = "Field `RETREG_EN` writer - Retention Regulator Enable."]
pub type RETREG_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RETREG_EN_A>;
impl<'a, REG, const O: u8> RETREG_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RETREG_EN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RETREG_EN_A::ENABLED)
    }
}
#[doc = "Field `FAST_WK_EN` reader - Fast Wake-Up Mode."]
pub type FAST_WK_EN_R = crate::BitReader<FAST_WK_EN_A>;
#[doc = "Fast Wake-Up Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAST_WK_EN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<FAST_WK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_WK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FAST_WK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_WK_EN_A {
        match self.bits {
            false => FAST_WK_EN_A::DISABLED,
            true => FAST_WK_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAST_WK_EN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FAST_WK_EN_A::ENABLED
    }
}
#[doc = "Field `FAST_WK_EN` writer - Fast Wake-Up Mode."]
pub type FAST_WK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FAST_WK_EN_A>;
impl<'a, REG, const O: u8> FAST_WK_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAST_WK_EN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAST_WK_EN_A::ENABLED)
    }
}
#[doc = "Field `BG_OFF` reader - Band Gap Disable for DEEPSLEEP and BACKUP Mode."]
pub type BG_OFF_R = crate::BitReader<BG_OFF_A>;
#[doc = "Band Gap Disable for DEEPSLEEP and BACKUP Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BG_OFF_A {
    #[doc = "0: Bandgap is always ON."]
    ON = 0,
    #[doc = "1: Bandgap is OFF in DeepSleep mode(default)."]
    OFF = 1,
}
impl From<BG_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: BG_OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl BG_OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BG_OFF_A {
        match self.bits {
            false => BG_OFF_A::ON,
            true => BG_OFF_A::OFF,
        }
    }
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == BG_OFF_A::ON
    }
    #[doc = "Bandgap is OFF in DeepSleep mode(default)."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BG_OFF_A::OFF
    }
}
#[doc = "Field `BG_OFF` writer - Band Gap Disable for DEEPSLEEP and BACKUP Mode."]
pub type BG_OFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BG_OFF_A>;
impl<'a, REG, const O: u8> BG_OFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(BG_OFF_A::ON)
    }
    #[doc = "Bandgap is OFF in DeepSleep mode(default)."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(BG_OFF_A::OFF)
    }
}
#[doc = "Field `VCORE_POR_DIS` reader - V CORE POR Disable for DEEPSLEEP and BACKUP Mode."]
pub type VCORE_POR_DIS_R = crate::BitReader<VCORE_POR_DIS_A>;
#[doc = "V CORE POR Disable for DEEPSLEEP and BACKUP Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCORE_POR_DIS_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<VCORE_POR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: VCORE_POR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl VCORE_POR_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORE_POR_DIS_A {
        match self.bits {
            false => VCORE_POR_DIS_A::DISABLED,
            true => VCORE_POR_DIS_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VCORE_POR_DIS_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VCORE_POR_DIS_A::ENABLED
    }
}
#[doc = "Field `VCORE_POR_DIS` writer - V CORE POR Disable for DEEPSLEEP and BACKUP Mode."]
pub type VCORE_POR_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCORE_POR_DIS_A>;
impl<'a, REG, const O: u8> VCORE_POR_DIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VCORE_POR_DIS_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VCORE_POR_DIS_A::ENABLED)
    }
}
#[doc = "Field `LDO_DIS` reader - LDO Disable."]
pub type LDO_DIS_R = crate::BitReader<LDO_DIS_A>;
#[doc = "LDO Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDO_DIS_A {
    #[doc = "0: Enable if Bandgap is ON(default)."]
    ENABLED = 0,
    #[doc = "1: Disabled."]
    DISABLED = 1,
}
impl From<LDO_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: LDO_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl LDO_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDO_DIS_A {
        match self.bits {
            false => LDO_DIS_A::ENABLED,
            true => LDO_DIS_A::DISABLED,
        }
    }
    #[doc = "Enable if Bandgap is ON(default)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LDO_DIS_A::ENABLED
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LDO_DIS_A::DISABLED
    }
}
#[doc = "Field `LDO_DIS` writer - LDO Disable."]
pub type LDO_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LDO_DIS_A>;
impl<'a, REG, const O: u8> LDO_DIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable if Bandgap is ON(default)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LDO_DIS_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LDO_DIS_A::DISABLED)
    }
}
#[doc = "Field `VCORE_SVM_DIS` reader - V CORE Supply Voltage Monitor Disable."]
pub type VCORE_SVM_DIS_R = crate::BitReader<VCORE_SVM_DIS_A>;
#[doc = "V CORE Supply Voltage Monitor Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCORE_SVM_DIS_A {
    #[doc = "0: Enable if Bandgap is ON(default)."]
    ENABLED = 0,
    #[doc = "1: Disabled."]
    DISABLED = 1,
}
impl From<VCORE_SVM_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: VCORE_SVM_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl VCORE_SVM_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORE_SVM_DIS_A {
        match self.bits {
            false => VCORE_SVM_DIS_A::ENABLED,
            true => VCORE_SVM_DIS_A::DISABLED,
        }
    }
    #[doc = "Enable if Bandgap is ON(default)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VCORE_SVM_DIS_A::ENABLED
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VCORE_SVM_DIS_A::DISABLED
    }
}
#[doc = "Field `VCORE_SVM_DIS` writer - V CORE Supply Voltage Monitor Disable."]
pub type VCORE_SVM_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCORE_SVM_DIS_A>;
impl<'a, REG, const O: u8> VCORE_SVM_DIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable if Bandgap is ON(default)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VCORE_SVM_DIS_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VCORE_SVM_DIS_A::DISABLED)
    }
}
#[doc = "Field `VDDIO_POR_DIS` reader - VDDIO Power-On Reset Monitor Disable."]
pub type VDDIO_POR_DIS_R = crate::BitReader<VDDIO_POR_DIS_A>;
#[doc = "VDDIO Power-On Reset Monitor Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDIO_POR_DIS_A {
    #[doc = "0: Enabled."]
    ENABLED = 0,
    #[doc = "1: Disabled."]
    DISABLED = 1,
}
impl From<VDDIO_POR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: VDDIO_POR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDIO_POR_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDIO_POR_DIS_A {
        match self.bits {
            false => VDDIO_POR_DIS_A::ENABLED,
            true => VDDIO_POR_DIS_A::DISABLED,
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VDDIO_POR_DIS_A::ENABLED
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VDDIO_POR_DIS_A::DISABLED
    }
}
#[doc = "Field `VDDIO_POR_DIS` writer - VDDIO Power-On Reset Monitor Disable."]
pub type VDDIO_POR_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VDDIO_POR_DIS_A>;
impl<'a, REG, const O: u8> VDDIO_POR_DIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VDDIO_POR_DIS_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VDDIO_POR_DIS_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - System RAM 0 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel0(&self) -> RAMRET_SEL0_R {
        RAMRET_SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System RAM 1 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel1(&self) -> RAMRET_SEL1_R {
        RAMRET_SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System RAM 2 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel2(&self) -> RAMRET_SEL2_R {
        RAMRET_SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System RAM 3 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel3(&self) -> RAMRET_SEL3_R {
        RAMRET_SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Operating Voltage Range."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Bypass V CORE External Supply Detection."]
    #[inline(always)]
    pub fn vcore_det_bypass(&self) -> VCORE_DET_BYPASS_R {
        VCORE_DET_BYPASS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Retention Regulator Enable."]
    #[inline(always)]
    pub fn retreg_en(&self) -> RETREG_EN_R {
        RETREG_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Wake-Up Mode."]
    #[inline(always)]
    pub fn fast_wk_en(&self) -> FAST_WK_EN_R {
        FAST_WK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Band Gap Disable for DEEPSLEEP and BACKUP Mode."]
    #[inline(always)]
    pub fn bg_off(&self) -> BG_OFF_R {
        BG_OFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - V CORE POR Disable for DEEPSLEEP and BACKUP Mode."]
    #[inline(always)]
    pub fn vcore_por_dis(&self) -> VCORE_POR_DIS_R {
        VCORE_POR_DIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - LDO Disable."]
    #[inline(always)]
    pub fn ldo_dis(&self) -> LDO_DIS_R {
        LDO_DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - V CORE Supply Voltage Monitor Disable."]
    #[inline(always)]
    pub fn vcore_svm_dis(&self) -> VCORE_SVM_DIS_R {
        VCORE_SVM_DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - VDDIO Power-On Reset Monitor Disable."]
    #[inline(always)]
    pub fn vddio_por_dis(&self) -> VDDIO_POR_DIS_R {
        VDDIO_POR_DIS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM 0 Data retention in BACKUP mode."]
    #[inline(always)]
    #[must_use]
    pub fn ramret_sel0(&mut self) -> RAMRET_SEL0_W<LP_CTRL_SPEC, 0> {
        RAMRET_SEL0_W::new(self)
    }
    #[doc = "Bit 1 - System RAM 1 Data retention in BACKUP mode."]
    #[inline(always)]
    #[must_use]
    pub fn ramret_sel1(&mut self) -> RAMRET_SEL1_W<LP_CTRL_SPEC, 1> {
        RAMRET_SEL1_W::new(self)
    }
    #[doc = "Bit 2 - System RAM 2 Data retention in BACKUP mode."]
    #[inline(always)]
    #[must_use]
    pub fn ramret_sel2(&mut self) -> RAMRET_SEL2_W<LP_CTRL_SPEC, 2> {
        RAMRET_SEL2_W::new(self)
    }
    #[doc = "Bit 3 - System RAM 3 Data retention in BACKUP mode."]
    #[inline(always)]
    #[must_use]
    pub fn ramret_sel3(&mut self) -> RAMRET_SEL3_W<LP_CTRL_SPEC, 3> {
        RAMRET_SEL3_W::new(self)
    }
    #[doc = "Bits 4:5 - Operating Voltage Range."]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<LP_CTRL_SPEC, 4> {
        OVR_W::new(self)
    }
    #[doc = "Bit 6 - Bypass V CORE External Supply Detection."]
    #[inline(always)]
    #[must_use]
    pub fn vcore_det_bypass(&mut self) -> VCORE_DET_BYPASS_W<LP_CTRL_SPEC, 6> {
        VCORE_DET_BYPASS_W::new(self)
    }
    #[doc = "Bit 8 - Retention Regulator Enable."]
    #[inline(always)]
    #[must_use]
    pub fn retreg_en(&mut self) -> RETREG_EN_W<LP_CTRL_SPEC, 8> {
        RETREG_EN_W::new(self)
    }
    #[doc = "Bit 10 - Fast Wake-Up Mode."]
    #[inline(always)]
    #[must_use]
    pub fn fast_wk_en(&mut self) -> FAST_WK_EN_W<LP_CTRL_SPEC, 10> {
        FAST_WK_EN_W::new(self)
    }
    #[doc = "Bit 11 - Band Gap Disable for DEEPSLEEP and BACKUP Mode."]
    #[inline(always)]
    #[must_use]
    pub fn bg_off(&mut self) -> BG_OFF_W<LP_CTRL_SPEC, 11> {
        BG_OFF_W::new(self)
    }
    #[doc = "Bit 12 - V CORE POR Disable for DEEPSLEEP and BACKUP Mode."]
    #[inline(always)]
    #[must_use]
    pub fn vcore_por_dis(&mut self) -> VCORE_POR_DIS_W<LP_CTRL_SPEC, 12> {
        VCORE_POR_DIS_W::new(self)
    }
    #[doc = "Bit 16 - LDO Disable."]
    #[inline(always)]
    #[must_use]
    pub fn ldo_dis(&mut self) -> LDO_DIS_W<LP_CTRL_SPEC, 16> {
        LDO_DIS_W::new(self)
    }
    #[doc = "Bit 20 - V CORE Supply Voltage Monitor Disable."]
    #[inline(always)]
    #[must_use]
    pub fn vcore_svm_dis(&mut self) -> VCORE_SVM_DIS_W<LP_CTRL_SPEC, 20> {
        VCORE_SVM_DIS_W::new(self)
    }
    #[doc = "Bit 25 - VDDIO Power-On Reset Monitor Disable."]
    #[inline(always)]
    #[must_use]
    pub fn vddio_por_dis(&mut self) -> VDDIO_POR_DIS_W<LP_CTRL_SPEC, 25> {
        VDDIO_POR_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Low Power Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CTRL_SPEC;
impl crate::RegisterSpec for LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_CTRL to value 0"]
impl crate::Resettable for LP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
