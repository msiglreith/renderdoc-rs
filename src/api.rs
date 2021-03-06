//! Traits providing compile-time API functionality.

use {CaptureOption, DevicePointer, OverlayBits, InputButton, WindowHandle};
use entry::{EntryV100, EntryV110};

use std::ffi::{CStr, CString};
use std::mem;
use std::path::Path;

/// Base implementation of API version 1.0.0.
pub trait RenderDocV100: Sized {
    /// Returns the raw `EntryV100` entry point struct.
    unsafe fn entry_v100(&self) -> &EntryV100;

    /// Provides the major, minor, and patch version numbers of the RenderDoc
    /// API given to the application.
    ///
    /// Note that RenderDoc will usually provide a higher API version than the
    /// one requested by the user if it's backwards compatible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::{RenderDoc, V100};
    /// # use renderdoc::prelude::*;
    /// # fn init() -> Result<(), String> {
    /// # let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// let (major, minor, patch) = renderdoc.get_api_version();
    /// assert_eq!(major, 1u32);
    /// assert_eq!(minor, 0u32);
    /// # Ok(())
    /// # }
    /// ```
    fn get_api_version(&self) -> (u32, u32, u32) {
        unsafe {
            let (mut major, mut minor, mut patch) = (0, 0, 0);
            (self.entry_v100().get_api_version)(&mut major, &mut minor, &mut patch);
            (major as u32, minor as u32, patch as u32)
        }
    }

    /// Sets the specified `CaptureOption` to the given `f32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option and/or the value are invalid.
    fn set_capture_option_f32(&mut self, opt: CaptureOption, val: f32) {
        let err = unsafe { (self.entry_v100().set_capture_option_f32)(opt, val) };
        assert_eq!(err, 1);
    }

    /// Sets the specified `CaptureOption` to the given `u32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option and/or the value are invalid.
    fn set_capture_option_u32(&mut self, opt: CaptureOption, val: u32) {
        let err = unsafe { (self.entry_v100().set_capture_option_u32)(opt, val) };
        assert_eq!(err, 1);
    }

    #[allow(missing_docs)]
    fn get_capture_option_f32(&self, opt: CaptureOption) -> f32 {
        use std::f32::MAX;
        let val = unsafe { (self.entry_v100().get_capture_option_f32)(opt) };
        assert_ne!(val, -MAX);
        val
    }

    #[allow(missing_docs)]
    fn get_capture_option_u32(&self, opt: CaptureOption) -> u32 {
        use std::u32::MAX;
        let val = unsafe { (self.entry_v100().get_capture_option_u32)(opt) };
        assert_ne!(val, MAX);
        val
    }

    #[allow(missing_docs)]
    fn set_capture_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let k: Vec<_> = keys.iter().cloned().map(|k| k.into()).collect();
            (self.entry_v100().set_capture_keys)(k.as_ptr(), k.len() as i32)
        }
    }

    #[allow(missing_docs)]
    fn set_focus_toggle_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let k: Vec<_> = keys.iter().cloned().map(|k| k.into()).collect();
            (self.entry_v100().set_focus_toggle_keys)(k.as_ptr(), k.len() as i32)
        }
    }

    /// Attempts to shut down RenderDoc.
    ///
    /// # Safety
    ///
    /// Note that this will work correctly if done _immediately_ after the
    /// dynamic library is loaded, before any API work happens. At that point,
    /// RenderDoc will remove its injected hooks and shut down. Behavior is
    /// undefined if this is called after any API functions have been called.
    unsafe fn shutdown(self) {
        (self.entry_v100().shutdown)();
    }

    #[allow(missing_docs)]
    fn unload_crash_handler(&mut self) {
        unsafe {
            (self.entry_v100().unload_crash_handler)();
        }
    }

    #[allow(missing_docs)]
    fn get_overlay_bits(&self) -> OverlayBits {
        unsafe { (self.entry_v100().get_overlay_bits)() }
    }

    #[allow(missing_docs)]
    fn mask_overlay_bits(&mut self, and: OverlayBits, or: OverlayBits) {
        unsafe {
            (self.entry_v100().mask_overlay_bits)(and, or);
        }
    }

    #[allow(missing_docs)]
    fn get_log_file_path_template(&self) -> &str {
        unsafe {
            let raw = (self.entry_v100().get_log_file_path_template)();
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    #[allow(missing_docs)]
    fn set_log_file_path_template<P: AsRef<Path>>(&mut self, path_template: P) {
        unsafe {
            let bytes = mem::transmute(path_template.as_ref().as_os_str());
            let cstr = CStr::from_bytes_with_nul_unchecked(bytes);
            (self.entry_v100().set_log_file_path_template)(cstr.as_ptr());
        }
    }

    #[allow(missing_docs)]
    fn get_num_captures(&self) -> u32 {
        unsafe { (self.entry_v100().get_num_captures)() }
    }

    #[allow(missing_docs)]
    fn get_capture(&self, index: u32) -> Option<(String, u64)> {
        unsafe {
            let mut len = self.get_log_file_path_template().len() as u32 + 128;
            let mut path = Vec::with_capacity(len as usize);
            let mut time = 0u64;

            if (self.entry_v100().get_capture)(index, path.as_mut_ptr(), &mut len, &mut time) == 1 {
                let raw_path = CString::from_raw(path.as_mut_ptr());
                let mut path = raw_path.into_string().unwrap();
                path.shrink_to_fit();

                Some((path, time))
            } else {
                None
            }
        }
    }

    /// Captures the next frame from the currently active window and API device.
    ///
    /// Data is saved to a capture log file at the location specified via
    /// `set_log_file_path_template()`.
    fn trigger_capture(&mut self) {
        unsafe {
            (self.entry_v100().trigger_capture)();
        }
    }

    #[allow(missing_docs)]
    fn is_target_control_connected(&self) -> bool {
        unsafe { (self.entry_v100().is_target_control_connected)() == 1 }
    }

    #[allow(missing_docs)]
    fn launch_replay_ui<C>(&self, cmd_line: C) -> Result<u32, ()>
    where
        C: Into<Option<&'static str>>,
    {
        unsafe {
            let with_cmd = cmd_line.into();
            let (enabled, text) = if let Some(ref cmd) = with_cmd {
                let bytes = cmd.as_bytes();
                (1, CStr::from_bytes_with_nul_unchecked(bytes))
            } else {
                (0, Default::default())
            };

            match (self.entry_v100().launch_replay_ui)(enabled, text.as_ptr()) {
                0 => Err(()),
                pid => Ok(pid),
            }
        }
    }

    #[allow(missing_docs)]
    fn set_active_window<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            (self.entry_v100().set_active_window)(dev.into(), win);
        }
    }

    #[allow(missing_docs)]
    fn start_frame_capture<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            (self.entry_v100().start_frame_capture)(dev.into(), win);
        }
    }

    /// Returns whether or not a frame capture is currently ongoing anywhere.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::{RenderDoc, V100};
    /// # use renderdoc::prelude::*;
    /// # fn init() -> Result<(), String> {
    /// # let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// if renderdoc.is_frame_capturing() {
    ///     println!("Frames are being captured.");
    /// } else {
    ///     println!("No frame capture is occurring.");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn is_frame_capturing(&self) -> bool {
        unsafe { (self.entry_v100().is_frame_capturing)() == 1 }
    }

    #[allow(missing_docs)]
    fn end_frame_capture<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            (self.entry_v100().end_frame_capture)(dev.into(), win);
        }
    }
}

/// Additional features for API version 1.1.0.
pub trait RenderDocV110: RenderDocV100 {
    /// Returns the raw `EntryV110` entry point struct.
    unsafe fn entry_v110(&self) -> &EntryV110;

    /// Captures the next _n_ frames from the currently active window and API
    /// device.
    ///
    /// Data is saved to a capture log file at the location specified via
    /// `set_log_file_path_template()`.
    fn trigger_multi_frame_capture(&self, num_frames: u32) {
        unsafe {
            (self.entry_v110().trigger_multi_frame_capture)(num_frames);
        }
    }
}
