use core::fmt::Error;

pub trait WriterTrait {
    fn send() -> Result<(), Error>;
}
