use bit_vec::BitVec;

use crate::pri_header::{Identification, SequenceControl, PrimaryHeader};
use crate::data::{DataField, SecondaryHeader};

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

        let final_lenght: usize = data.len() / 8;
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