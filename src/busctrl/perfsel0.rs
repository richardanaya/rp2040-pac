#[doc = "Register `PERFSEL0` reader"]
pub struct R(crate::R<PERFSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PERFSEL0_SPEC>> for R {
    fn from(reader: crate::R<PERFSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERFSEL0` writer"]
pub struct W(crate::W<PERFSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFSEL0_SPEC>;
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
impl core::convert::From<crate::W<PERFSEL0_SPEC>> for W {
    fn from(writer: crate::W<PERFSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERFSEL0` reader - Select a performance event for PERFCTR0"]
pub struct PERFSEL0_R(crate::FieldReader<u8, u8>);
impl PERFSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERFSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERFSEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERFSEL0` writer - Select a performance event for PERFCTR0"]
pub struct PERFSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR0"]
    #[inline(always)]
    pub fn perfsel0(&self) -> PERFSEL0_R {
        PERFSEL0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR0"]
    #[inline(always)]
    pub fn perfsel0(&mut self) -> PERFSEL0_W {
        PERFSEL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus fabric performance event select for PERFCTR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfsel0](index.html) module"]
pub struct PERFSEL0_SPEC;
impl crate::RegisterSpec for PERFSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfsel0::R](R) reader structure"]
impl crate::Readable for PERFSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perfsel0::W](W) writer structure"]
impl crate::Writable for PERFSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERFSEL0 to value 0x1f"]
impl crate::Resettable for PERFSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
