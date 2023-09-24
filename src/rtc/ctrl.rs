#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `RTCE` reader - Real Time Clock Enable."]
pub type RTCE_R = crate::BitReader<RTCE_A>;
#[doc = "Real Time Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCE_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RTCE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCE_A {
        match self.bits {
            false => RTCE_A::DISABLED,
            true => RTCE_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCE_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCE_A::ENABLED
    }
}
#[doc = "Field `RTCE` writer - Real Time Clock Enable."]
pub type RTCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTCE_A>;
impl<'a, REG, const O: u8> RTCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCE_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCE_A::ENABLED)
    }
}
#[doc = "Field `ADE` reader - Alarm Time-of-Day Interrupt Enable."]
pub type ADE_R = crate::BitReader<ADE_A>;
#[doc = "Alarm Time-of-Day Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADE_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<ADE_A> for bool {
    #[inline(always)]
    fn from(variant: ADE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADE_A {
        match self.bits {
            false => ADE_A::DISABLED,
            true => ADE_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADE_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADE_A::ENABLED
    }
}
#[doc = "Field `ADE` writer - Alarm Time-of-Day Interrupt Enable."]
pub type ADE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADE_A>;
impl<'a, REG, const O: u8> ADE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADE_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADE_A::ENABLED)
    }
}
#[doc = "Field `ASE` reader - Alarm Sub-second Interrupt Enable."]
pub type ASE_R = crate::BitReader<ASE_A>;
#[doc = "Alarm Sub-second Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASE_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<ASE_A> for bool {
    #[inline(always)]
    fn from(variant: ASE_A) -> Self {
        variant as u8 != 0
    }
}
impl ASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASE_A {
        match self.bits {
            false => ASE_A::DISABLED,
            true => ASE_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASE_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASE_A::ENABLED
    }
}
#[doc = "Field `ASE` writer - Alarm Sub-second Interrupt Enable."]
pub type ASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ASE_A>;
impl<'a, REG, const O: u8> ASE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASE_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASE_A::ENABLED)
    }
}
#[doc = "Field `BUSY` reader - RTC Busy."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "RTC Busy.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Idle."]
    IDLE = 0,
    #[doc = "1: Busy."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `RDY` reader - RTC Ready."]
pub type RDY_R = crate::BitReader<RDY_A>;
#[doc = "RTC Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY_A {
    #[doc = "0: Register has not updated."]
    BUSY = 0,
    #[doc = "1: Ready."]
    READY = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::BUSY,
            true => RDY_A::READY,
        }
    }
    #[doc = "Register has not updated."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RDY_A::BUSY
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RDY_A::READY
    }
}
#[doc = "Field `RDY` writer - RTC Ready."]
pub type RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RDY_A>;
impl<'a, REG, const O: u8> RDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Register has not updated."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RDY_A::BUSY)
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(RDY_A::READY)
    }
}
#[doc = "Field `RDYE` reader - RTC Ready Interrupt Enable."]
pub type RDYE_R = crate::BitReader<RDYE_A>;
#[doc = "RTC Ready Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDYE_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RDYE_A> for bool {
    #[inline(always)]
    fn from(variant: RDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDYE_A {
        match self.bits {
            false => RDYE_A::DISABLED,
            true => RDYE_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDYE_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDYE_A::ENABLED
    }
}
#[doc = "Field `RDYE` writer - RTC Ready Interrupt Enable."]
pub type RDYE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RDYE_A>;
impl<'a, REG, const O: u8> RDYE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDYE_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDYE_A::ENABLED)
    }
}
#[doc = "Field `ALDF` reader - Time-of-Day Alarm Interrupt Flag."]
pub type ALDF_R = crate::BitReader<ALDF_A>;
#[doc = "Time-of-Day Alarm Interrupt Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALDF_A {
    #[doc = "0: Not active."]
    INACTIVE = 0,
    #[doc = "1: Active."]
    PENDING = 1,
}
impl From<ALDF_A> for bool {
    #[inline(always)]
    fn from(variant: ALDF_A) -> Self {
        variant as u8 != 0
    }
}
impl ALDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALDF_A {
        match self.bits {
            false => ALDF_A::INACTIVE,
            true => ALDF_A::PENDING,
        }
    }
    #[doc = "Not active."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ALDF_A::INACTIVE
    }
    #[doc = "Active."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ALDF_A::PENDING
    }
}
#[doc = "Field `ALDF` writer - Time-of-Day Alarm Interrupt Flag."]
pub type ALDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ALDF_A>;
impl<'a, REG, const O: u8> ALDF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not active."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ALDF_A::INACTIVE)
    }
    #[doc = "Active."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ALDF_A::PENDING)
    }
}
#[doc = "Field `ALSF` reader - Sub-second Alarm Interrupt Flag."]
pub type ALSF_R = crate::BitReader<ALSF_A>;
#[doc = "Sub-second Alarm Interrupt Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALSF_A {
    #[doc = "0: Not active."]
    INACTIVE = 0,
    #[doc = "1: Active."]
    PENDING = 1,
}
impl From<ALSF_A> for bool {
    #[inline(always)]
    fn from(variant: ALSF_A) -> Self {
        variant as u8 != 0
    }
}
impl ALSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALSF_A {
        match self.bits {
            false => ALSF_A::INACTIVE,
            true => ALSF_A::PENDING,
        }
    }
    #[doc = "Not active."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ALSF_A::INACTIVE
    }
    #[doc = "Active."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ALSF_A::PENDING
    }
}
#[doc = "Field `ALSF` writer - Sub-second Alarm Interrupt Flag."]
pub type ALSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ALSF_A>;
impl<'a, REG, const O: u8> ALSF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not active."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ALSF_A::INACTIVE)
    }
    #[doc = "Active."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ALSF_A::PENDING)
    }
}
#[doc = "Field `SQE` reader - Square Wave Output Enable."]
pub type SQE_R = crate::BitReader<SQE_A>;
#[doc = "Square Wave Output Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQE_A {
    #[doc = "0: Not active."]
    INACTIVE = 0,
    #[doc = "1: Active."]
    PENDING = 1,
}
impl From<SQE_A> for bool {
    #[inline(always)]
    fn from(variant: SQE_A) -> Self {
        variant as u8 != 0
    }
}
impl SQE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQE_A {
        match self.bits {
            false => SQE_A::INACTIVE,
            true => SQE_A::PENDING,
        }
    }
    #[doc = "Not active."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SQE_A::INACTIVE
    }
    #[doc = "Active."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SQE_A::PENDING
    }
}
#[doc = "Field `SQE` writer - Square Wave Output Enable."]
pub type SQE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SQE_A>;
impl<'a, REG, const O: u8> SQE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not active."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(SQE_A::INACTIVE)
    }
    #[doc = "Active."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(SQE_A::PENDING)
    }
}
#[doc = "Field `FT` reader - Frequency Output Selection."]
pub type FT_R = crate::FieldReader<FT_A>;
#[doc = "Frequency Output Selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FT_A {
    #[doc = "0: Compensated 1 Hz."]
    FREQ_1_HZ = 0,
    #[doc = "1: Compensated 512 Hz."]
    FREQ_512_HZ = 1,
    #[doc = "2: 4 KHz."]
    FREQ_4_KHZ = 2,
    #[doc = "3: RTC Input Clock divided by 8."]
    CLK_DIV_8 = 3,
}
impl From<FT_A> for u8 {
    #[inline(always)]
    fn from(variant: FT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FT_A {
    type Ux = u8;
}
impl FT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT_A {
        match self.bits {
            0 => FT_A::FREQ_1_HZ,
            1 => FT_A::FREQ_512_HZ,
            2 => FT_A::FREQ_4_KHZ,
            3 => FT_A::CLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Compensated 1 Hz."]
    #[inline(always)]
    pub fn is_freq_1_hz(&self) -> bool {
        *self == FT_A::FREQ_1_HZ
    }
    #[doc = "Compensated 512 Hz."]
    #[inline(always)]
    pub fn is_freq_512_hz(&self) -> bool {
        *self == FT_A::FREQ_512_HZ
    }
    #[doc = "4 KHz."]
    #[inline(always)]
    pub fn is_freq_4_khz(&self) -> bool {
        *self == FT_A::FREQ_4_KHZ
    }
    #[doc = "RTC Input Clock divided by 8."]
    #[inline(always)]
    pub fn is_clk_div_8(&self) -> bool {
        *self == FT_A::CLK_DIV_8
    }
}
#[doc = "Field `FT` writer - Frequency Output Selection."]
pub type FT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, FT_A>;
impl<'a, REG, const O: u8> FT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compensated 1 Hz."]
    #[inline(always)]
    pub fn freq_1_hz(self) -> &'a mut crate::W<REG> {
        self.variant(FT_A::FREQ_1_HZ)
    }
    #[doc = "Compensated 512 Hz."]
    #[inline(always)]
    pub fn freq_512_hz(self) -> &'a mut crate::W<REG> {
        self.variant(FT_A::FREQ_512_HZ)
    }
    #[doc = "4 KHz."]
    #[inline(always)]
    pub fn freq_4_khz(self) -> &'a mut crate::W<REG> {
        self.variant(FT_A::FREQ_4_KHZ)
    }
    #[doc = "RTC Input Clock divided by 8."]
    #[inline(always)]
    pub fn clk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(FT_A::CLK_DIV_8)
    }
}
#[doc = "Field `X32KMD` reader - 32KHz Oscillator Mode."]
pub type X32KMD_R = crate::FieldReader<X32KMD_A>;
#[doc = "32KHz Oscillator Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum X32KMD_A {
    #[doc = "0: Always operate in Noise Immune Mode."]
    NOISE_IMMUNE = 0,
    #[doc = "1: Always operate in Quiet Mode."]
    QUIET = 1,
    #[doc = "2: Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry."]
    QUIET_IN_STOP_WITH_WARMUP = 2,
    #[doc = "3: Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry."]
    QUIET_IN_STOP_NO_WARMUP = 3,
}
impl From<X32KMD_A> for u8 {
    #[inline(always)]
    fn from(variant: X32KMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for X32KMD_A {
    type Ux = u8;
}
impl X32KMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X32KMD_A {
        match self.bits {
            0 => X32KMD_A::NOISE_IMMUNE,
            1 => X32KMD_A::QUIET,
            2 => X32KMD_A::QUIET_IN_STOP_WITH_WARMUP,
            3 => X32KMD_A::QUIET_IN_STOP_NO_WARMUP,
            _ => unreachable!(),
        }
    }
    #[doc = "Always operate in Noise Immune Mode."]
    #[inline(always)]
    pub fn is_noise_immune(&self) -> bool {
        *self == X32KMD_A::NOISE_IMMUNE
    }
    #[doc = "Always operate in Quiet Mode."]
    #[inline(always)]
    pub fn is_quiet(&self) -> bool {
        *self == X32KMD_A::QUIET
    }
    #[doc = "Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry."]
    #[inline(always)]
    pub fn is_quiet_in_stop_with_warmup(&self) -> bool {
        *self == X32KMD_A::QUIET_IN_STOP_WITH_WARMUP
    }
    #[doc = "Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry."]
    #[inline(always)]
    pub fn is_quiet_in_stop_no_warmup(&self) -> bool {
        *self == X32KMD_A::QUIET_IN_STOP_NO_WARMUP
    }
}
#[doc = "Field `X32KMD` writer - 32KHz Oscillator Mode."]
pub type X32KMD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, X32KMD_A>;
impl<'a, REG, const O: u8> X32KMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Always operate in Noise Immune Mode."]
    #[inline(always)]
    pub fn noise_immune(self) -> &'a mut crate::W<REG> {
        self.variant(X32KMD_A::NOISE_IMMUNE)
    }
    #[doc = "Always operate in Quiet Mode."]
    #[inline(always)]
    pub fn quiet(self) -> &'a mut crate::W<REG> {
        self.variant(X32KMD_A::QUIET)
    }
    #[doc = "Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry."]
    #[inline(always)]
    pub fn quiet_in_stop_with_warmup(self) -> &'a mut crate::W<REG> {
        self.variant(X32KMD_A::QUIET_IN_STOP_WITH_WARMUP)
    }
    #[doc = "Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry."]
    #[inline(always)]
    pub fn quiet_in_stop_no_warmup(self) -> &'a mut crate::W<REG> {
        self.variant(X32KMD_A::QUIET_IN_STOP_NO_WARMUP)
    }
}
#[doc = "Field `WE` reader - Write Enable."]
pub type WE_R = crate::BitReader<WE_A>;
#[doc = "Write Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_A {
    #[doc = "0: Not active."]
    INACTIVE = 0,
    #[doc = "1: Active."]
    PENDING = 1,
}
impl From<WE_A> for bool {
    #[inline(always)]
    fn from(variant: WE_A) -> Self {
        variant as u8 != 0
    }
}
impl WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WE_A {
        match self.bits {
            false => WE_A::INACTIVE,
            true => WE_A::PENDING,
        }
    }
    #[doc = "Not active."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == WE_A::INACTIVE
    }
    #[doc = "Active."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == WE_A::PENDING
    }
}
#[doc = "Field `WE` writer - Write Enable."]
pub type WE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WE_A>;
impl<'a, REG, const O: u8> WE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not active."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(WE_A::INACTIVE)
    }
    #[doc = "Active."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(WE_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Real Time Clock Enable."]
    #[inline(always)]
    pub fn rtce(&self) -> RTCE_R {
        RTCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Time-of-Day Interrupt Enable."]
    #[inline(always)]
    pub fn ade(&self) -> ADE_R {
        ADE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alarm Sub-second Interrupt Enable."]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Busy."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Ready."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Ready Interrupt Enable."]
    #[inline(always)]
    pub fn rdye(&self) -> RDYE_R {
        RDYE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Time-of-Day Alarm Interrupt Flag."]
    #[inline(always)]
    pub fn aldf(&self) -> ALDF_R {
        ALDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sub-second Alarm Interrupt Flag."]
    #[inline(always)]
    pub fn alsf(&self) -> ALSF_R {
        ALSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqe(&self) -> SQE_R {
        SQE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Frequency Output Selection."]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - 32KHz Oscillator Mode."]
    #[inline(always)]
    pub fn x32kmd(&self) -> X32KMD_R {
        X32KMD_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 15 - Write Enable."]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rtce(&mut self) -> RTCE_W<CTRL_SPEC, 0> {
        RTCE_W::new(self)
    }
    #[doc = "Bit 1 - Alarm Time-of-Day Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ade(&mut self) -> ADE_W<CTRL_SPEC, 1> {
        ADE_W::new(self)
    }
    #[doc = "Bit 2 - Alarm Sub-second Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ase(&mut self) -> ASE_W<CTRL_SPEC, 2> {
        ASE_W::new(self)
    }
    #[doc = "Bit 4 - RTC Ready."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<CTRL_SPEC, 4> {
        RDY_W::new(self)
    }
    #[doc = "Bit 5 - RTC Ready Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdye(&mut self) -> RDYE_W<CTRL_SPEC, 5> {
        RDYE_W::new(self)
    }
    #[doc = "Bit 6 - Time-of-Day Alarm Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn aldf(&mut self) -> ALDF_W<CTRL_SPEC, 6> {
        ALDF_W::new(self)
    }
    #[doc = "Bit 7 - Sub-second Alarm Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn alsf(&mut self) -> ALSF_W<CTRL_SPEC, 7> {
        ALSF_W::new(self)
    }
    #[doc = "Bit 8 - Square Wave Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn sqe(&mut self) -> SQE_W<CTRL_SPEC, 8> {
        SQE_W::new(self)
    }
    #[doc = "Bits 9:10 - Frequency Output Selection."]
    #[inline(always)]
    #[must_use]
    pub fn ft(&mut self) -> FT_W<CTRL_SPEC, 9> {
        FT_W::new(self)
    }
    #[doc = "Bits 11:12 - 32KHz Oscillator Mode."]
    #[inline(always)]
    #[must_use]
    pub fn x32kmd(&mut self) -> X32KMD_W<CTRL_SPEC, 11> {
        X32KMD_W::new(self)
    }
    #[doc = "Bit 15 - Write Enable."]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WE_W<CTRL_SPEC, 15> {
        WE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x08"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
