#[doc = "Register `CH3_DBG_TCR` reader"]
pub struct R(crate::R<CH3_DBG_TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3_DBG_TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH3_DBG_TCR_SPEC>> for R {
    fn from(reader: crate::R<CH3_DBG_TCR_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_dbg_tcr](index.html) module"]
pub struct CH3_DBG_TCR_SPEC;
impl crate::RegisterSpec for CH3_DBG_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3_dbg_tcr::R](R) reader structure"]
impl crate::Readable for CH3_DBG_TCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH3_DBG_TCR to value 0"]
impl crate::Resettable for CH3_DBG_TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
