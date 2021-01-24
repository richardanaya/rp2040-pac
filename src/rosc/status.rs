#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STATUS_SPEC>> for R {
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl core::convert::From<crate::W<STATUS_SPEC>> for W {
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STABLE` reader - Oscillator is running and stable"]
pub struct STABLE_R(crate::FieldReader<bool, bool>);
impl STABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BADWRITE` reader - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FRFEQA or FREQB or DORMANT"]
pub struct BADWRITE_R(crate::FieldReader<bool, bool>);
impl BADWRITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BADWRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BADWRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BADWRITE` writer - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FRFEQA or FREQB or DORMANT"]
pub struct BADWRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> BADWRITE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `DIV_RUNNING` reader - post-divider is running\\n this resets to 0 but transitions to 1 during chip startup"]
pub struct DIV_RUNNING_R(crate::FieldReader<bool, bool>);
impl DIV_RUNNING_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV_RUNNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_RUNNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` reader - Oscillator is enabled but not necessarily running and stable\\n this resets to 0 but transitions to 1 during chip startup"]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Oscillator is running and stable"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FRFEQA or FREQB or DORMANT"]
    #[inline(always)]
    pub fn badwrite(&self) -> BADWRITE_R {
        BADWRITE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - post-divider is running\\n this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn div_running(&self) -> DIV_RUNNING_R {
        DIV_RUNNING_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Oscillator is enabled but not necessarily running and stable\\n this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FRFEQA or FREQB or DORMANT"]
    #[inline(always)]
    pub fn badwrite(&mut self) -> BADWRITE_W {
        BADWRITE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ring Oscillator Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
