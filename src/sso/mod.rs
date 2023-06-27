use std::ffi::{CStr, CString, NulError};

mod traits;
pub use traits::*;

#[cfg(test)]
mod test;

const ALLOCATION: usize = 16;
const CAPACITY: usize = ALLOCATION - 1; // for the tag byte

/// A string representation that stores small strings inline,
/// rather than in heap-allocate memory.
#[repr(C)]
pub union Storage {
    bytes: [u8; ALLOCATION],
    words: [*const u8; 2],
}

impl Storage {
    /// # Safety
    ///
    /// This constructor sets up the preconditions that all other `unsafe` code
    /// relies on,
    ///
    /// # Errors
    ///
    /// If the `str` parameter contains a `NUL` character [`NulError`] is returned.
    ///
    pub fn from_str(str: &str) -> Result<Storage, NulError> {
        let mut new = Self::default();
        let len = str.len();

        unsafe {
            if len > CAPACITY {
                let ptr = (CString::new(str)?.into_raw()) as *const u8;
                new.bytes[0] = ALLOCATION as u8;
                new.words[1] = ptr;
            } else {
                if str.contains('\0') {
                    CString::new(str)?; // returns NulError
                }

                new.bytes[0] = len as u8;
                std::ptr::copy_nonoverlapping(
                    str.as_bytes().as_ptr(),
                    new.bytes[1..].as_mut_ptr(),
                    len,
                );
            }
        }

        Ok(new)
    }

    /// # Safety
    ///
    /// The [`Storage`] must have been created with [`Storage::from_str()`]
    /// to guarantee that is is correctly tagged as `is_heap` or `is_inline`.
    ///
    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe {
            if self.is_heap() {
                std::str::from_utf8_unchecked(CStr::from_ptr(self.words[1] as *const i8).to_bytes())
            } else {
                let len = self.bytes[0] as usize + 1; // offset by the tag byte
                std::str::from_utf8_unchecked(&self.bytes[1..len])
            }
        }
    }

    #[inline]
    /// Returns whether elements are on heap.
    pub fn is_heap(&self) -> bool {
        unsafe { self.bytes[0] == ALLOCATION as u8 }
    }

    #[inline]
    /// Returns whether elements are held inline.
    pub fn is_inline(&self) -> bool {
        self.is_heap() == false
    }
}

impl Drop for Storage {
    /// # Safety
    ///
    /// The [`Storage`] must have been created with [`Storage::from_str()`]
    /// to guarantee that is is correctly tagged as `is_heap` or `is_inline`.    
    ///
    fn drop(&mut self) {
        unsafe {
            if self.is_heap() {
                let _ = CString::from_raw(self.words[1] as *mut i8);
            }
        }
    }
}
