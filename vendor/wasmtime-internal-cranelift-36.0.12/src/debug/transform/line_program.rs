use super::address_transform::AddressTransform;
use super::{Reader, TransformError};
use anyhow::{Error, bail};
use gimli::{DebugLineOffset, LineEncoding, Unit, write};
use wasmtime_environ::DefinedFuncIndex;

#[derive(Debug)]
enum SavedLineProgramRow {
    Normal {
        address: u64,
        op_index: u64,
        file_index: u64,
        line: u64,
        column: u64,
        discriminator: u64,
        is_stmt: bool,
        basic_block: bool,
        prologue_end: bool,
        epilogue_begin: bool,
        isa: u64,
    },
    EndOfSequence,
}

#[derive(Debug)]
struct FuncRows {
    index: DefinedFuncIndex,
    sorted_rows: Vec<(u64, SavedLineProgramRow)>,
}

#[derive(Debug, Eq, PartialEq)]
enum ReadLineProgramState {
    SequenceEnded,
    ReadSequence(DefinedFuncIndex),
    IgnoreSequence,
}

pub(crate) fn clone_line_program<R>(
    dwarf: &gimli::Dwarf<R>,
    unit: &Unit<R, R::Offset>,
    comp_name: Option<R>,
    addr_tr: &AddressTransform,
    out_encoding: gimli::Encoding,
    out_strings: &mut write::StringTable,
) -> Result<(write::LineProgram, DebugLineOffset, Vec<write::FileId>, u64), Error>
where
    R: Reader,
{
    if let Some(program) = unit.line_program.clone() {
        let header = program.header();
        let offset = header.offset();
        let file_index_base = if header.version() < 5 { 1 } else { 0 };
        assert!(header.version() <= 5, "not supported 6");
        let line_encoding = LineEncoding {
            minimum_instruction_length: header.minimum_instruction_length(),
            maximum_operations_per_instruction: header.maximum_operations_per_instruction(),
            default_is_stmt: header.default_is_stmt(),
            line_base: header.line_base(),
            line_range: header.line_range(),
        };
        let out_comp_dir = match header.directory(0) {
            Some(comp_dir) => clone_line_string(
                dwarf.attr_string(unit, comp_dir)?,
                gimli::DW_FORM_string,
                out_strings,
            )?,
            None => write::LineString::String(Vec::new()),
        };
        let out_comp_name = match comp_name {
            Some(comp_name) => clone_line_string(comp_name, gimli::DW_FORM_strp, out_strings)?,
            None => write::LineString::String(Vec::new()),
        };

        let mut out_program = write::LineProgram::new(
            out_encoding,
            line_encoding,
            out_comp_dir,
            None,
            out_comp_name,
            None,
        );
        let mut dirs = Vec::new();
        dirs.push(out_program.default_directory());
        for dir_attr in header.include_directories() {
            let dir_id = out_program.add_directory(clone_line_string(
                dwarf.attr_string(unit, dir_attr.clone())?,
                gimli::DW_FORM_string,
                out_strings,
            )?);
            dirs.push(dir_id);
        }
        let mut files = Vec::new();
        // Since we are outputting DWARF-4, perform base change.
        let directory_index_correction = if header.version() >= 5 { 1 } else { 0 };
        for file_entry in header.file_names() {
            let dir_index = file_entry.directory_index() + directory_index_correction;
            let dir_id = dirs[dir_index as usize];
            let file_id = out_program.add_file(
                clone_line_string(
                    dwarf.attr_string(unit, file_entry.path_name())?,
                    gimli::DW_FORM_string,
                    out_strings,
                )?,
                dir_id,
                None,
            );
            files.push(file_id);
        }

        let mut rows = program.rows();
        let mut func_rows = Vec::new();
        let mut saved_rows: Vec<(u64, SavedLineProgramRow)> = Vec::new();
        let mut state = ReadLineProgramState::SequenceEnded;
        while let Some((_header, row)) = rows.next_row()? {
            if state == ReadLineProgramState::IgnoreSequence {
                if row.end_sequence() {
                    state = ReadLineProgramState::SequenceEnded;
                }
                continue;
            }
            let saved_row = if row.end_sequence() {
                let index = match state {
                    ReadLineProgramState::ReadSequence(index) => index,
                    _ => panic!(),
                };
                saved_rows.sort_by_key(|r| r.0);
                func_rows.push(FuncRows {
                    index,
                    sorted_rows: saved_rows,
                });

                saved_rows = Vec::new();
                state = ReadLineProgramState::SequenceEnded;
                SavedLineProgramRow::EndOfSequence
            } else {
                if state == ReadLineProgramState::SequenceEnded {
                    // Discard sequences for non-existent code.
                    if row.address() == 0 {
                        state = ReadLineProgramState::IgnoreSequence;
                        continue;
                    }
                    match addr_tr.find_func_index(row.address()) {
                        Some(index) => {
                            state = ReadLineProgramState::ReadSequence(index);
                        }
                        None => {
                            // Some non-existent address found.
                            state = ReadLineProgramState::IgnoreSequence;
                            continue;
                        }
                    }
                }
                SavedLineProgramRow::Normal {
                    address: row.address(),
                    op_index: row.op_index(),
                    file_index: row.file_index(),
                    line: row.line().map(|nonzero| nonzero.get()).unwrap_or(0),
                    column: match row.column() {
                        gimli::ColumnType::LeftEdge => 0,
                        gimli::ColumnType::Column(val) => val.get(),
                    },
                    discriminator: row.discriminator(),
                    is_stmt: row.is_stmt(),
                    basic_block: row.basic_block(),
                    prologue_end: row.prologue_end(),
                    epilogue_begin: row.epilogue_begin(),
                    isa: row.isa(),
                }
            };
            saved_rows.push((row.address(), saved_row));
        }

        for FuncRows {
            index,
            sorted_rows: saved_rows,
        } in func_rows
        {
            let map = match addr_tr.map().get(index) {
                Some(map) if map.len > 0 => map,
                _ => {
                    continue; // no code generated
                }
            };
            let symbol = map.symbol;
            let base_addr = map.offset;
            out_program.begin_sequence(Some(write::Address::Symbol { symbol, addend: 0 }));
            // TODO track and place function declaration line here
            let mut last_address = None;
            for addr_map in map.addresses.iter() {
                let saved_row = match saved_rows.binary_search_by_key(&addr_map.wasm, |i| i.0) {
                    Ok(i) => Some(&saved_rows[i].1),
                    Err(i) => {
                        if i > 0 {
                            Some(&saved_rows[i - 1].1)
                        } else {
                            None
                        }
                    }
                };
                if let Some(SavedLineProgramRow::Normal {
                    address,
                    op_index,
                    file_index,
                    line,
                    column,
                    discriminator,
                    is_stmt,
                    basic_block,
                    prologue_end,
                    epilogue_begin,
                    isa,
                }) = saved_row
                {
                    // Ignore duplicates
                    if Some(*address) != last_address {
                        let address_offset = if last_address.is_none() {
                            // Extend first entry to the function declaration
                            // TODO use the function declaration line instead
                            0
                        } else {
                            (addr_map.generated - base_addr) as u64
                        };
                        out_program.row().address_offset = address_offset;
                        out_program.row().op_index = *op_index;
                        out_program.row().file = files[(file_index - file_index_base) as usize];
                        out_program.row().line = *line;
                        out_program.row().column = *column;
                        out_program.row().discriminator = *discriminator;
                        out_program.row().is_statement = *is_stmt;
                        out_program.row().basic_block = *basic_block;
                        out_program.row().prologue_end = *prologue_end;
                        out_program.row().epilogue_begin = *epilogue_begin;
                        out_program.row().isa = *isa;
                        out_program.generate_row();
                        last_address = Some(*address);
                    }
                }
            }
            let end_addr = (map.offset + map.len) as u64;
            out_program.end_sequence(end_addr);
        }
        Ok((out_program, offset, files, file_index_base))
    } else {
        Err(TransformError("Valid line program not found").into())
    }
}

fn clone_line_string<R>(
    value: R,
    form: gimli::DwForm,
    out_strings: &mut write::StringTable,
) -> Result<write::LineString, Error>
where
    R: Reader,
{
    let content = value.to_string_lossy()?.into_owned();
    Ok(match form {
        gimli::DW_FORM_strp => {
            let id = out_strings.add(content);
            write::LineString::StringRef(id)
        }
        gimli::DW_FORM_string => write::LineString::String(content.into()),
        _ => bail!("DW_FORM_line_strp or other not supported"),
    })
}
