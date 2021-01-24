#[doc = "Register `DIV_QUOTIENT` reader"]
pub struct R(crate::R<DIV_QUOTIENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_QUOTIENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DIV_QUOTIENT_SPEC>> for R {
    fn from(reader: crate::R<DIV_QUOTIENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_QUOTIENT` writer"]
pub struct W(crate::W<DIV_QUOTIENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_QUOTIENT_SPEC>;
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
impl core::convert::From<crate::W<DIV_QUOTIENT_SPEC>> for W {
    fn from(writer: crate::W<DIV_QUOTIENT_SPEC>) -> Self {
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
#[doc = "Divider result quotient\\n The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low.\\n For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ.\\n This register can be written to directly, for context save/restore purposes. This halts any\\n in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.\\n Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order\\n REMAINDER, QUOTIENT if CSR_DIRTY is used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_quotient](index.html) module"]
pub struct DIV_QUOTIENT_SPEC;
impl crate::RegisterSpec for DIV_QUOTIENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_quotient::R](R) reader structure"]
impl crate::Readable for DIV_QUOTIENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_quotient::W](W) writer structure"]
impl crate::Writable for DIV_QUOTIENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_QUOTIENT to value 0"]
impl crate::Resettable for DIV_QUOTIENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
