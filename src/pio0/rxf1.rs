#[doc = "Register `RXF1` reader"]
pub struct R(crate::R<RXF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RXF1_SPEC>> for R {
    fn from(reader: crate::R<RXF1_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1](index.html) module"]
pub struct RXF1_SPEC;
impl crate::RegisterSpec for RXF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf1::R](R) reader structure"]
impl crate::Readable for RXF1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXF1 to value 0"]
impl crate::Resettable for RXF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
