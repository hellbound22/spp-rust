use bitvec::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct UserData<'a> {
    data: &'a BitSlice<u8>,
}

impl<'a> UserData<'a> {
    pub fn new(data: &'a BitSlice<u8>) -> Self {
        Self { data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
    fn to_bits(&self, _aux: &mut (&mut usize, &'a mut BitSlice<u8>)) {
        _aux.1[*_aux.0..*_aux.0 + self.len()].copy_from_bitslice(self.data);
        *_aux.0 += self.len();
    }
}


#[derive(Debug, Default)]
pub struct DataField<'a> {
    sec_header: Option<&'a SecondaryHeader<'a>>, // WARN: 4.1.4.2.1.3 | See Notes 2
    user_data: Option<&'a UserData<'a>>, // WARN: shall contain at least one octet.
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

    pub fn to_bits(&self, _aux: &mut (&mut usize, &'a mut BitSlice<u8>)) {
        if let Some(sh) = self.sec_header {
            sh.to_bits(_aux);
        }
        if let Some(ud) = self.user_data {
            ud.to_bits(_aux);
        }
    }
}

// https://sanaregistry.org/r/space_packet_protocol_secondary_header_format_document : Still a canditate
#[derive(Debug, Default)]
pub struct SecondaryHeader<'a> {
    time_code: Option<&'a BitSlice<u8>>, // TODO: Implement timecode formats See Note 4.1.4.2.2.2
    ancillary: Option<&'a BitSlice<u8>>, // WARN: 4.1.4.3.2
}

impl<'a> SecondaryHeader<'a> {
    pub fn new(time_code: Option<&'a BitSlice<u8>>, ancillary: Option<&'a BitSlice<u8>>) -> Self {
        Self { time_code, ancillary, ..Default::default() }
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

    fn to_bits(&self, _aux: &mut (&mut usize, &'a mut BitSlice<u8>)) {
        let tc = self.time_code.unwrap_or_default();
        let ac = self.ancillary.unwrap_or_default();


        _aux.1[*_aux.0..tc.len()].copy_from_bitslice(tc);
        _aux.1[*_aux.0 + tc.len()..].copy_from_bitslice(ac);
        *_aux.0 += self.len();
    }
}

