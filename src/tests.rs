use crate::cpu;
use crate::gpu;
use crate::OSProfile;
use crate::ARCH;
use crate::OS;

#[test]
pub fn test_profile() {
    let profile = OSProfile::new().build();
    assert_eq!(profile.os, OS);
    assert_eq!(profile.arch, ARCH);
}

#[cfg(target_os = "windows")]
#[test]
pub fn test_computer_name() {
    let profile = OSProfile::new().computer_name().build();
    let name = profile.computer_name.unwrap();
    assert_eq!(name, "WORK");
}

#[cfg(target_os = "linux")]
#[test]
pub fn test_distro() {
    let profile = OSProfile::new().distro().build();
    assert!(profile.distro.unwrap().starts_with("Fedora"));
}

#[cfg(target_os = "linux")]
#[test]
pub fn test_hostname() {
    let profile = OSProfile::new().hostname().build();
    assert_eq!(profile.hostname, Some("Work".to_string()));
}

#[cfg(target_os = "linux")]
#[test]
fn test_wsl() {
    let profile = OSProfile::new().is_wsl().build();
    assert_eq!(profile.is_wsl, Some(false));
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
pub fn test_cpu() {
    let cpu = cpu();
    assert!(cpu.model.as_str().starts_with("AMD"));
    assert_eq!(cpu.cores, 16);
}

#[test]
pub fn test_gpu() {
    let gpu = gpu();
    assert!(gpu.is_some());
}
