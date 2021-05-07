use std::convert::TryInto;

pub const PACKET_START: u8 = b'$';
pub const CHECKSUM_START: u8 = b'#';
pub const ACK: u8 = b'+';
pub const NACK: u8 = b'-';
pub const INTERRUPT: u8 = 0x03;
pub const Q_HALT_REASON: &[u8] = b"?";

pub fn compute_checksum(body: &[u8]) -> u8 {
    let mut sum: u32 = 0;
    for char in body {
        sum = (sum + u32::from(*char)) % 256;
    }
    sum.try_into().unwrap()
}
