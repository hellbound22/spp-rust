#![cfg_attr(not(test), no_std)]

extern crate alloc;

use bit_vec::BitVec;

pub const OCTET: usize = 8;


#[derive(Default)]
pub struct Builder {
    id: Option<Identification>,
    seq: Option<SequenceControl>,
    sec_head: Option<SecondaryHeader>,
    user_data: Option<BitVec>,
    idle: bool,
}

impl Builder {
    pub fn idle(&mut self, set: bool) {
        self.idle = set
    }

    pub fn identification(&mut self, id: Option<Identification>) {
        self.id = id
    }
    
    pub fn sequence_control(&mut self, sequence_control: Option<SequenceControl>) {
        self.seq = sequence_control
    }

    pub fn secondary_header(&mut self, sec_head: Option<SecondaryHeader>) {
        self.sec_head = sec_head
    }

    pub fn user_data(&mut self, user_data: Option<BitVec>) {
        self.user_data = user_data
    }

    // TODO: change error type
    // TODO: Implement idle behaviour
    pub fn build(&mut self) -> Result<SpacePacket, u8> {
        let mut pri_head = PrimaryHeader::new(self.id.as_ref().unwrap(), self.seq.as_ref().unwrap());
        let mut data = DataField::new();
        data.sec_header(&self.sec_head);
        data.user_data(&self.user_data);

        let final_lenght: usize = data.lenght() / 8;
        pri_head.data_lenght(final_lenght);
        
        let sp = SpacePacket::new(pri_head, data);

        Ok(sp)
    }
}

#[derive(Debug)]
pub struct SpacePacket {
    primary_header: PrimaryHeader,
    pub data_field: DataField,
}


impl SpacePacket {
    fn new(ph: PrimaryHeader, df: DataField) -> Self {
        Self { primary_header: ph, data_field: df }
    }
    
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn to_bits(&self) -> BitVec {
        // Order: Primary Header - Data Field
        let mut bit_rep = BitVec::new();

        bit_rep.extend(&self.primary_header.to_bits());
        bit_rep.extend(&self.data_field.to_bits());

        bit_rep
    }
}

#[derive(Debug)]
pub struct DataField {
    sec_header: Option<SecondaryHeader>, // WARN: 4.1.4.2.1.3 | See Notes 2
    user_data: BitVec, // WARN: shall contain at least one octet.
}

impl DataField {
    fn new() -> Self {
        Self { sec_header: None, user_data: BitVec::from_elem(8, false) }
    }

    fn lenght(&self) -> usize {
        self.sec_header.as_ref().unwrap_or(&SecondaryHeader::new()).lenght() + self.user_data.len()
    }

    fn user_data(&mut self, data: &Option<BitVec>) {
        if let Some(d) = data {
            self.user_data = d.clone();   
        } else {
            BitVec::from_elem(OCTET, false);
        }
    }

    fn sec_header(&mut self, data: &Option<SecondaryHeader>) {
        self.sec_header = data.clone()
    }

    pub fn to_bits(&self) -> BitVec {
        if let Some(head) = &self.sec_header {
            let mut x = head.clone().to_bits();
            x.extend(&self.user_data);
            x
        } else {
            self.user_data.clone()
        }
    }
}

// https://sanaregistry.org/r/space_packet_protocol_secondary_header_format_document.
#[derive(Clone, Debug)]
pub struct SecondaryHeader {
    time_code: BitVec, // TODO: See Note 4.1.4.2.2.2
    ancillary: BitVec, // WARN: 4.1.4.3.2
}

impl SecondaryHeader {
    fn new() -> Self {
        Self { time_code: BitVec::new(), ancillary: BitVec::new() }
    }
    fn lenght(&self) -> usize {
        self.time_code.len() + self.ancillary.len()
    }

    fn to_bits(&self) -> BitVec {
        let mut comp = BitVec::new();

        comp.extend(&self.time_code);
        comp.extend(&self.ancillary);

        comp
    }
}

#[derive(Debug)]
struct PrimaryHeader {
    version_number: BitVec, // 3 bits
    id: Identification,
    sequence_control: SequenceControl,
    data_length: BitVec, // 16 bits
}

impl PrimaryHeader {
    fn new(id: &Identification, seq: &SequenceControl) -> Self {
        Self {version_number: BitVec::from_elem(3, false), id: id.clone(), sequence_control: seq.clone(), data_length: BitVec::from_elem(16, false)}
    }

    // NOTE: 4.1.3.5
    fn data_lenght(&mut self, size: usize) {
        // TODO: Check if 'size' does not exceed 2 OCTETS
        let number = &size.to_be_bytes()[6..];
        let v = BitVec::from_bytes(number);
        
        self.data_length = v
    }

    fn to_bits(&self) -> BitVec {
        let mut bit_rep = BitVec::new();

        bit_rep.extend(&self.version_number);
        bit_rep.extend(&self.id.to_bits());
        bit_rep.extend(&self.sequence_control.to_bits());
        bit_rep.extend(&self.data_length);

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
    packet_type: PacketType,
    sec_header_flag: SecHeaderFlag, // For Idle Packets: '0'
    app_process_id:  BitVec // 11 bits // For Idle Packets: '11111111111'
}

impl Identification {
    // TODO: Check for bit count
    pub fn new(t: PacketType, head: SecHeaderFlag, app: BitVec ) -> Self {
        Self { packet_type: t, sec_header_flag: head, app_process_id: app }
    }

    pub fn new_idle(t: PacketType) -> Self {
        Self { packet_type: t, sec_header_flag: SecHeaderFlag::Idle, 
            app_process_id: BitVec::from_elem(11, true)}
    }

    fn to_bits(&self) -> BitVec {
        let mut aux = BitVec::new();
        match self.packet_type.to_bool() {
            x => aux.push(x)
        }

        match self.sec_header_flag.to_bool() {
            x => aux.push(x)
        }

        aux.extend(&self.app_process_id); // TODO: Check for 11 bits
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
    pub fn new(flag: SeqFlags, count: BitVec) -> Self {
        Self { sequence_flags: flag, sequence_count_pkg_name: count }
    }

    fn to_bits(&self) -> BitVec {
        let mut aux = BitVec::new();
        for b in self.sequence_flags.to_bool() {
            aux.push(b);
        }
        
        aux.extend(&self.sequence_count_pkg_name); // TODO: check for 14 bits
        aux
    }
}