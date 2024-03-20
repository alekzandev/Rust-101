enum DiskType {
    SSD,
    HDD,
    Hybrid,
}
#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
    TB(u32),
}

fn main() {
    let disk_type = DiskType::SSD;
    match disk_type {
        DiskType::SSD => println!("Solid State Drive"),
        DiskType::HDD => println!("Hard Disk Drive"),
        DiskType::Hybrid => println!("Hybrid Drive"),
    }

    let disk_size = DiskSize::GB(512);
    println!("{:?}", disk_size);
}
