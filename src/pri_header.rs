use bitvec::prelude::*;

use crate::errors::SPPError;

#[derive(Debug)]
pub struct PrimaryHeader {
    version_number: BitArr!(for 3, in u8),             // 3 bits
    id: Identification,                 // 13 bits
    sequence_control: SequenceControl,  // 16 bits
    pub data_length: BitArr!(for 16, in u8),                // 16 bits
}

impl PrimaryHeader {
    pub fn new(id: &Identification, seq: &SequenceControl) -> Self {
        Self {version_number: bitarr![u8, LocalBits; 0; 3], id: id.clone(), sequence_control: seq.clone(), data_length: bitarr![u8, LocalBits; 0; 16]}
    }

    pub fn new_from_slice(s: &BitSlice<u8>) -> Self {
        let mut version_number = bitarr!(u8, LocalBits; 0; 3);
        version_number[..3].copy_from_bitslice(&s[..3]);

        let id = Identification::new_from_slice(&s[3..16]);
        let sequence_control = SequenceControl::new_from_slice(&s[16..32]);

        let mut data_length = bitarr!(u8, LocalBits; 0; 16);
        data_length[32..48].copy_from_bitslice(&s[32..48]);

        Self { version_number, id, sequence_control, data_length }
    }

    // NOTE: 4.1.3.5
    pub fn data_lenght(&mut self, size: usize) {
        // TODO: this does not fail if size is bigger than u16_max
        let binding = [size.to_be()];
        let v: &BitSlice<usize, LocalBits> = BitSlice::from_slice(&binding);
        
        
        let m = self.data_length.as_mut_bitslice();
        m.clone_from_bitslice(v);
    }

    pub fn to_bits<'a>(&self, _aux: &'a mut BitSlice<u8>) {
        _aux[..3].copy_from_bitslice(&self.version_number[..3]);

        self.id.to_bits(&mut _aux[3..3+13]);

        self.sequence_control.to_bits(&mut _aux[16..32]);

        _aux[32..48].copy_from_bitslice(&self.data_length[..16]);
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
    app_process_id:  BitArray<[u8; 2]> // 11 bits // For Idle Packets: '11111111111'
}

impl<'a> Identification {
    pub fn new(t: PacketType, head: SecHeaderFlag, app: &BitSlice<u8> ) -> Result<Self, SPPError> {
        if app.len() != 11 {
            return Err(SPPError::APIDLenMismatch(app.len())); // APID is more than 11 bits
        }

        let mut a = bitarr!(u8, LocalBits; 0; 11);
        a[..11].copy_from_bitslice(app);

        Ok(Self { packet_type: t, sec_header_flag: head, app_process_id: a })
    }

    pub fn new_idle(t: PacketType) -> Self {
        Self { packet_type: t, sec_header_flag: SecHeaderFlag::Idle, 
            app_process_id: bitarr![u8, LocalBits; 1; 11]}
    }

    pub fn new_from_slice(s: &BitSlice<u8>) -> Self {
        unimplemented!()
    }
    
    pub fn to_bits(&self, _aux: &'a mut BitSlice<u8>) {
        *_aux.get_mut(0).unwrap() = self.packet_type.to_bool();
        *_aux.get_mut(1).unwrap() = self.sec_header_flag.to_bool();

        _aux[2..2 + 11].copy_from_bitslice(&self.app_process_id[..11]);
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
    fn to_bool(&self) -> &'static BitSlice<u8> {
        match self {
            Self::Continuation => bits![static u8, LocalBits; 0, 0],
            Self::First => bits![static u8, LocalBits;  0, 1],
            Self::Last => bits![static u8, LocalBits;  1, 0],
            Self::Unsegmented => bits![static u8, LocalBits;  1, 1],
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
    sequence_count_pkg_name: BitArray<[u8; 2], LocalBits>, // 14 bits
}

impl SequenceControl {
    pub fn new(flag: SeqFlags, count: BitArray<[u8; 2]>) -> Result<Self, SPPError> {
        /*
        Commented out because https://github.com/ferrilab/bitvec/issues/159
        Workaround in the builder
        if count.len() != 14 {
            return Err(SPPError::SequenceControlLenMismatch); // Sequence count/pkg name is more than 14 bits
        }
        */
        Ok(Self { sequence_flags: flag, sequence_count_pkg_name: count })
    }

    fn new_from_slice(s: &BitSlice<u8>) -> Self {
        unimplemented!()
    }

    fn to_bits<'a>(&self, _aux: &'a mut BitSlice<u8>) {
        
        let sf = self.sequence_flags.to_bool();
        _aux[..2].copy_from_bitslice(&sf);

        _aux[2..].copy_from_bitslice(&self.sequence_count_pkg_name[..14]);
    }
}