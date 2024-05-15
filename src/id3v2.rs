#[derive(Debug)]
pub struct ID3v2 {
    tit2: String,  // Title/songname/content description
    tpe1: String,  // Lead performer(s)/Soloist(s)
    talb: String,  // Album/Movie/Show title
    tyer: u16,     // Year
    trck: u8,      // Track number/Position in set
    tcon: String,  // Content type (genre)
    tcom: String,  // Composer
    tpe2: String,  // Band/orchestra/accompaniment
    tpe3: String,  // Conductor/performer refinement
    tpe4: String,  // Interpreted, remixed, or otherwise modified by
    text: String,  // Lyricist/text writer
    tpos: u8,      // Part of a set
    tpub: String,  // Publisher
    tope: String,  // Original artist(s)/performer(s)
    apic: Vec<u8>, // Attached picture
}

impl ID3v2 {
    pub fn from(data: &[u8]) -> Option<Self> {
        if !ID3v2::is_id3v2(data) {
            return None;
        }

        let mut id3v2 = ID3v2 {
            tit2: String::new(),
            tpe1: String::new(),
            talb: String::new(),
            tyer: 0,
            trck: 0,
            tcon: String::new(),
            tcom: String::new(),
            tpe2: String::new(),
            tpe3: String::new(),
            tpe4: String::new(),
            text: String::new(),
            tpos: 0,
            tpub: String::new(),
            tope: String::new(),
            apic: Vec::new(),
        };

        let mut offset = 10;
        while offset < data.len() {
            println!("Offset: {}", offset);
            let frame_id = &data[offset..offset + 4];
            let frame_size =
                u32::from_be_bytes([0, data[offset + 4], data[offset + 5], data[offset + 6]])
                    as usize;

            let frame_flags = u16::from_be_bytes([data[offset + 8], data[offset + 9]]) as usize;
            let read = &data[offset + 10..offset + 10 + frame_size];

            match frame_id {
                b"TIT2" => id3v2.tit2 = String::from_utf8_lossy(read).to_string(),
                b"TPE1" => id3v2.tpe1 = String::from_utf8_lossy(read).to_string(),
                b"TALB" => id3v2.talb = String::from_utf8_lossy(read).to_string(),
                b"TYER" => id3v2.tyer = u16::from_be_bytes([read[0], read[1]]),
                b"TRCK" => id3v2.trck = read[0],
                b"TCON" => id3v2.tcon = String::from_utf8_lossy(read).to_string(),
                b"TCOM" => id3v2.tcom = String::from_utf8_lossy(read).to_string(),
                b"TPE2" => id3v2.tpe2 = String::from_utf8_lossy(read).to_string(),
                b"TPE3" => id3v2.tpe3 = String::from_utf8_lossy(read).to_string(),
                b"TPE4" => id3v2.tpe4 = String::from_utf8_lossy(read).to_string(),
                b"TEXT" => id3v2.text = String::from_utf8_lossy(read).to_string(),
                b"TPOS" => id3v2.tpos = read[0],
                b"TPUB" => id3v2.tpub = String::from_utf8_lossy(read).to_string(),
                b"TOPE" => id3v2.tope = String::from_utf8_lossy(read).to_string(),
                b"APIC" => id3v2.apic = read.to_vec(),

                _ => {
                    println!("Unknown frame: {:?}", frame_id);
                }
            };

            offset += 10 + frame_size;

            if frame_flags & 0b10000000 != 0 {
                offset += 10;
            }
        }

        Some(id3v2)
    }

    pub fn is_id3v2(data: &[u8]) -> bool {
        data.len() >= 10 && &data[0..3] == b"ID3"
    }

    #[inline]
    pub fn title(&self) -> &str {
        &self.tit2
    }

    #[inline]
    pub fn artist(&self) -> &str {
        &self.tpe1
    }
}
