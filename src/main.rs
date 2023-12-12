use nix::sys::socket::{bind, VsockAddr};
use anyhow::Result;

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;

fn main() {
    println!("Staring vsock-parent");
    listen().expect("Failed to listen on vsock");
}

fn listen() -> Result<()> {
    let port = 8001;
    let sockaddr = VsockAddr::new(2021, port);

    match bind(8001, &sockaddr) {
        Ok(_) => println!("bound to vsock connection"),
        Err(e) => println!("bind failed: {}", e),
    }


    Ok(())
}
