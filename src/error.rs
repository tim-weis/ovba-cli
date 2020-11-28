use std::io;

#[derive(Debug)]
pub(crate) enum Error {
    Sxd,
    OoxmlManifest(sxd_xpath::Error),
    Zip(zip::result::ZipError),
    Io(io::Error),
    Ovba(ovba::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<ovba::Error> for Error {
    fn from(e: ovba::Error) -> Self {
        Error::Ovba(e)
    }
}

impl From<sxd_xpath::ParserError> for Error {
    fn from(_: sxd_xpath::ParserError) -> Self {
        Error::Sxd
    }
}

impl From<sxd_document::parser::Error> for Error {
    fn from(_: sxd_document::parser::Error) -> Self {
        Error::Sxd
    }
}

impl From<sxd_xpath::ExecutionError> for Error {
    fn from(e: sxd_xpath::ExecutionError) -> Self {
        Error::OoxmlManifest(sxd_xpath::Error::Executing(e))
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(e: zip::result::ZipError) -> Self {
        Error::Zip(e)
    }
}
