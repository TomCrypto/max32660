#[doc = "Register `INTFL0` reader"]
pub type R = crate::R<INTFL0_SPEC>;
#[doc = "Register `INTFL0` writer"]
pub type W = crate::W<INTFL0_SPEC>;
#[doc = "Field `DONEI` reader - Transfer Done Interrupt."]
pub type DONEI_R = crate::BitReader<DONEI_A>;
#[doc = "Transfer Done Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONEI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DONEI_A> for bool {
    #[inline(always)]
    fn from(variant: DONEI_A) -> Self {
        variant as u8 != 0
    }
}
impl DONEI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONEI_A {
        match self.bits {
            false => DONEI_A::INACTIVE,
            true => DONEI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DONEI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DONEI_A::PENDING
    }
}
#[doc = "Field `DONEI` writer - Transfer Done Interrupt."]
pub type DONEI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DONEI_A>;
impl<'a, REG, const O: u8> DONEI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DONEI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DONEI_A::PENDING)
    }
}
#[doc = "Field `IRXMI` reader - Interactive Receive Interrupt."]
pub type IRXMI_R = crate::BitReader<IRXMI_A>;
#[doc = "Interactive Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRXMI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IRXMI_A> for bool {
    #[inline(always)]
    fn from(variant: IRXMI_A) -> Self {
        variant as u8 != 0
    }
}
impl IRXMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRXMI_A {
        match self.bits {
            false => IRXMI_A::INACTIVE,
            true => IRXMI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IRXMI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IRXMI_A::PENDING
    }
}
#[doc = "Field `IRXMI` writer - Interactive Receive Interrupt."]
pub type IRXMI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRXMI_A>;
impl<'a, REG, const O: u8> IRXMI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IRXMI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(IRXMI_A::PENDING)
    }
}
#[doc = "Field `GCI` reader - Slave General Call Address Match Interrupt."]
pub type GCI_R = crate::BitReader<GCI_A>;
#[doc = "Slave General Call Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<GCI_A> for bool {
    #[inline(always)]
    fn from(variant: GCI_A) -> Self {
        variant as u8 != 0
    }
}
impl GCI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCI_A {
        match self.bits {
            false => GCI_A::INACTIVE,
            true => GCI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == GCI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == GCI_A::PENDING
    }
}
#[doc = "Field `GCI` writer - Slave General Call Address Match Interrupt."]
pub type GCI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GCI_A>;
impl<'a, REG, const O: u8> GCI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(GCI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(GCI_A::PENDING)
    }
}
#[doc = "Field `AMI` reader - Slave Address Match Interrupt."]
pub type AMI_R = crate::BitReader<AMI_A>;
#[doc = "Slave Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<AMI_A> for bool {
    #[inline(always)]
    fn from(variant: AMI_A) -> Self {
        variant as u8 != 0
    }
}
impl AMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMI_A {
        match self.bits {
            false => AMI_A::INACTIVE,
            true => AMI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == AMI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == AMI_A::PENDING
    }
}
#[doc = "Field `AMI` writer - Slave Address Match Interrupt."]
pub type AMI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMI_A>;
impl<'a, REG, const O: u8> AMI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(AMI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(AMI_A::PENDING)
    }
}
#[doc = "Field `RXTHI` reader - Receive Threshold Interrupt."]
pub type RXTHI_R = crate::BitReader<RXTHI_A>;
#[doc = "Receive Threshold Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXTHI_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<RXTHI_A> for bool {
    #[inline(always)]
    fn from(variant: RXTHI_A) -> Self {
        variant as u8 != 0
    }
}
impl RXTHI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTHI_A {
        match self.bits {
            false => RXTHI_A::INACTIVE,
            true => RXTHI_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RXTHI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXTHI_A::PENDING
    }
}
#[doc = "Field `RXTHI` writer - Receive Threshold Interrupt."]
pub type RXTHI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXTHI_A>;
impl<'a, REG, const O: u8> RXTHI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(RXTHI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RXTHI_A::PENDING)
    }
}
#[doc = "Field `TXTHI` reader - Transmit Threshold Interrupt."]
pub type TXTHI_R = crate::BitReader<TXTHI_A>;
#[doc = "Transmit Threshold Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTHI_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<TXTHI_A> for bool {
    #[inline(always)]
    fn from(variant: TXTHI_A) -> Self {
        variant as u8 != 0
    }
}
impl TXTHI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTHI_A {
        match self.bits {
            false => TXTHI_A::INACTIVE,
            true => TXTHI_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TXTHI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TXTHI_A::PENDING
    }
}
#[doc = "Field `TXTHI` writer - Transmit Threshold Interrupt."]
pub type TXTHI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXTHI_A>;
impl<'a, REG, const O: u8> TXTHI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(TXTHI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TXTHI_A::PENDING)
    }
}
#[doc = "Field `STOPI` reader - STOP Interrupt."]
pub type STOPI_R = crate::BitReader<STOPI_A>;
#[doc = "STOP Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPI_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<STOPI_A> for bool {
    #[inline(always)]
    fn from(variant: STOPI_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPI_A {
        match self.bits {
            false => STOPI_A::INACTIVE,
            true => STOPI_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == STOPI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STOPI_A::PENDING
    }
}
#[doc = "Field `STOPI` writer - STOP Interrupt."]
pub type STOPI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STOPI_A>;
impl<'a, REG, const O: u8> STOPI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(STOPI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(STOPI_A::PENDING)
    }
}
#[doc = "Field `ADRACKI` reader - Address Acknowledge Interrupt."]
pub type ADRACKI_R = crate::BitReader<ADRACKI_A>;
#[doc = "Address Acknowledge Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRACKI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ADRACKI_A> for bool {
    #[inline(always)]
    fn from(variant: ADRACKI_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRACKI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRACKI_A {
        match self.bits {
            false => ADRACKI_A::INACTIVE,
            true => ADRACKI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ADRACKI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADRACKI_A::PENDING
    }
}
#[doc = "Field `ADRACKI` writer - Address Acknowledge Interrupt."]
pub type ADRACKI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADRACKI_A>;
impl<'a, REG, const O: u8> ADRACKI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ADRACKI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADRACKI_A::PENDING)
    }
}
#[doc = "Field `ARBERI` reader - Arbritation error Interrupt."]
pub type ARBERI_R = crate::BitReader<ARBERI_A>;
#[doc = "Arbritation error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBERI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ARBERI_A> for bool {
    #[inline(always)]
    fn from(variant: ARBERI_A) -> Self {
        variant as u8 != 0
    }
}
impl ARBERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBERI_A {
        match self.bits {
            false => ARBERI_A::INACTIVE,
            true => ARBERI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ARBERI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ARBERI_A::PENDING
    }
}
#[doc = "Field `ARBERI` writer - Arbritation error Interrupt."]
pub type ARBERI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ARBERI_A>;
impl<'a, REG, const O: u8> ARBERI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ARBERI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ARBERI_A::PENDING)
    }
}
#[doc = "Field `TOERI` reader - timeout Error Interrupt."]
pub type TOERI_R = crate::BitReader<TOERI_A>;
#[doc = "timeout Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOERI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<TOERI_A> for bool {
    #[inline(always)]
    fn from(variant: TOERI_A) -> Self {
        variant as u8 != 0
    }
}
impl TOERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOERI_A {
        match self.bits {
            false => TOERI_A::INACTIVE,
            true => TOERI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TOERI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TOERI_A::PENDING
    }
}
#[doc = "Field `TOERI` writer - timeout Error Interrupt."]
pub type TOERI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TOERI_A>;
impl<'a, REG, const O: u8> TOERI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(TOERI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TOERI_A::PENDING)
    }
}
#[doc = "Field `ADRERI` reader - Address NACK Error Interrupt."]
pub type ADRERI_R = crate::BitReader<ADRERI_A>;
#[doc = "Address NACK Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRERI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ADRERI_A> for bool {
    #[inline(always)]
    fn from(variant: ADRERI_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRERI_A {
        match self.bits {
            false => ADRERI_A::INACTIVE,
            true => ADRERI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ADRERI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADRERI_A::PENDING
    }
}
#[doc = "Field `ADRERI` writer - Address NACK Error Interrupt."]
pub type ADRERI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADRERI_A>;
impl<'a, REG, const O: u8> ADRERI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ADRERI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADRERI_A::PENDING)
    }
}
#[doc = "Field `DATERI` reader - Data NACK Error Interrupt."]
pub type DATERI_R = crate::BitReader<DATERI_A>;
#[doc = "Data NACK Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATERI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DATERI_A> for bool {
    #[inline(always)]
    fn from(variant: DATERI_A) -> Self {
        variant as u8 != 0
    }
}
impl DATERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATERI_A {
        match self.bits {
            false => DATERI_A::INACTIVE,
            true => DATERI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DATERI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DATERI_A::PENDING
    }
}
#[doc = "Field `DATERI` writer - Data NACK Error Interrupt."]
pub type DATERI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DATERI_A>;
impl<'a, REG, const O: u8> DATERI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DATERI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DATERI_A::PENDING)
    }
}
#[doc = "Field `DNRERI` reader - Do Not Respond Error Interrupt."]
pub type DNRERI_R = crate::BitReader<DNRERI_A>;
#[doc = "Do Not Respond Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNRERI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DNRERI_A> for bool {
    #[inline(always)]
    fn from(variant: DNRERI_A) -> Self {
        variant as u8 != 0
    }
}
impl DNRERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNRERI_A {
        match self.bits {
            false => DNRERI_A::INACTIVE,
            true => DNRERI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DNRERI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DNRERI_A::PENDING
    }
}
#[doc = "Field `DNRERI` writer - Do Not Respond Error Interrupt."]
pub type DNRERI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DNRERI_A>;
impl<'a, REG, const O: u8> DNRERI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DNRERI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DNRERI_A::PENDING)
    }
}
#[doc = "Field `STRTERI` reader - Start Error Interrupt."]
pub type STRTERI_R = crate::BitReader<STRTERI_A>;
#[doc = "Start Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTERI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<STRTERI_A> for bool {
    #[inline(always)]
    fn from(variant: STRTERI_A) -> Self {
        variant as u8 != 0
    }
}
impl STRTERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRTERI_A {
        match self.bits {
            false => STRTERI_A::INACTIVE,
            true => STRTERI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == STRTERI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STRTERI_A::PENDING
    }
}
#[doc = "Field `STRTERI` writer - Start Error Interrupt."]
pub type STRTERI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STRTERI_A>;
impl<'a, REG, const O: u8> STRTERI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(STRTERI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(STRTERI_A::PENDING)
    }
}
#[doc = "Field `STOPERI` reader - Stop Error Interrupt."]
pub type STOPERI_R = crate::BitReader<STOPERI_A>;
#[doc = "Stop Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPERI_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<STOPERI_A> for bool {
    #[inline(always)]
    fn from(variant: STOPERI_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPERI_A {
        match self.bits {
            false => STOPERI_A::INACTIVE,
            true => STOPERI_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == STOPERI_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STOPERI_A::PENDING
    }
}
#[doc = "Field `STOPERI` writer - Stop Error Interrupt."]
pub type STOPERI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STOPERI_A>;
impl<'a, REG, const O: u8> STOPERI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(STOPERI_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(STOPERI_A::PENDING)
    }
}
#[doc = "Field `TXLOI` reader - Transmit Lock Out Interrupt."]
pub type TXLOI_R = crate::BitReader;
#[doc = "Field `TXLOI` writer - Transmit Lock Out Interrupt."]
pub type TXLOI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAMI` reader - Multiple Slave Address Match Interrupt."]
pub type MAMI_R = crate::FieldReader;
#[doc = "Field `MAMI` writer - Multiple Slave Address Match Interrupt."]
pub type MAMI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Transfer Done Interrupt."]
    #[inline(always)]
    pub fn donei(&self) -> DONEI_R {
        DONEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interactive Receive Interrupt."]
    #[inline(always)]
    pub fn irxmi(&self) -> IRXMI_R {
        IRXMI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave General Call Address Match Interrupt."]
    #[inline(always)]
    pub fn gci(&self) -> GCI_R {
        GCI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Address Match Interrupt."]
    #[inline(always)]
    pub fn ami(&self) -> AMI_R {
        AMI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Threshold Interrupt."]
    #[inline(always)]
    pub fn rxthi(&self) -> RXTHI_R {
        RXTHI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Threshold Interrupt."]
    #[inline(always)]
    pub fn txthi(&self) -> TXTHI_R {
        TXTHI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STOP Interrupt."]
    #[inline(always)]
    pub fn stopi(&self) -> STOPI_R {
        STOPI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Address Acknowledge Interrupt."]
    #[inline(always)]
    pub fn adracki(&self) -> ADRACKI_R {
        ADRACKI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Arbritation error Interrupt."]
    #[inline(always)]
    pub fn arberi(&self) -> ARBERI_R {
        ARBERI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - timeout Error Interrupt."]
    #[inline(always)]
    pub fn toeri(&self) -> TOERI_R {
        TOERI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Address NACK Error Interrupt."]
    #[inline(always)]
    pub fn adreri(&self) -> ADRERI_R {
        ADRERI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data NACK Error Interrupt."]
    #[inline(always)]
    pub fn dateri(&self) -> DATERI_R {
        DATERI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Do Not Respond Error Interrupt."]
    #[inline(always)]
    pub fn dnreri(&self) -> DNRERI_R {
        DNRERI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start Error Interrupt."]
    #[inline(always)]
    pub fn strteri(&self) -> STRTERI_R {
        STRTERI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop Error Interrupt."]
    #[inline(always)]
    pub fn stoperi(&self) -> STOPERI_R {
        STOPERI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit Lock Out Interrupt."]
    #[inline(always)]
    pub fn txloi(&self) -> TXLOI_R {
        TXLOI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Multiple Slave Address Match Interrupt."]
    #[inline(always)]
    pub fn mami(&self) -> MAMI_R {
        MAMI_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Done Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn donei(&mut self) -> DONEI_W<INTFL0_SPEC, 0> {
        DONEI_W::new(self)
    }
    #[doc = "Bit 1 - Interactive Receive Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn irxmi(&mut self) -> IRXMI_W<INTFL0_SPEC, 1> {
        IRXMI_W::new(self)
    }
    #[doc = "Bit 2 - Slave General Call Address Match Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gci(&mut self) -> GCI_W<INTFL0_SPEC, 2> {
        GCI_W::new(self)
    }
    #[doc = "Bit 3 - Slave Address Match Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ami(&mut self) -> AMI_W<INTFL0_SPEC, 3> {
        AMI_W::new(self)
    }
    #[doc = "Bit 4 - Receive Threshold Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxthi(&mut self) -> RXTHI_W<INTFL0_SPEC, 4> {
        RXTHI_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Threshold Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txthi(&mut self) -> TXTHI_W<INTFL0_SPEC, 5> {
        TXTHI_W::new(self)
    }
    #[doc = "Bit 6 - STOP Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stopi(&mut self) -> STOPI_W<INTFL0_SPEC, 6> {
        STOPI_W::new(self)
    }
    #[doc = "Bit 7 - Address Acknowledge Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adracki(&mut self) -> ADRACKI_W<INTFL0_SPEC, 7> {
        ADRACKI_W::new(self)
    }
    #[doc = "Bit 8 - Arbritation error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arberi(&mut self) -> ARBERI_W<INTFL0_SPEC, 8> {
        ARBERI_W::new(self)
    }
    #[doc = "Bit 9 - timeout Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn toeri(&mut self) -> TOERI_W<INTFL0_SPEC, 9> {
        TOERI_W::new(self)
    }
    #[doc = "Bit 10 - Address NACK Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adreri(&mut self) -> ADRERI_W<INTFL0_SPEC, 10> {
        ADRERI_W::new(self)
    }
    #[doc = "Bit 11 - Data NACK Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dateri(&mut self) -> DATERI_W<INTFL0_SPEC, 11> {
        DATERI_W::new(self)
    }
    #[doc = "Bit 12 - Do Not Respond Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dnreri(&mut self) -> DNRERI_W<INTFL0_SPEC, 12> {
        DNRERI_W::new(self)
    }
    #[doc = "Bit 13 - Start Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn strteri(&mut self) -> STRTERI_W<INTFL0_SPEC, 13> {
        STRTERI_W::new(self)
    }
    #[doc = "Bit 14 - Stop Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stoperi(&mut self) -> STOPERI_W<INTFL0_SPEC, 14> {
        STOPERI_W::new(self)
    }
    #[doc = "Bit 15 - Transmit Lock Out Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txloi(&mut self) -> TXLOI_W<INTFL0_SPEC, 15> {
        TXLOI_W::new(self)
    }
    #[doc = "Bits 16:19 - Multiple Slave Address Match Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mami(&mut self) -> MAMI_W<INTFL0_SPEC, 16> {
        MAMI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFL0_SPEC;
impl crate::RegisterSpec for INTFL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl0::R`](R) reader structure"]
impl crate::Readable for INTFL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intfl0::W`](W) writer structure"]
impl crate::Writable for INTFL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFL0 to value 0"]
impl crate::Resettable for INTFL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
