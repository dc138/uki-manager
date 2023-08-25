use std::process as proc;

mod config;
mod opts;
mod uki;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), anyhow::Error> {
    let opts = opts::parse_opts()?;

    if opts.help {
        print_help(&opts);
        proc::exit(0);
    } else if opts.version {
        print_version();
        proc::exit(0);
    }

    let config = config::Config::parse_with_default(opts.config)?;
    dbg!(config);

    //let mut uki =
    //uki::UnifiedKernelImage::new("/usr/lib/systemd/boot/efi/linuxx64.efi.stub", "output.efi")?;
    //
    //uki.add_section_path(".osrel", "/usr/lib/os-release")?;
    //uki.add_section_buf(".uname", "6.4.8-zen1-1-zen")?;
    //uki.add_section_path(".cmdline", "/etc/kernel/cmdline")?;
    //uki.add_section_paths(
    //".initrd",
    //vec!["/boot/intel-ucode.img", "/boot/initramfs-linux-zen.img"],
    //)?;
    //uki.add_section_path(".linux", "/boot/vmlinuz-linux-zen")?;
    //
    //uki.output()?;

    Ok(())
}

fn print_help(opts: &opts::Opts) {
    println!("{}", opts.usage);
}

fn print_version() {
    println!("uki-manager v{}\n", VERSION);
    println!(
        "Copyright (C) 2023 Antonio de Haro. \n\
        This program is distributed under the MIT license, see the attatched LICENSE.txt file for terms and conditions. \n\
        This software is provided without any warranty of any kind. \n\
        Copyright atributions for any third party code included are provided in the attatched COPYRIGHT.md file."
    );
}
