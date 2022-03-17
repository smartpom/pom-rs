#[derive(Debug,Copy,Clone)]
struct challenger<'a> {
    pubKey: &'a str,
    macAddress: str,
    ipAddress: str,
    p2pAddress: str,
}