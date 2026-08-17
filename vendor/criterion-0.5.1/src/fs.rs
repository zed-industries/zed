use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::error::{Error, Result};
use crate::report::BenchmarkId;

pub fn load<A, P: ?Sized>(path: &P) -> Result<A>
where
    A: DeserializeOwned,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let mut f = File::open(path).map_err(|inner| Error::AccessError {
        inner,
        path: path.to_owned(),
    })?;
    let mut string = String::new();
    let _ = f.read_to_string(&mut string);
    let result: A = serde_json::from_str(string.as_str()).map_err(|inner| Error::SerdeError {
        inner,
        path: path.to_owned(),
    })?;

    Ok(result)
}

pub fn is_dir<P>(path: &P) -> bool
where
    P: AsRef<Path>,
{
    let path: &Path = path.as_ref();
    path.is_dir()
}

pub fn mkdirp<P>(path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    fs::create_dir_all(path.as_ref()).map_err(|inner| Error::AccessError {
        inner,
        path: path.as_ref().to_owned(),
    })?;
    Ok(())
}

pub fn cp(from: &Path, to: &Path) -> Result<()> {
    fs::copy(from, to).map_err(|inner| Error::CopyError {
        inner,
        from: from.to_owned(),
        to: to.to_owned(),
    })?;
    Ok(())
}

pub fn save<D, P>(data: &D, path: &P) -> Result<()>
where
    D: Serialize,
    P: AsRef<Path>,
{
    let buf = serde_json::to_string(&data).map_err(|inner| Error::SerdeError {
        path: path.as_ref().to_owned(),
        inner,
    })?;
    save_string(&buf, path)
}

pub fn save_string<P>(data: &str, path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    use std::io::Write;

    File::create(path)
        .and_then(|mut f| f.write_all(data.as_bytes()))
        .map_err(|inner| Error::AccessError {
            inner,
            path: path.as_ref().to_owned(),
        })?;

    Ok(())
}

pub fn list_existing_benchmarks<P>(directory: &P) -> Result<Vec<BenchmarkId>>
where
    P: AsRef<Path>,
{
    fn is_benchmark(entry: &DirEntry) -> bool {
        // Look for benchmark.json files inside folders named "new" (because we want to ignore
        // the baselines)
        entry.file_name() == OsStr::new("benchmark.json")
            && entry.path().parent().unwrap().file_name().unwrap() == OsStr::new("new")
    }

    let mut ids = vec![];

    for entry in WalkDir::new(directory)
        .into_iter()
        // Ignore errors.
        .filter_map(::std::result::Result::ok)
        .filter(is_benchmark)
    {
        let id: BenchmarkId = load(entry.path())?;
        ids.push(id);
    }

    Ok(ids)
}
