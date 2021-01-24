#[doc = "Register `SM0_ADDR` reader"]
pub struct R(crate::R<SM0_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM0_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SM0_ADDR_SPEC>> for R {
    fn from(reader: crate::R<SM0_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SM0_ADDR` reader - "]
pub struct SM0_ADDR_R(crate::FieldReader<u8, u8>);
impl SM0_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SM0_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sm0_addr(&self) -> SM0_ADDR_R {
        SM0_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Current instruction address of state machine 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm0_addr](index.html) module"]
pub struct SM0_ADDR_SPEC;
impl crate::RegisterSpec for SM0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm0_addr::R](R) reader structure"]
impl crate::Readable for SM0_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SM0_ADDR to value 0"]
impl crate::Resettable for SM0_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
