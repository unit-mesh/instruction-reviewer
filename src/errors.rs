#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format,
}


#[derive(Debug, Clone)]
pub enum SaveError {
    File,
    Write,
    Format,
}
