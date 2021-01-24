#[doc = "Register `CLK_REF_SELECTED` reader"]
pub struct R(crate::R<CLK_REF_SELECTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REF_SELECTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_REF_SELECTED_SPEC>> for R {
    fn from(reader: crate::R<CLK_REF_SELECTED_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ref_selected](index.html) module"]
pub struct CLK_REF_SELECTED_SPEC;
impl crate::RegisterSpec for CLK_REF_SELECTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ref_selected::R](R) reader structure"]
impl crate::Readable for CLK_REF_SELECTED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_REF_SELECTED to value 0x01"]
impl crate::Resettable for CLK_REF_SELECTED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
