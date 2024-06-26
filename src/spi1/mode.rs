#[doc = "Register `MODE` reader"]
pub type R = crate::R<MODE_SPEC>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<MODE_SPEC>;
#[doc = "Field `SSV` reader - Slave Select Value."]
pub type SSV_R = crate::BitReader<SSV_A>;
#[doc = "Slave Select Value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSV_A {
    #[doc = "0: The SSEL pin will be driven low."]
    LO = 0,
    #[doc = "1: The SSEL pin will be driven high."]
    HI = 1,
}
impl From<SSV_A> for bool {
    #[inline(always)]
    fn from(variant: SSV_A) -> Self {
        variant as u8 != 0
    }
}
impl SSV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSV_A {
        match self.bits {
            false => SSV_A::LO,
            true => SSV_A::HI,
        }
    }
    #[doc = "The SSEL pin will be driven low."]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == SSV_A::LO
    }
    #[doc = "The SSEL pin will be driven high."]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == SSV_A::HI
    }
}
#[doc = "Field `SSV` writer - Slave Select Value."]
pub type SSV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSV_A>;
impl<'a, REG, const O: u8> SSV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SSEL pin will be driven low."]
    #[inline(always)]
    pub fn lo(self) -> &'a mut crate::W<REG> {
        self.variant(SSV_A::LO)
    }
    #[doc = "The SSEL pin will be driven high."]
    #[inline(always)]
    pub fn hi(self) -> &'a mut crate::W<REG> {
        self.variant(SSV_A::HI)
    }
}
#[doc = "Field `SS_IO` reader - Slave Select I/O."]
pub type SS_IO_R = crate::BitReader<SS_IO_A>;
#[doc = "Slave Select I/O.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SS_IO_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
}
impl From<SS_IO_A> for bool {
    #[inline(always)]
    fn from(variant: SS_IO_A) -> Self {
        variant as u8 != 0
    }
}
impl SS_IO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_IO_A {
        match self.bits {
            false => SS_IO_A::INPUT,
            true => SS_IO_A::OUTPUT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == SS_IO_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == SS_IO_A::OUTPUT
    }
}
#[doc = "Field `SS_IO` writer - Slave Select I/O."]
pub type SS_IO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SS_IO_A>;
impl<'a, REG, const O: u8> SS_IO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(SS_IO_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(SS_IO_A::OUTPUT)
    }
}
#[doc = "Field `NUMBITS` reader - "]
pub type NUMBITS_R = crate::FieldReader<NUMBITS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NUMBITS_A {
    #[doc = "0: 16 bits per character."]
    _16 = 0,
    #[doc = "1: 1 bit per character."]
    _1 = 1,
    #[doc = "2: 2 bits per character."]
    _2 = 2,
    #[doc = "3: 3 bits per character."]
    _3 = 3,
    #[doc = "4: 4 bits per character."]
    _4 = 4,
    #[doc = "5: 5 bits per character."]
    _5 = 5,
    #[doc = "6: 6 bits per character."]
    _6 = 6,
    #[doc = "7: 7 bits per character."]
    _7 = 7,
    #[doc = "8: 8 bits per character."]
    _8 = 8,
    #[doc = "9: 9 bits per character."]
    _9 = 9,
    #[doc = "10: 10 bits per character."]
    _10 = 10,
    #[doc = "11: 11 bits per character."]
    _11 = 11,
    #[doc = "12: 12 bits per character."]
    _12 = 12,
    #[doc = "13: 13 bits per character."]
    _13 = 13,
    #[doc = "14: 14 bits per character."]
    _14 = 14,
    #[doc = "15: 15 bits per character."]
    _15 = 15,
}
impl From<NUMBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMBITS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NUMBITS_A {
    type Ux = u8;
}
impl NUMBITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUMBITS_A {
        match self.bits {
            0 => NUMBITS_A::_16,
            1 => NUMBITS_A::_1,
            2 => NUMBITS_A::_2,
            3 => NUMBITS_A::_3,
            4 => NUMBITS_A::_4,
            5 => NUMBITS_A::_5,
            6 => NUMBITS_A::_6,
            7 => NUMBITS_A::_7,
            8 => NUMBITS_A::_8,
            9 => NUMBITS_A::_9,
            10 => NUMBITS_A::_10,
            11 => NUMBITS_A::_11,
            12 => NUMBITS_A::_12,
            13 => NUMBITS_A::_13,
            14 => NUMBITS_A::_14,
            15 => NUMBITS_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == NUMBITS_A::_16
    }
    #[doc = "1 bit per character."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NUMBITS_A::_1
    }
    #[doc = "2 bits per character."]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == NUMBITS_A::_2
    }
    #[doc = "3 bits per character."]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == NUMBITS_A::_3
    }
    #[doc = "4 bits per character."]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == NUMBITS_A::_4
    }
    #[doc = "5 bits per character."]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == NUMBITS_A::_5
    }
    #[doc = "6 bits per character."]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == NUMBITS_A::_6
    }
    #[doc = "7 bits per character."]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == NUMBITS_A::_7
    }
    #[doc = "8 bits per character."]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == NUMBITS_A::_8
    }
    #[doc = "9 bits per character."]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == NUMBITS_A::_9
    }
    #[doc = "10 bits per character."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NUMBITS_A::_10
    }
    #[doc = "11 bits per character."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NUMBITS_A::_11
    }
    #[doc = "12 bits per character."]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == NUMBITS_A::_12
    }
    #[doc = "13 bits per character."]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == NUMBITS_A::_13
    }
    #[doc = "14 bits per character."]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == NUMBITS_A::_14
    }
    #[doc = "15 bits per character."]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == NUMBITS_A::_15
    }
}
#[doc = "Field `NUMBITS` writer - "]
pub type NUMBITS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, NUMBITS_A>;
impl<'a, REG, const O: u8> NUMBITS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_16)
    }
    #[doc = "1 bit per character."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_1)
    }
    #[doc = "2 bits per character."]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_2)
    }
    #[doc = "3 bits per character."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_3)
    }
    #[doc = "4 bits per character."]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_4)
    }
    #[doc = "5 bits per character."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_5)
    }
    #[doc = "6 bits per character."]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_6)
    }
    #[doc = "7 bits per character."]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_7)
    }
    #[doc = "8 bits per character."]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_8)
    }
    #[doc = "9 bits per character."]
    #[inline(always)]
    pub fn _9(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_9)
    }
    #[doc = "10 bits per character."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_10)
    }
    #[doc = "11 bits per character."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_11)
    }
    #[doc = "12 bits per character."]
    #[inline(always)]
    pub fn _12(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_12)
    }
    #[doc = "13 bits per character."]
    #[inline(always)]
    pub fn _13(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_13)
    }
    #[doc = "14 bits per character."]
    #[inline(always)]
    pub fn _14(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_14)
    }
    #[doc = "15 bits per character."]
    #[inline(always)]
    pub fn _15(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_15)
    }
}
#[doc = "Field `TX_LJ` reader - Transmit Left Justify."]
pub type TX_LJ_R = crate::BitReader<TX_LJ_A>;
#[doc = "Transmit Left Justify.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_LJ_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<TX_LJ_A> for bool {
    #[inline(always)]
    fn from(variant: TX_LJ_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_LJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_LJ_A {
        match self.bits {
            false => TX_LJ_A::DISABLED,
            true => TX_LJ_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_LJ_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_LJ_A::ENABLED
    }
}
#[doc = "Field `TX_LJ` writer - Transmit Left Justify."]
pub type TX_LJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TX_LJ_A>;
impl<'a, REG, const O: u8> TX_LJ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_LJ_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_LJ_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Select Value."]
    #[inline(always)]
    pub fn ssv(&self) -> SSV_R {
        SSV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Select I/O."]
    #[inline(always)]
    pub fn ss_io(&self) -> SS_IO_R {
        SS_IO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn numbits(&self) -> NUMBITS_R {
        NUMBITS_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Transmit Left Justify."]
    #[inline(always)]
    pub fn tx_lj(&self) -> TX_LJ_R {
        TX_LJ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Select Value."]
    #[inline(always)]
    #[must_use]
    pub fn ssv(&mut self) -> SSV_W<MODE_SPEC, 0> {
        SSV_W::new(self)
    }
    #[doc = "Bit 1 - Slave Select I/O."]
    #[inline(always)]
    #[must_use]
    pub fn ss_io(&mut self) -> SS_IO_W<MODE_SPEC, 1> {
        SS_IO_W::new(self)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    #[must_use]
    pub fn numbits(&mut self) -> NUMBITS_W<MODE_SPEC, 2> {
        NUMBITS_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Left Justify."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lj(&mut self) -> TX_LJ_W<MODE_SPEC, 7> {
        TX_LJ_W::new(self)
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
#[doc = "SPI Mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
