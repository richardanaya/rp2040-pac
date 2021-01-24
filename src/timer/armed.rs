#[doc = "Register `ARMED` reader"]
pub struct R(crate::R<ARMED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARMED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ARMED_SPEC>> for R {
    fn from(reader: crate::R<ARMED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARMED` writer"]
pub struct W(crate::W<ARMED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARMED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<ARMED_SPEC>> for W {
    fn from(writer: crate::W<ARMED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARMED` reader - "]
pub struct ARMED_R(crate::FieldReader<u8, u8>);
impl ARMED_R {
    pub(crate) fn new(bits: u8) -> Self {
        ARMED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARMED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARMED` writer - "]
pub struct ARMED_W<'a> {
    w: &'a mut W,
}
impl<'a> ARMED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn armed(&self) -> ARMED_R {
        ARMED_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn armed(&mut self) -> ARMED_W {
        ARMED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicates the armed/disarmed status of each alarm.\\n A write to the corresponding ALARMx register arms the alarm.\\n Alarms automatically disarm upon firing, but writing ones here\\n will disarm immediately without waiting to fire.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [armed](index.html) module"]
pub struct ARMED_SPEC;
impl crate::RegisterSpec for ARMED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [armed::R](R) reader structure"]
impl crate::Readable for ARMED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [armed::W](W) writer structure"]
impl crate::Writable for ARMED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARMED to value 0"]
impl crate::Resettable for ARMED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
