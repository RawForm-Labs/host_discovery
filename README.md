# A simple Rust crate for host discovery

> Make more decisions based on the characteristics of the environment you are working in.

- Basic Usage

```rust
use host_discovery::{detect_arch, detect_distro, detect_os};

fn main() {
    let os = detect_os();
    let distro = detect_distro();
    let arch = detect_arch();

    println!("OS: {}\nDistro: {}\nArch: {}", os, distro, arch);
}
```

- API
  - ***fn*** `detect_os`: Returns a variant of OperatingSystem
  - ***fn*** `detect_arch`: Returns a variant of Architecture
  - ***fn*** `cpu_cores`: Returns a u32 representing the CPU core count
  - ***fn*** `cpu_model`: Returns a String containing the name of the CPU model
  - ***fn*** `detect_distro`: Returns a String containing the name of the running Linux distribution
  - ***fn*** `detect_distro_version`: Returns a String containing the version id of the running Linux distribution
  - ***fn*** `is_subsystem_env`: Returns a boolean based on whether the Linux environment is a Windows subsystem
  - ***fn*** `lookup_windows_edition` Returns a String containing the Windows edition via the registry
  - ***fn*** `lookup_product_name` Returns a String containing the version & edition of Windows via the registry

### Add to your project
```sh 
    cargo add host_discovery
```

- Planned Features
  - GPU enumeration & extended device detection.

