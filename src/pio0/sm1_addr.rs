#[doc = "Register `SM1_ADDR` reader"]
pub struct R(crate::R<SM1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SM1_ADDR_SPEC>> for R {
    fn from(reader: crate::R<SM1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SM1_ADDR` reader - "]
pub struct SM1_ADDR_R(crate::FieldReader<u8, u8>);
impl SM1_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SM1_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sm1_addr(&self) -> SM1_ADDR_R {
        SM1_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Current instruction address of state machine 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_addr](index.html) module"]
pub struct SM1_ADDR_SPEC;
impl crate::RegisterSpec for SM1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm1_addr::R](R) reader structure"]
impl crate::Readable for SM1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SM1_ADDR to value 0"]
impl crate::Resettable for SM1_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
