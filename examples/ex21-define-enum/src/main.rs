#[derive(Debug)]
enum DiskType {
    SSD,
    HDD,
    Hybrid,
}

#[derive(Debug)]
enum FileSize {
    B(usize),
    KB(usize),
    MB(usize),
    GB(usize),
    TB(usize),
}

fn format_size(size: usize) -> String {
    let filesize = match size {
        0..=999 => FileSize::B(size),
        1000..=999_999 => FileSize::KB(size / 1000),
        1_000_000..=999_999_999 => FileSize::MB(size / 1_000_000),
        1_000_000_000..=999_999_999_999 => FileSize::GB(size / 1_000_000_000),
        _ => FileSize::TB(size / 1_000_000_000_000)
    };
    
    match filesize {
        FileSize::B(bytes) => format!("{:?} B", bytes),
        FileSize::KB(kb) => format!("{:?} KB", kb/1000),
        FileSize::MB(mb) => format!("{:?} MB", mb/1000),
        FileSize::GB(gb) => format!("{:?} GB", gb/1000),
        FileSize::TB(tb) => format!("{:?} TB", tb/1000),
    }
}

fn main() {
    let disk = DiskType::SSD;
    let size = 1230;
    println!("Disk type: {:?}", disk);
    println!("Size: {:.6}", format_size(size));
}