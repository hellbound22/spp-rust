use bitvec::prelude::*;

use spp_rust::{packet::{SpacePacket, OctetStringSpacePacket}, pri_header::{Identification, PacketType, SequenceControl, SeqFlags}, data::{SecondaryHeader, UserData}};

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

    let data = BitSlice::from_slice("Teste ooooi".as_bytes());
    let ud = UserData::new(&data);
    builder.user_data(Some(&ud));

    let mut sp = builder.build().unwrap();
 
    let bits = sp.to_bits();

    assert_eq!(bits.len(), MIN_SP_SIZE_BITS);

    let s = OctetStringSpacePacket::new_from_slice(bits);
    
    let data: Vec<char> = s.data_field.domain().map(|x| x as char).collect();
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