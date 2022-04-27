/// Call a C function that accepts a `const char *`.
///
/// The signature for the C function (found in `bindings.rs` in the `target/` directory) is:
/// ```rs
/// pub fn cmodule_pass_const_char_ptr(ptr: *const ::std::os::raw::c_char);
/// ```

fn main() {
    println!("See the tests");
}

/// A collection of examples/experiments.
/// Each experiment is run as a unit tests.
/// While some tests to verify an assertion,
/// most tests aren't really "tests" - there is no meaningful assertion.
/// These "tests" generally should:
/// * compile
/// * not crash when run
/// * print to the console
#[cfg(test)]
mod test {
    /// Examples using `CStr::from_bytes_with_nul()`.
    /// The source bytes must be nul terminated!
    /// A `CStr` references existing memory.
    mod cstr_from_bytes_with_nul {
        use std::ffi::CStr;

        #[test]
        fn test_bytes_must_end_with_nul() {
            let byte_slice = b"The byte slice must end with a nul character";
            let result = CStr::from_bytes_with_nul(byte_slice);
            assert!(result.is_err());
        }

        #[test]
        fn test_from_byte_slice() {
            let byte_slice = b"A byte slice with nul that is owned by Rust\0";
            let cstr = CStr::from_bytes_with_nul(byte_slice).unwrap();
            let raw_ptr = cstr.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }

        #[test]
        fn test_from_vec_u8() {
            let byte_vec = b"A vector of bytes with nul that is owned by Rust\0".to_vec();
            // The Rust compiler automatically can convert &Vec to a slice
            let cstr = CStr::from_bytes_with_nul(&byte_vec).unwrap();
            let raw_ptr = cstr.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }

        #[test]
        fn test_from_vec_u8_as_slice() {
            let byte_vec = b"A vector of bytes with nul that is owned by Rust\0".to_vec();
            // It's ok but unnecessary to convert the Vec to a slice;
            // the Rust compiler does this automatically
            let byte_slice = byte_vec.as_slice();
            let cstr = CStr::from_bytes_with_nul(byte_slice).unwrap();
            let raw_ptr = cstr.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }

        #[test]
        fn test_from_string() {
            let string = String::from("An String with nul that is owned by Rust\0");
            let bytes_slice = string.as_bytes();
            let cstr = CStr::from_bytes_with_nul(bytes_slice).unwrap();
            let raw_ptr = cstr.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }

        // TODO force a lifetime error!
    }

    /// Examples using `CString::from_bytes_with_nul()`.
    /// A `CString` allocates new memory.
    /// The source bytes must be nul terminated!
    mod cstring_from_vec_with_nul {
        use std::ffi::CString;

        #[test]
        fn test_bytes_must_end_with_nul() {
            let byte_slice = b"The byte slice must end with a nul character".to_vec();
            let result = CString::from_vec_with_nul(byte_slice);
            assert!(result.is_err());
        }

        #[test]
        fn test_from_vec_u8() {
            let rust_bytes = b"A byte slice with nul that is owned by Rust\0".to_vec();
            let cstring = CString::from_vec_with_nul(rust_bytes).unwrap();
            let ptr = cstring.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(ptr);
            }
        }

        #[test]
        fn test_from_byte_slice() {
            let byte_slice = b"A byte slice with nul that is owned by Rust\0";
            let byte_vec = byte_slice.to_vec();
            let cstring = CString::from_vec_with_nul(byte_vec).unwrap();
            let ptr = cstring.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(ptr);
            }
        }
    }

    /// Examples using CString::new()
    /// The source bytes *must not* be nul terminated!
    ///
    /// A `CString` allocates new memory.
    ///
    /// [CString::new()](https://doc.rust-lang.org/std/ffi/struct.CString.html#method.new) requires
    /// that its argument can be converted into a `Vec<u8>` using the `Into` trait.
    /// This is apparent from the function signature: the type `T` is `T: Into<Vec<u8>`
    /// (that is, the type `T` must implement `Into<Vec<u8>>`).
    ///
    /// Put another way, the compiler must be able to call:
    /// ```
    /// let data = /* your type */;
    /// let vec_of_data: Vec<u8> = data.into();
    /// ```
    ///
    /// `CString::new()` requires that the bytes *do not* contain a nul terminator!
    mod cstring_new {
        use std::ffi::CString;

        #[test]
        fn test_bytes_must_not_end_with_nul() {
            let byte_vec = b"The byte slice must not end with nul\0".to_vec();
            let result = CString::new(byte_vec);
            assert!(result.is_err());
        }

        #[test]
        fn test_bytes_must_not_contain_nul() {
            let byte_vec = b"The byte slice must not contain\0 nul".to_vec();
            let result = CString::new(byte_vec);
            assert!(result.is_err());
        }

        #[test]
        fn test_from_vec_u8() {
            let byte_vec = b"A byte slice that is owned by Rust".to_vec();
            let cstring = CString::new(byte_vec).unwrap();
            let raw_ptr = cstring.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }

        #[test]
        fn test_from_byte_slice() {
            let byte_slice = b"A byte slice that is owned by Rust";
            let byte_vec = byte_slice.to_vec();
            let cstring = CString::new(byte_vec).unwrap();
            let raw_ptr = cstring.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }

        #[test]
        fn test_from_string() {
            let string = String::from("A String that is owned by Rust");
            // The Rust compiler automatically converts a String into a Vec<u8>
            let cstring = CString::new(string).unwrap();
            let raw_ptr = cstring.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }

        #[test]
        fn test_from_string_using_into() {
            let string = String::from("A String that is owned by Rust");
            // A String can be manually converted to Vec<u8> using the String type's implementation
            // of Into::into()
            let rust_bytes: Vec<u8> = string.into();
            let cstring = CString::new(rust_bytes).unwrap();
            let raw_ptr = cstring.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }

        #[test]
        fn test_from_string_using_turbofish() {
            let string = String::from("A String that is owned by Rust");
            // A String can be manually converted to Vec<u8> using the Into trait's into() method
            // and Rust's "turbofish" operator
            let rust_bytes = Into::<Vec<u8>>::into(string);
            let cstring = CString::new(rust_bytes).unwrap();
            let raw_ptr = cstring.as_ptr();
            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }
    }

    /// Directly convert types to raw pointers.
    /// **Warning!** This approach might not be sound!
    mod raw_ptr {
        #[test]
        fn test_from_string_with_typecast() {
            let rust_string = String::from("A String with nul that is owned by Rust\0");

            // This almost works but the pointer is of the wrong type
            // This example works when forced using a typecast, but this could be a very bad idea...
            let raw_ptr = rust_string.as_ptr() as *const i8;

            unsafe {
                rust_bindings_demo::cmodule_pass_const_char_ptr(raw_ptr);
            }
        }
    }
}
