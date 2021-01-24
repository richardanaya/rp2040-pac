#[doc = "Register `CH7_AL1_READ_ADDR` reader"]
pub struct R(crate::R<CH7_AL1_READ_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH7_AL1_READ_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH7_AL1_READ_ADDR_SPEC>> for R {
    fn from(reader: crate::R<CH7_AL1_READ_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Alias for channel 7 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al1_read_addr](index.html) module"]
pub struct CH7_AL1_READ_ADDR_SPEC;
impl crate::RegisterSpec for CH7_AL1_READ_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch7_al1_read_addr::R](R) reader structure"]
impl crate::Readable for CH7_AL1_READ_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH7_AL1_READ_ADDR to value 0"]
impl crate::Resettable for CH7_AL1_READ_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
