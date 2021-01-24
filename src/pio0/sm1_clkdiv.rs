#[doc = "Register `SM1_CLKDIV` reader"]
pub struct R(crate::R<SM1_CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM1_CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SM1_CLKDIV_SPEC>> for R {
    fn from(reader: crate::R<SM1_CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM1_CLKDIV` writer"]
pub struct W(crate::W<SM1_CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM1_CLKDIV_SPEC>;
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
impl core::convert::From<crate::W<SM1_CLKDIV_SPEC>> for W {
    fn from(writer: crate::W<SM1_CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - Effective frequency is sysclk/int.\\n Value of 0 is interpreted as max possible value"]
pub struct INT_R(crate::FieldReader<u16, u16>);
impl INT_R {
    pub(crate) fn new(bits: u16) -> Self {
        INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT` writer - Effective frequency is sysclk/int.\\n Value of 0 is interpreted as max possible value"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `FRAC` reader - Fractional part of clock divider"]
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
#[doc = "Field `FRAC` writer - Fractional part of clock divider"]
pub struct FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Effective frequency is sysclk/int.\\n Value of 0 is interpreted as max possible value"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - Fractional part of clock divider"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Effective frequency is sysclk/int.\\n Value of 0 is interpreted as max possible value"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bits 8:15 - Fractional part of clock divider"]
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
#[doc = "Clock divider register for state machine 1\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_clkdiv](index.html) module"]
pub struct SM1_CLKDIV_SPEC;
impl crate::RegisterSpec for SM1_CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm1_clkdiv::R](R) reader structure"]
impl crate::Readable for SM1_CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm1_clkdiv::W](W) writer structure"]
impl crate::Writable for SM1_CLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SM1_CLKDIV to value 0x0001_0000"]
impl crate::Resettable for SM1_CLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
