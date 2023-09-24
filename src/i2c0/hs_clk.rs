#[doc = "Register `HS_CLK` reader"]
pub type R = crate::R<HS_CLK_SPEC>;
#[doc = "Register `HS_CLK` writer"]
pub type W = crate::W<HS_CLK_SPEC>;
#[doc = "Field `HS_CLK_LO` reader - Slave Address."]
pub type HS_CLK_LO_R = crate::FieldReader;
#[doc = "Field `HS_CLK_LO` writer - Slave Address."]
pub type HS_CLK_LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HS_CLK_HI` reader - Slave Address."]
pub type HS_CLK_HI_R = crate::FieldReader;
#[doc = "Field `HS_CLK_HI` writer - Slave Address."]
pub type HS_CLK_HI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_lo(&self) -> HS_CLK_LO_R {
        HS_CLK_LO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_hi(&self) -> HS_CLK_HI_R {
        HS_CLK_HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Address."]
    #[inline(always)]
    #[must_use]
    pub fn hs_clk_lo(&mut self) -> HS_CLK_LO_W<HS_CLK_SPEC, 0> {
        HS_CLK_LO_W::new(self)
    }
    #[doc = "Bits 8:15 - Slave Address."]
    #[inline(always)]
    #[must_use]
    pub fn hs_clk_hi(&mut self) -> HS_CLK_HI_W<HS_CLK_SPEC, 8> {
        HS_CLK_HI_W::new(self)
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
#[doc = "HS-Mode Clock Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HS_CLK_SPEC;
impl crate::RegisterSpec for HS_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_clk::R`](R) reader structure"]
impl crate::Readable for HS_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hs_clk::W`](W) writer structure"]
impl crate::Writable for HS_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HS_CLK to value 0"]
impl crate::Resettable for HS_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
