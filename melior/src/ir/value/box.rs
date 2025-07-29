use super::{Value, ValueLike};
use crate::utility::print_callback;
use mlir_sys::{mlirValueEqual, mlirValuePrint};
use std::{
    ffi::c_void,
    fmt::{self, Debug, Display, Formatter},
};

impl Display for Box<dyn ValueLike<'_>> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let mut data = (formatter, Ok(()));

        unsafe {
            mlirValuePrint(
                self.to_raw(),
                Some(print_callback),
                &mut data as *mut _ as *mut c_void,
            );
        }

        data.1
    }
}

impl Debug for Box<dyn ValueLike<'_>> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "Value(")?;
        Display::fmt(self, formatter)?;
        write!(formatter, ")")
    }
}

impl PartialEq for Box<dyn ValueLike<'_>> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirValueEqual(self.to_raw(), other.to_raw()) }
    }
}

impl Eq for Box<dyn ValueLike<'_>> {}

impl<'c> From<Box<dyn ValueLike<'c>>> for Value<'c, '_> {
    fn from(value: Box<dyn ValueLike<'c>>) -> Self {
        unsafe { Self::from_raw(value.to_raw()) }
    }
}
