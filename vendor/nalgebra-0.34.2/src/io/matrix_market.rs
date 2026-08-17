use std::fs;
use std::path::Path;

use crate::RealField;
use crate::sparse::CsMatrix;
use pest::Parser;

#[derive(Parser)]
#[grammar = "io/matrix_market.pest"]
struct MatrixMarketParser;

// TODO: return an Error instead of an Option.
/// Parses a Matrix Market file at the given path, and returns the corresponding sparse matrix.
pub fn cs_matrix_from_matrix_market<T: RealField, P: AsRef<Path>>(path: P) -> Option<CsMatrix<T>> {
    let file = fs::read_to_string(path).ok()?;
    cs_matrix_from_matrix_market_str(&file)
}

// TODO: return an Error instead of an Option.
/// Parses a Matrix Market file described by the given string, and returns the corresponding sparse matrix.
pub fn cs_matrix_from_matrix_market_str<T: RealField>(data: &str) -> Option<CsMatrix<T>> {
    let file = MatrixMarketParser::parse(Rule::Document, data)
        .unwrap()
        .next()?;
    let mut shape = (0, 0, 0);
    let mut rows: Vec<usize> = Vec::new();
    let mut cols: Vec<usize> = Vec::new();
    let mut data: Vec<T> = Vec::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::Header => {}
            Rule::Shape => {
                let mut inner = line.into_inner();
                shape.0 = inner.next()?.as_str().parse::<usize>().ok()?;
                shape.1 = inner.next()?.as_str().parse::<usize>().ok()?;
                shape.2 = inner.next()?.as_str().parse::<usize>().ok()?;
            }
            Rule::Entry => {
                let mut inner = line.into_inner();
                // NOTE: indices are 1-based.
                rows.push(inner.next()?.as_str().parse::<usize>().ok()? - 1);
                cols.push(inner.next()?.as_str().parse::<usize>().ok()? - 1);
                data.push(crate::convert(inner.next()?.as_str().parse::<f64>().ok()?));
            }
            _ => return None, // TODO: return an Err instead.
        }
    }

    Some(CsMatrix::from_triplet(
        shape.0, shape.1, &rows, &cols, &data,
    ))
}
