#[derive(Debug,Copy,Clone)]
struct witness<'a> {
    pubKey: &'a str,
    ipAddress: str,
    p2pAddress: str,
}