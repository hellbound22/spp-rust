use bit_vec::BitVec;
use spp_rust::{packet::SpacePacket, pri_header::{Identification, PacketType, SequenceControl, SeqFlags}, data::SecondaryHeader};


#[test]
fn test_output() {

    let mut builder = SpacePacket::builder();

    let id = Identification::new_idle(PacketType::Telemetry);
    let seq = SequenceControl::new(SeqFlags::Unsegmented, BitVec::from_fn(14, |i| { i % 2 == 0 })).unwrap();
    
    builder.identification(Some(id));
    builder.sequence_control(Some(seq));

    let data = BitVec::from_elem(8, true);
    builder.user_data(Some(data));

    let sp = builder.build().unwrap();

    dbg!(&sp);
    dbg!(sp.to_bits());
}

#[test]
fn test_sec_header_req() {
    let mut builder = SpacePacket::builder();

    let id = Identification::new(PacketType::Telemetry, spp_rust::pri_header::SecHeaderFlag::Present, BitVec::from_elem(11, false)).unwrap();
    let seq = SequenceControl::new(SeqFlags::Unsegmented, BitVec::from_fn(14, |i| { i % 2 == 0 })).unwrap();
    let sec_head = SecondaryHeader::new(Some(BitVec::from_elem(8, true)), None);
    
    builder.identification(Some(id));
    builder.sequence_control(Some(seq));
    builder.secondary_header(Some(sec_head));

    let data = BitVec::from_elem(8, true);
    builder.user_data(Some(data));

    let sp = builder.build().unwrap();
}