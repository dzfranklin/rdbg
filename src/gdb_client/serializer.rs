use crate::gdb_client::common::compute_checksum;
use std::io::Write;

use super::{common, ThreadId};

pub const Q_HALT_REASON: &[u8] = b"?";
pub const INTERRUPT: u8 = 0x03;
pub const Q_THREAD_INFO: &[u8] = b"qfThreadInfo";
pub const Q_READ_GENERAL_REGISTERS: &[u8] = b"g";

macro_rules! vwrite {
    ($vec:expr, $($arg:tt)+) => {{
        let ensure_is_vec: &mut Vec<u8> = $vec;
        write!(ensure_is_vec, $($arg)+).expect("Write to vec infallible");
    }};
}

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

pub fn hardware_breakpoint_req(is_add: bool, addr: u64, kind: u64) -> Vec<u8> {
    let mut req = Vec::new();
    let ty = if is_add { 'Z' } else { 'z' };
    vwrite!(&mut req, "{ty}1,{addr:x},{kind:x}");
    req
}

pub fn vcont(action: &[u8], thread: &ThreadId) -> Vec<u8> {
    let mut req = Vec::new();
    vwrite!(&mut req, "vCont;");
    req.extend_from_slice(action);
    vwrite!(&mut req, ":");
    write_thread_id(&mut req, thread);
    req
}

fn write_thread_id(out: &mut Vec<u8>, id: &ThreadId) {
    match id {
        ThreadId::MultiProcess { pid, tid } => vwrite!(out, "p{pid:x}.{tid:x}"),
        ThreadId::SingleProcess { tid } => vwrite!(out, "{tid:x}"),
        ThreadId::All => vwrite!(out, "-1"),
        ThreadId::Any => vwrite!(out, "0"),
    }
}
