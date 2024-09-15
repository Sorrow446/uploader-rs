use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ValueEnum)]
pub enum Host {
    Fileio,
    Gofile,
    Pixeldrain,
}

impl Host {
    pub fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}