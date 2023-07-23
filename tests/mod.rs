use bitvec::prelude::*;

use spp_rust::{packet::SpacePacket, pri_header::{Identification, PacketType, SequenceControl, SeqFlags}, data::SecondaryHeader};

const MIN_SP_SIZE_BITS: usize = 7 * 8;
const MAX_SP_SIZE_BITS: usize = 65542 * 8;


#[test]
fn test_output() {

    let mut builder = SpacePacket::builder();

    let id = Identification::new_idle(PacketType::Telemetry);
    let seq = SequenceControl::new(SeqFlags::Unsegmented, bitvec![1; 14]).unwrap();
    
    builder.identification(Some(id));
    builder.sequence_control(Some(seq));

    let data = bitvec![0; 8];
    builder.user_data(Some(data));

    let sp = builder.build().unwrap();
 
    dbg!(&sp);
    dbg!(&sp.to_bits());

    assert_eq!(sp.len(), MIN_SP_SIZE_BITS);
}

#[test]
fn test_sec_header_req() {
    let mut builder = SpacePacket::builder();

    let id = Identification::new(PacketType::Telemetry, spp_rust::pri_header::SecHeaderFlag::Present, bitvec![0; 11]).unwrap();
    let seq = SequenceControl::new(SeqFlags::Unsegmented, bitvec![0; 14]).unwrap();
    let sec_head = SecondaryHeader::new(Some(bitvec![1; 8]), None);
    
    builder.identification(Some(id));
    builder.sequence_control(Some(seq));
    builder.secondary_header(Some(sec_head));

    let data = bitvec![1; 8];
    builder.user_data(Some(data));

    let sp = builder.build().unwrap();
}