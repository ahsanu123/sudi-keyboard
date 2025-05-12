use core::fmt::Error;

pub trait MiddlewareTrait {
    fn invoke() -> Result<(), Error>;
}
