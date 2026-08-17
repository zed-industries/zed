use time::serde;

serde::format_description!(); // unexpected end of input
serde::format_description!("bad string", OffsetDateTime, "[year] [month]"); // module name is not ident
serde::format_description!(my_format: OffsetDateTime, "[year] [month]"); // not a comma
serde::format_description!(my_format,); // missing formattable and string
serde::format_description!(my_format, "[year] [month]"); // missing formattable
serde::format_description!(OffsetDateTime, "[year] [month]"); // missing ident
serde::format_description!(my_format, OffsetDateTime); // missing string format
serde::format_description!(my_format, OffsetDateTime,); // missing string format
serde::format_description!(my_format, OffsetDateTime "[year] [month]"); // missing comma
serde::format_description!(my_format, OffsetDateTime : "[year] [month]"); // not a comma
serde::format_description!(my_format, OffsetDateTime, "[bad]"); // bad component name
serde::format_description!(my_format, OffsetDateTime, not_string); // not in scope

fn main() {}
