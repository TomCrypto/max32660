#[doc = "Register `SLADDR` reader"]
pub type R = crate::R<SLADDR_SPEC>;
#[doc = "Register `SLADDR` writer"]
pub type W = crate::W<SLADDR_SPEC>;
#[doc = "Field `SLA` reader - Slave Address."]
pub type SLA_R = crate::FieldReader<u16>;
#[doc = "Field `SLA` writer - Slave Address."]
pub type SLA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `SLADIS` reader - Slave Address Disable."]
pub type SLADIS_R = crate::BitReader;
#[doc = "Field `SLADIS` writer - Slave Address Disable."]
pub type SLADIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLAIDX` reader - Slave Address Index."]
pub type SLAIDX_R = crate::FieldReader;
#[doc = "Field `SLAIDX` writer - Slave Address Index."]
pub type SLAIDX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EA` reader - Extended Address Select."]
pub type EA_R = crate::BitReader<EA_A>;
#[doc = "Extended Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EA_A {
    #[doc = "0: 7-bit address."]
    _7_BITS_ADDRESS = 0,
    #[doc = "1: 10-bit address."]
    _10_BITS_ADDRESS = 1,
}
impl From<EA_A> for bool {
    #[inline(always)]
    fn from(variant: EA_A) -> Self {
        variant as u8 != 0
    }
}
impl EA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EA_A {
        match self.bits {
            false => EA_A::_7_BITS_ADDRESS,
            true => EA_A::_10_BITS_ADDRESS,
        }
    }
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        *self == EA_A::_7_BITS_ADDRESS
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        *self == EA_A::_10_BITS_ADDRESS
    }
}
#[doc = "Field `EA` writer - Extended Address Select."]
pub type EA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EA_A>;
impl<'a, REG, const O: u8> EA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(EA_A::_7_BITS_ADDRESS)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(EA_A::_10_BITS_ADDRESS)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    pub fn sla(&self) -> SLA_R {
        SLA_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Slave Address Disable."]
    #[inline(always)]
    pub fn sladis(&self) -> SLADIS_R {
        SLADIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - Slave Address Index."]
    #[inline(always)]
    pub fn slaidx(&self) -> SLAIDX_R {
        SLAIDX_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    #[must_use]
    pub fn sla(&mut self) -> SLA_W<SLADDR_SPEC, 0> {
        SLA_W::new(self)
    }
    #[doc = "Bit 10 - Slave Address Disable."]
    #[inline(always)]
    #[must_use]
    pub fn sladis(&mut self) -> SLADIS_W<SLADDR_SPEC, 10> {
        SLADIS_W::new(self)
    }
    #[doc = "Bits 11:14 - Slave Address Index."]
    #[inline(always)]
    #[must_use]
    pub fn slaidx(&mut self) -> SLAIDX_W<SLADDR_SPEC, 11> {
        SLAIDX_W::new(self)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    #[must_use]
    pub fn ea(&mut self) -> EA_W<SLADDR_SPEC, 15> {
        EA_W::new(self)
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
#[doc = "Slave Address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sladdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sladdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLADDR_SPEC;
impl crate::RegisterSpec for SLADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sladdr::R`](R) reader structure"]
impl crate::Readable for SLADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sladdr::W`](W) writer structure"]
impl crate::Writable for SLADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLADDR to value 0"]
impl crate::Resettable for SLADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
