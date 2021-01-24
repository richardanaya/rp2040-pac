#[doc = "Register `CH6_AL2_TRANS_COUNT` reader"]
pub struct R(crate::R<CH6_AL2_TRANS_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH6_AL2_TRANS_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH6_AL2_TRANS_COUNT_SPEC>> for R {
    fn from(reader: crate::R<CH6_AL2_TRANS_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Alias for channel 6 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al2_trans_count](index.html) module"]
pub struct CH6_AL2_TRANS_COUNT_SPEC;
impl crate::RegisterSpec for CH6_AL2_TRANS_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch6_al2_trans_count::R](R) reader structure"]
impl crate::Readable for CH6_AL2_TRANS_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH6_AL2_TRANS_COUNT to value 0"]
impl crate::Resettable for CH6_AL2_TRANS_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
