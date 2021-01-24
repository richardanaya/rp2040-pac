#[doc = "Register `INTERP0_PEEK_LANE0` reader"]
pub struct R(crate::R<INTERP0_PEEK_LANE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP0_PEEK_LANE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTERP0_PEEK_LANE0_SPEC>> for R {
    fn from(reader: crate::R<INTERP0_PEEK_LANE0_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Read LANE0 result, without altering any internal state (PEEK).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_peek_lane0](index.html) module"]
pub struct INTERP0_PEEK_LANE0_SPEC;
impl crate::RegisterSpec for INTERP0_PEEK_LANE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp0_peek_lane0::R](R) reader structure"]
impl crate::Readable for INTERP0_PEEK_LANE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERP0_PEEK_LANE0 to value 0"]
impl crate::Resettable for INTERP0_PEEK_LANE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
