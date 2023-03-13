use std::fmt;

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{{ \n  ip address: {:?} \n  address: {} \n}}", self.kind, self.address)
    }
}

fn main() {
    let ipv4 = IpAddrKind::V4(192, 168, 0, 1);

    let net_tv = IpAddr {
        kind: ipv4,
        address: String::from("192.168.0.1")
    };

    println!("tv ip address: {}", net_tv);
}