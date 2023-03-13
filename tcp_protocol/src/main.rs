use std::io;
use std::collections::HashMap;
use std::net::Ipv4Addr;


struct TcpState{

}

#![derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src : (Ipv4Addr, u16),
    dst : (Ipv4Addr, u16),
}


fn main() -> io::Result<()> {
    example_read_package_loop_using_etherparser_flags_protocol_ignore_no_ipv4()
}


fn example_read_single_package() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    let nbytes = nic.recv(&mut buf[..])?;
    eprintln!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    Ok(())
}


fn example_read_package_loop() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        eprintln!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    }
    Ok(())
}


fn example_read_package_loop_parsing_flags_protocol() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let protocol = u16::from_be_bytes([buf[2], buf[3]]);
        eprintln!(
            "read {} bytes (flags: {:x}, protocol: {:x}): {:x?}", nbytes - 4, flags, protocol, &buf[4..nbytes]);
    }
    Ok(())
}


fn example_read_package_loop_parsing_flags_protocol_ignore_no_ipv4() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let protocol = u16::from_be_bytes([buf[2], buf[3]]);

        if protocol != 0x0800 {
            // no ipv4
            continue;
        }
        eprintln!(
            "read {} bytes (flags: {:x}, protocol: {:x}): {:x?}", nbytes - 4, flags, protocol, &buf[4..nbytes]);
    }
    Ok(())
}

fn example_read_package_loop_using_etherparser_flags_protocol_ignore_no_ipv4() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_protocol = u16::from_be_bytes([buf[2], buf[3]]);

        if eth_protocol != 0x0800 {
            // no ipv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                // (src_ip, src_port, dst_ip, dst_port) -> quad
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                eprintln!(
                    "{} -> {} {} bytes of protocol: {:x}", src, dst, p.payload_len(), proto);
            },
            Err(e) => {
                eprintln!("ignoring weird packet {:?}", e);
            }
        }
    }
    Ok(())
}