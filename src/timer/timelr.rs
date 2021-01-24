#[doc = "Register `TIMELR` reader"]
pub struct R(crate::R<TIMELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TIMELR_SPEC>> for R {
    fn from(reader: crate::R<TIMELR_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Read from bits 31:0 of time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timelr](index.html) module"]
pub struct TIMELR_SPEC;
impl crate::RegisterSpec for TIMELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timelr::R](R) reader structure"]
impl crate::Readable for TIMELR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMELR to value 0"]
impl crate::Resettable for TIMELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
