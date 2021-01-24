#[doc = "Register `CH1_DIV` reader"]
pub struct R(crate::R<CH1_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH1_DIV_SPEC>> for R {
    fn from(reader: crate::R<CH1_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1_DIV` writer"]
pub struct W(crate::W<CH1_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1_DIV_SPEC>;
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
impl core::convert::From<crate::W<CH1_DIV_SPEC>> for W {
    fn from(writer: crate::W<CH1_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - "]
pub struct INT_R(crate::FieldReader<u8, u8>);
impl INT_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT` writer - "]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Field `FRAC` reader - "]
pub struct FRAC_R(crate::FieldReader<u8, u8>);
impl FRAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAC` writer - "]
pub struct FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn frac(&mut self) -> FRAC_W {
        FRAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_div](index.html) module"]
pub struct CH1_DIV_SPEC;
impl crate::RegisterSpec for CH1_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1_div::R](R) reader structure"]
impl crate::Readable for CH1_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1_div::W](W) writer structure"]
impl crate::Writable for CH1_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH1_DIV to value 0x10"]
impl crate::Resettable for CH1_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
