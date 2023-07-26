use bitvec::prelude::*;

use spp_rust::{packet::SpacePacket, pri_header::{Identification, PacketType, SequenceControl, SeqFlags}, data::{SecondaryHeader, UserData}};

const MIN_SP_SIZE_BITS: usize = 7 * 8;
const MAX_SP_SIZE_BITS: usize = 65542 * 8;


#[test]
fn test_output() {
    let mut builder = SpacePacket::builder();

    let id = Identification::new_idle(PacketType::Telemetry);

    let ba =  bitarr!(u8, LocalBits; 0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1);
    let seq = SequenceControl::new(SeqFlags::Unsegmented, ba).unwrap();
    
    builder.identification(Some(id));
    builder.sequence_control(Some(seq));

    let data = bits![u8, LocalBits; 0,1,1,1,1,1,1,0];
    let ud = UserData::new(&data);
    builder.user_data(Some(&ud));

    let mut sp = builder.build().unwrap();
 
    let bits = sp.to_bits();

    assert_eq!(bits.len(), MIN_SP_SIZE_BITS);
}

#[test]
fn test_sec_header_req() {
    let mut builder = SpacePacket::builder();

    let id = Identification::new(PacketType::Telemetry, spp_rust::pri_header::SecHeaderFlag::Present, bits![u8, LocalBits; 0; 11]).unwrap();
    let ba =  bitarr!(u8, LocalBits; 0;14);
    let seq = SequenceControl::new(SeqFlags::Unsegmented, ba).unwrap();

    let tc = bits![u8, LocalBits; 0,0,0,0];

    let sec_head = SecondaryHeader::new(Some(tc), None);
    
    builder.identification(Some(id));
    builder.sequence_control(Some(seq));
    builder.secondary_header(Some(&sec_head));
    
    let data = bitarr!(u8, LocalBits; 1; 65536);
    let ud = UserData::new(&data);
    builder.user_data(Some(&ud));

    let sp = builder.build().unwrap();
}