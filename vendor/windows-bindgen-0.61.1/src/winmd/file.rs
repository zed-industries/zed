use super::*;

pub struct File {
    pub(crate) reader: *const Reader,
    bytes: Vec<u8>,
    strings: usize,
    blobs: usize,
    tables: [Table; 17],
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{:?}", self.bytes.as_ptr())
    }
}

impl std::hash::Hash for File {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes.as_ptr().hash(state);
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.bytes.as_ptr(), other.bytes.as_ptr())
    }
}

impl Eq for File {}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bytes.as_ptr().cmp(&other.bytes.as_ptr())
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

unsafe impl Sync for File {}

impl File {
    pub fn new(bytes: Vec<u8>) -> Option<Self> {
        let mut result = File {
            bytes,
            reader: std::ptr::null(),
            strings: 0,
            blobs: 0,
            tables: Default::default(),
        };

        let dos = result.bytes.view_as::<IMAGE_DOS_HEADER>(0)?;

        if dos.e_magic != IMAGE_DOS_SIGNATURE
            || result.bytes.copy_as::<u32>(dos.e_lfanew as usize)? != IMAGE_NT_SIGNATURE
        {
            return None;
        }

        let file_offset = dos.e_lfanew as usize + std::mem::size_of::<u32>();
        let file = result.bytes.view_as::<IMAGE_FILE_HEADER>(file_offset)?;

        let optional_offset = file_offset + std::mem::size_of::<IMAGE_FILE_HEADER>();

        let (com_virtual_address, sections) = match result.bytes.copy_as::<u16>(optional_offset)? {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => {
                let optional = result
                    .bytes
                    .view_as::<IMAGE_OPTIONAL_HEADER32>(optional_offset)?;
                (
                    optional.DataDirectory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize]
                        .VirtualAddress,
                    result.bytes.view_as_slice_of::<IMAGE_SECTION_HEADER>(
                        optional_offset + std::mem::size_of::<IMAGE_OPTIONAL_HEADER32>(),
                        file.NumberOfSections as usize,
                    )?,
                )
            }
            IMAGE_NT_OPTIONAL_HDR64_MAGIC => {
                let optional = result
                    .bytes
                    .view_as::<IMAGE_OPTIONAL_HEADER64>(optional_offset)?;
                (
                    optional.DataDirectory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize]
                        .VirtualAddress,
                    result.bytes.view_as_slice_of::<IMAGE_SECTION_HEADER>(
                        optional_offset + std::mem::size_of::<IMAGE_OPTIONAL_HEADER64>(),
                        file.NumberOfSections as usize,
                    )?,
                )
            }
            _ => return None,
        };

        let clr = result.bytes.view_as::<IMAGE_COR20_HEADER>(offset_from_rva(
            section_from_rva(sections, com_virtual_address)?,
            com_virtual_address,
        ))?;

        if clr.cb != std::mem::size_of::<IMAGE_COR20_HEADER>() as u32 {
            return None;
        }

        let metadata_offset = offset_from_rva(
            section_from_rva(sections, clr.MetaData.VirtualAddress)?,
            clr.MetaData.VirtualAddress,
        );
        let metadata = result.bytes.view_as::<METADATA_HEADER>(metadata_offset)?;

        if metadata.signature != METADATA_SIGNATURE {
            return None;
        }

        // The METADATA_HEADER struct is not a fixed size so have to offset a little more carefully.
        let mut view = metadata_offset + metadata.length as usize + 20;
        let mut tables_data: (usize, usize) = (0, 0);

        for _ in 0..result
            .bytes
            .copy_as::<u16>(metadata_offset + metadata.length as usize + 18)?
        {
            let stream_offset = result.bytes.copy_as::<u32>(view)? as usize;
            let stream_len = result.bytes.copy_as::<u32>(view + 4)? as usize;
            let stream_name = result.bytes.view_as_str(view + 8)?;
            match stream_name {
                b"#Strings" => result.strings = metadata_offset + stream_offset,
                b"#Blob" => result.blobs = metadata_offset + stream_offset,
                b"#~" => tables_data = (metadata_offset + stream_offset, stream_len),
                b"#GUID" => {}
                b"#US" => {}
                rest => panic!("{rest:?}"),
            }
            let mut padding = 4 - stream_name.len() % 4;
            if padding == 0 {
                padding = 4;
            }
            view += 8 + stream_name.len() + padding;
        }

        let heap_sizes = result.bytes.copy_as::<u8>(tables_data.0 + 6)?;
        let string_index_size = if (heap_sizes & 1) == 1 { 4 } else { 2 };
        let guid_index_size = if ((heap_sizes >> 1) & 1) == 1 { 4 } else { 2 };
        let blob_index_size = if ((heap_sizes >> 2) & 1) == 1 { 4 } else { 2 };
        let valid_bits = result.bytes.copy_as::<u64>(tables_data.0 + 8)?;
        view = tables_data.0 + 24;

        // These tables are unused by the reader, but needed temporarily to calculate sizes and offsets for subsequent tables.
        let unused_empty = Table::default();
        let mut unused_assembly = Table::default();
        let mut unused_assembly_os = Table::default();
        let mut unused_assembly_processor = Table::default();
        let mut unused_assembly_ref_os = Table::default();
        let mut unused_assembly_ref = Table::default();
        let mut unused_assembly_ref_processor = Table::default();
        let mut unused_decl_security = Table::default();
        let mut unused_event = Table::default();
        let mut unused_event_map = Table::default();
        let mut unused_exported_type = Table::default();
        let mut unused_field_layout = Table::default();
        let mut unused_field_marshal = Table::default();
        let mut unused_field_rva = Table::default();
        let mut unused_file = Table::default();
        let mut unused_generic_param_constraint = Table::default();
        let mut unused_manifest_resource = Table::default();
        let mut unused_method_impl = Table::default();
        let mut unused_method_semantics = Table::default();
        let mut unused_method_spec = Table::default();
        let mut unused_property = Table::default();
        let mut unused_property_map = Table::default();
        let mut unused_standalone_sig = Table::default();
        let mut unused_module = Table::default();

        for i in 0..64 {
            if ((valid_bits >> i) & 1) == 0 {
                continue;
            }

            let len = result.bytes.copy_as::<u32>(view)? as usize;
            view += 4;

            match i {
                0x00 => unused_module.len = len,
                0x01 => result.tables[TypeRef::TABLE].len = len,
                0x02 => result.tables[TypeDef::TABLE].len = len,
                0x04 => result.tables[Field::TABLE].len = len,
                0x06 => result.tables[MethodDef::TABLE].len = len,
                0x08 => result.tables[MethodParam::TABLE].len = len,
                0x09 => result.tables[InterfaceImpl::TABLE].len = len,
                0x0a => result.tables[MemberRef::TABLE].len = len,
                0x0b => result.tables[Constant::TABLE].len = len,
                0x0c => result.tables[Attribute::TABLE].len = len,
                0x0d => unused_field_marshal.len = len,
                0x0e => unused_decl_security.len = len,
                0x0f => result.tables[ClassLayout::TABLE].len = len,
                0x10 => unused_field_layout.len = len,
                0x11 => unused_standalone_sig.len = len,
                0x12 => unused_event_map.len = len,
                0x14 => unused_event.len = len,
                0x15 => unused_property_map.len = len,
                0x17 => unused_property.len = len,
                0x18 => unused_method_semantics.len = len,
                0x19 => unused_method_impl.len = len,
                0x1a => result.tables[ModuleRef::TABLE].len = len,
                0x1b => result.tables[TypeSpec::TABLE].len = len,
                0x1c => result.tables[ImplMap::TABLE].len = len,
                0x1d => unused_field_rva.len = len,
                0x20 => unused_assembly.len = len,
                0x21 => unused_assembly_processor.len = len,
                0x22 => unused_assembly_os.len = len,
                0x23 => unused_assembly_ref.len = len,
                0x24 => unused_assembly_ref_processor.len = len,
                0x25 => unused_assembly_ref_os.len = len,
                0x26 => unused_file.len = len,
                0x27 => unused_exported_type.len = len,
                0x28 => unused_manifest_resource.len = len,
                0x29 => result.tables[NestedClass::TABLE].len = len,
                0x2a => result.tables[GenericParam::TABLE].len = len,
                0x2b => unused_method_spec.len = len,
                0x2c => unused_generic_param_constraint.len = len,
                _ => unreachable!(),
            };
        }

        let tables = &result.tables;
        let type_def_or_ref = coded_index_size(&[
            tables[TypeDef::TABLE].len,
            tables[TypeRef::TABLE].len,
            tables[TypeSpec::TABLE].len,
        ]);
        let has_constant = coded_index_size(&[
            tables[Field::TABLE].len,
            tables[MethodParam::TABLE].len,
            unused_property.len,
        ]);
        let has_field_marshal =
            coded_index_size(&[tables[Field::TABLE].len, tables[MethodParam::TABLE].len]);
        let has_decl_security = coded_index_size(&[
            tables[TypeDef::TABLE].len,
            tables[MethodDef::TABLE].len,
            unused_assembly.len,
        ]);
        let member_ref_parent = coded_index_size(&[
            tables[TypeDef::TABLE].len,
            tables[TypeRef::TABLE].len,
            tables[ModuleRef::TABLE].len,
            tables[MethodDef::TABLE].len,
            tables[TypeSpec::TABLE].len,
        ]);
        let has_semantics = coded_index_size(&[unused_event.len, unused_property.len]);
        let method_def_or_ref =
            coded_index_size(&[tables[MethodDef::TABLE].len, tables[MemberRef::TABLE].len]);
        let member_forwarded =
            coded_index_size(&[tables[Field::TABLE].len, tables[MethodDef::TABLE].len]);
        let implementation = coded_index_size(&[
            unused_file.len,
            unused_assembly_ref.len,
            unused_exported_type.len,
        ]);
        let custom_attribute_type = coded_index_size(&[
            tables[MethodDef::TABLE].len,
            tables[MemberRef::TABLE].len,
            unused_empty.len,
            unused_empty.len,
            unused_empty.len,
        ]);
        let resolution_scope = coded_index_size(&[
            unused_module.len,
            tables[ModuleRef::TABLE].len,
            unused_assembly_ref.len,
            tables[TypeRef::TABLE].len,
        ]);
        let type_or_method_def =
            coded_index_size(&[tables[TypeDef::TABLE].len, tables[MethodDef::TABLE].len]);

        let has_custom_attribute = coded_index_size(&[
            tables[MethodDef::TABLE].len,
            tables[Field::TABLE].len,
            tables[TypeRef::TABLE].len,
            tables[TypeDef::TABLE].len,
            tables[MethodParam::TABLE].len,
            tables[InterfaceImpl::TABLE].len,
            tables[MemberRef::TABLE].len,
            unused_module.len,
            unused_property.len,
            unused_event.len,
            unused_standalone_sig.len,
            tables[ModuleRef::TABLE].len,
            tables[TypeSpec::TABLE].len,
            unused_assembly.len,
            unused_assembly_ref.len,
            unused_file.len,
            unused_exported_type.len,
            unused_manifest_resource.len,
            tables[GenericParam::TABLE].len,
            unused_generic_param_constraint.len,
            unused_method_spec.len,
        ]);

        unused_assembly.set_columns(
            4,
            8,
            4,
            blob_index_size,
            string_index_size,
            string_index_size,
        );
        unused_assembly_os.set_columns(4, 4, 4, 0, 0, 0);
        unused_assembly_processor.set_columns(4, 0, 0, 0, 0, 0);
        unused_assembly_ref.set_columns(
            8,
            4,
            blob_index_size,
            string_index_size,
            string_index_size,
            blob_index_size,
        );
        unused_assembly_ref_os.set_columns(4, 4, 4, unused_assembly_ref.index_width(), 0, 0);
        unused_assembly_ref_processor.set_columns(4, unused_assembly_ref.index_width(), 0, 0, 0, 0);
        result.tables[ClassLayout::TABLE].set_columns(
            2,
            4,
            result.tables[TypeDef::TABLE].index_width(),
            0,
            0,
            0,
        );
        result.tables[Constant::TABLE].set_columns(2, has_constant, blob_index_size, 0, 0, 0);
        result.tables[Attribute::TABLE].set_columns(
            has_custom_attribute,
            custom_attribute_type,
            blob_index_size,
            0,
            0,
            0,
        );
        unused_decl_security.set_columns(2, has_decl_security, blob_index_size, 0, 0, 0);
        unused_event_map.set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            unused_event.index_width(),
            0,
            0,
            0,
            0,
        );
        unused_event.set_columns(2, string_index_size, type_def_or_ref, 0, 0, 0);
        unused_exported_type.set_columns(
            4,
            4,
            string_index_size,
            string_index_size,
            implementation,
            0,
        );
        result.tables[Field::TABLE].set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        unused_field_layout.set_columns(4, result.tables[Field::TABLE].index_width(), 0, 0, 0, 0);
        unused_field_marshal.set_columns(has_field_marshal, blob_index_size, 0, 0, 0, 0);
        unused_field_rva.set_columns(4, result.tables[Field::TABLE].index_width(), 0, 0, 0, 0);
        unused_file.set_columns(4, string_index_size, blob_index_size, 0, 0, 0);
        result.tables[GenericParam::TABLE].set_columns(
            2,
            2,
            type_or_method_def,
            string_index_size,
            0,
            0,
        );
        unused_generic_param_constraint.set_columns(
            result.tables[GenericParam::TABLE].index_width(),
            type_def_or_ref,
            0,
            0,
            0,
            0,
        );
        result.tables[ImplMap::TABLE].set_columns(
            2,
            member_forwarded,
            string_index_size,
            result.tables[ModuleRef::TABLE].index_width(),
            0,
            0,
        );
        result.tables[InterfaceImpl::TABLE].set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            type_def_or_ref,
            0,
            0,
            0,
            0,
        );
        unused_manifest_resource.set_columns(4, 4, string_index_size, implementation, 0, 0);
        result.tables[MemberRef::TABLE].set_columns(
            member_ref_parent,
            string_index_size,
            blob_index_size,
            0,
            0,
            0,
        );
        result.tables[MethodDef::TABLE].set_columns(
            4,
            2,
            2,
            string_index_size,
            blob_index_size,
            result.tables[MethodParam::TABLE].index_width(),
        );
        unused_method_impl.set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            method_def_or_ref,
            method_def_or_ref,
            0,
            0,
            0,
        );
        unused_method_semantics.set_columns(
            2,
            result.tables[MethodDef::TABLE].index_width(),
            has_semantics,
            0,
            0,
            0,
        );
        unused_method_spec.set_columns(method_def_or_ref, blob_index_size, 0, 0, 0, 0);
        unused_module.set_columns(
            2,
            string_index_size,
            guid_index_size,
            guid_index_size,
            guid_index_size,
            0,
        );
        result.tables[ModuleRef::TABLE].set_columns(string_index_size, 0, 0, 0, 0, 0);
        result.tables[NestedClass::TABLE].set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            result.tables[TypeDef::TABLE].index_width(),
            0,
            0,
            0,
            0,
        );
        result.tables[MethodParam::TABLE].set_columns(2, 2, string_index_size, 0, 0, 0);
        unused_property.set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        unused_property_map.set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            unused_property.index_width(),
            0,
            0,
            0,
            0,
        );
        unused_standalone_sig.set_columns(blob_index_size, 0, 0, 0, 0, 0);
        result.tables[TypeDef::TABLE].set_columns(
            4,
            string_index_size,
            string_index_size,
            type_def_or_ref,
            result.tables[Field::TABLE].index_width(),
            result.tables[MethodDef::TABLE].index_width(),
        );
        result.tables[TypeRef::TABLE].set_columns(
            resolution_scope,
            string_index_size,
            string_index_size,
            0,
            0,
            0,
        );
        result.tables[TypeSpec::TABLE].set_columns(blob_index_size, 0, 0, 0, 0, 0);

        unused_module.set_data(&mut view);
        result.tables[TypeRef::TABLE].set_data(&mut view);
        result.tables[TypeDef::TABLE].set_data(&mut view);
        result.tables[Field::TABLE].set_data(&mut view);
        result.tables[MethodDef::TABLE].set_data(&mut view);
        result.tables[MethodParam::TABLE].set_data(&mut view);
        result.tables[InterfaceImpl::TABLE].set_data(&mut view);
        result.tables[MemberRef::TABLE].set_data(&mut view);
        result.tables[Constant::TABLE].set_data(&mut view);
        result.tables[Attribute::TABLE].set_data(&mut view);
        unused_field_marshal.set_data(&mut view);
        unused_decl_security.set_data(&mut view);
        result.tables[ClassLayout::TABLE].set_data(&mut view);
        unused_field_layout.set_data(&mut view);
        unused_standalone_sig.set_data(&mut view);
        unused_event_map.set_data(&mut view);
        unused_event.set_data(&mut view);
        unused_property_map.set_data(&mut view);
        unused_property.set_data(&mut view);
        unused_method_semantics.set_data(&mut view);
        unused_method_impl.set_data(&mut view);
        result.tables[ModuleRef::TABLE].set_data(&mut view);
        result.tables[TypeSpec::TABLE].set_data(&mut view);
        result.tables[ImplMap::TABLE].set_data(&mut view);
        unused_field_rva.set_data(&mut view);
        unused_assembly.set_data(&mut view);
        unused_assembly_processor.set_data(&mut view);
        unused_assembly_os.set_data(&mut view);
        unused_assembly_ref.set_data(&mut view);
        unused_assembly_ref_processor.set_data(&mut view);
        unused_assembly_ref_os.set_data(&mut view);
        unused_file.set_data(&mut view);
        unused_exported_type.set_data(&mut view);
        unused_manifest_resource.set_data(&mut view);
        result.tables[NestedClass::TABLE].set_data(&mut view);
        result.tables[GenericParam::TABLE].set_data(&mut view);

        Some(result)
    }

    pub(crate) fn usize(&self, row: usize, table: usize, column: usize) -> usize {
        let table = &self.tables[table];
        let column = &table.columns[column];
        let offset = table.offset + row * table.width + column.offset;
        match column.width {
            1 => self.bytes.copy_as::<u8>(offset).map_or(0, |v| v as usize),
            2 => self.bytes.copy_as::<u16>(offset).map_or(0, |v| v as usize),
            4 => self.bytes.copy_as::<u32>(offset).map_or(0, |v| v as usize),
            _ => self.bytes.copy_as::<u64>(offset).map_or(0, |v| v as usize),
        }
    }

    pub(crate) fn str(&'static self, row: usize, table: usize, column: usize) -> &'static str {
        let offset = self.strings + self.usize(row, table, column);
        let bytes = &self.bytes[offset..];
        let nul_pos = bytes
            .iter()
            .position(|&c| c == 0)
            .expect("expected null-terminated C-string");
        std::str::from_utf8(&bytes[..nul_pos]).expect("expected valid utf-8 C-string")
    }

    pub(crate) fn blob(&'static self, row: usize, table: usize, column: usize) -> Blob {
        let offset = self.blobs + self.usize(row, table, column);
        let initial_byte = self.bytes[offset];

        let (blob_size, blob_size_bytes) = match initial_byte >> 5 {
            0..=3 => (initial_byte & 0x7f, 1),
            4..=5 => (initial_byte & 0x3f, 2),
            6 => (initial_byte & 0x1f, 4),
            rest => panic!("{rest:?}"),
        };

        let mut blob_size = blob_size as usize;

        for byte in &self.bytes[offset + 1..offset + blob_size_bytes] {
            blob_size = blob_size.checked_shl(8).unwrap_or(0) + (*byte as usize);
        }

        let offset = offset + blob_size_bytes;
        Blob::new(self, &self.bytes[offset..offset + blob_size])
    }

    pub(crate) fn list<R: AsRow>(
        &'static self,
        row: usize,
        table: usize,
        column: usize,
    ) -> RowIterator<R> {
        let first = self.usize(row, table, column) - 1;
        let next = row + 1;
        let last = if next < self.tables[table].len {
            self.usize(next, table, column) - 1
        } else {
            self.tables[R::TABLE].len
        };
        RowIterator::new(self, first..last)
    }

    pub(crate) fn equal_range<L: AsRow>(
        &'static self,
        column: usize,
        value: usize,
    ) -> RowIterator<L> {
        let mut first = 0;
        let mut last = self.tables[L::TABLE].len;
        let mut count = last;

        loop {
            if count == 0 {
                last = first;
                break;
            }

            let count2 = count / 2;
            let middle = first + count2;
            let middle_value = self.usize(middle, L::TABLE, column);

            match middle_value.cmp(&value) {
                Ordering::Less => {
                    first = middle + 1;
                    count -= count2 + 1;
                }
                Ordering::Greater => count = count2,
                Ordering::Equal => {
                    let first2 = self.lower_bound_of(L::TABLE, first, middle, column, value);
                    first += count;
                    last = self.upper_bound_of(L::TABLE, middle + 1, first, column, value);
                    first = first2;
                    break;
                }
            }
        }

        RowIterator::new(self, first..last)
    }

    pub(crate) fn parent<P: AsRow, C: AsRow>(&'static self, column: usize, child: C) -> P {
        P::from_row(Row::new(
            self,
            self.upper_bound_of(
                P::TABLE,
                0,
                self.tables[P::TABLE].len,
                column,
                child.index() + 1,
            ) - 1,
        ))
    }

    fn lower_bound_of(
        &self,
        table: usize,
        mut first: usize,
        last: usize,
        column: usize,
        value: usize,
    ) -> usize {
        let mut count = last - first;
        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if self.usize(middle, table, column) < value {
                first = middle + 1;
                count -= count2 + 1;
            } else {
                count = count2;
            }
        }
        first
    }

    fn upper_bound_of(
        &self,
        table: usize,
        mut first: usize,
        last: usize,
        column: usize,
        value: usize,
    ) -> usize {
        let mut count = last - first;
        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if value < self.usize(middle, table, column) {
                count = count2
            } else {
                first = middle + 1;
                count -= count2 + 1;
            }
        }
        first
    }

    pub(crate) fn table<R: AsRow>(&'static self) -> RowIterator<R> {
        RowIterator::new(self, 0..self.tables[R::TABLE].len)
    }

    pub(crate) fn reader(&self) -> &'static Reader {
        // Safety: At this point the File is already pointing to a valid Reader.
        unsafe { &*self.reader }
    }
}

fn section_from_rva(sections: &[IMAGE_SECTION_HEADER], rva: u32) -> Option<&IMAGE_SECTION_HEADER> {
    sections.iter().find(|&s| {
        rva >= s.VirtualAddress && rva < s.VirtualAddress + unsafe { s.Misc.VirtualSize }
    })
}

fn offset_from_rva(section: &IMAGE_SECTION_HEADER, rva: u32) -> usize {
    (rva - section.VirtualAddress + section.PointerToRawData) as usize
}

trait View {
    fn view_as<T>(&self, offset: usize) -> Option<&T>;
    fn view_as_slice_of<T>(&self, offset: usize, len: usize) -> Option<&[T]>;
    fn copy_as<T: Copy>(&self, offset: usize) -> Option<T>;
    fn view_as_str(&self, offset: usize) -> Option<&[u8]>;
    fn is_proper_length<T>(&self, offset: usize) -> Option<()>;
    fn is_proper_length_and_alignment<T>(&self, offset: usize, count: usize) -> Option<*const T>;
}

impl View for [u8] {
    fn view_as<T>(&self, offset: usize) -> Option<&T> {
        unsafe { Some(&*self.is_proper_length_and_alignment(offset, 1)?) }
    }

    fn view_as_slice_of<T>(&self, offset: usize, len: usize) -> Option<&[T]> {
        unsafe {
            Some(std::slice::from_raw_parts(
                self.is_proper_length_and_alignment(offset, len)?,
                len,
            ))
        }
    }

    fn copy_as<T>(&self, offset: usize) -> Option<T> {
        self.is_proper_length::<T>(offset)?;

        unsafe {
            let mut data = std::mem::MaybeUninit::zeroed().assume_init();
            core::ptr::copy_nonoverlapping(
                self[offset..].as_ptr(),
                &mut data as *mut T as *mut u8,
                std::mem::size_of::<T>(),
            );
            Some(data)
        }
    }

    fn view_as_str(&self, offset: usize) -> Option<&[u8]> {
        let buffer = &self[offset..];
        let index = buffer.iter().position(|c| *c == b'\0')?;
        Some(&self[offset..offset + index])
    }

    fn is_proper_length<T>(&self, offset: usize) -> Option<()> {
        if offset + std::mem::size_of::<T>() <= self.len() {
            Some(())
        } else {
            None
        }
    }

    fn is_proper_length_and_alignment<T>(&self, offset: usize, count: usize) -> Option<*const T> {
        self.is_proper_length::<T>(offset * count)?;
        let ptr = &self[offset] as *const u8 as *const T;

        if ptr.align_offset(std::mem::align_of::<T>()) == 0 {
            Some(ptr)
        } else {
            None
        }
    }
}

#[derive(Default)]
struct Table {
    offset: usize,
    len: usize,
    width: usize,
    columns: [Column; 6],
}

impl Table {
    fn index_width(&self) -> usize {
        if self.len < (1 << 16) {
            2
        } else {
            4
        }
    }

    fn set_columns(&mut self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) {
        self.width = a + b + c + d + e + f;
        self.columns[0] = Column::new(0, a);
        if b != 0 {
            self.columns[1] = Column::new(a, b);
        }
        if c != 0 {
            self.columns[2] = Column::new(a + b, c);
        }
        if d != 0 {
            self.columns[3] = Column::new(a + b + c, d);
        }
        if e != 0 {
            self.columns[4] = Column::new(a + b + c + d, e);
        }
        if f != 0 {
            self.columns[5] = Column::new(a + b + c + d + e, f);
        }
    }

    fn set_data(&mut self, offset: &mut usize) {
        if self.len != 0 {
            let next = *offset + self.len * self.width;
            self.offset = *offset;
            *offset = next;
        }
    }
}

#[derive(Default)]
struct Column {
    offset: usize,
    width: usize,
}

impl Column {
    fn new(offset: usize, width: usize) -> Self {
        Self { offset, width }
    }
}

#[repr(C)]
#[derive(Default)]
struct METADATA_HEADER {
    signature: u32,
    major_version: u16,
    minor_version: u16,
    reserved: u32,
    length: u32,
    version: [u8; 20],
    flags: u16,
    streams: u16,
}

const METADATA_SIGNATURE: u32 = 0x424A_5342;

// A coded index (see codes.rs) is a table index that may refer to different tables. The size of the column in memory
// must therefore be large enough to hold an index for a row in the largest possible table. This function determines
// this size for the given winmd file.
fn coded_index_size(tables: &[usize]) -> usize {
    fn small(row_count: usize, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }

    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        while {
            value >>= 1;
            value != 0
        } {
            bits += 1;
        }
        bits
    }

    let bits_needed = bits_needed(tables.len());

    if tables.iter().all(|table| small(*table, bits_needed)) {
        2
    } else {
        4
    }
}
