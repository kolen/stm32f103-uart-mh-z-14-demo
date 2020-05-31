#![no_std]
#![no_main]
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{self, Serial},
};
use nb::block;
use cortex_m_semihosting::hprintln;


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let tx_pin = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx_pin = gpioa.pa3;

    let serial = Serial::usart2(
        dp.USART2,
        (tx_pin, rx_pin),
        &mut afio.mapr,
        serial::Config::default().baudrate(9_600.bps()),
        clocks,
        &mut rcc.apb1,
    );
    let (mut tx, mut rx) = serial.split();

    let to_send: [u8; 9] = [0xff, 0x01, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79];
    let mut recv: [u8; 9] = [0; 9];

    hprintln!("Sending");
    for &byte in to_send.iter() {
        block!(tx.write(byte)).unwrap();
    }

    for byte in recv.iter_mut() {
        *byte = block!(rx.read()).unwrap();
    }
    hprintln!("Received data: {:x?}", recv);

    loop {}
}
