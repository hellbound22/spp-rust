use bit_vec::BitVec;
use space_packet_protocol::*;


#[test]
fn test_output() {
    let mut builder = SpacePacket::builder();

    let id = Identification::new_idle(PacketType::Telemetry);
    let seq = SequenceControl::new(SeqFlags::Unsegmented, BitVec::new());
    
    builder.identification(Some(id));
    builder.sequence_control(Some(seq));

    let sp = builder.build().unwrap();

    dbg!(sp.to_bits());
}