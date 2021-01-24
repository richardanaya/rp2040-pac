#[doc = "Register `CH3_CTR` reader"]
pub struct R(crate::R<CH3_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH3_CTR_SPEC>> for R {
    fn from(reader: crate::R<CH3_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3_CTR` writer"]
pub struct W(crate::W<CH3_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3_CTR_SPEC>;
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
impl core::convert::From<crate::W<CH3_CTR_SPEC>> for W {
    fn from(writer: crate::W<CH3_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH3_CTR` reader - "]
pub struct CH3_CTR_R(crate::FieldReader<u16, u16>);
impl CH3_CTR_R {
    pub(crate) fn new(bits: u16) -> Self {
        CH3_CTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_CTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_CTR` writer - "]
pub struct CH3_CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_CTR_W<'a> {
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
    pub fn ch3_ctr(&self) -> CH3_CTR_R {
        CH3_CTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch3_ctr(&mut self) -> CH3_CTR_W {
        CH3_CTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct access to the PWM counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_ctr](index.html) module"]
pub struct CH3_CTR_SPEC;
impl crate::RegisterSpec for CH3_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3_ctr::R](R) reader structure"]
impl crate::Readable for CH3_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3_ctr::W](W) writer structure"]
impl crate::Writable for CH3_CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH3_CTR to value 0"]
impl crate::Resettable for CH3_CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
