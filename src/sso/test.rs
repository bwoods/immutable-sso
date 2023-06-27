use super::*;
use std::ffi::NulError;

#[test]
fn assumptions() {
    assert_eq!(std::mem::size_of::<Storage>(), 16);
}

#[test]
fn boundary_strings() -> Result<(), NulError> {
    let storage = Storage::from_str("123456789012345")?;
    assert!(storage.is_inline());

    let storage = Storage::from_str("1234567890123456")?;
    assert!(storage.is_heap());
    Ok(())
}

#[test]
fn embedded_null_fails() {
    let storage = Storage::from_str("123456789\012345");
    assert!(storage.is_err());

    let storage = Storage::from_str("123456789\0123456");
    assert!(storage.is_err());
}

#[test]
fn empty_string_succeeds() -> Result<(), NulError> {
    let storage = Storage::from_str("")?;
    let result = storage.as_str();

    assert_eq!(result, "");
    Ok(())
}

#[test]
fn deref() -> Result<(), NulError> {
    let a = Storage::from_str("A")?;
    let b = Storage::from_str("B")?;

    assert_ne!(a.to_owned(), b.to_owned());
    Ok(())
}

#[test]
fn cmp() -> Result<(), NulError> {
    let a = Storage::from_str("A")?;
    let b = Storage::from_str("B")?;

    assert!(a < b);
    assert!(b > a);
    assert!(a == a);
    assert!(a != b);
    Ok(())
}

use quickcheck_macros::quickcheck;

#[quickcheck]
fn property_testing(mut string: String) -> Result<(), NulError> {
    string.retain(|ch| ch != '\0'); // NULs are a different test…

    let storage = Storage::from_str(string.as_str())?;
    let result = storage.as_str();

    assert_eq!(result, string);
    Ok(())
}
