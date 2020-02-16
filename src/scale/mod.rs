pub mod notes;

use notes::Note;

pub const C_MAJOR: [Note; 6] = [Note::C, Note::D, Note::E, Note::G, Note::A, Note::B];
pub const D_MAJOR: [Note; 7] = [
   Note::D,
   Note::E,
   Note::F_,
   Note::G,
   Note::A,
   Note::B,
   Note::C_,
];

pub const A_MINOR: [Note; 7] = [
   Note::A,
   Note::B,
   Note::C,
   Note::D,
   Note::E,
   Note::F,
   Note::G,
];

pub const Db_MINOR: [Note; 7] = [
   Note::C_,
   Note::D_,
   Note::E,
   Note::F_,
   Note::G_,
   Note::A,
   Note::B,
];

pub const F_MINOR: [Note; 7] = [
   Note::F,
   Note::G,
   Note::G_,
   Note::A_,
   Note::C,
   Note::C_,
   Note::D_,
];
