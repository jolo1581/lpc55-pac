#[doc = "Reader of register RES2"]
pub type R = crate::R<u32, super::RES2>;
#[doc = "Writer for register RES2"]
pub type W = crate::W<u32, super::RES2>;
#[doc = "Register RES2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RES2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REG_VALUE`"]
pub type REG_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REG_VALUE`"]
pub struct REG_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&self) -> REG_VALUE_R {
        REG_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&mut self) -> REG_VALUE_W {
        REG_VALUE_W { w: self }
    }
}
