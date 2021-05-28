use std::path::PathBuf;
use std::io;

pub struct FileInfo
{
    pub dir: PathBuf,
    pub is_valid: bool
}

enum FErrors
{
    NoSuchDir(io::Error),
    NoSuchFile(String),
    CreateFileErr(String)
}

pub trait ErrFuncs
{
    fn no_file(file: String) -> FErrors;
    fn file_creation_error(file: String) -> FErrors;
}

pub trait Funcs
{
    fn new() -> Result<FileInfo, io::Error>;
    fn append(&mut self, filename: String) -> Result<FileInfo, String>;
}
