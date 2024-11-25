> This library is a work in progress

# A simple Rust crate for host discovery

### Basic Usage
```rust
use host_discovery::{OSProfile, gpu};

fn main() {
    // linux example
    //let profile = OSProfile::new().distro().build();
    
    // and/or
    //let profile = OSProfile::new().is_wsl().build();
    let profile = OSProfile::new().win_edition().build();
    let arch = profile.arch;
    let os = profile.os;
    let edition = profile.win_edition;
    let gpu = gpu().expect("Unreachable");

    println!(
        " Architecture: {}\n OS: {}\n Edition: {}\n GPU: {}", 
        arch,
        os,
        edition.unwrap(),
        gpu,
    )
}
```

#### V3 Roadmap
- Reduce dependency load by implementing a smaller utility crate for detecting CPUs (migrate from raw_cpuid).

> Current Features

- OS
- Architecture
- Hostname
- Windows Edition
- WSL Detection
- Linux Distro
- GPU Enumeration
- CPU Detection (x86_64, aarch64 - macOS Only)
