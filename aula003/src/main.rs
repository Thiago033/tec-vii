fn main() {

    let current_arch = std::env::consts::ARCH;

    match current_arch {
        "x86" => println!("Detected architecture: x86"),
        "x86_64" => println!("Detected architecture: x86_64"),
        "arm" => println!("Detected architecture: ARM"),
        "aarch64" => println!("Detected architecture: AArch64"),
        _ => println!("Unknown architecture"),
    }
}
