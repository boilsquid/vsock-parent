use nix::sys::socket::{bind, VsockAddr};
use anyhow::Result;

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;

fn main() {
    println!("Staring vsock-parent");
    listen().expect("Failed to listen on vsock");
}

fn listen() -> Result<()> {
    let port = 8001;
    let sockaddr = VsockAddr::new(VMADDR_CID_ANY, port);
    let server_fd = 2021;


    match bind(server_fd, &sockaddr) {
        Ok(_) => println!("bound to vsock connection"),
        Err(e) => println!("bind failed: {}", e),
    }


    Ok(())
}