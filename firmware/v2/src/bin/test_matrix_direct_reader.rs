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
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_time::Timer;

// row is output
// and column is input
enum MatrixDirection {
    RowColumn,
    ColumnRow,
}

// active low will use pullup
// active high will use pulldown
enum PinReadCondition {
    ActiveLow,
    ActiveHigh,
}

struct MatrixDirectPinReaderConfig {
    matrix_direction: MatrixDirection,
    pin_read_condition: PinReadCondition,
}

impl Default for MatrixDirectPinReaderConfig {
    fn default() -> Self {
        Self {
            matrix_direction: MatrixDirection::ColumnRow,
            pin_read_condition: PinReadCondition::ActiveLow,
        }
    }
}

struct MatrixDirectPinReader<RowType, ColType, const ROWSIZE: usize, const COLSIZE: usize>
where
    RowType: InputPin,
    ColType: OutputPin,
{
    config: MatrixDirectPinReaderConfig,
    rows: [RowType; ROWSIZE],
    cols: [ColType; COLSIZE],
}

impl<RowType, ColType, const ROWSIZE: usize, const COLSIZE: usize>
    MatrixDirectPinReader<RowType, ColType, ROWSIZE, COLSIZE>
where
    RowType: InputPin,
    ColType: OutputPin,
{
    pub async fn read(&mut self) -> Result<ReaderResult<bool, 3, 6>, core::fmt::Error> {
        let mut data: ReaderResult<bool, 3, 6> = [[false; 3]; 6];

        for (col_index, col) in self.cols.iter_mut().enumerate() {
            col.set_high().unwrap();

            for (row_index, row) in self.rows.iter_mut().enumerate() {
                if row.is_high().unwrap() {
                    info!("row {}, column {}", row_index, col_index);

                    // data[row_index][col_index] = true;
                }
            }

            col.set_low().unwrap();
        }

        Timer::after_millis(50).await;

        Ok(data)
    }

    pub fn new(rows: [RowType; ROWSIZE], cols: [ColType; COLSIZE]) -> Self {
        Self {
            rows,
            cols,
            config: MatrixDirectPinReaderConfig::default(),
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripheral = embassy_nrf::init(Default::default());

    let direction: MatrixDirection = MatrixDirection::ColumnRow;
    let pin_read_condition: PinReadCondition = PinReadCondition::ActiveLow;

    let rows: [Input; 3] = [
        Input::new(peripheral.P0_10, Pull::Down),
        Input::new(peripheral.P0_11, Pull::Down),
        Input::new(peripheral.P0_12, Pull::Down),
    ];

    let cols: [Output; 6] = [
        Output::new(peripheral.P0_13, Level::Low, OutputDrive::Standard),
        Output::new(peripheral.P0_14, Level::Low, OutputDrive::Standard),
        Output::new(peripheral.P0_15, Level::Low, OutputDrive::Standard),
        Output::new(peripheral.P0_16, Level::Low, OutputDrive::Standard),
        Output::new(peripheral.P0_17, Level::Low, OutputDrive::Standard),
        Output::new(peripheral.P0_18, Level::Low, OutputDrive::Standard),
    ];

    let mut matrix_reader = MatrixDirectPinReader::new(rows, cols);

    loop {
        let _ = matrix_reader.read().await;
    }
}
