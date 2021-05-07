use crate::gdb_client::common::compute_checksum;

use super::common;

pub fn packet(body: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(body.len() + 4);
    packet.push(common::PACKET_START);
    packet.extend_from_slice(body);
    packet.push(common::CHECKSUM_START);
    let checksum = checksum(body);
    packet.extend_from_slice(&checksum);
    packet
}

fn checksum(body: &[u8]) -> Vec<u8> {
    let checksum = compute_checksum(body);
    let checksum = format!("{:02x}", checksum);
    assert_eq!(2, checksum.len());
    checksum.into_bytes()
}
