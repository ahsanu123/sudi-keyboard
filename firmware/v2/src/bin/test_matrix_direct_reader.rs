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
enum DiodeDirection {
    RowToColumn,
    ColumnToRow,
}

// active low will use pullup
// active high will use pulldown
enum PinReadCondition {
    ActiveLow,
    ActiveHigh,
}

struct MatrixDirectPinReaderConfig {
    read_interval_in_micros: u64,
    diode_direction: DiodeDirection,
    pin_read_condition: PinReadCondition,
}

impl Default for MatrixDirectPinReaderConfig {
    fn default() -> Self {
        Self {
            diode_direction: DiodeDirection::ColumnToRow,
            pin_read_condition: PinReadCondition::ActiveLow,
            read_interval_in_micros: 100,
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
    async fn read_as_diode_column_to_row(
        &mut self,
    ) -> Result<ReaderResult<bool, 3, 6>, core::fmt::Error> {
        let mut data: ReaderResult<bool, 3, 6> = [[false; 3]; 6];

        for (col_index, col) in self.cols.iter_mut().enumerate() {
            col.set_high().unwrap();

            for (row_index, row) in self.rows.iter_mut().enumerate() {
                if row.is_high().unwrap() {
                    info!("row {}, column {}", row_index, col_index);

                    data[col_index][row_index] = true;
                }
            }

            col.set_low().unwrap();
        }
        Timer::after_micros(self.config.read_interval_in_micros).await;

        Ok(data)
    }

    async fn read_as_diode_row_to_column(
        &mut self,
    ) -> Result<ReaderResult<bool, 3, 6>, core::fmt::Error> {
        let mut data: ReaderResult<bool, 3, 6> = [[false; 3]; 6];

        for (col_index, col) in self.cols.iter_mut().enumerate() {
            col.set_low().unwrap();

            for (row_index, row) in self.rows.iter_mut().enumerate() {
                if row.is_high().unwrap() {
                    info!("row {}, column {}", row_index, col_index);

                    data[col_index][row_index] = true;
                }
            }

            col.set_high().unwrap();
        }
        Timer::after_micros(self.config.read_interval_in_micros).await;

        Ok(data)
    }

    pub async fn read(&mut self) -> Result<ReaderResult<bool, 3, 6>, core::fmt::Error> {
        match self.config.diode_direction {
            DiodeDirection::ColumnToRow => self.read_as_diode_column_to_row().await,
            DiodeDirection::RowToColumn => self.read_as_diode_row_to_column().await,
        }
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
