#[doc = "Register `INTERP0_ACCUM0` reader"]
pub struct R(crate::R<INTERP0_ACCUM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP0_ACCUM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTERP0_ACCUM0_SPEC>> for R {
    fn from(reader: crate::R<INTERP0_ACCUM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERP0_ACCUM0` writer"]
pub struct W(crate::W<INTERP0_ACCUM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERP0_ACCUM0_SPEC>;
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
impl core::convert::From<crate::W<INTERP0_ACCUM0_SPEC>> for W {
    fn from(writer: crate::W<INTERP0_ACCUM0_SPEC>) -> Self {
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
#[doc = "Read/write access to accumulator 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_accum0](index.html) module"]
pub struct INTERP0_ACCUM0_SPEC;
impl crate::RegisterSpec for INTERP0_ACCUM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp0_accum0::R](R) reader structure"]
impl crate::Readable for INTERP0_ACCUM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interp0_accum0::W](W) writer structure"]
impl crate::Writable for INTERP0_ACCUM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERP0_ACCUM0 to value 0"]
impl crate::Resettable for INTERP0_ACCUM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
