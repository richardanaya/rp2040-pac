#[doc = "Register `CH4_TOP` reader"]
pub struct R(crate::R<CH4_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH4_TOP_SPEC>> for R {
    fn from(reader: crate::R<CH4_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4_TOP` writer"]
pub struct W(crate::W<CH4_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4_TOP_SPEC>;
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
impl core::convert::From<crate::W<CH4_TOP_SPEC>> for W {
    fn from(writer: crate::W<CH4_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH4_TOP` reader - "]
pub struct CH4_TOP_R(crate::FieldReader<u16, u16>);
impl CH4_TOP_R {
    pub(crate) fn new(bits: u16) -> Self {
        CH4_TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4_TOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4_TOP` writer - "]
pub struct CH4_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch4_top(&self) -> CH4_TOP_R {
        CH4_TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch4_top(&mut self) -> CH4_TOP_W {
        CH4_TOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter wrap value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_top](index.html) module"]
pub struct CH4_TOP_SPEC;
impl crate::RegisterSpec for CH4_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4_top::R](R) reader structure"]
impl crate::Readable for CH4_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4_top::W](W) writer structure"]
impl crate::Writable for CH4_TOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH4_TOP to value 0xffff"]
impl crate::Resettable for CH4_TOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
