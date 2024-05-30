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
use panic_halt as _;

mod note;
mod songs;

use note::NoteDesc;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

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

#[avr_device::interrupt(atmega328p)]
fn TIMER2_COMPA() {
    let beep = unsafe { &mut *BEEP_STATE.as_mut_ptr() };

    let notes = &songs::saria_song::NOTES;

    let NoteDesc {
        frequency,
        duration,
    } = notes[beep.song_tick as usize];

    beep.timer = (beep.timer + 1) % (160000000 / 256 / 1700 * duration as u32) as u16;
    if beep.timer == 0 {
        beep.song_tick = (beep.song_tick + 1) % (notes.len() as u8);
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
