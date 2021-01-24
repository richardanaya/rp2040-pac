#[doc = "Register `CH3_AL3_WRITE_ADDR` reader"]
pub struct R(crate::R<CH3_AL3_WRITE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3_AL3_WRITE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH3_AL3_WRITE_ADDR_SPEC>> for R {
    fn from(reader: crate::R<CH3_AL3_WRITE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Alias for channel 3 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al3_write_addr](index.html) module"]
pub struct CH3_AL3_WRITE_ADDR_SPEC;
impl crate::RegisterSpec for CH3_AL3_WRITE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3_al3_write_addr::R](R) reader structure"]
impl crate::Readable for CH3_AL3_WRITE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH3_AL3_WRITE_ADDR to value 0"]
impl crate::Resettable for CH3_AL3_WRITE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
