#[doc = "Register `CH4_AL1_TRANS_COUNT_TRIG` reader"]
pub struct R(crate::R<CH4_AL1_TRANS_COUNT_TRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4_AL1_TRANS_COUNT_TRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH4_AL1_TRANS_COUNT_TRIG_SPEC>> for R {
    fn from(reader: crate::R<CH4_AL1_TRANS_COUNT_TRIG_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Alias for channel 4 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al1_trans_count_trig](index.html) module"]
pub struct CH4_AL1_TRANS_COUNT_TRIG_SPEC;
impl crate::RegisterSpec for CH4_AL1_TRANS_COUNT_TRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4_al1_trans_count_trig::R](R) reader structure"]
impl crate::Readable for CH4_AL1_TRANS_COUNT_TRIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH4_AL1_TRANS_COUNT_TRIG to value 0"]
impl crate::Resettable for CH4_AL1_TRANS_COUNT_TRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
