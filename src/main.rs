mod uki;

fn main() -> Result<(), uki::Error> {
    let mut uki =
        uki::UnifiedKernelImage::new("/usr/lib/systemd/boot/efi/linuxx64.efi.stub", "output.efi")?;

    uki.add_section_path(".osrel", "/usr/lib/os-release")?;
    uki.add_section_buf(".uname", "6.4.8-zen1-1-zen")?;
    uki.add_section_path(".cmdline", "/etc/kernel/cmdline")?;
    uki.add_section_paths(
        ".initrd",
        vec!["/boot/intel-ucode.img", "/boot/initramfs-linux-zen.img"],
    )?;
    uki.add_section_path(".linux", "/boot/vmlinuz-linux-zen")?;

    uki.output()?;

    Ok(())
}
