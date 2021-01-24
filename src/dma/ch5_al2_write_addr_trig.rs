#[doc = "Register `CH5_AL2_WRITE_ADDR_TRIG` reader"]
pub struct R(crate::R<CH5_AL2_WRITE_ADDR_TRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5_AL2_WRITE_ADDR_TRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH5_AL2_WRITE_ADDR_TRIG_SPEC>> for R {
    fn from(reader: crate::R<CH5_AL2_WRITE_ADDR_TRIG_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Alias for channel 5 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al2_write_addr_trig](index.html) module"]
pub struct CH5_AL2_WRITE_ADDR_TRIG_SPEC;
impl crate::RegisterSpec for CH5_AL2_WRITE_ADDR_TRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5_al2_write_addr_trig::R](R) reader structure"]
impl crate::Readable for CH5_AL2_WRITE_ADDR_TRIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH5_AL2_WRITE_ADDR_TRIG to value 0"]
impl crate::Resettable for CH5_AL2_WRITE_ADDR_TRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
