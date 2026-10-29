use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: smth-hypervisor <file> [options]");
        return;
    }

    let file_path = &args[1];
    let extension = Path::new(file_path).extension().and_then(|s| s.to_str());

    match extension {
        Some("iso") => launch_qemu(file_path),
        Some("exe") => launch_box64(file_path),
        Some("apk") => launch_lxc_apk(file_path),
        Some("img") => launch_qemu_raw(file_path),
        _ => println!("Unsupported file type"),
    }
}

fn launch_qemu(file_path: &str) {
    log::info!("Launching QEMU with ISO: {}", file_path);
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.args([
        "-m", "4G",
        "-cpu", "host",
        "-drive", &format!("file={},if=virtio", file_path),
        "-boot", "d",
        "-nographic",
    ]);
    let _ = cmd.status();
}

fn launch_box64(file_path: &str) {
    log::info!("Launching Box64 with EXE: {}", file_path);
    let mut cmd = Command::new("box64");
    cmd.arg(file_path);
    let _ = cmd.status();
}

fn launch_lxc_apk(file_path: &str) {
    log::info!("Launching APK in LXC sandbox: {}", file_path);
    // Logic to mount APK into LXC container and launch Android runtime
}

fn launch_qemu_raw(file_path: &str) {
    log::info!("Launching QEMU with raw image: {}", file_path);
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.args([
        "-m", "4G",
        "-cpu", "host",
        "-drive", &format!("file={},if=virtio", file_path),
        "-boot", "c",
        "-nographic",
    ]);
    let _ = cmd.status();
}
