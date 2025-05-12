#![no_std]
#![no_main]

use sudi_firmware::{info, warn};

#[cfg(not(feature = "defmt"))]
use panic_halt as _;

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_time::Timer;

enum MatrixDirection {
    RowColumn,
    ColumnRow,
}

enum PinReadCondition {
    ActiveLow,
    ActiveHigh,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripheral = embassy_nrf::init(Default::default());

    let direction: MatrixDirection = MatrixDirection::ColumnRow;
    let pin_read_condition: PinReadCondition = PinReadCondition::ActiveLow;

    let mut columns: [Input; 6] = [
        Input::new(peripheral.P0_13, Pull::Up),
        Input::new(peripheral.P0_14, Pull::Up),
        Input::new(peripheral.P0_15, Pull::Up),
        Input::new(peripheral.P0_16, Pull::Up),
        Input::new(peripheral.P0_17, Pull::Up),
        Input::new(peripheral.P0_18, Pull::Up),
    ];

    let mut rows: [Output; 3] = [
        Output::new(peripheral.P0_10, Level::Low, OutputDrive::Standard),
        Output::new(peripheral.P0_11, Level::Low, OutputDrive::Standard),
        Output::new(peripheral.P0_12, Level::Low, OutputDrive::Standard),
    ];

    loop {
        for (row_index, row) in rows.iter_mut().enumerate() {
            row.set_low();

            for (col_index, col) in columns.iter_mut().enumerate() {
                if col.is_low() {
                    info!("row {}, column {}", row_index, col_index);
                }
            }

            row.set_high();

            Timer::after_millis(50).await;
        }
    }
}
