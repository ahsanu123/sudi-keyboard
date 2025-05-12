use core::fmt::Error;

pub type ReaderResult<ResultType, const ROWSIZE: usize, const COLSIZE: usize> =
    [[ResultType; ROWSIZE]; COLSIZE];

pub trait ReaderTrait {
    fn read(&self) -> Result<ReaderResult<bool, 3, 3>, Error>;
}
