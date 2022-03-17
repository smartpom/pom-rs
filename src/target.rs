#[derive(Debug,Copy,Clone)]
struct target<'a> {
    pubKey: &'a str,
    ipAddress: str,
    p2pAddress: str,
}