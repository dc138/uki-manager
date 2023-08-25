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

mod test {
    #[derive(uki_manager_proc::TomlFromStrDefault)]
    struct A {
        #[nest]
        b: B,
        d: u8,
    }

    #[derive(uki_manager_proc::TomlFromStrDefault)]
    struct B {
        c: u8,
    }
}

#[derive(Clone)]
struct A {
    b: B,
    d: u8,
}

struct AOption {
    b: Option<BOption>,
    d: Option<u8>,
}

impl AOption {
    fn toml_unwrap_default(self, default: &A) -> A {
        A {
            b: match self.b {
                Some(b) => b.toml_unwrap_default(&default.b),
                None => default.b.clone(),
            },
            d: self.d.unwrap_or(default.d.clone()),
        }
    }
}

#[derive(Clone)]
struct B {
    c: u8,
}

struct BOption {
    c: Option<u8>,
}

impl BOption {
    fn toml_unwrap_default(self, default: &B) -> B {
        B {
            c: self.c.unwrap_or(default.c.clone()),
        }
    }
}
