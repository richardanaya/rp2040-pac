#[doc = "Register `PERFSEL3` reader"]
pub struct R(crate::R<PERFSEL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFSEL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PERFSEL3_SPEC>> for R {
    fn from(reader: crate::R<PERFSEL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERFSEL3` writer"]
pub struct W(crate::W<PERFSEL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFSEL3_SPEC>;
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
impl core::convert::From<crate::W<PERFSEL3_SPEC>> for W {
    fn from(writer: crate::W<PERFSEL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERFSEL3` reader - Select a performance event for PERFCTR3"]
pub struct PERFSEL3_R(crate::FieldReader<u8, u8>);
impl PERFSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERFSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERFSEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERFSEL3` writer - Select a performance event for PERFCTR3"]
pub struct PERFSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR3"]
    #[inline(always)]
    pub fn perfsel3(&self) -> PERFSEL3_R {
        PERFSEL3_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR3"]
    #[inline(always)]
    pub fn perfsel3(&mut self) -> PERFSEL3_W {
        PERFSEL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus fabric performance event select for PERFCTR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfsel3](index.html) module"]
pub struct PERFSEL3_SPEC;
impl crate::RegisterSpec for PERFSEL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfsel3::R](R) reader structure"]
impl crate::Readable for PERFSEL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perfsel3::W](W) writer structure"]
impl crate::Writable for PERFSEL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERFSEL3 to value 0x1f"]
impl crate::Resettable for PERFSEL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
