#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(const_fn_floating_point_arithmetic)]

use core::{cell::RefCell, mem::MaybeUninit};

use arduino_hal::{
    hal::port::PB3,
    port::{mode::Output, Pin},
};
use avr_device::interrupt;
use const_soft_float::soft_f64::SoftF64;
use panic_halt as _;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

enum Note {
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

struct NoteDesc {
    frequency: u16,
    duration: u8,
}

const fn note(note: Note, octave: u8, duration: u8) -> NoteDesc {
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

struct BeepState {
    pin: Pin<Output, PB3>,
    divider: u16,
    cnt: u8,
    song_tick: u8,
    timer: u16,
}

impl BeepState {
    fn new(pin: Pin<Output, PB3>) -> Self {
        Self {
            pin,
            divider: (16000000 / 256) as u16,
            cnt: 0,
            song_tick: 0,
            timer: 0,
        }
    }

    fn modulo(&self, freq: u16) -> u16 {
        self.divider / freq
    }
}

static mut BEEP_STATE: MaybeUninit<BeepState> = MaybeUninit::uninit();

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

static NOTES: [NoteDesc; 144] = [
    note(Note::SILENCE, 7, 50),
    note(Note::G, 7, 12),
    note(Note::FS, 7, 12),
    note(Note::E, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::E, 7, 12),
    // ===
    note(Note::FS, 7, 50),
    note(Note::SILENCE, 7, 50),
    // ===
    note(Note::SILENCE, 7, 12),
    note(Note::SILENCE, 7, 25),
    note(Note::A, 6, 12),
    note(Note::G, 7, 12),
    note(Note::FS, 7, 12),
    note(Note::E, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::E, 7, 12),
    // ===
    note(Note::E, 7, 12),
    note(Note::FS, 7, 37),
    note(Note::D, 7, 25),
    note(Note::E, 7, 12),
    note(Note::A, 6, 12),
    // =====
    // =====
    note(Note::A, 6, 50),
    note(Note::SILENCE, 7, 37),
    note(Note::A, 6, 12),
    // ===
    note(Note::E, 7, 25),
    note(Note::FS, 7, 12),
    note(Note::G, 7, 12),
    note(Note::G, 7, 25),
    note(Note::E, 7, 12),
    note(Note::CS, 7, 12),
    // ===
    note(Note::SILENCE, 7, 1),
    note(Note::CS, 7, 12),
    note(Note::D, 7, 37),
    note(Note::E, 7, 25),
    note(Note::A, 6, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::A, 6, 12),
    // ===
    note(Note::A, 6, 12),
    note(Note::FS, 7, 37),
    note(Note::SILENCE, 7, 50),
    // =====
    // =====
    note(Note::SILENCE, 7, 50),
    note(Note::G, 7, 12),
    note(Note::FS, 7, 12),
    note(Note::E, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::E, 7, 12),
    // ===
    note(Note::FS, 7, 50),
    note(Note::SILENCE, 7, 50),
    // ===
    note(Note::SILENCE, 7, 25),
    note(Note::SILENCE, 7, 12),
    note(Note::A, 6, 12),
    note(Note::G, 7, 12),
    note(Note::FS, 7, 12),
    note(Note::E, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::E, 7, 12),
    // ===
    note(Note::E, 7, 25),
    note(Note::FS, 7, 12),
    note(Note::D, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::D, 7, 25),
    note(Note::E, 7, 12),
    note(Note::A, 6, 12),
    // =====
    // =====
    note(Note::A, 6, 50),
    note(Note::SILENCE, 7, 50),
    // ===
    note(Note::E, 7, 25),
    note(Note::FS, 7, 12),
    note(Note::G, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::G, 7, 25),
    note(Note::E, 7, 12),
    note(Note::CS, 7, 12),
    // ===
    note(Note::SILENCE, 7, 1),
    note(Note::CS, 7, 25),
    note(Note::D, 7, 12),
    note(Note::E, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::E, 7, 12),
    note(Note::A, 6, 12),
    note(Note::D, 7, 12),
    note(Note::E, 7, 12),
    // =====
    // =====
    note(Note::F, 7, 12),
    note(Note::E, 7, 12),
    note(Note::D, 7, 12),
    note(Note::C, 7, 12),
    note(Note::SILENCE, 7, 25),
    note(Note::A, 6, 12),
    note(Note::BB, 6, 12),
    // ===
    note(Note::C, 7, 25),
    note(Note::F, 7, 25),
    note(Note::E, 7, 12),
    note(Note::D, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::D, 7, 12),
    note(Note::C, 7, 12),
    // ===
    note(Note::D, 7, 12),
    note(Note::C, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::C, 7, 25),
    note(Note::SILENCE, 7, 1),
    note(Note::C, 7, 25),
    note(Note::A, 6, 12),
    note(Note::BB, 6, 12),
    // =====
    // =====
    note(Note::C, 7, 25),
    note(Note::F, 7, 25),
    note(Note::G, 7, 12),
    note(Note::F, 7, 12),
    note(Note::E, 7, 12),
    note(Note::D, 7, 12),
    // ===
    note(Note::SILENCE, 7, 1),
    note(Note::D, 7, 12),
    note(Note::E, 7, 12),
    note(Note::F, 7, 25),
    note(Note::SILENCE, 7, 1),
    note(Note::F, 7, 25),
    note(Note::G, 7, 12),
    note(Note::A, 7, 12),
    // ===
    note(Note::B, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::B, 7, 12),
    note(Note::A, 7, 25),
    note(Note::G, 7, 25),
    note(Note::F, 7, 12),
    note(Note::G, 7, 12),
    // =====
    // =====
    note(Note::A, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::A, 7, 12),
    note(Note::G, 7, 25),
    note(Note::F, 7, 25),
    note(Note::SILENCE, 7, 1),
    note(Note::D, 7, 12),
    note(Note::C, 7, 12),
    // ===
    note(Note::D, 7, 12),
    note(Note::F, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::F, 7, 12),
    note(Note::E, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::E, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::E, 7, 12),
    note(Note::GB, 7, 12),
    note(Note::SILENCE, 7, 1),
    note(Note::GB, 7, 12),
];

#[avr_device::interrupt(atmega328p)]
fn TIMER2_COMPA() {
    let beep = unsafe { &mut *BEEP_STATE.as_mut_ptr() };

    let NoteDesc {
        frequency,
        duration,
    } = NOTES[beep.song_tick as usize];

    beep.timer = (beep.timer + 1) % (160000000 / 256 / 1700 * duration as u32) as u16;
    if beep.timer == 0 {
        beep.song_tick = (beep.song_tick + 1) % (NOTES.len() as u8);
    }

    beep.cnt = ((beep.cnt + 1) as u16 % beep.modulo(frequency)) as u8;
    if beep.cnt == 0 {
        beep.pin.toggle();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);

    put_console(serial);

    unsafe {
        BEEP_STATE = MaybeUninit::new(BeepState::new(pins.d11.into_output()));
    }

    dp.TC2.tccr2a.write(|w| w.wgm2().ctc());
    dp.TC2.ocr2a.write(|w| w.bits(0));
    dp.TC2.tccr2b.write(|w| w.cs2().prescale_256());
    dp.TC2.timsk2.write(|w| w.ocie2a().set_bit());

    unsafe { avr_device::interrupt::enable() };

    loop {}
}
