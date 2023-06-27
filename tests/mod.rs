use bit_vec::BitVec;
use spp_rust::{packet::SpacePacket, pri_header::{Identification, PacketType, SequenceControl, SeqFlags}};


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