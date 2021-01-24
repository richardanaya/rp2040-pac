#[doc = "Register `PERFSEL2` reader"]
pub struct R(crate::R<PERFSEL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFSEL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PERFSEL2_SPEC>> for R {
    fn from(reader: crate::R<PERFSEL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERFSEL2` writer"]
pub struct W(crate::W<PERFSEL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFSEL2_SPEC>;
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
impl core::convert::From<crate::W<PERFSEL2_SPEC>> for W {
    fn from(writer: crate::W<PERFSEL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERFSEL2` reader - Select a performance event for PERFCTR2"]
pub struct PERFSEL2_R(crate::FieldReader<u8, u8>);
impl PERFSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERFSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERFSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERFSEL2` writer - Select a performance event for PERFCTR2"]
pub struct PERFSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR2"]
    #[inline(always)]
    pub fn perfsel2(&self) -> PERFSEL2_R {
        PERFSEL2_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR2"]
    #[inline(always)]
    pub fn perfsel2(&mut self) -> PERFSEL2_W {
        PERFSEL2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus fabric performance event select for PERFCTR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfsel2](index.html) module"]
pub struct PERFSEL2_SPEC;
impl crate::RegisterSpec for PERFSEL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfsel2::R](R) reader structure"]
impl crate::Readable for PERFSEL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perfsel2::W](W) writer structure"]
impl crate::Writable for PERFSEL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERFSEL2 to value 0x1f"]
impl crate::Resettable for PERFSEL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
