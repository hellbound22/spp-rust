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
    sec_header: Option<SecondaryHeader<'a>>, // WARN: 4.1.4.2.1.3 | See Notes 2
    sec_header_size: usize,
    user_data: Option<UserData<'a>>, // WARN: shall contain at least one octet.
    user_data_size: usize,
}

impl<'a> DataField<'a> {
    pub fn new() -> Self {
        Self { sec_header: None, user_data: None, ..Default::default() }
    }

    pub fn len(&self) -> usize {
        let ud_size = if let Some(_s) = &self.user_data {
            self.user_data_size
        } else {
            0
        };

        let sh_size = if let Some(_s) = &self.sec_header {
            self.sec_header_size
        } else {
            0
        };

        sh_size + ud_size
    }

    pub fn user_data(&mut self, data: Option<&'a UserData>) {
        if let Some(data) = data {
            
            self.user_data_size = data.len();
            self.user_data = Some(data.clone());
        } else {
            self.user_data_size = 0;
            self.user_data = None;
        }
    }

    pub fn sec_header(&mut self, data: Option<&'a SecondaryHeader>) {
        
        if let Some(data) = data {
            self.sec_header_size = data.len();
            self.sec_header = Some(data.clone());
        } else {
            self.sec_header_size = 0;
            self.sec_header = None;
        }
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
    time_code_size: usize,
    ancillary: Option<&'a BitSlice>, // WARN: 4.1.4.3.2
    ancillary_size: usize,
}

impl<'a> SecondaryHeader<'a> {
    pub fn new(time_code: Option<&'a BitSlice>, ancillary: Option<&'a BitSlice>) -> Self {
        let (time_code_size, time_code) = if let Some(data) = time_code {
            (data.len(), Some(data.clone()))
        } else {
            (0, None)
        };

        let (ancillary_size, ancillary) = if let Some(data) = ancillary {
            (data.len(), Some(data.clone()))
        } else {
            (0, None)
        };

        Self { time_code, ancillary, time_code_size, ancillary_size }
    }
    pub fn len(&self) -> usize {
        self.ancillary_size + self.time_code_size
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

