use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    AVeryLongVariantName { a_long_piece_of_information: i32 },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[test]
fn accepts_trailing_commas() {
    fn example(a_long_piece_of_information: i32) -> Result<()> {
        ensure!(
            a_long_piece_of_information > a_long_piece_of_information - 1,
            AVeryLongVariantNameSnafu {
                a_long_piece_of_information,
            },
        );
        Ok(())
    }
    let _ = example(42);
}
