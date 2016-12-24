macro_rules! impl_simple_alobject {
    ($name:ident, $is:ident $(, $name_str:expr)*) => {
        impl ALObject for $name {
            #[inline(always)]
            fn raw(&self) -> ALuint { self.0 }

            #[inline(always)]
            fn into_raw(mut self) -> ALuint {
                mem::replace(&mut self.0, 0)
            }

            #[inline(always)]
            fn is_valid(&self) -> bool {
                AL_TRUE == unsafe { $is(self.0) }
            }

            #[inline(always)]
            fn check(&self) -> ALResult<()> {
                if self.is_valid() { Ok(()) } else {
                    $(println!("Invalid {}", $name_str);)*
                    Err(ALError::InvalidValue)
                }
            }
        }
    }
}

macro_rules! impl_simple_alobject_struct {
    ($name:ident, $is:ident $(, $name_str:expr)*) => {
        impl ALObject for $name {
            #[inline(always)]
            fn raw(&self) -> ALuint { self.raw }

            #[inline(always)]
            fn into_raw(mut self) -> ALuint {
                mem::replace(&mut self.raw, 0)
            }

            #[inline(always)]
            fn is_valid(&self) -> bool {
                AL_TRUE == unsafe { $is(self.raw) }
            }

            #[inline(always)]
            fn check(&self) -> ALResult<()> {
                if self.is_valid() { Ok(()) } else {
                    $(println!("Invalid {}", $name_str);)*
                    Err(ALError::InvalidValue)
                }
            }
        }
    }
}