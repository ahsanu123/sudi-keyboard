#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin};
use sudi_firmware::{
    info,
    reader::reader_trait::{ReaderResult, ReaderTrait},
    warn,
};

#[cfg(not(feature = "defmt"))]
use panic_halt as _;

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::peripherals::UARTE0;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Input, Level, Output, OutputDrive, Pull},
    uarte,
};
use embassy_time::Timer;

bind_interrupts!(
    struct Irqs {
        UARTE0 => uarte::InterruptHandler<UARTE0>;
    }
);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripheral = embassy_nrf::init(Default::default());

    let mut config = uarte::Config::default();

    config.baudrate = uarte::Baudrate::BAUD57600;

    let uart = uarte::Uarte::new(
        peripheral.UARTE0,
        Irqs,
        peripheral.P0_09,
        peripheral.P0_08,
        config,
    );

    let (mut tx, rx) = uart.split();

    let green: u32 = 100;
    let red: u32 = 255;
    let blue: u32 = 100;
    let brightness: u32 = 150;

    loop {
        let message: u32 = green >> 24 | red >> 16 | blue;
        let _ = tx.write(&message.to_be_bytes()).await;

        Timer::after_micros(10).await;
    }
}
