#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA1_ITRIG_ENA_SET {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Proxy"]
pub struct _SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 32767;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:14 - Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA1_ITRIG_ENA register"]
    #[inline]
    pub fn set(&mut self) -> _SETW {
        _SETW { w: self }
    }
}
