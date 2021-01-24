#[doc = "Register `SPINLOCK8` reader"]
pub struct R(crate::R<SPINLOCK8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPINLOCK8_SPEC>> for R {
    fn from(reader: crate::R<SPINLOCK8_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock8](index.html) module"]
pub struct SPINLOCK8_SPEC;
impl crate::RegisterSpec for SPINLOCK8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock8::R](R) reader structure"]
impl crate::Readable for SPINLOCK8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPINLOCK8 to value 0"]
impl crate::Resettable for SPINLOCK8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
