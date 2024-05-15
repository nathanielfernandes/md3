// kept as just a reader because it's a simple format
pub struct ID3v1<'a>(&'a [u8]);
impl<'a> ID3v1<'a> {
    #[rustfmt::skip]
    const GENRES: [&'static str; 148] = [
        "Blues", "Classic Rock", "Country", "Dance", "Disco",
        "Funk", "Grunge", "Hip-Hop", "Jazz", "Metal",
        "New Age", "Oldies", "Other", "Pop", "R&B",
        "Rap", "Reggae", "Rock", "Techno", "Industrial",
        "Alternative", "Ska", "Death Metal", "Pranks", "Soundtrack",
        "Euro-Techno", "Ambient", "Trip-Hop", "Vocal", "Jazz+Funk",
        "Fusion", "Trance", "Classical", "Instrumental", "Acid",
        "House", "Game", "Sound Clip", "Gospel", "Noise",
        "AlternRock", "Bass", "Soul", "Punk", "Space",
        "Meditative", "Instrumental Pop", "Instrumental Rock", "Ethnic", "Gothic",
        "Darkwave", "Techno-Industrial", "Electronic", "Pop-Folk", "Eurodance",
        "Dream", "Southern Rock", "Comedy", "Cult", "Gangsta",
        "Top 40", "Christian Rap", "Pop/Funk", "Jungle", "Native American",
        "Cabaret", "New Wave", "Psychedelic", "Rave", "Showtunes",
        "Trailer", "Lo-Fi", "Tribal", "Acid Punk", "Acid Jazz",
        "Polka", "Retro", "Musical", "Rock & Roll", "Hard Rock",
        "Folk", "Folk-Rock", "National Folk", "Swing", "Fast Fusion",
        "Bebob", "Latin", "Revival", "Celtic", "Bluegrass",
        "Avantgarde", "Gothic Rock", "Progressive Rock", "Psychedelic Rock", "Symphonic Rock",
        "Slow Rock", "Big Band", "Chorus", "Easy Listening", "Acoustic",
        "Humour", "Speech", "Chanson", "Opera", "Chamber Music",
        "Sonata", "Symphony", "Booty Bass", "Primus", "Porn Groove",
        "Satire", "Slow Jam", "Club", "Tango", "Samba",
        "Folklore", "Ballad", "Power Ballad", "Rhythmic Soul", "Freestyle",
        "Duet", "Punk Rock", "Drum Solo", "A capella", "Euro-House",
        "Dance Hall", "Goa", "Drum & Bass", "Club-House", "Hardcore Techno",
        "Terror", "Indie", "BritPop", "Negerpunk", "Polsk Punk",
        "Beat", "Christian Gangsta Rap", "Heavy Metal", "Black Metal", "Crossover",
        "Contemporary Christian", "Christian Rock", "Merengue", "Salsa", "Thrash Metal",
        "Anime", "JPop", "Synthpop"
    ];

    pub fn from(data: &'a [u8]) -> Option<Self> {
        if !ID3v1::is_id3v1(data) {
            return None;
        }

        Some(ID3v1(data))
    }

    #[inline]
    pub fn title_raw(&self) -> &[u8] {
        &self.0[3..33]
    }

    #[inline]
    pub fn title(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.title_raw()).map(|s| s.trim())
    }

    #[inline]
    pub fn artist_raw(&self) -> &[u8] {
        &self.0[33..63]
    }

    #[inline]
    pub fn artist(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.artist_raw()).map(|s| s.trim())
    }

    #[inline]
    pub fn album_raw(&self) -> &[u8] {
        &self.0[63..93]
    }

    #[inline]
    pub fn album(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.album_raw()).map(|s| s.trim())
    }

    #[inline]
    pub fn year_raw(&self) -> &[u8] {
        &self.0[93..97]
    }

    #[inline]
    pub fn year(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.year_raw()).map(|s| s.trim())
    }

    #[inline]
    pub fn comment_raw(&self) -> &[u8] {
        &self.0[97..127]
    }

    #[inline]
    pub fn comment(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.comment_raw()).map(|s| s.trim())
    }

    #[inline]
    pub fn genre(&self) -> Option<&str> {
        let genre = self.0[127];
        if genre == 255 {
            return None;
        }

        Some(ID3v1::GENRES[genre as usize])
    }

    pub fn is_id3v1(data: &[u8]) -> bool {
        data.len() >= 128 && &data[data.len() - 128..data.len() - 125] == b"TAG"
    }
}
