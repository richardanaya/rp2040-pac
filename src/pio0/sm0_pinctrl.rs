#[doc = "Register `SM0_PINCTRL` reader"]
pub struct R(crate::R<SM0_PINCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM0_PINCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SM0_PINCTRL_SPEC>> for R {
    fn from(reader: crate::R<SM0_PINCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM0_PINCTRL` writer"]
pub struct W(crate::W<SM0_PINCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM0_PINCTRL_SPEC>;
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
impl core::convert::From<crate::W<SM0_PINCTRL_SPEC>> for W {
    fn from(writer: crate::W<SM0_PINCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIDESET_COUNT` reader - The number of delay bits co-opted for side-set. Inclusive of the enable bit, if present."]
pub struct SIDESET_COUNT_R(crate::FieldReader<u8, u8>);
impl SIDESET_COUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIDESET_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIDESET_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIDESET_COUNT` writer - The number of delay bits co-opted for side-set. Inclusive of the enable bit, if present."]
pub struct SIDESET_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDESET_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Field `SET_COUNT` reader - The number of pins asserted by a SET. Max of 5"]
pub struct SET_COUNT_R(crate::FieldReader<u8, u8>);
impl SET_COUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SET_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET_COUNT` writer - The number of pins asserted by a SET. Max of 5"]
pub struct SET_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Field `OUT_COUNT` reader - The number of pins asserted by an OUT. Value of 0 -> 32 pins"]
pub struct OUT_COUNT_R(crate::FieldReader<u8, u8>);
impl OUT_COUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUT_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_COUNT` writer - The number of pins asserted by an OUT. Value of 0 -> 32 pins"]
pub struct OUT_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "Field `IN_BASE` reader - The virtual pin corresponding to IN bit 0"]
pub struct IN_BASE_R(crate::FieldReader<u8, u8>);
impl IN_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        IN_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_BASE` writer - The virtual pin corresponding to IN bit 0"]
pub struct IN_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `SIDESET_BASE` reader - The virtual pin corresponding to delay field bit 0"]
pub struct SIDESET_BASE_R(crate::FieldReader<u8, u8>);
impl SIDESET_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIDESET_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIDESET_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIDESET_BASE` writer - The virtual pin corresponding to delay field bit 0"]
pub struct SIDESET_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDESET_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `SET_BASE` reader - The virtual pin corresponding to SET bit 0"]
pub struct SET_BASE_R(crate::FieldReader<u8, u8>);
impl SET_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SET_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET_BASE` writer - The virtual pin corresponding to SET bit 0"]
pub struct SET_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `OUT_BASE` reader - The virtual pin corresponding to OUT bit 0"]
pub struct OUT_BASE_R(crate::FieldReader<u8, u8>);
impl OUT_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUT_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_BASE` writer - The virtual pin corresponding to OUT bit 0"]
pub struct OUT_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - The number of delay bits co-opted for side-set. Inclusive of the enable bit, if present."]
    #[inline(always)]
    pub fn sideset_count(&self) -> SIDESET_COUNT_R {
        SIDESET_COUNT_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - The number of pins asserted by a SET. Max of 5"]
    #[inline(always)]
    pub fn set_count(&self) -> SET_COUNT_R {
        SET_COUNT_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 20:25 - The number of pins asserted by an OUT. Value of 0 -> 32 pins"]
    #[inline(always)]
    pub fn out_count(&self) -> OUT_COUNT_R {
        OUT_COUNT_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 15:19 - The virtual pin corresponding to IN bit 0"]
    #[inline(always)]
    pub fn in_base(&self) -> IN_BASE_R {
        IN_BASE_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - The virtual pin corresponding to delay field bit 0"]
    #[inline(always)]
    pub fn sideset_base(&self) -> SIDESET_BASE_R {
        SIDESET_BASE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The virtual pin corresponding to SET bit 0"]
    #[inline(always)]
    pub fn set_base(&self) -> SET_BASE_R {
        SET_BASE_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - The virtual pin corresponding to OUT bit 0"]
    #[inline(always)]
    pub fn out_base(&self) -> OUT_BASE_R {
        OUT_BASE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - The number of delay bits co-opted for side-set. Inclusive of the enable bit, if present."]
    #[inline(always)]
    pub fn sideset_count(&mut self) -> SIDESET_COUNT_W {
        SIDESET_COUNT_W { w: self }
    }
    #[doc = "Bits 26:28 - The number of pins asserted by a SET. Max of 5"]
    #[inline(always)]
    pub fn set_count(&mut self) -> SET_COUNT_W {
        SET_COUNT_W { w: self }
    }
    #[doc = "Bits 20:25 - The number of pins asserted by an OUT. Value of 0 -> 32 pins"]
    #[inline(always)]
    pub fn out_count(&mut self) -> OUT_COUNT_W {
        OUT_COUNT_W { w: self }
    }
    #[doc = "Bits 15:19 - The virtual pin corresponding to IN bit 0"]
    #[inline(always)]
    pub fn in_base(&mut self) -> IN_BASE_W {
        IN_BASE_W { w: self }
    }
    #[doc = "Bits 10:14 - The virtual pin corresponding to delay field bit 0"]
    #[inline(always)]
    pub fn sideset_base(&mut self) -> SIDESET_BASE_W {
        SIDESET_BASE_W { w: self }
    }
    #[doc = "Bits 5:9 - The virtual pin corresponding to SET bit 0"]
    #[inline(always)]
    pub fn set_base(&mut self) -> SET_BASE_W {
        SET_BASE_W { w: self }
    }
    #[doc = "Bits 0:4 - The virtual pin corresponding to OUT bit 0"]
    #[inline(always)]
    pub fn out_base(&mut self) -> OUT_BASE_W {
        OUT_BASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "State machine pin control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm0_pinctrl](index.html) module"]
pub struct SM0_PINCTRL_SPEC;
impl crate::RegisterSpec for SM0_PINCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm0_pinctrl::R](R) reader structure"]
impl crate::Readable for SM0_PINCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm0_pinctrl::W](W) writer structure"]
impl crate::Writable for SM0_PINCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SM0_PINCTRL to value 0x1400_0000"]
impl crate::Resettable for SM0_PINCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1400_0000
    }
}
