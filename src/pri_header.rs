use bitvec::prelude::*;

use crate::errors::SPPError;

#[derive(Debug)]
pub struct PrimaryHeader {
    version_number: BitVec,             // 3 bits
    id: Identification,                 // 13 bits
    sequence_control: SequenceControl,  // 16 bits
    data_length: BitVec<u16, Msb0>,                // 16 bits
}

impl PrimaryHeader {
    pub fn new(id: &Identification, seq: &SequenceControl) -> Self {
        Self {version_number: bitvec![0; 3], id: id.clone(), sequence_control: seq.clone(), data_length: bitvec![u16, Msb0; 0; 16]}
    }

    // NOTE: 4.1.3.5
    pub fn data_lenght(&mut self, size: usize) {
        // TODO: this does not fail if size is bigger than u16_max
        let v: BitVec<u16, Msb0> = BitVec::from_slice(&[size as u16]);
        
        self.data_length = v
    }

    pub fn to_bits(&self) -> BitVec {
        let mut bit_rep = BitVec::new();

        bit_rep.extend(&self.version_number);
        bit_rep.extend(&self.id.to_bits());
        bit_rep.extend(&self.sequence_control.to_bits());
        bit_rep.extend(&self.data_length);

        bit_rep
    }

    pub fn new_from_octet_string(st: BitVec) -> Self {
    
        /* 
        Self {
            version_number: v
            id: 
            sequence_control: 
            data_lenght: 
        }
        */

        unimplemented!()
    }

}

#[derive(Debug, Clone)]
pub enum PacketType {
    Telemetry,
    Telecommand,
}

impl PacketType {
    fn to_bool(&self) -> bool {
        match self {
            Self::Telecommand => true,
            Self::Telemetry => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SecHeaderFlag {
    Present,
    NotPresent,
    Idle,
}

impl SecHeaderFlag {
    fn to_bool(&self) -> bool {
        match self {
            Self::Present => true,
            Self::NotPresent => false,
            Self::Idle => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identification {
    pub packet_type: PacketType,
    pub sec_header_flag: SecHeaderFlag, // For Idle Packets: '0'
    app_process_id:  BitVec // 11 bits // For Idle Packets: '11111111111'
}

impl Identification {
    pub fn new(t: PacketType, head: SecHeaderFlag, app: BitVec ) -> Result<Self, SPPError> {
        if app.len() != 11 {
            return Err(SPPError::APIDLenMismatch); // APID is more than 11 bits
        }

        Ok(Self { packet_type: t, sec_header_flag: head, app_process_id: app })
    }

    pub fn new_idle(t: PacketType) -> Self {
        Self { packet_type: t, sec_header_flag: SecHeaderFlag::Idle, 
            app_process_id: bitvec![1; 11]}
    }

    fn to_bits(&self) -> BitVec {
        let mut aux = BitVec::new();
        match self.packet_type.to_bool() {
            x => aux.push(x)
        }

        match self.sec_header_flag.to_bool() {
            x => aux.push(x)
        }

        aux.extend(&self.app_process_id);
        aux
    }
}

// WARN: 4.1.3.4.2.3
#[derive(Debug, Clone)]
pub enum SeqFlags {
    Continuation,
    First,
    Last,
    Unsegmented,
}

impl SeqFlags {
    fn to_bool(&self) -> [bool; 2] {
        match self {
            Self::Continuation => [false, false],
            Self::First => [false, true],
            Self::Last => [true, false],
            Self::Unsegmented => [true, true],
        }
    }
}


#[derive(Debug, Clone)]
pub struct SequenceControl {
    sequence_flags: SeqFlags,

    // WARN: For a Packet with the Packet Type set to ‘0’ (i.e., a telemetry Packet), this field
    // shall contain the Packet Sequence Count. For a Packet with the Packet Type set to ‘1’ (i.e., a
    // telecommand Packet), this field shall contain either the Packet Sequence Count or Packet Name.
    // WARN: This will most likely be set at the end of the 'Builder' of the packet: 4.1.3.4.3.4
    sequence_count_pkg_name: BitVec, // 14 bits
}

impl SequenceControl {
    pub fn new(flag: SeqFlags, count: BitVec) -> Result<Self, SPPError> {
        if count.len() != 14 {
            return Err(SPPError::SequenceControlLenMismatch); // Sequence count/pkg name is more than 14 bits
        }
        Ok(Self { sequence_flags: flag, sequence_count_pkg_name: count })
    }

    fn to_bits(&self) -> BitVec {
        let mut aux = BitVec::new();
        for b in self.sequence_flags.to_bool() {
            aux.push(b);
        }
        
        aux.extend(&self.sequence_count_pkg_name);
        aux
    }
}