use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};
use rayon::prelude::*;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    let socket_address: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("Port scanning: Creating socket address")
        .collect();
    if socket_address.len() == 0 {
        return subdomain;
    }
    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scan_port(socket_address[0], *port))
        .filter(|port| port.is_open)
        .collect();
    subdomain
}

fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_address.set_port(port);
    // let socket_address: Vec<SocketAddr> = format!("{}:{}", hostname, port)
    //     .to_socket_addrs()
    //     .expect("Creating socket address")
    //     .collect();
    // if socket_address.len() == 0 {
    //     return Port {
    //         port,
    //         is_open: false,
    //     };
    // }

    let is_open = if let Ok(_) = TcpStream::connect_timeout(&socket_address, timeout) {
        true
    } else {
        false
    };
    Port { port, is_open }
}
