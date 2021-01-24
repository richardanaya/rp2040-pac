#[doc = "Register `DIV_UDIVIDEND` reader"]
pub struct R(crate::R<DIV_UDIVIDEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_UDIVIDEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DIV_UDIVIDEND_SPEC>> for R {
    fn from(reader: crate::R<DIV_UDIVIDEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_UDIVIDEND` writer"]
pub struct W(crate::W<DIV_UDIVIDEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_UDIVIDEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<DIV_UDIVIDEND_SPEC>> for W {
    fn from(writer: crate::W<DIV_UDIVIDEND_SPEC>) -> Self {
        W(writer)
    }
}
impl R {}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divider unsigned dividend\\n Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`.\\n Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.\\n UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an\\n unsigned calculation, and the S alias starts a signed calculation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_udividend](index.html) module"]
pub struct DIV_UDIVIDEND_SPEC;
impl crate::RegisterSpec for DIV_UDIVIDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_udividend::R](R) reader structure"]
impl crate::Readable for DIV_UDIVIDEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_udividend::W](W) writer structure"]
impl crate::Writable for DIV_UDIVIDEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_UDIVIDEND to value 0"]
impl crate::Resettable for DIV_UDIVIDEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
