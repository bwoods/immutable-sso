use std::ffi::{CStr, CString, NulError};
use std::str::FromStr;

mod traits;
pub use traits::*;

#[cfg(test)]
mod test;

const ALLOCATION: usize = 16;
const CAPACITY: usize = ALLOCATION - 1; // for the tag byte

/// A string representation that stores small strings inline,
/// rather than in heap-allocate memory.
///
/// # Examples
///
/// You can create `Storage` from a [string slice][`std::str`] with [`Storage::from_str`]:
///
///
/// ```ignore
/// use std::str::FromStr;
///
/// let key = sso::Storage::from_str("blue")?;
/// ```
#[repr(C)]
pub union Storage {
    bytes: [u8; ALLOCATION],
    words: [*const u8; 2],
}

impl FromStr for Storage {
    type Err = NulError;

    /// # Safety
    ///
    /// This constructor sets up the preconditions that all other `unsafe` code
    /// relies on,
    ///
    /// # Errors
    ///
    /// If the `str` parameter contains a NUL character [`NulError`] is returned.
    ///
    fn from_str(s: &str) -> Result<Self, NulError> {
        let mut new = Self::default();
        let len = s.len();

        unsafe {
            if len > CAPACITY {
                let ptr = (CString::new(s)?.into_raw()) as *const u8;
                new.bytes[0] = ALLOCATION as u8;
                new.words[1] = ptr;
            } else {
                if s.contains('\0') {
                    CString::new(s)?; // returns NulError
                }

                new.bytes[0] = len as u8;
                std::ptr::copy_nonoverlapping(
                    s.as_bytes().as_ptr(),
                    new.bytes[1..].as_mut_ptr(),
                    len,
                );
            }
        }

        Ok(new)
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

impl Storage {
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
