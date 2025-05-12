use core::fmt::Error;

pub trait ReaderTrait {
    fn read() -> Result<(), Error>;
}
