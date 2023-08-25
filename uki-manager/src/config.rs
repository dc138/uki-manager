use uki_manager_proc as ump;

#[derive(ump::TomlParseWithDefault, Debug)]
pub struct Config {
    #[default("/boot/".to_owned())]
    pub vm_dir: String,

    #[default(
        KernelConfig {
            output_dir: "/efi/EFI/Linux/".to_owned(),
            output_name: "%name%.efi".to_owned(),
            stub_path: "/usr/lib/systemd/boot/efi/linuxx64.efi.stub".to_owned(),
            cmdline_path: "/etc/kernel/cmdline".to_owned(),
            initrd_paths: vec!["/boot/amd-ucode.img".to_owned(), "/boot/intel-ucode.img".to_owned(), "/boot/initramfs-%name%.img".to_owned()],
            vmlinuz_path: "/boot/vmlinuz-%name%".to_owned(),
            splash_path: "/usr/share/systemd/bootctl/splash-arch.bmp".to_owned(),
        })]
    pub default_kernel_config: KernelConfig,
}

#[derive(serde::Deserialize, Debug)]
pub struct KernelConfig {
    pub output_dir: String,
    pub output_name: String,
    pub stub_path: String,
    pub cmdline_path: String,
    pub initrd_paths: Vec<String>,
    pub vmlinuz_path: String,
    pub splash_path: String,
}

pub mod test {
    #[derive(uki_manager_proc::TomlFromStrDefault, Clone, Copy, Debug)]
    struct A {
        #[nest]
        b: B,
        d: u8,
    }

    #[derive(uki_manager_proc::TomlFromStrDefault, Clone, Copy, Debug)]
    struct B {
        c: u8,
    }

    pub fn test() {
        let def = A {
            b: B { c: 1 },
            d: 2,
        };

        let parsed0 = AOption {
            b: Some(BOption { c: Some(3) }),
            d: Some(4),
        };

        let parsed1 = AOption {
            b: Some(BOption { c: None }),
            d: Some(4),
        };

        let parsed2 = AOption {
            b: None,
            d: Some(4),
        };

        let parsed3 = AOption {
            b: Some(BOption { c: Some(3) }),
            d: None,
        };

        let parsed4 = AOption {
            b: Some(BOption { c: None }),
            d: None,
        };

        let parsed5 = AOption { b: None, d: None };

        dbg!(parsed0.toml_unwrap_default(def));
        dbg!(parsed1.toml_unwrap_default(def));
        dbg!(parsed2.toml_unwrap_default(def));
        dbg!(parsed3.toml_unwrap_default(def));
        dbg!(parsed4.toml_unwrap_default(def));
        dbg!(parsed5.toml_unwrap_default(def));
    }
}
