pub mod common;
pub mod unix;
pub mod windows;

// Re-export types and detection functions
pub use common::{OperatingSystem, ShellConfig, detect_os};

// Re-export all public functions based on OS
pub use self::get_os_impl::*;

// OS-specific implementations
#[cfg(target_os = "windows")]
mod get_os_impl {
    pub use super::windows::*;
}

#[cfg(not(target_os = "windows"))]
mod get_os_impl {
    pub use super::unix::*;
}

// Common functions that work across all OS
pub use common::{
    get_home_dir, copy_dir_all, migrate_old_pkit_dir, get_pkit_dir_with_migration,
    get_pkit_directories_info, print_pkit_directories
};
