use anyhow as ah;
use uki_manager_proc as ump;

#[ump::option_copy]
pub struct KernelConfig {
    pub output_dir: String,
    pub output_name: String,
    pub stub_path: String,
    pub uname: String,
    pub cmdline_path: String,
    pub initrd_paths: Vec<String>,
    pub vmlinuz_path: String,
    pub splash_path: Option<String>,
}

impl KernelConfig {
    pub fn parse_default(path: String) -> ah::Result<Self> {
        todo!();
    }
}
