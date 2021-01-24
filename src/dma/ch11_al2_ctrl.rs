#[doc = "Register `CH11_AL2_CTRL` reader"]
pub struct R(crate::R<CH11_AL2_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH11_AL2_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH11_AL2_CTRL_SPEC>> for R {
    fn from(reader: crate::R<CH11_AL2_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Alias for channel 11 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al2_ctrl](index.html) module"]
pub struct CH11_AL2_CTRL_SPEC;
impl crate::RegisterSpec for CH11_AL2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch11_al2_ctrl::R](R) reader structure"]
impl crate::Readable for CH11_AL2_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH11_AL2_CTRL to value 0"]
impl crate::Resettable for CH11_AL2_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
