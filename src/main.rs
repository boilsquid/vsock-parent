use nix::sys::socket::{bind, VsockAddr, socket, AddressFamily, SockFlag, SockType, accept};
use anyhow::Result;
use std::os::fd::{IntoRawFd, AsFd};
use nix::sys::socket::listen as listen_vsock;
use std::thread;

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;

const BACKLOG: usize = 128;

fn main() {
    println!("Staring vsock-parent");
    listen().expect("Failed to listen on vsock");
}

fn listen() -> Result<()> {
    let fd = socket(AddressFamily::Vsock, SockType::Stream, SockFlag::empty(), None)
        .map_err(|e| anyhow::anyhow!("Failed to create socket: {}", e))?;
    let raw_fd = fd.try_clone().expect("test").into_raw_fd();
    let as_raw = fd.as_fd();

    let port = 8001;
    let sockaddr = VsockAddr::new(VMADDR_CID_ANY, port);

    match bind(raw_fd, &sockaddr) {
        Ok(_) => println!("bound to vsock connection"),
        Err(e) => println!("bind failed: {}", e),
    }

    listen_vsock(&as_raw, BACKLOG)
        .map_err(|e| anyhow::anyhow!("Failed to listen on vsock: {}", e))?;

    thread::spawn(move || loop {
        let fd = match accept(raw_fd) {
            Ok(fd) => fd,
            Err(e) => {
                println!("accept failed: {}", e);
                continue;
            }
        };



        println!("Accepted connection on vsock");
    });

    Ok(())
}
