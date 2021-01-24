#[doc = "Register `CH3_CC` reader"]
pub struct R(crate::R<CH3_CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3_CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH3_CC_SPEC>> for R {
    fn from(reader: crate::R<CH3_CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3_CC` writer"]
pub struct W(crate::W<CH3_CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3_CC_SPEC>;
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
impl core::convert::From<crate::W<CH3_CC_SPEC>> for W {
    fn from(writer: crate::W<CH3_CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B` reader - "]
pub struct B_R(crate::FieldReader<u16, u16>);
impl B_R {
    pub(crate) fn new(bits: u16) -> Self {
        B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B` writer - "]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `A` reader - "]
pub struct A_R(crate::FieldReader<u16, u16>);
impl A_R {
    pub(crate) fn new(bits: u16) -> Self {
        A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A` writer - "]
pub struct A_W<'a> {
    w: &'a mut W,
}
impl<'a> A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn a(&mut self) -> A_W {
        A_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter compare values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_cc](index.html) module"]
pub struct CH3_CC_SPEC;
impl crate::RegisterSpec for CH3_CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3_cc::R](R) reader structure"]
impl crate::Readable for CH3_CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3_cc::W](W) writer structure"]
impl crate::Writable for CH3_CC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH3_CC to value 0"]
impl crate::Resettable for CH3_CC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
