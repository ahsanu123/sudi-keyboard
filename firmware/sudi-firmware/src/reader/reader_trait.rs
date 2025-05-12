use core::fmt::Error;

pub trait ReaderTrait {
    fn read(&self) -> Result<(), Error>;
}
