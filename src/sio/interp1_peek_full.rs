#[doc = "Register `INTERP1_PEEK_FULL` reader"]
pub struct R(crate::R<INTERP1_PEEK_FULL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP1_PEEK_FULL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTERP1_PEEK_FULL_SPEC>> for R {
    fn from(reader: crate::R<INTERP1_PEEK_FULL_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Read FULL result, without altering any internal state (PEEK).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_peek_full](index.html) module"]
pub struct INTERP1_PEEK_FULL_SPEC;
impl crate::RegisterSpec for INTERP1_PEEK_FULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp1_peek_full::R](R) reader structure"]
impl crate::Readable for INTERP1_PEEK_FULL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERP1_PEEK_FULL to value 0"]
impl crate::Resettable for INTERP1_PEEK_FULL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
