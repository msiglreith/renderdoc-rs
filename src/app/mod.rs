//! RenderDoc Application API.

use std::os::raw::{c_ulonglong, c_void};
use std::rc::Rc;
use std::u32;

#[cfg(target_os = "windows")]
use winapi::guiddef::GUID;

pub use self::entry::version::{ApiVersion, V100, V110, V111};

pub mod entry;

/// Magic value used for when applications pass a path where shader debug
/// information can be found to match up with a stripped shader.
///
/// Windows GUID representation intended for consumption by D3D.
#[cfg(target_os = "windows")]
pub const SHADER_MAGIC_DEBUG_VALUE_STRUCT: GUID = GUID {
    Data1: 0xeab25520,
    Data2: 0x6670,
    Data3: 0x4865,
    Data4: [0x84, 0x29, 0x6c, 0x8, 0x51, 0x00, 0xff],
};

/// Magic value used for when applications pass a path where shader debug
/// information can be found to match up with a stripped shader.
///
/// Raw byte array representation (assuming x86 endianness).
pub const SHADER_MAGIC_DEBUG_VALUE_BYTE_ARRAY: &[u8] = &[
    0x20,
    0x55,
    0xb2,
    0xea,
    0x70,
    0x66,
    0x65,
    0x48,
    0x84,
    0x29,
    0x6c,
    0x8,
    0x51,
    0x54,
    0x00,
    0xff,
];

/// Magic value used for when applications pass a path where shader debug
/// information can be found to match up with a stripped shader.
///
/// Truncated version when only a `uint64_t` is available (e.g. Vulkan tags).
pub const SHADER_MAGIC_DEBUG_VALUE_TRUNCATED: c_ulonglong = 0x4856670eab25520;

/// RenderDoc capture options.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CaptureOption {
    /// Let the application enable vertical synchronization.
    AllowVSync = 0,
    /// Let the application enter fullscreen mode.
    AllowFullscreen = 1,
    /// Record API debugging events and messages.
    ///
    /// This option also goes by the deprecated name of `DebugDeviceMode`.
    ApiValidation = 2,
    /// Capture CPU callstacks for API events.
    CaptureCallstacks = 3,
    /// When capturing CPU callstacks, only capture them from drawcalls.
    ///
    /// This option does nothing without the above option being enabled.
    CaptureCallstacksOnlyDraws = 4,
    /// Specify a delay, measured in seconds, to wait for a debugger to attach
    /// to the application after being injected.
    DelayForDebugger = 5,
    /// Verify any writes to mapped buffers by checking the memory after the
    /// bounds of the returned pointer to detect any modification.
    VerifyMapWrites = 6,
    /// Hooks any system API calls that create child processes and injects
    /// RenderDoc into them recursively with the same options.
    HookIntoChildren = 7,
    /// Reference all resources available at the time of capture.
    ///
    /// By default, RenderDoc only includes resources in the final capture file
    /// necessary for that frame. This option allows you to override that
    /// behavior.
    RefAllResources = 8,
    /// Save the initial state for all resources, regardless of usage.
    ///
    /// By default, RenderDoc skips saving initial states for resources where
    /// the previous contents don't appear to be used (assuming that writes
    /// before reads indicate the previous contents aren't used).
    SaveAllInitials = 9,
    /// Capture all command lists generated from the start of the application.
    ///
    /// In APIs that allow for recording of command lists to be replayed later,
    /// RenderDoc may choose to not capture command lists before a frame capture
    /// is triggered to reduce overhead. This means any command lists that are
    /// recorded one and replayed many times will not be available, potentially
    /// causing a failure to capture.
    ///
    /// Note that this is only true for APIs where multithreading is difficult
    /// or otherwise discouraged. Newer APIs, e.g. Vulkan and D3D12, will ignore
    /// this option and always capture all command lists since they are heavily
    /// oriented around them and the associated overhead is mostly reduced due
    /// to superior API design.
    CaptureAllCmdLists = 10,
    /// Mute API debug output when `CaptureOption::ApiValidation` is enabled.
    DebugOutputMute = 11,
}

/// Raw mutable pointer to the API's root handle.
///
/// For example, this could be a pointer to an `ID3D11Device`,
/// `HGLRC`/`GLXContext`, `ID3D12Device`, etc.
pub type DevicePointer = *mut c_void;

/// User input key codes.
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InputButton {
    /// The '0' key over the letters.
    Key0 = 0x30,
    /// The '1' key over the letters.
    Key1 = 0x31,
    /// The '2' key over the letters.
    Key2 = 0x32,
    /// The '3' key over the letters.
    Key3 = 0x33,
    /// The '4' key over the letters.
    Key4 = 0x34,
    /// The '5' key over the letters.
    Key5 = 0x35,
    /// The '6' key over the letters.
    Key6 = 0x36,
    /// The '7' key over the letters.
    Key7 = 0x37,
    /// The '8' key over the letters.
    Key8 = 0x38,
    /// The '9' key over the letters.
    Key9 = 0x39,

    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,

    /// Leave the rest of the ASCII range free, in case the RenderDoc developers
    /// decide to use it later.
    NonPrintable = 0x100,

    /// Division key on the numpad.
    Divide,
    /// Multiplication key on the numpad.
    Multiply,
    /// Subtraction key on the numpad.
    Subtract,
    /// Addition key on the numpad.
    Plus,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Home,
    End,
    Insert,
    Delete,
    PageUp,
    PageDn,

    Backspace,
    Tab,
    PrtScrn,
    Pause,

    Max,
}

bitflags! {
    /// Bit flags for customizing the RenderDoc overlay.
    pub struct OverlayBits: u32 {
        /// Controls whether the overlay is enabled or disabled globally.
        const ENABLED = 0x1;
        /// Shows the average, minimum, and maximum sampled frame rate.
        const FRAME_RATE = 0x2;
        /// Shows the current frame number.
        const FRAME_NUMBER = 0x4;
        /// Shows a list of recent captures, out of the total captures made.
        const CAPTURE_LIST = 0x8;
        /// Sets the default configuration for the overlay.
        const DEFAULT = (0x1 | 0x2 | 0x4 | 0x8);
        /// Enables all overlay configuration bits.
        const ALL = u32::MAX;
        /// Disables all overlay configuration bits.
        const NONE = u32::MIN;
    }
}

/// Raw mutable pointer to the OS-provided window handle.
pub type WindowHandle = *mut c_void;

/// An instance of the RenderDoc API with baseline version `V`.
#[derive(Clone, Debug)]
pub struct RenderDoc<V: ApiVersion> {
    api: Rc<V::Entry>,
}

impl<V: ApiVersion> RenderDoc<V> {
    /// Initializes a new instance of the RenderDoc API.
    pub fn new() -> Result<RenderDoc<V>, String> {
        let api = V::load()?;
        Ok(RenderDoc { api })
    }
}

#[allow(missing_docs)]
impl RenderDoc<V110> {
    pub fn get_api_version(&self) -> (u32, u32, u32) {
        let (mut major, mut minor, mut patch) = (0, 0, 0);
        unsafe { (self.api.get_api_version)(&mut major, &mut minor, &mut patch); }
        (major as u32, minor as u32, patch as u32)
    }

    pub fn set_capture_option_f32(&self, opt: CaptureOption, val: f32) {
        let err = unsafe { (self.api.set_capture_option_f32)(opt, val) };
        assert_eq!(err, 1);
    }

    pub fn set_capture_option_u32(&self, opt: CaptureOption, val: u32) {
        let err = unsafe { (self.api.set_capture_option_u32)(opt, val) };
        assert_eq!(err, 1);
    }

    pub fn get_capture_option_f32(&self, opt: CaptureOption) -> f32 {
        use std::f32::MAX;
        let val = unsafe { (self.api.get_capture_option_f32)(opt) };
        assert_ne!(val, -MAX);
        val
    }

    pub fn get_capture_option_u32(&self, opt: CaptureOption) -> u32 {
        use std::u32::MAX;
        let val = unsafe { (self.api.get_capture_option_u32)(opt) };
        assert_ne!(val, MAX);
        val
    }

    pub fn set_capture_keys(&self) {}
    pub fn set_focus_toggle_keys(&self) {}

    pub fn shutdown(&self) {}
    pub fn unload_crash_handler(&self) {}

    pub fn get_overlay_bits(&self) -> OverlayBits {
        unsafe { (self.api.get_overlay_bits)() }
    }

    pub fn mask_overlay_bits(&self, and: OverlayBits, or: OverlayBits) {
        unsafe { (self.api.mask_overlay_bits)(and, or) }
    }

    pub fn get_log_file_path_template(&self) -> &str {
        use std::ffi::CStr;
        unsafe {
            let raw = (self.api.get_log_file_path_template)();
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    pub fn set_log_file_path_template<P: AsRef<str>>(&self, path_template: P) {
        use std::ffi::CStr;
        let bytes = path_template.as_ref().as_bytes();
        unsafe {
            let pt = CStr::from_bytes_with_nul_unchecked(bytes);
            (self.api.set_log_file_path_template)(pt.as_ptr());
        }
    }

    pub fn get_num_captures(&self) -> u32 {
        unsafe { (self.api.get_num_captures)() }
    }

    pub fn get_capture(&self) {}

    pub fn trigger_capture(&self) {
        unsafe { (self.api.trigger_capture)() };
    }

    pub fn is_target_control_connected(&self) -> bool {
        unsafe { (self.api.is_target_control_connected)() == 1 }
    }

    pub fn launch_replay_ui(&self) {
        unsafe { (self.api.launch_replay_ui)(0, ::std::ptr::null()); }
    }

    pub fn set_active_window(&self, dev: ::app::DevicePointer, win: ::app::WindowHandle) {
        unsafe { (self.api.set_active_window)(dev, win); }
    }

    pub fn start_frame_capture(&self) {}
    pub fn is_frame_capturing(&self) {}
    pub fn end_frame_capture(&self) {}

    pub fn trigger_multi_frame_capture(&self, num_frames: u32) {
        unsafe { (self.api.trigger_multi_frame_capture)(num_frames); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_api_version() {
        let rd: RenderDoc<V110> = RenderDoc::new().expect("Failed to init");
        let (major, minor, _) = rd.get_api_version();
        assert!(major >= 1u32);
        assert!(minor >= 1u32);
    }

    #[test]
    fn get_set_capture_option_f32() {
        let rd: RenderDoc<V110> = RenderDoc::new().expect("Failed to init");

        let delay = rd.get_capture_option_f32(CaptureOption::DelayForDebugger);
        assert_eq!(delay, 0.0f32);

        rd.set_capture_option_f32(CaptureOption::DelayForDebugger, 2.5f32);
        let delay = rd.get_capture_option_f32(CaptureOption::DelayForDebugger);
        assert_eq!(delay, 2.0f32);
    }

    #[test]
    fn get_set_capture_option_u32() {
        let rd: RenderDoc<V110> = RenderDoc::new().expect("Failed to init");

        let vsync = rd.get_capture_option_u32(CaptureOption::AllowVSync);
        assert_eq!(vsync, 1u32);

        let is_full = rd.get_capture_option_u32(CaptureOption::AllowFullscreen);
        assert_eq!(is_full, 1u32);

        let api_val_mode = rd.get_capture_option_u32(CaptureOption::ApiValidation);
        let debug_mode = rd.get_capture_option_u32(CaptureOption::ApiValidation);
        assert_eq!(api_val_mode, 0u32);
        assert_eq!(api_val_mode, debug_mode);

        let cc = rd.get_capture_option_u32(CaptureOption::CaptureCallstacks);
        assert_eq!(cc, 0u32);

        let cc_draw = rd.get_capture_option_u32(CaptureOption::CaptureCallstacksOnlyDraws);
        assert_eq!(cc_draw, 0u32);

        let ver_map = rd.get_capture_option_u32(CaptureOption::VerifyMapWrites);
        assert_eq!(ver_map, 0u32);

        let hook_in = rd.get_capture_option_u32(CaptureOption::HookIntoChildren);
        assert_eq!(hook_in, 0u32);

        let ref_all = rd.get_capture_option_u32(CaptureOption::RefAllResources);
        assert_eq!(ref_all, 0u32);

        let intls = rd.get_capture_option_u32(CaptureOption::SaveAllInitials);
        assert_eq!(intls, 0u32);

        let cmds = rd.get_capture_option_u32(CaptureOption::CaptureAllCmdLists);
        assert_eq!(cmds, 0u32);

        let is_muted = rd.get_capture_option_u32(CaptureOption::DebugOutputMute);
        assert_eq!(is_muted, 1u32);
    }
}