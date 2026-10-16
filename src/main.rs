#![no_std]
#![no_main]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embedded_hal::digital::OutputPin;
// The boot label. The chip won't start your program without it.
use panic_halt as _;
use pio::Program;
use rp235x_hal::{self as hal, pio::PIOExt};

#[used]
#[unsafe(link_section = ".start_block")]

pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();
//use can_controller::pack_mit;

#[hal::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    );
    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks.unwrap());

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let program = pio::pio_asm!("set pindirs, 1", "set pins, 1",);
    //let mut led = pins.gpio25.into_push_pull_output();
    let _led = pins.gpio25.into_function::<hal::gpio::FunctionPio0>();
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let installed = pio.install(&program.program).unwrap();
    let return_val = hal::pio::PIOBuilder::from_installed_program(installed)
        .set_pins(25, 1)
        .build(sm0);

    //led.set_high();
    loop {}
}
