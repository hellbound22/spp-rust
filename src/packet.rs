use bit_vec::BitVec;

use crate::{OCTET, PRIMARY_HEADER, MAX_DATA_SIZE};
use crate::pri_header::{Identification, SequenceControl, PrimaryHeader, SecHeaderFlag};
use crate::data::{DataField, SecondaryHeader};
use crate::errors::SPPError;

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

    pub fn build(&mut self) -> Result<SpacePacket, SPPError> {
        let (id, seq) = match (&self.id, &self.seq) {
            (None, None) => return Err(SPPError::MandatoryFieldNotPresent),
            (None, _) => return Err(SPPError::MandatoryFieldNotPresent),
            (_, None) => return Err(SPPError::MandatoryFieldNotPresent),
            (id, seq) => (id.clone().unwrap(), seq.clone().unwrap()),
        };

        if self.idle {
            let new_id = Identification::new_idle(id.packet_type.clone());
            self.identification(Some(new_id));
        }
        
        let mut pri_head = PrimaryHeader::new(&id, &seq);
        let mut data = DataField::new();

        if let SecHeaderFlag::Present = id.sec_header_flag {
            if self.sec_head.is_none() {
                return Err(SPPError::SecondaryHeaderNotPresent);
            } else if self.sec_head.as_ref().unwrap().len() < 1 {
                return Err(SPPError::SecondaryHeaderNotPresent);
            }
        }

        data.sec_header(&self.sec_head);
        data.user_data(&self.user_data);

        let final_lenght: usize = data.len() / OCTET;

        if final_lenght > MAX_DATA_SIZE {
            return Err(SPPError::MaxDataSizeExedded); // Total data size exceeds max data size
        }

        pri_head.data_lenght(final_lenght);
        
        let sp = SpacePacket::new(pri_head, data);

        if sp.data_field.len() < OCTET {
            return Err(SPPError::MinDataLen); // Data field must be at least 1 octet long
        }

        Ok(sp)
    }
}

#[derive(Debug)]
pub struct SpacePacket {
    primary_header: PrimaryHeader,
    data_field: DataField,
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

    pub fn len(&self) -> usize {
        self.data_field.len() + PRIMARY_HEADER
    }
}