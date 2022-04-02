use std::net::IpAddr;

fn main() {
    // dev knows that it will not fail
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    let home = "127.0.0.1".parse::<IpAddr>().unwrap();
}
