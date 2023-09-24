#[doc = "Register `RST` writer"]
pub type W = crate::W<RST_SPEC>;
#[doc = "Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_RST_AW {
    #[doc = "165: The first value to be written to reset the WDT."]
    SEQ0 = 165,
    #[doc = "90: The second value to be written to reset the WDT."]
    SEQ1 = 90,
}
impl From<WDT_RST_AW> for u8 {
    #[inline(always)]
    fn from(variant: WDT_RST_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_RST_AW {
    type Ux = u8;
}
#[doc = "Field `WDT_RST` writer - Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter."]
pub type WDT_RST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, WDT_RST_AW>;
impl<'a, REG, const O: u8> WDT_RST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The first value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq0(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_RST_AW::SEQ0)
    }
    #[doc = "The second value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq1(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_RST_AW::SEQ1)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_rst(&mut self) -> WDT_RST_W<RST_SPEC, 0> {
        WDT_RST_W::new(self)
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
#[doc = "Watchdog Timer Reset.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_SPEC;
impl crate::RegisterSpec for RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rst::W`](W) writer structure"]
impl crate::Writable for RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST to value 0"]
impl crate::Resettable for RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
