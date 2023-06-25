use bit_vec::BitVec;
use space_packet_protocol::*;


#[test]
fn test_output() {

    let mut builder = SpacePacket::builder();

    let id = Identification::new_idle(PacketType::Telemetry);
    let seq = SequenceControl::new(SeqFlags::Unsegmented, BitVec::from_fn(14, |i| { i % 2 == 0 }));
    
    builder.identification(Some(id));
    builder.sequence_control(Some(seq));

    let data = BitVec::from_elem(10, true);
    builder.user_data(Some(data));

    let sp = builder.build().unwrap();
}