#[doc = "Register `RXF0` reader"]
pub struct R(crate::R<RXF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RXF0_SPEC>> for R {
    fn from(reader: crate::R<RXF0_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0](index.html) module"]
pub struct RXF0_SPEC;
impl crate::RegisterSpec for RXF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf0::R](R) reader structure"]
impl crate::Readable for RXF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXF0 to value 0"]
impl crate::Resettable for RXF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
