//! API versioning.

use entry::{EntryV100, EntryV110};

/// Available versions of the RenderDoc API.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Version {
    /// Version 1.0.0.
    V100 = 10000,
    /// Version 1.0.1.
    V101 = 10001,
    /// Version 1.0.2.
    V102 = 10002,
    /// Version 1.1.0.
    V110 = 10100,
    /// Version 1.1.1.
    V111 = 10101,
}

/// Initializes a new instance of the RenderDoc API.
///
/// # Safety
///
/// This function is not thread-safe and should not be called on multiple
/// threads at once.
type GetApiFn<T> = unsafe extern "C" fn(ver: Version, out: *mut *mut T) -> i32;

/// Entry point into the RenderDoc API.
pub trait ApiVersion {
    /// Minimum compatible version number.
    const VERSION: Version;

    /// Entry point struct.
    type Entry: Clone;

    /// Initializes a new instance of the RenderDoc API.
    ///
    /// # Safety
    ///
    /// This function is not thread-safe and should not be called on multiple
    /// threads at once.
    fn load() -> Result<Self::Entry, String> {
        use std::{mem, ptr};

        let api = unsafe {
            let get_api = match *super::RD_LIB {
                Ok(ref lib) => {
                    let f = lib.symbol::<()>("RENDERDOC_GetAPI")?;
                    Ok(mem::transmute::<_, GetApiFn<Self::Entry>>(f))
                }
                Err(ref err) => Err(err.to_string()),
            }?;

            let mut obj = ptr::null_mut();
            match get_api(Self::VERSION, &mut obj) {
                1 => ptr::read(obj),
                _ => Err("Compatible API version not available.")?,
            }
        };

        Ok(api)
    }
}

/// Requests a minimum version number of 1.0.0.
pub enum V100 {}

impl ApiVersion for V100 {
    const VERSION: Version = Version::V100;

    type Entry = EntryV100;
}

/// Requests a minimum version number of 1.1.0.
pub enum V110 {}

impl ApiVersion for V110 {
    const VERSION: Version = Version::V110;

    type Entry = EntryV110;
}
