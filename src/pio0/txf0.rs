#[doc = "Register `TXF0` writer"]
pub struct W(crate::W<TXF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXF0_SPEC>;
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
impl core::convert::From<crate::W<TXF0_SPEC>> for W {
    fn from(writer: crate::W<TXF0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txf0](index.html) module"]
pub struct TXF0_SPEC;
impl crate::RegisterSpec for TXF0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txf0::W](W) writer structure"]
impl crate::Writable for TXF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXF0 to value 0"]
impl crate::Resettable for TXF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
