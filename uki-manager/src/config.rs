use uki_manager_proc as ump;

#[derive(ump::TomlParseWithDefault, Debug)]
pub struct KernelConfig {
    #[default("/efi/EFI/Linux/".to_owned())]
    pub output_dir: String,

    #[default("%name%.efi".to_owned())]
    pub output_name: String,

    #[default("/usr/lib/systemd/boot/efi/linuxx64.efi.stub".to_owned())]
    pub stub_path: String,

    #[default("/etc/kernel/cmdline".to_owned())]
    pub cmdline_path: String,

    #[default(vec!["/boot/amd-ucode.img".to_owned(),
                   "/boot/intel-ucode.img".to_owned(),
                   "/boot/initramfs-%name%.img".to_owned()])]
    pub initrd_paths: Vec<String>,

    #[default("/boot/vmlinuz-%name%".to_owned())]
    pub vmlinuz_path: String,

    #[default("/usr/share/systemd/bootctl/splash-arch.bmp".to_owned())]
    pub splash_path: String,
}
