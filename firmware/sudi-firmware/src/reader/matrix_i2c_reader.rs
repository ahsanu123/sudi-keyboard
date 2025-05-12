use super::reader_trait::ReaderTrait;

pub struct MatrixI2CReader {}

impl ReaderTrait for MatrixI2CReader {
    fn read(&self) -> Result<(), core::fmt::Error> {
        todo!()
    }
}
