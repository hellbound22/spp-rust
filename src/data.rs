use bitvec::prelude::*;

const MAX_DATA: usize = 65536;
type DataSize = BitArr!(for MAX_DATA);

#[derive(Debug, Default, Clone)]
pub struct UserData<'a> {
    pub data: &'a BitSlice,
}

impl<'a> UserData<'a> {
    pub fn new(data: &'a BitSlice) -> Self {
        Self { data: data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}


#[derive(Debug, Default)]
pub struct DataField<'a> {
    sec_header: Option<&'a SecondaryHeader<'a>>, // WARN: 4.1.4.2.1.3 | See Notes 2
    //sec_header_size: usize,
    user_data: Option<&'a UserData<'a>>, // WARN: shall contain at least one octet.
    //user_data_size: usize,
}

impl<'a> DataField<'a> {
    pub fn new() -> Self {
        Self { sec_header: None, user_data: None, ..Default::default() }
    }

    pub fn len(&self) -> usize {
        let ud_size = if let Some(s) = &self.user_data {
            s.len()
        } else {
            0
        };

        let sh_size = if let Some(s) = &self.sec_header {
            s.len()
        } else {
            0
        };

        sh_size + ud_size
    }

    pub fn user_data(&mut self, data: Option<&'a UserData>) {
        self.user_data = data;
    }

    pub fn sec_header(&mut self, data: Option<&'a SecondaryHeader>) {
        self.sec_header = data;
    }

    pub fn to_bits(&self) -> DataSize {
        let mut bits = bitarr!(0; MAX_DATA);

        let mut fin_sec_header: usize = 0;


        if let Some(head) = &self.sec_header {
            let head = head.to_bits();
            fin_sec_header = head.len();
            for (i, mut mb) in bits[..fin_sec_header].iter_mut().enumerate() {
                *mb = head[i]
            }
        } 

        if let Some(data) = &self.user_data {
            for (i, mut mb) in bits[fin_sec_header..data.len()].iter_mut().enumerate() {
                *mb = data.data[i];
            }
        }

        
        bits
    }
}

// https://sanaregistry.org/r/space_packet_protocol_secondary_header_format_document : Still a canditate
#[derive(Clone, Debug, Default)]
pub struct SecondaryHeader<'a> {
    time_code: Option<&'a BitSlice>, // TODO: Implement timecode formats See Note 4.1.4.2.2.2
    ancillary: Option<&'a BitSlice>, // WARN: 4.1.4.3.2
}

impl<'a> SecondaryHeader<'a> {
    pub fn new(time_code: Option<&'a BitSlice>, ancillary: Option<&'a BitSlice>) -> Self {
        Self { time_code, ancillary }
    }
    pub fn len(&self) -> usize {
        let tc_size = if let Some(s) = &self.time_code {
            s.len()
        } else {
            0
        };

        let anc_size = if let Some(s) = &self.ancillary {
            s.len()
        } else {
            0
        };

        tc_size + anc_size
    }

    fn to_bits(&self) -> DataSize {
        let mut bits = bitarr!(0; MAX_DATA);

        let mut fin_sec_header: usize = 0;


        if let Some(tc) = &self.time_code {
            fin_sec_header = tc.len();

            for mut mb in bits.iter_mut() {
                for b in tc.iter() {
                    *mb = *b;
                }
            }

        } 
        if let Some(data) = &self.ancillary {
            for mut mb in bits[fin_sec_header..].iter_mut() {
                for b in data.iter() {
                    *mb = *b;
                }
            }
        }

        bits
    }
}

