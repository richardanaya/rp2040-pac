#[doc = "Register `CH9_AL1_CTRL` reader"]
pub struct R(crate::R<CH9_AL1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH9_AL1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH9_AL1_CTRL_SPEC>> for R {
    fn from(reader: crate::R<CH9_AL1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Alias for channel 9 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al1_ctrl](index.html) module"]
pub struct CH9_AL1_CTRL_SPEC;
impl crate::RegisterSpec for CH9_AL1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch9_al1_ctrl::R](R) reader structure"]
impl crate::Readable for CH9_AL1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH9_AL1_CTRL to value 0"]
impl crate::Resettable for CH9_AL1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
