use embedded_hal::digital::{InputPin, OutputPin};

use super::reader_trait::ReaderTrait;

// pub struct MatrixDirectPinReader<Row, Col, const ROWSIZE: usize, const COLSIZE: usize>
// where
//     Row: InputPin,
//     Col: OutputPin,
// {
//     rows: [Row; ROWSIZE],
//     cols: [Col; COLSIZE],
// }
//
// impl<Row, Col, const ROWSIZE: usize, const COLSIZE: usize> ReaderTrait
//     for MatrixDirectPinReader<Row, Col, ROWSIZE, COLSIZE>
// where
//     Row: InputPin,
//     Col: OutputPin,
// {
//     fn read(&self) -> Result<(), core::fmt::Error> {
//         todo!()
//     }
// }
