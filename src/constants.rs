#[cfg(target_os = "linux")]
pub const WSL_INTEROP_PATH: &str = "/proc/sys/fs/binfmt_misc/WSLInterop";
#[cfg(target_os = "windows")]
pub const COMPUTER_NAME_PATH: &str =
    "SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName";
#[cfg(target_os = "windows")]
pub const WIN_EDITION_PATH: &str = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion";
