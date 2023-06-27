use crate::OCTET;

use bit_vec::BitVec;

#[derive(Debug)]
pub struct DataField {
    sec_header: Option<SecondaryHeader>, // WARN: 4.1.4.2.1.3 | See Notes 2
    user_data: BitVec, // WARN: shall contain at least one octet.
}

impl DataField {
    pub fn new() -> Self {
        Self { sec_header: None, user_data: BitVec::from_elem(8, false) }
    }

    pub fn len(&self) -> usize {
        self.sec_header.as_ref().unwrap_or(&SecondaryHeader::new()).len() + self.user_data.len()
    }

    pub fn user_data(&mut self, data: &Option<BitVec>) {
        if let Some(d) = data {
            self.user_data = d.clone();   
        } else {
            BitVec::from_elem(OCTET, false);
        }
    }

    pub fn sec_header(&mut self, data: &Option<SecondaryHeader>) {
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
    fn len(&self) -> usize {
        self.time_code.len() + self.ancillary.len()
    }

    fn to_bits(&self) -> BitVec {
        let mut comp = BitVec::new();

        comp.extend(&self.time_code);
        comp.extend(&self.ancillary);

        comp
    }
}

