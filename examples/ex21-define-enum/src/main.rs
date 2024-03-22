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
        FileSize::B(bytes) => format!("{:?}", bytes),
        FileSize::KB(kb) => format!("{:?} KB", kb),
        FileSize::MB(mb) => format!("{:?} MB", mb),
        FileSize::GB(gb) => format!("{:?} GB", gb),
        FileSize::TB(tb) => format!("{:?} TB", tb),
    }
}

fn main() {
    let disk = DiskType::SSD;
    let size = 12340032432324;
    println!("Disk type: {:?}", disk);
    println!("Size: {}", format_size(size));
}