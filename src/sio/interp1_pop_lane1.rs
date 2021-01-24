#[doc = "Register `INTERP1_POP_LANE1` reader"]
pub struct R(crate::R<INTERP1_POP_LANE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP1_POP_LANE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTERP1_POP_LANE1_SPEC>> for R {
    fn from(reader: crate::R<INTERP1_POP_LANE1_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_pop_lane1](index.html) module"]
pub struct INTERP1_POP_LANE1_SPEC;
impl crate::RegisterSpec for INTERP1_POP_LANE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp1_pop_lane1::R](R) reader structure"]
impl crate::Readable for INTERP1_POP_LANE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERP1_POP_LANE1 to value 0"]
impl crate::Resettable for INTERP1_POP_LANE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
