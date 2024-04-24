use smol::prelude::Future;
use std::io::Result;

pub enum Child {
    Standard(Result<std::process::Child>),
    Smol(Result<smol::process::Child>),
}

impl Child {
    pub fn standard(self) -> Result<std::process::Child> {
        match self {
            Self::Standard(inner) => inner,
            _ => panic!("Cannot get smol child from Child::Standard"),
        }
    }

    pub fn smol(self) -> Result<smol::process::Child> {
        match self {
            Self::Smol(inner) => inner,
            _ => panic!("Cannot get standard child from Child::Smol"),
        }
    }
}

pub enum Output {
    Standard(Result<std::process::Output>),
    Smol(Box<dyn Future<Output = Result<smol::process::Output>>>),
}

impl Output {
    pub fn standard(self) -> Result<std::process::Output> {
        match self {
            Self::Standard(inner) => inner,
            _ => panic!("Cannot get smol output from Output::Standard"),
        }
    }

    pub fn smol(self) -> Box<dyn Future<Output = Result<smol::process::Output>>> {
        match self {
            Self::Smol(inner) => inner,
            _ => panic!("Cannot get standard output from Output::Smol"),
        }
    }
}

pub enum ExitStatus {
    Standard(Result<std::process::ExitStatus>),
    Smol(Box<dyn Future<Output = Result<std::process::ExitStatus>>>),
}

impl ExitStatus {
    pub fn standard(self) -> Result<std::process::ExitStatus> {
        match self {
            Self::Standard(inner) => inner,
            _ => panic!("Cannot get smol exit status from ExitStatus::Standard"),
        }
    }

    pub fn smol(self) -> Box<dyn Future<Output = Result<std::process::ExitStatus>>> {
        match self {
            Self::Smol(inner) => inner,
            _ => panic!("Cannot get standard exit status from ExitStatus::Smol"),
        }
    }
}
