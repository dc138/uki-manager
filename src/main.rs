use std::fs;
use std::io;

struct UnifiedKernelImage {
    osrel: fs::File,
    cmdline: fs::File,
    splash: fs::File,
    linux: fs::File,
    initrd: fs::File,
    output: fs::File,
}

impl UnifiedKernelImage {
    fn new(
        osrel: &str,
        cmdline: &str,
        splash: &str,
        linux: &str,
        initrds: Vec<&str>,
        output: &str,
    ) -> Result<Self, io::Error> {
        let initrds = initrds
            .iter()
            .map(|path| {
                let file = fs::File::open(path)?;
                Ok((file.metadata()?, file))
            })
            .collect::<Result<Vec<(fs::Metadata, fs::File)>, io::Error>>()?;

        for (meta, _) in &initrds {
            dbg!(meta.len());
        }

        let initrds_size = initrds.iter().fold(0, |size, (meta, _)| size + meta.len());

        dbg!(initrds_size);

        Err(io::Error::new(io::ErrorKind::Other, "todo"))
    }
}

fn main() -> io::Result<()> {
    let uki = UnifiedKernelImage::new(
        "",
        "",
        "",
        "",
        vec!["/boot/initramfs-linux-zen.img", "/boot/intel-ucode.img"],
        "",
    )?;

    Ok(())
}
