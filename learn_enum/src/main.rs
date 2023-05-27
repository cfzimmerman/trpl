enum IpAddrKind {
    V4(i16, i16, i16, i16),
    V6(String)
}

fn print_ip(addr: &Option<IpAddrKind>) -> () {
    match addr {
        Some(IpAddrKind::V4(a, b, c, d)) => println!("IPV4: {a}.{b}.{c}.{d}"),
        Some(IpAddrKind::V6(str)) => println!("IPV6: {}", str),
        None => println!("Empty IP address")
    };
}

fn main() {
    let v4 = Some(IpAddrKind::V4(0, 0, 0, 0));
    let v6 = Some(IpAddrKind::V6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334")));
    print_ip(&v4);
    print_ip(&v6);
    print_ip(&None);
}
