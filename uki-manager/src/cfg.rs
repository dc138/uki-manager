use uki_manager_proc as ump;

// The macro TomlFromStrDefault provides a from_str_default, that functions similar to
// toml::from_str, but replaces empty fields with the corresponding field from a provided default
// value

#[derive(ump::TomlFromStrDefault, Debug)]
pub struct Config {
    pub vm_dir: String,
    #[nest]
    pub default_kernel_config: KernelConfig,
}

#[derive(ump::TomlFromStrDefault, Debug)]
pub struct KernelConfig {
    pub output_dir: String,
    pub output_name: String,
    pub stub_path: String,
    pub cmdline_path: String,
    pub initrd_paths: Vec<String>,
    pub vmlinuz_path: String,
    pub splash_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vm_dir: "/boot/".into(),
            default_kernel_config: Default::default(),
        }
    }
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            output_dir: "/efi/EFI/Linux/".into(),
            output_name: "%name%".into(),
            stub_path: "/usr/lib/systemd/boot/efi/linuxx64.efi.stub".into(),
            cmdline_path: "/etc/kernel/cmdline".into(),
            initrd_paths: vec![
                "/boot/amd-ucode.img".into(),
                "/boot/intel-ucode.img".into(),
                "/boot/initramfs-%name%.img".into(),
            ],
            vmlinuz_path: "/boot/vmlinuz-%name%".into(),
            splash_path: "/usr/share/systemd/bootctl/splash-arch.bmp".into(),
        }
    }
}
