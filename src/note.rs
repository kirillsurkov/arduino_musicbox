use const_soft_float::soft_f64::SoftF64;

pub enum Note {
    SILENCE,
    AB,
    A,
    AS,
    BB,
    B,
    BS,
    CB,
    C,
    CS,
    DB,
    D,
    DS,
    EB,
    E,
    ES,
    FB,
    F,
    FS,
    GB,
    G,
    GS,
}

pub struct NoteDesc {
    pub frequency: u16,
    pub duration: u8,
}

pub const fn note(note: Note, octave: u8, duration: u8) -> NoteDesc {
    let base_freq = 432.0;
    let base_octave = 4;

    //let octave = octave - 1;

    let semitones = match note {
        Note::SILENCE => {
            return NoteDesc {
                frequency: 1,
                duration,
            }
        }
        Note::CB => -10,
        Note::C => -9,
        Note::CS | Note::DB => -8,
        Note::D => -7,
        Note::DS | Note::EB => -6,
        Note::E | Note::FB => -5,
        Note::F | Note::ES => -4,
        Note::FS | Note::GB => -3,
        Note::G => -2,
        Note::GS | Note::AB => -1,
        Note::A => 0,
        Note::AS | Note::BB => 1,
        Note::B => 2,
        Note::BS => 3,
    } + (octave as i16 - base_octave) * 12;

    let base = SoftF64(2.0);
    let exp = SoftF64(semitones as f64 / 12.0);
    let frequency = (base_freq * base.powf(exp).to_f64()) as u16;

    NoteDesc {
        frequency,
        duration,
    }
}
