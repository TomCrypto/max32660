#[doc = "Register `SCON` reader"]
pub type R = crate::R<SCON_SPEC>;
#[doc = "Register `SCON` writer"]
pub type W = crate::W<SCON_SPEC>;
#[doc = "Field `FLASH_PAGE_FLIP` reader - Flips the Flash bottom and top halves."]
pub type FLASH_PAGE_FLIP_R = crate::BitReader<FLASH_PAGE_FLIP_A>;
#[doc = "Flips the Flash bottom and top halves.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASH_PAGE_FLIP_A {
    #[doc = "0: Physical layout matches logical layout."]
    NORMAL = 0,
    #[doc = "1: Bottom half mapped to logical top half and vice versa."]
    SWAPPED = 1,
}
impl From<FLASH_PAGE_FLIP_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PAGE_FLIP_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASH_PAGE_FLIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_PAGE_FLIP_A {
        match self.bits {
            false => FLASH_PAGE_FLIP_A::NORMAL,
            true => FLASH_PAGE_FLIP_A::SWAPPED,
        }
    }
    #[doc = "Physical layout matches logical layout."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FLASH_PAGE_FLIP_A::NORMAL
    }
    #[doc = "Bottom half mapped to logical top half and vice versa."]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == FLASH_PAGE_FLIP_A::SWAPPED
    }
}
#[doc = "Field `FLASH_PAGE_FLIP` writer - Flips the Flash bottom and top halves."]
pub type FLASH_PAGE_FLIP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLASH_PAGE_FLIP_A>;
impl<'a, REG, const O: u8> FLASH_PAGE_FLIP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Physical layout matches logical layout."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_PAGE_FLIP_A::NORMAL)
    }
    #[doc = "Bottom half mapped to logical top half and vice versa."]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_PAGE_FLIP_A::SWAPPED)
    }
}
#[doc = "Field `FPU_DIS` reader - Floating Point Unit Disable."]
pub type FPU_DIS_R = crate::BitReader<FPU_DIS_A>;
#[doc = "Floating Point Unit Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_DIS_A {
    #[doc = "0: enable Floating point unit."]
    ENABLED = 0,
    #[doc = "1: disable floating point unit."]
    DISABLED = 1,
}
impl From<FPU_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FPU_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_DIS_A {
        match self.bits {
            false => FPU_DIS_A::ENABLED,
            true => FPU_DIS_A::DISABLED,
        }
    }
    #[doc = "enable Floating point unit."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_DIS_A::ENABLED
    }
    #[doc = "disable floating point unit."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_DIS_A::DISABLED
    }
}
#[doc = "Field `FPU_DIS` writer - Floating Point Unit Disable."]
pub type FPU_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FPU_DIS_A>;
impl<'a, REG, const O: u8> FPU_DIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Floating point unit."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_DIS_A::ENABLED)
    }
    #[doc = "disable floating point unit."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_DIS_A::DISABLED)
    }
}
#[doc = "Field `ICC_FLUSH` reader - Instruction Cache Controller Flush."]
pub type ICC_FLUSH_R = crate::BitReader<ICC_FLUSH_A>;
#[doc = "Instruction Cache Controller Flush.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICC_FLUSH_A {
    #[doc = "0: Normal Code Cache Operation."]
    NORMAL = 0,
    #[doc = "1: Code Caches and CPU instruction buffer are flushed."]
    FLUSH = 1,
}
impl From<ICC_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: ICC_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl ICC_FLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICC_FLUSH_A {
        match self.bits {
            false => ICC_FLUSH_A::NORMAL,
            true => ICC_FLUSH_A::FLUSH,
        }
    }
    #[doc = "Normal Code Cache Operation."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ICC_FLUSH_A::NORMAL
    }
    #[doc = "Code Caches and CPU instruction buffer are flushed."]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == ICC_FLUSH_A::FLUSH
    }
}
#[doc = "Field `ICC_FLUSH` writer - Instruction Cache Controller Flush."]
pub type ICC_FLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ICC_FLUSH_A>;
impl<'a, REG, const O: u8> ICC_FLUSH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Code Cache Operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ICC_FLUSH_A::NORMAL)
    }
    #[doc = "Code Caches and CPU instruction buffer are flushed."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(ICC_FLUSH_A::FLUSH)
    }
}
#[doc = "Field `SWD_DIS` reader - Serial Wire Debug Disable."]
pub type SWD_DIS_R = crate::BitReader<SWD_DIS_A>;
#[doc = "Serial Wire Debug Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWD_DIS_A {
    #[doc = "0: Enable JTAG SWD."]
    ENABLED = 0,
    #[doc = "1: Disable JTAG SWD."]
    DISABLED = 1,
}
impl From<SWD_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: SWD_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SWD_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWD_DIS_A {
        match self.bits {
            false => SWD_DIS_A::ENABLED,
            true => SWD_DIS_A::DISABLED,
        }
    }
    #[doc = "Enable JTAG SWD."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWD_DIS_A::ENABLED
    }
    #[doc = "Disable JTAG SWD."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWD_DIS_A::DISABLED
    }
}
#[doc = "Field `SWD_DIS` writer - Serial Wire Debug Disable."]
pub type SWD_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWD_DIS_A>;
impl<'a, REG, const O: u8> SWD_DIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable JTAG SWD."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWD_DIS_A::ENABLED)
    }
    #[doc = "Disable JTAG SWD."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWD_DIS_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 4 - Flips the Flash bottom and top halves."]
    #[inline(always)]
    pub fn flash_page_flip(&self) -> FLASH_PAGE_FLIP_R {
        FLASH_PAGE_FLIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Floating Point Unit Disable."]
    #[inline(always)]
    pub fn fpu_dis(&self) -> FPU_DIS_R {
        FPU_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Instruction Cache Controller Flush."]
    #[inline(always)]
    pub fn icc_flush(&self) -> ICC_FLUSH_R {
        ICC_FLUSH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable."]
    #[inline(always)]
    pub fn swd_dis(&self) -> SWD_DIS_R {
        SWD_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Flips the Flash bottom and top halves."]
    #[inline(always)]
    #[must_use]
    pub fn flash_page_flip(&mut self) -> FLASH_PAGE_FLIP_W<SCON_SPEC, 4> {
        FLASH_PAGE_FLIP_W::new(self)
    }
    #[doc = "Bit 5 - Floating Point Unit Disable."]
    #[inline(always)]
    #[must_use]
    pub fn fpu_dis(&mut self) -> FPU_DIS_W<SCON_SPEC, 5> {
        FPU_DIS_W::new(self)
    }
    #[doc = "Bit 6 - Instruction Cache Controller Flush."]
    #[inline(always)]
    #[must_use]
    pub fn icc_flush(&mut self) -> ICC_FLUSH_W<SCON_SPEC, 6> {
        ICC_FLUSH_W::new(self)
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable."]
    #[inline(always)]
    #[must_use]
    pub fn swd_dis(&mut self) -> SWD_DIS_W<SCON_SPEC, 14> {
        SWD_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCON_SPEC;
impl crate::RegisterSpec for SCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scon::R`](R) reader structure"]
impl crate::Readable for SCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scon::W`](W) writer structure"]
impl crate::Writable for SCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCON to value 0"]
impl crate::Resettable for SCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
