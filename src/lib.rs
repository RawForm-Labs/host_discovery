#[cfg(target_os = "macos")]
use std::process::Command;
#[warn(missing_docs, missing_debug_implementations)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use raw_cpuid::CpuId;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use raw_cpuid::ProcessorBrandString;
#[cfg(target_os = "linux")]
use rayon::prelude::*;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::path::Path;
use std::{
    env::consts::{ARCH, OS},
    thread,
};
use wgpu::{Backends, Instance};
#[cfg(target_os = "windows")]
use windows_registry::LOCAL_MACHINE;

mod constants;
#[cfg(target_os = "linux")]
use constants::WSL_INTEROP_PATH;
#[cfg(target_os = "windows")]
use constants::{COMPUTER_NAME_PATH, WIN_EDITION_PATH};
mod display;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct OSProfile<'o, 'a> {
    pub os: &'o str,
    pub arch: &'a str,
    pub win_edition: Option<String>,
    pub computer_name: Option<String>,
    pub is_wsl: Option<bool>,
    pub distro: Option<String>,
    pub hostname: Option<String>,
}

#[derive(Debug)]
pub struct Processor<Model, Cores> {
    pub model: Model,
    pub cores: Cores,
}

#[derive(Debug)]
pub struct GraphicsCard {
    pub model: String,
    pub driver_version: String,
}

impl<'o, 'a> std::fmt::Display for OSProfile<'o, 'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<Model, Cores> std::fmt::Display for Processor<Model, Cores> 
where Model: std::fmt::Debug, Cores: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

display!(GraphicsCard);

impl<'o, 'a> OSProfile<'o, 'a> {
    pub fn new() -> Self {
        Self {
            os: OS,
            arch: ARCH,
            win_edition: None,
            computer_name: None,
            is_wsl: None,
            distro: None,
            hostname: None,
        }
    }

    /// Returns the Windows Edition if a Windows system is available
    #[cfg(target_os = "windows")]
    pub fn win_edition(mut self) -> Self {
        let key = LOCAL_MACHINE;
        let sub_key = key
            .open(WIN_EDITION_PATH)
            .expect("Failed to find registry entry for: CurrentVersion");
        let edition = sub_key
            .get_string("EditionID")
            .expect("Failed to identify Windows Edition");

        self.win_edition = Some(edition);
        self
    }

    /// Returns the ComputerName if a Windows system is available
    #[cfg(target_os = "windows")]
    pub fn computer_name(mut self) -> Self {
        let key = LOCAL_MACHINE;
        let sub_key = key
            .open(COMPUTER_NAME_PATH)
            .expect("Failed to find registry entry for: ComputerName");
        let name = sub_key
            .get_string("ComputerName")
            .expect("Failed to find key: ComputerName");

        self.computer_name = Some(name);
        self
    }

    /// Returns the Linux distro if a Linux system is available
    #[cfg(target_os = "linux")]
    pub fn distro(mut self) -> Self {
        let text = fs::read_to_string("/etc/os-release").expect("Failed to read /etc/os-release");
        let tokens = text.split("\n").collect::<Vec<&str>>();
        let pretty_name = tokens
            .par_iter()
            .filter(|line| line.contains("PRETTY_NAME"))
            .collect::<Vec<&&str>>();

        let distro = pretty_name[0].split("=").collect::<Vec<&str>>()[1].replace("\"", "");
        self.distro = Some(distro);
        self
    }

    /// Returns the hostname if a Linux system is available
    #[cfg(target_os = "linux")]
    pub fn hostname(mut self) -> Self {
        let name = fs::read_to_string("/etc/hostname").expect("Failed to read /etc/hostname");
        self.hostname = Some(name.split("\n").collect::<String>());
        self
    }

    /// Returns true if the Linux host is running on WSL
    #[cfg(target_os = "linux")]
    pub fn is_wsl(mut self) -> Self {
        let path = Path::new(WSL_INTEROP_PATH).exists();
        self.is_wsl = Some(path);
        self
    }

    pub fn build(self) -> Self {
        Self {
            os: self.os,
            arch: self.arch,
            win_edition: self.win_edition,
            computer_name: self.computer_name,
            is_wsl: self.is_wsl,
            distro: self.distro,
            hostname: self.hostname,
        }
    }
}

/// Returns a `Processor` object containing the CPU model and logical core count  (x86 only)
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn x86_cpu() -> Processor<ProcessorBrandString, u32> {
    let cpuid = CpuId::new();
    let brand = cpuid.get_processor_brand_string().unwrap();
    let cores = cpuid.get_processor_capacity_feature_info().unwrap();

    let cpu = Processor {
        model: brand,
        cores: cores.maximum_logical_processors() as u32,
    };
    cpu
}

#[cfg(target_os = "macos")]
pub fn macos_cpu() -> Processor<String, String> {
    let cpu_output = Command::new("sysctl")
        .arg("machdep.cpu.brand_string")
        .output()
        .expect("Failed to execute sysctl command");

    let core_output = Command::new("sysctl")
        .arg("hw.logicalcpu")
        .output()
        .expect("Failed to execute sysctl command");
   
    let model = String::from_utf8(cpu_output.stdout).unwrap();
    let cores = String::from_utf8(core_output.stdout).unwrap();
    let cpu = Processor {
        model: model,
        cores: cores,
    };
    cpu
}

/// Returns a `GraphicsCard` object containing the GPU model and driver version
pub fn gpu() -> Option<GraphicsCard> {
    let instance = Instance::default();
    let t = thread::spawn(move || {
        for adapter in instance.enumerate_adapters(Backends::all()) {
            let info = adapter.get_info();
            let gpu = GraphicsCard {
                model: info.name,
                driver_version: info.driver_info,
            };
            return Some(gpu);
        }
        None
    });
    t.join().unwrap()
}
