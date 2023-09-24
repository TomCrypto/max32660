#[doc = "Register `MSTR_MODE` reader"]
pub type R = crate::R<MSTR_MODE_SPEC>;
#[doc = "Register `MSTR_MODE` writer"]
pub type W = crate::W<MSTR_MODE_SPEC>;
#[doc = "Field `START` reader - Setting this bit to 1 will start a master transfer."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Setting this bit to 1 will start a master transfer."]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESTART` reader - Setting this bit to 1 will generate a repeated START."]
pub type RESTART_R = crate::BitReader;
#[doc = "Field `RESTART` writer - Setting this bit to 1 will generate a repeated START."]
pub type RESTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOP` reader - Setting this bit to 1 will generate a STOP condition."]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Setting this bit to 1 will generate a STOP condition."]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEA` reader - Slave Extend Address Select."]
pub type SEA_R = crate::BitReader<SEA_A>;
#[doc = "Slave Extend Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEA_A {
    #[doc = "0: 7-bit address."]
    _7_BITS_ADDRESS = 0,
    #[doc = "1: 10-bit address."]
    _10_BITS_ADDRESS = 1,
}
impl From<SEA_A> for bool {
    #[inline(always)]
    fn from(variant: SEA_A) -> Self {
        variant as u8 != 0
    }
}
impl SEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEA_A {
        match self.bits {
            false => SEA_A::_7_BITS_ADDRESS,
            true => SEA_A::_10_BITS_ADDRESS,
        }
    }
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        *self == SEA_A::_7_BITS_ADDRESS
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        *self == SEA_A::_10_BITS_ADDRESS
    }
}
#[doc = "Field `SEA` writer - Slave Extend Address Select."]
pub type SEA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SEA_A>;
impl<'a, REG, const O: u8> SEA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(SEA_A::_7_BITS_ADDRESS)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(SEA_A::_10_BITS_ADDRESS)
    }
}
impl R {
    #[doc = "Bit 0 - Setting this bit to 1 will start a master transfer."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 will generate a repeated START."]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting this bit to 1 will generate a STOP condition."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave Extend Address Select."]
    #[inline(always)]
    pub fn sea(&self) -> SEA_R {
        SEA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit to 1 will start a master transfer."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<MSTR_MODE_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Setting this bit to 1 will generate a repeated START."]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<MSTR_MODE_SPEC, 1> {
        RESTART_W::new(self)
    }
    #[doc = "Bit 2 - Setting this bit to 1 will generate a STOP condition."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<MSTR_MODE_SPEC, 2> {
        STOP_W::new(self)
    }
    #[doc = "Bit 7 - Slave Extend Address Select."]
    #[inline(always)]
    #[must_use]
    pub fn sea(&mut self) -> SEA_W<MSTR_MODE_SPEC, 7> {
        SEA_W::new(self)
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
#[doc = "Master Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstr_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstr_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTR_MODE_SPEC;
impl crate::RegisterSpec for MSTR_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstr_mode::R`](R) reader structure"]
impl crate::Readable for MSTR_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mstr_mode::W`](W) writer structure"]
impl crate::Writable for MSTR_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTR_MODE to value 0"]
impl crate::Resettable for MSTR_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
