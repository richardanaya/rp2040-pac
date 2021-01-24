#[doc = "Register `INSTR_MEM23` reader"]
pub struct R(crate::R<INSTR_MEM23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSTR_MEM23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INSTR_MEM23_SPEC>> for R {
    fn from(reader: crate::R<INSTR_MEM23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INSTR_MEM23` writer"]
pub struct W(crate::W<INSTR_MEM23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSTR_MEM23_SPEC>;
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
impl core::convert::From<crate::W<INSTR_MEM23_SPEC>> for W {
    fn from(writer: crate::W<INSTR_MEM23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR_MEM23` reader - "]
pub struct INSTR_MEM23_R(crate::FieldReader<u16, u16>);
impl INSTR_MEM23_R {
    pub(crate) fn new(bits: u16) -> Self {
        INSTR_MEM23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTR_MEM23_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSTR_MEM23` writer - "]
pub struct INSTR_MEM23_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR_MEM23_W<'a> {
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
    pub fn instr_mem23(&self) -> INSTR_MEM23_R {
        INSTR_MEM23_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn instr_mem23(&mut self) -> INSTR_MEM23_W {
        INSTR_MEM23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write-only access to instruction memory location 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem23](index.html) module"]
pub struct INSTR_MEM23_SPEC;
impl crate::RegisterSpec for INSTR_MEM23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [instr_mem23::R](R) reader structure"]
impl crate::Readable for INSTR_MEM23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [instr_mem23::W](W) writer structure"]
impl crate::Writable for INSTR_MEM23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INSTR_MEM23 to value 0"]
impl crate::Resettable for INSTR_MEM23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
