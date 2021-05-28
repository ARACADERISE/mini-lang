use std::path::PathBuf;
use std::io;

#[derive(Debug, Clone)]
pub struct FileInfo
{
    pub dir: PathBuf,
    pub is_valid: bool,
    pub content: String
}

#[derive(Debug)]
pub enum FErrors
{
    NoSuchDir(io::Error),
    NoSuchFile(PathBuf),
    CreateFileErr(PathBuf)
}

pub trait ErrFuncs
{
    fn no_file(file: PathBuf) -> FErrors;
    fn file_creation_error(file: PathBuf) -> FErrors;
}

pub trait Funcs
{
    fn new() -> Result<FileInfo, io::Error>;
    fn append(&mut self, filename: String) -> Result<FileInfo, FErrors>;
    fn read_file(&mut self) -> io::Result<FileInfo>;
}
