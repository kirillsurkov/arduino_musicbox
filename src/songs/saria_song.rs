use crate::note::{note, Note, NoteDesc};

pub static NOTES: [NoteDesc; 91] = [
    // 1 ===
    note(Note::F, 6, 12),
    note(Note::A, 6, 12),
    note(Note::B, 6, 25),
    note(Note::F, 6, 12),
    note(Note::A, 6, 12),
    note(Note::B, 6, 25),
    // ===
    note(Note::F, 6, 12),
    note(Note::A, 6, 12),
    note(Note::B, 6, 12),
    note(Note::E, 7, 12),
    note(Note::D, 7, 25),
    note(Note::B, 6, 12),
    note(Note::C, 7, 12),
    // ===
    note(Note::B, 6, 12),
    note(Note::G, 6, 12),
    note(Note::E, 6, 50),
    note(Note::SILENCE, 7, 1),
    note(Note::E, 6, 12),
    note(Note::D, 6, 12),
    // ===
    note(Note::E, 6, 12),
    note(Note::G, 6, 12),
    note(Note::E, 6, 75),
    // 2 ===
    note(Note::F, 6, 12),
    note(Note::A, 6, 12),
    note(Note::B, 6, 25),
    note(Note::F, 6, 12),
    note(Note::A, 6, 12),
    note(Note::B, 6, 25),
    // ===
    note(Note::F, 6, 12),
    note(Note::A, 6, 12),
    note(Note::B, 6, 12),
    note(Note::E, 7, 12),
    note(Note::D, 7, 25),
    note(Note::B, 6, 12),
    note(Note::C, 7, 12),
    // ===
    note(Note::E, 7, 12),
    note(Note::B, 6, 12),
    note(Note::G, 6, 50),
    note(Note::SILENCE, 7, 1),
    note(Note::G, 6, 12),
    note(Note::B, 6, 12),
    // ===
    note(Note::G, 6, 12),
    note(Note::D, 6, 12),
    note(Note::E, 6, 75),
    // 3 ===
    note(Note::D, 6, 12),
    note(Note::E, 6, 12),
    note(Note::F, 6, 25),
    note(Note::G, 6, 12),
    note(Note::A, 6, 12),
    note(Note::B, 6, 25),
    // ===
    note(Note::C, 7, 12),
    note(Note::B, 6, 12),
    note(Note::E, 6, 75),
    // ===
    note(Note::F, 6, 12),
    note(Note::G, 6, 12),
    note(Note::A, 6, 25),
    note(Note::B, 6, 12),
    note(Note::C, 7, 12),
    note(Note::D, 7, 25),
    // ===
    note(Note::E, 7, 12),
    note(Note::F, 7, 12),
    note(Note::G, 7, 75),
    // 4 ===
    note(Note::D, 6, 12),
    note(Note::E, 6, 12),
    note(Note::F, 6, 25),
    note(Note::G, 6, 12),
    note(Note::A, 6, 12),
    note(Note::B, 6, 25),
    // ===
    note(Note::C, 7, 12),
    note(Note::B, 6, 12),
    note(Note::E, 6, 75),
    // ===
    note(Note::F, 6, 12),
    note(Note::E, 6, 12),
    note(Note::A, 6, 12),
    note(Note::G, 6, 12),
    note(Note::B, 6, 12),
    note(Note::A, 6, 12),
    note(Note::C, 7, 12),
    note(Note::B, 6, 12),
    // ===
    note(Note::D, 7, 12),
    note(Note::C, 7, 12),
    note(Note::E, 7, 12),
    note(Note::D, 7, 12),
    note(Note::F, 7, 12),
    note(Note::E, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::E, 7, 6),
    note(Note::F, 7, 12),
    note(Note::D, 7, 6),
    // ===
    note(Note::E, 7, 100),
    note(Note::SILENCE, 7, 25)
];