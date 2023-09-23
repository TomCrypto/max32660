#[doc = "Register `CLK_CFG` reader"]
pub type R = crate::R<CLK_CFG_SPEC>;
#[doc = "Register `CLK_CFG` writer"]
pub type W = crate::W<CLK_CFG_SPEC>;
#[doc = "Field `LO` reader - Low duty cycle control."]
pub type LO_R = crate::FieldReader<LO_A>;
#[doc = "Low duty cycle control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LO_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DISABLED = 0,
}
impl From<LO_A> for u8 {
    #[inline(always)]
    fn from(variant: LO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LO_A {
    type Ux = u8;
}
impl LO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LO_A> {
        match self.bits {
            0 => Some(LO_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LO_A::DISABLED
    }
}
#[doc = "Field `LO` writer - Low duty cycle control."]
pub type LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, LO_A>;
impl<'a, REG, const O: u8> LO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LO_A::DISABLED)
    }
}
#[doc = "Field `HI` reader - High duty cycle control."]
pub type HI_R = crate::FieldReader<HI_A>;
#[doc = "High duty cycle control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HI_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DISABLED = 0,
}
impl From<HI_A> for u8 {
    #[inline(always)]
    fn from(variant: HI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HI_A {
    type Ux = u8;
}
impl HI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HI_A> {
        match self.bits {
            0 => Some(HI_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HI_A::DISABLED
    }
}
#[doc = "Field `HI` writer - High duty cycle control."]
pub type HI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, HI_A>;
impl<'a, REG, const O: u8> HI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HI_A::DISABLED)
    }
}
#[doc = "Field `SCALE` reader - System Clock scale factor."]
pub type SCALE_R = crate::FieldReader;
#[doc = "Field `SCALE` writer - System Clock scale factor."]
pub type SCALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Low duty cycle control."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High duty cycle control."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - System Clock scale factor."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low duty cycle control."]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<CLK_CFG_SPEC, 0> {
        LO_W::new(self)
    }
    #[doc = "Bits 8:15 - High duty cycle control."]
    #[inline(always)]
    #[must_use]
    pub fn hi(&mut self) -> HI_W<CLK_CFG_SPEC, 8> {
        HI_W::new(self)
    }
    #[doc = "Bits 16:19 - System Clock scale factor."]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<CLK_CFG_SPEC, 16> {
        SCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CFG_SPEC;
impl crate::RegisterSpec for CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cfg::R`](R) reader structure"]
impl crate::Readable for CLK_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_cfg::W`](W) writer structure"]
impl crate::Writable for CLK_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CFG to value 0"]
impl crate::Resettable for CLK_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
