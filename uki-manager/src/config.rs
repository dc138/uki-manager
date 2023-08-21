use anyhow as ah;
use serde::Deserialize;
use std::fs;

pub struct KernelConfig {
    pub output_dir: String,
    pub output_name: String,
    pub stub_path: String,
    pub cmdline_path: String,
    pub initrd_paths: Vec<String>,
    pub vmlinuz_path: String,
    pub splash_path: String,
}

#[derive(Deserialize)]
pub struct KernelConfigOption {
    pub output_dir: Option<String>,
    pub output_name: Option<String>,
    pub stub_path: Option<String>,
    pub cmdline_path: Option<String>,
    pub initrd_paths: Option<Vec<String>>,
    pub vmlinuz_path: Option<String>,
    pub splash_path: Option<String>,
}

impl KernelConfig {
    pub fn parse_with_default(path: String, default: Self) -> ah::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: KernelConfigOption = toml::from_str(&content)?;

        Ok(Self {
            output_dir: config.output_dir.unwrap_or(default.output_dir),
            output_name: config.output_name.unwrap_or(default.output_name),
            stub_path: config.stub_path.unwrap_or(default.stub_path),
            cmdline_path: config.cmdline_path.unwrap_or(default.cmdline_path),
            initrd_paths: config.initrd_paths.unwrap_or(default.initrd_paths),
            vmlinuz_path: config.vmlinuz_path.unwrap_or(default.vmlinuz_path),
            splash_path: config.splash_path.unwrap_or(default.splash_path),
        })
    }

    pub fn parse_with_global_default(path: String) -> ah::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: KernelConfigOption = toml::from_str(&content)?;

        let default = KernelConfig {
            output_dir: "/efi/EFI/Linux/".to_owned(),
            output_name: "%name%.efi".to_owned(),
            stub_path: "/usr/lib/systemd/boot/efi/linuxx64.efi.stub".to_owned(),
            cmdline_path: "/etc/kernel/cmdline".to_owned(),
            initrd_paths: vec![
                "/boot/amd-ucode.img".to_owned(),
                "/boot/intel-ucode.img".to_owned(),
                "/boot/initramfs-%name%.img".to_owned(),
            ],
            vmlinuz_path: "/boot/vmlinuz-%name%".to_owned(),
            splash_path: "/usr/share/systemd/bootctl/splash-arch.bmp".to_owned(),
        };

        Ok(Self {
            output_dir: config.output_dir.unwrap_or(default.output_dir),
            output_name: config.output_name.unwrap_or(default.output_name),
            stub_path: config.stub_path.unwrap_or(default.stub_path),
            cmdline_path: config.cmdline_path.unwrap_or(default.cmdline_path),
            initrd_paths: config.initrd_paths.unwrap_or(default.initrd_paths),
            vmlinuz_path: config.vmlinuz_path.unwrap_or(default.vmlinuz_path),
            splash_path: config.splash_path.unwrap_or(default.splash_path),
        })
    }
}
