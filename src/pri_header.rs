use bitvec::prelude::*;

use crate::errors::SPPError;

#[derive(Debug)]
pub struct PrimaryHeader {
    version_number: BitArray,             // 3 bits
    id: Identification,                 // 13 bits
    sequence_control: SequenceControl,  // 16 bits
    pub data_length: BitArr!(for 16, in u16, Msb0),                // 16 bits
}

impl PrimaryHeader {
    pub fn new(id: &Identification, seq: &SequenceControl) -> Self {
        Self {version_number: bitarr![0; 3], id: id.clone(), sequence_control: seq.clone(), data_length: bitarr![u16, Msb0; 0; 16]}
    }

    // NOTE: 4.1.3.5
    pub fn data_lenght(&mut self, size: usize) {
        // TODO: this does not fail if size is bigger than u16_max
        let binding = [size as u16];
        let v: &BitSlice<u16, Msb0> = BitSlice::from_slice(&binding);
        
        
        let m = self.data_length.as_mut_bitslice();
        m.clone_from_bitslice(v);
    }

    pub fn to_bits(&self) -> BitArr!(for 48) {
        let mut bit_rep = bitarr!(0; 48);

        for (i, mut mb) in bit_rep[..3].iter_mut().enumerate() {
            *mb = self.version_number[i];
        }
        

        let id = self.id.to_bits();
        for (i, mut mb) in bit_rep[3..16].iter_mut().enumerate() {
            *mb = id[i];
        }

        let sc = self.sequence_control.to_bits();
        for (i, mut mb) in bit_rep[16..32].iter_mut().enumerate() {
            *mb = sc[i];
        }

        for (i, mut mb) in bit_rep[32..48].iter_mut().enumerate() {
            *mb = self.data_length[i];
        }

        bit_rep
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
    app_process_id:  BitArray // 11 bits // For Idle Packets: '11111111111'
}

impl<'a> Identification {
    pub fn new(t: PacketType, head: SecHeaderFlag, app: BitArray ) -> Result<Self, SPPError> {
        if app.len() != 11 {
            return Err(SPPError::APIDLenMismatch); // APID is more than 11 bits
        }

        Ok(Self { packet_type: t, sec_header_flag: head, app_process_id: app })
    }

    pub fn new_idle(t: PacketType) -> Self {
        Self { packet_type: t, sec_header_flag: SecHeaderFlag::Idle, 
            app_process_id: bitarr![1; 11]}
    }
    
    pub fn to_bits(&self) -> BitArray {
        let mut aux = bitarr!(0;13);
        {
            let mut pt = aux.get_mut(0).unwrap();
            *pt = self.packet_type.to_bool();
        }
        {
            let mut sf = aux.get_mut(1).unwrap();
            *sf = self.sec_header_flag.to_bool();
        }

        for (i, mut mb) in aux[2..13].iter_mut().enumerate() {    
            *mb = self.app_process_id[i];
        }

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
    sequence_count_pkg_name: BitArray<[u16; 1], LocalBits>, // 14 bits
}

impl SequenceControl {
    pub fn new(flag: SeqFlags, count: BitArray<[u16; 1]>) -> Result<Self, SPPError> {
        /*
        Commented out because https://github.com/ferrilab/bitvec/issues/159
        Workaround in the builder
        if count.len() != 14 {
            return Err(SPPError::SequenceControlLenMismatch); // Sequence count/pkg name is more than 14 bits
        }
        */
        Ok(Self { sequence_flags: flag, sequence_count_pkg_name: count })
    }

    fn to_bits(&self) -> BitArr!(for 16) {
        let mut aux = bitarr!(0; 16);

        let sf = self.sequence_flags.to_bool();
        for (i, mut mb) in aux[..2].iter_mut().enumerate() {
            *mb = sf[i];
        }
        
        let pk = self.sequence_count_pkg_name;
        for (i, mut mb) in aux[2..16].iter_mut().enumerate() {
            *mb = pk[i];
        }
        
        aux
    }
}