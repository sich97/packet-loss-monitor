//! Platform-specific implementations for cross-platform support

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use self::linux::PlatformImpl;

#[cfg(target_os = "macos")]
pub use self::macos::PlatformImpl;

#[cfg(target_os = "windows")]
pub use self::windows::PlatformImpl;
