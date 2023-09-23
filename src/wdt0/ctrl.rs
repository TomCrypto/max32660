#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `INT_PERIOD` reader - Watchdog Interrupt Period."]
pub type INT_PERIOD_R = crate::FieldReader<INT_PERIOD_A>;
#[doc = "Watchdog Interrupt Period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INT_PERIOD_A {
    #[doc = "0: 2**31 clock cycles."]
    WDT2POW31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    WDT2POW30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    WDT2POW29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    WDT2POW28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    WDT2POW27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    WDT2POW26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    WDT2POW25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    WDT2POW24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    WDT2POW23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    WDT2POW22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    WDT2POW21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    WDT2POW20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    WDT2POW19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    WDT2POW18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    WDT2POW17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    WDT2POW16 = 15,
}
impl From<INT_PERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: INT_PERIOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INT_PERIOD_A {
    type Ux = u8;
}
impl INT_PERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_PERIOD_A {
        match self.bits {
            0 => INT_PERIOD_A::WDT2POW31,
            1 => INT_PERIOD_A::WDT2POW30,
            2 => INT_PERIOD_A::WDT2POW29,
            3 => INT_PERIOD_A::WDT2POW28,
            4 => INT_PERIOD_A::WDT2POW27,
            5 => INT_PERIOD_A::WDT2POW26,
            6 => INT_PERIOD_A::WDT2POW25,
            7 => INT_PERIOD_A::WDT2POW24,
            8 => INT_PERIOD_A::WDT2POW23,
            9 => INT_PERIOD_A::WDT2POW22,
            10 => INT_PERIOD_A::WDT2POW21,
            11 => INT_PERIOD_A::WDT2POW20,
            12 => INT_PERIOD_A::WDT2POW19,
            13 => INT_PERIOD_A::WDT2POW18,
            14 => INT_PERIOD_A::WDT2POW17,
            15 => INT_PERIOD_A::WDT2POW16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == INT_PERIOD_A::WDT2POW16
    }
}
#[doc = "Field `INT_PERIOD` writer - Watchdog Interrupt Period."]
pub type INT_PERIOD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, INT_PERIOD_A>;
impl<'a, REG, const O: u8> INT_PERIOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(INT_PERIOD_A::WDT2POW16)
    }
}
#[doc = "Field `RST_PERIOD` reader - Watchdog Reset Period."]
pub type RST_PERIOD_R = crate::FieldReader<RST_PERIOD_A>;
#[doc = "Watchdog Reset Period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RST_PERIOD_A {
    #[doc = "0: 2**31 clock cycles."]
    WDT2POW31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    WDT2POW30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    WDT2POW29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    WDT2POW28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    WDT2POW27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    WDT2POW26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    WDT2POW25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    WDT2POW24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    WDT2POW23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    WDT2POW22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    WDT2POW21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    WDT2POW20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    WDT2POW19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    WDT2POW18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    WDT2POW17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    WDT2POW16 = 15,
}
impl From<RST_PERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: RST_PERIOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RST_PERIOD_A {
    type Ux = u8;
}
impl RST_PERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_PERIOD_A {
        match self.bits {
            0 => RST_PERIOD_A::WDT2POW31,
            1 => RST_PERIOD_A::WDT2POW30,
            2 => RST_PERIOD_A::WDT2POW29,
            3 => RST_PERIOD_A::WDT2POW28,
            4 => RST_PERIOD_A::WDT2POW27,
            5 => RST_PERIOD_A::WDT2POW26,
            6 => RST_PERIOD_A::WDT2POW25,
            7 => RST_PERIOD_A::WDT2POW24,
            8 => RST_PERIOD_A::WDT2POW23,
            9 => RST_PERIOD_A::WDT2POW22,
            10 => RST_PERIOD_A::WDT2POW21,
            11 => RST_PERIOD_A::WDT2POW20,
            12 => RST_PERIOD_A::WDT2POW19,
            13 => RST_PERIOD_A::WDT2POW18,
            14 => RST_PERIOD_A::WDT2POW17,
            15 => RST_PERIOD_A::WDT2POW16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == RST_PERIOD_A::WDT2POW16
    }
}
#[doc = "Field `RST_PERIOD` writer - Watchdog Reset Period."]
pub type RST_PERIOD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, RST_PERIOD_A>;
impl<'a, REG, const O: u8> RST_PERIOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(RST_PERIOD_A::WDT2POW16)
    }
}
#[doc = "Field `WDT_EN` reader - Watchdog Timer Enable."]
pub type WDT_EN_R = crate::BitReader<WDT_EN_A>;
#[doc = "Watchdog Timer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_EN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<WDT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_EN_A {
        match self.bits {
            false => WDT_EN_A::DISABLED,
            true => WDT_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDT_EN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDT_EN_A::ENABLED
    }
}
#[doc = "Field `WDT_EN` writer - Watchdog Timer Enable."]
pub type WDT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDT_EN_A>;
impl<'a, REG, const O: u8> WDT_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_EN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_EN_A::ENABLED)
    }
}
#[doc = "Field `INT_FLAG` reader - Watchdog Timer Interrupt Flag."]
pub type INT_FLAG_R = crate::BitReader<INT_FLAG_A>;
#[doc = "Watchdog Timer Interrupt Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_FLAG_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<INT_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: INT_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_FLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_FLAG_A {
        match self.bits {
            false => INT_FLAG_A::INACTIVE,
            true => INT_FLAG_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == INT_FLAG_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == INT_FLAG_A::PENDING
    }
}
#[doc = "Field `INT_FLAG` writer - Watchdog Timer Interrupt Flag."]
pub type INT_FLAG_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, INT_FLAG_A>;
impl<'a, REG, const O: u8> INT_FLAG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(INT_FLAG_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(INT_FLAG_A::PENDING)
    }
}
#[doc = "Field `INT_EN` reader - Watchdog Timer Interrupt Enable."]
pub type INT_EN_R = crate::BitReader<INT_EN_A>;
#[doc = "Watchdog Timer Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN_A {
        match self.bits {
            false => INT_EN_A::DISABLED,
            true => INT_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INT_EN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INT_EN_A::ENABLED
    }
}
#[doc = "Field `INT_EN` writer - Watchdog Timer Interrupt Enable."]
pub type INT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT_EN_A>;
impl<'a, REG, const O: u8> INT_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EN_A::ENABLED)
    }
}
#[doc = "Field `RST_EN` reader - Watchdog Timer Reset Enable."]
pub type RST_EN_R = crate::BitReader<RST_EN_A>;
#[doc = "Watchdog Timer Reset Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_EN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_EN_A {
        match self.bits {
            false => RST_EN_A::DISABLED,
            true => RST_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RST_EN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RST_EN_A::ENABLED
    }
}
#[doc = "Field `RST_EN` writer - Watchdog Timer Reset Enable."]
pub type RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RST_EN_A>;
impl<'a, REG, const O: u8> RST_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EN_A::ENABLED)
    }
}
#[doc = "Field `RST_FLAG` reader - Watchdog Timer Reset Flag."]
pub type RST_FLAG_R = crate::BitReader<RST_FLAG_A>;
#[doc = "Watchdog Timer Reset Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_FLAG_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<RST_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: RST_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_FLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_FLAG_A {
        match self.bits {
            false => RST_FLAG_A::NO_EVENT,
            true => RST_FLAG_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RST_FLAG_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == RST_FLAG_A::OCCURRED
    }
}
#[doc = "Field `RST_FLAG` writer - Watchdog Timer Reset Flag."]
pub type RST_FLAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RST_FLAG_A>;
impl<'a, REG, const O: u8> RST_FLAG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(RST_FLAG_A::NO_EVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(RST_FLAG_A::OCCURRED)
    }
}
impl R {
    #[doc = "Bits 0:3 - Watchdog Interrupt Period."]
    #[inline(always)]
    pub fn int_period(&self) -> INT_PERIOD_R {
        INT_PERIOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Watchdog Reset Period."]
    #[inline(always)]
    pub fn rst_period(&self) -> RST_PERIOD_R {
        RST_PERIOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Watchdog Timer Enable."]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Watchdog Timer Interrupt Flag."]
    #[inline(always)]
    pub fn int_flag(&self) -> INT_FLAG_R {
        INT_FLAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Watchdog Timer Interrupt Enable."]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Watchdog Timer Reset Enable."]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - Watchdog Timer Reset Flag."]
    #[inline(always)]
    pub fn rst_flag(&self) -> RST_FLAG_R {
        RST_FLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Watchdog Interrupt Period."]
    #[inline(always)]
    #[must_use]
    pub fn int_period(&mut self) -> INT_PERIOD_W<CTRL_SPEC, 0> {
        INT_PERIOD_W::new(self)
    }
    #[doc = "Bits 4:7 - Watchdog Reset Period."]
    #[inline(always)]
    #[must_use]
    pub fn rst_period(&mut self) -> RST_PERIOD_W<CTRL_SPEC, 4> {
        RST_PERIOD_W::new(self)
    }
    #[doc = "Bit 8 - Watchdog Timer Enable."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_en(&mut self) -> WDT_EN_W<CTRL_SPEC, 8> {
        WDT_EN_W::new(self)
    }
    #[doc = "Bit 9 - Watchdog Timer Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn int_flag(&mut self) -> INT_FLAG_W<CTRL_SPEC, 9> {
        INT_FLAG_W::new(self)
    }
    #[doc = "Bit 10 - Watchdog Timer Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<CTRL_SPEC, 10> {
        INT_EN_W::new(self)
    }
    #[doc = "Bit 11 - Watchdog Timer Reset Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rst_en(&mut self) -> RST_EN_W<CTRL_SPEC, 11> {
        RST_EN_W::new(self)
    }
    #[doc = "Bit 31 - Watchdog Timer Reset Flag."]
    #[inline(always)]
    #[must_use]
    pub fn rst_flag(&mut self) -> RST_FLAG_W<CTRL_SPEC, 31> {
        RST_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Watchdog Timer Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0200;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
