const DOL_TEXT_SECTION_COUNT: usize = 7;
const DOL_DATA_SECTION_COUNT: usize = 11;

fn read_u32(data: &[u8], offset: usize) -> u32 {
    assert!(offset + 4 <= data.len());
    u32::from_be_bytes(data[offset..offset+4].try_into().unwrap())
}

fn write_u32(data: &mut [u8], offset: usize, value: u32) {
    let end = offset + 4;
    assert!(end <= data.len());
    let bytes = value.to_be_bytes();
    data[offset..end].copy_from_slice(&bytes);
}

pub struct Dol {
    pub data: Vec<u8>,
    pub sections: Vec<DolSection>,
    bss_address: u32,
    bss_size: u32,
    entry_point_address: u32,
}

impl Dol {
    pub fn new(data: Vec<u8>) -> Self {
        let mut sections: Vec<DolSection> = Vec::new();
        for section_index in 0..(DOL_TEXT_SECTION_COUNT + DOL_DATA_SECTION_COUNT) {
            sections.push(DolSection {
                offset: read_u32(data.as_slice(), 0x00 + section_index * 4),
                address: read_u32(data.as_slice(), 0x48 + section_index * 4),
                size: read_u32(data.as_slice(), 0x90 + section_index * 4),
            })
        }
        let bss_address = read_u32(data.as_slice(), 0xD8);
        let bss_size = read_u32(data.as_slice(), 0xDC);
        let entry_point_address = read_u32(data.as_slice(), 0xE0);

        Self {
            data,
            sections,
            bss_address,
            bss_size,
            entry_point_address,
        }
    }

    pub fn address_to_offset(&self, address: u32) -> Option<u32> {
        for section in &self.sections {
            if section.contains_address(address) {
                return Some(address - section.address + section.offset);
            }
        }

        None
    }

    pub fn write_data_u32(&mut self, address: u32, value: u32) {
        let offset = self.address_to_offset(address)
            .expect(format!("Address {address} is not found in any DOL sections.").as_str()) as usize;
        let end = offset + 4;
        if self.data.len() < end {
            self.data.resize(end, 0);
        }
        assert!(end <= self.data.len());
        let bytes = value.to_be_bytes();
        self.data[offset..end].copy_from_slice(&bytes);
    }

    pub fn write_data_bytes(&mut self, address: u32, bytes: &[u8]) {
        let offset = self.address_to_offset(address).unwrap() as usize;
        let end = offset + bytes.len();
        if self.data.len() < end {
            self.data.resize(end, 0);
        }
        assert!(end <= self.data.len());

        self.data[offset..end].copy_from_slice(bytes);
    }

    pub fn save_changes(&mut self) {
        let sections = self.sections.clone();
        let data = self.data.as_mut_slice();
        for (section_index, section) in sections.iter().enumerate() {
            write_u32(data, 0x00 + section_index * 4, section.offset);
            write_u32(data, 0x48 + section_index * 4, section.address);
            write_u32(data, 0x90 + section_index * 4, section.size);
        }

        write_u32(data, 0xD8, self.bss_address);
        write_u32(data, 0xDC, self.bss_size);
        write_u32(data, 0xE0, self.entry_point_address);
    }
}

#[derive(Copy, Clone)]
pub struct DolSection {
    pub offset: u32,
    pub address: u32,
    pub size: u32,
}

impl DolSection {
    fn contains_address(&self, address: u32) -> bool {
        self.address <= address && address < self.address + self.size
    }
}