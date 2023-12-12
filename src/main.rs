use nix::sys::socket::{bind, VsockAddr, socket, AddressFamily, SockFlag, SockType};
use anyhow::Result;
use std::os::{unix::io::RawFd, fd::IntoRawFd};

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;

fn main() {
    println!("Staring vsock-parent");
    listen().expect("Failed to listen on vsock");
}

fn listen() -> Result<()> {
    let fd = socket(AddressFamily::Vsock, SockType::Stream, SockFlag::empty(), None)
        .map_err(|e| anyhow::anyhow!("Failed to create socket: {}", e))?;

    let port = 8001;
    let sockaddr = VsockAddr::new(VMADDR_CID_ANY, port);

    match bind(fd.into_raw_fd(), &sockaddr) {
        Ok(_) => println!("bound to vsock connection"),
        Err(e) => println!("bind failed: {}", e),
    }


    Ok(())
}
