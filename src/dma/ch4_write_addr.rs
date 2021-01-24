#[doc = "Register `CH4_WRITE_ADDR` reader"]
pub struct R(crate::R<CH4_WRITE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4_WRITE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CH4_WRITE_ADDR_SPEC>> for R {
    fn from(reader: crate::R<CH4_WRITE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4_WRITE_ADDR` writer"]
pub struct W(crate::W<CH4_WRITE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4_WRITE_ADDR_SPEC>;
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
impl core::convert::From<crate::W<CH4_WRITE_ADDR_SPEC>> for W {
    fn from(writer: crate::W<CH4_WRITE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
impl R {}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 4 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_write_addr](index.html) module"]
pub struct CH4_WRITE_ADDR_SPEC;
impl crate::RegisterSpec for CH4_WRITE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4_write_addr::R](R) reader structure"]
impl crate::Readable for CH4_WRITE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4_write_addr::W](W) writer structure"]
impl crate::Writable for CH4_WRITE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH4_WRITE_ADDR to value 0"]
impl crate::Resettable for CH4_WRITE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
