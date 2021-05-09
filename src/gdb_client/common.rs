use std::convert::TryInto;

pub const PACKET_START: u8 = b'$';
pub const CHECKSUM_START: u8 = b'#';
pub const OK: &[u8] = b"OK";
pub const ACK: u8 = b'+';
pub const NACK: u8 = b'-';

pub fn compute_checksum(body: &[u8]) -> u8 {
    let mut sum: u32 = 0;
    for char in body {
        sum = (sum + u32::from(*char)) % 256;
    }
    sum.try_into().unwrap()
}
