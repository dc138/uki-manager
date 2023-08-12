use exe::SectionCharacteristics;
use exe::PE;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

struct UnifiedKernelImage {
    executable: exe::VecPE,
    output: String,
}

impl UnifiedKernelImage {
    fn new(stub: &str, output: &str) -> io::Result<Self> {
        let executable = exe::VecPE::from_disk_file(stub)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(Self {
            executable,
            output: output.to_owned(),
        })
    }

    fn add_section_buf<T: AsRef<[u8]>>(&mut self, name: &str, data: T) -> Result<(), exe::Error> {
        let mut sec = exe::ImageSectionHeader::default();

        let size: u32 = data
            .as_ref()
            .len()
            .try_into()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let virtual_size = size;

        let raw_size = self
            .executable
            .align_to_file(exe::Offset(size))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .0;

        sec.set_name(Some(name));
        sec.size_of_raw_data = raw_size;
        sec.virtual_size = virtual_size;
        sec.characteristics =
            SectionCharacteristics::MEM_READ | SectionCharacteristics::CNT_INITIALIZED_DATA;

        let sec = self.executable.append_section(&sec)?.to_owned();

        self.executable.resize(
            (sec.pointer_to_raw_data.0 + raw_size)
                .try_into()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
            0,
        );

        sec.write(&mut self.executable, data)?;

        Ok(())
    }

    fn add_section_path(&mut self, name: &str, path: &str) -> io::Result<()> {
        let mut file = fs::File::open(path)?;
        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf)?;

        Ok(self
            .add_section_buf(name, buf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?)
    }

    fn add_section_paths(&mut self, name: &str, paths: Vec<&str>) -> io::Result<()> {
        let mut buf: Vec<u8> = Vec::new();

        let files: Vec<fs::File> = paths
            .iter()
            .map(|path| Ok(fs::File::open(path)?))
            .collect::<io::Result<_>>()?;

        files
            .iter()
            .map(|mut f| Ok(f.read_to_end(&mut buf)?))
            .collect::<io::Result<Vec<usize>>>()?;

        Ok(self
            .add_section_buf(name, buf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?)
    }

    fn output(&mut self) -> io::Result<()> {
        self.executable
            .fix_image_size()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let checksum = self
            .executable
            .calculate_checksum()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let headers = self.executable.get_valid_mut_nt_headers_64().unwrap();
        headers.optional_header.checksum = checksum;

        let buf = self
            .executable
            .recreate_image(exe::PEType::Disk)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let mut output = fs::File::options()
            .write(true)
            .create(true)
            .open(&self.output)?;

        output.write(&buf)?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut uki =
        UnifiedKernelImage::new("/usr/lib/systemd/boot/efi/linuxx64.efi.stub", "output.efi")?;

    uki.add_section_path(".osrel", "/usr/lib/os-release")?;
    uki.add_section_buf(".uname", "6.4.8-zen1-1-zen")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    uki.add_section_path(".cmdline", "/etc/kernel/cmdline")?;
    uki.add_section_paths(
        ".initrd",
        vec!["/boot/intel-ucode.img", "/boot/initramfs-linux-zen.img"],
    )?;
    uki.add_section_path(".linux", "/boot/vmlinuz-linux-zen")?;

    uki.output()?;

    Ok(())
}
