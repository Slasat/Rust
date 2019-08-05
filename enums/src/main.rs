enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String, 
} 
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
     
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}
