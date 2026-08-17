// Reference:
//   https://learn.microsoft.com/en-us/windows-hardware/drivers/install/authenticode
//   https://download.microsoft.com/download/9/c/5/9c5b2167-8017-4bae-9fde-d599bac8184a/Authenticode_PE.docx

// Authenticode works by omiting sections of the PE binary from the digest
// those sections are:
//   - checksum
//   - data directory entry for certtable
//   - certtable

use alloc::collections::VecDeque;
use core::ops::Range;
use log::debug;

use super::{section_table::SectionTable, PE};

static PADDING: [u8; 7] = [0; 7];

impl PE<'_> {
    /// Returns the various ranges of the binary that are relevant for signature.
    pub fn authenticode_ranges(&self) -> ExcludedSectionsIter<'_> {
        ExcludedSectionsIter {
            pe: self,
            state: IterState::default(),
            sections: VecDeque::default(),
        }
    }
}

/// [`ExcludedSections`] holds the various ranges of the binary that are expected to be
/// excluded from the authenticode computation.
#[derive(Debug, Clone, Default)]
pub(super) struct ExcludedSections {
    checksum: Range<usize>,
    datadir_entry_certtable: Range<usize>,
    certificate_table_size: usize,
    end_image_header: usize,
}

impl ExcludedSections {
    pub(super) fn new(
        checksum: Range<usize>,
        datadir_entry_certtable: Range<usize>,
        certificate_table_size: usize,
        end_image_header: usize,
    ) -> Self {
        Self {
            checksum,
            datadir_entry_certtable,
            certificate_table_size,
            end_image_header,
        }
    }
}

pub struct ExcludedSectionsIter<'s> {
    pe: &'s PE<'s>,
    state: IterState,
    sections: VecDeque<SectionTable>,
}

#[derive(Debug, PartialEq)]
enum IterState {
    Initial,
    ChecksumEnd(usize),
    CertificateTableEnd(usize),
    HeaderEnd {
        end_image_header: usize,
        sum_of_bytes_hashed: usize,
    },
    Sections {
        tail: usize,
        sum_of_bytes_hashed: usize,
    },
    Final {
        sum_of_bytes_hashed: usize,
    },
    Padding(usize),
    Done,
}

impl Default for IterState {
    fn default() -> Self {
        Self::Initial
    }
}

impl<'s> Iterator for ExcludedSectionsIter<'s> {
    type Item = &'s [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = &self.pe.bytes;

        if let Some(sections) = self.pe.authenticode_excluded_sections.as_ref() {
            loop {
                match self.state {
                    IterState::Initial => {
                        // 3. Hash the image header from its base to immediately before the start of the
                        //    checksum address, as specified in Optional Header Windows-Specific Fields.
                        let out = Some(&bytes[..sections.checksum.start]);
                        debug!("hashing {:#x} {:#x}", 0, sections.checksum.start);

                        // 4. Skip over the checksum, which is a 4-byte field.
                        debug_assert_eq!(sections.checksum.end - sections.checksum.start, 4);
                        self.state = IterState::ChecksumEnd(sections.checksum.end);

                        return out;
                    }
                    IterState::ChecksumEnd(checksum_end) => {
                        // 5. Hash everything from the end of the checksum field to immediately before the start
                        //    of the Certificate Table entry, as specified in Optional Header Data Directories.
                        let out =
                            Some(&bytes[checksum_end..sections.datadir_entry_certtable.start]);
                        debug!(
                            "hashing {checksum_end:#x} {:#x}",
                            sections.datadir_entry_certtable.start
                        );

                        // 6. Get the Attribute Certificate Table address and size from the Certificate Table entry.
                        //    For details, see section 5.7 of the PE/COFF specification.
                        // 7. Exclude the Certificate Table entry from the calculation
                        self.state =
                            IterState::CertificateTableEnd(sections.datadir_entry_certtable.end);

                        return out;
                    }
                    IterState::CertificateTableEnd(start) => {
                        // 7. Exclude the Certificate Table entry from the calculation and hash everything from
                        //    the end of the Certificate Table entry to the end of image header, including
                        //    Section Table (headers). The Certificate Table entry is 8 bytes long, as specified
                        //    in Optional Header Data Directories.
                        let end_image_header = sections.end_image_header;
                        let buf = Some(&bytes[start..end_image_header]);
                        debug!("hashing {start:#x} {:#x}", end_image_header - start);

                        // 8. Create a counter called SUM_OF_BYTES_HASHED, which is not part of the signature.
                        //    Set this counter to the SizeOfHeaders field, as specified in
                        //    Optional Header Windows-Specific Field.
                        let sum_of_bytes_hashed = end_image_header;

                        self.state = IterState::HeaderEnd {
                            end_image_header,
                            sum_of_bytes_hashed,
                        };

                        return buf;
                    }
                    IterState::HeaderEnd {
                        end_image_header,
                        sum_of_bytes_hashed,
                    } => {
                        // 9. Build a temporary table of pointers to all of the section headers in the
                        //    image. The NumberOfSections field of COFF File Header indicates how big
                        //    the table should be. Do not include any section headers in the table whose
                        //    SizeOfRawData field is zero.

                        // Implementation detail:
                        // We require allocation here because the section table has a variable size and
                        // needs to be sorted.
                        let mut sections: VecDeque<SectionTable> = self
                            .pe
                            .sections
                            .iter()
                            .filter(|section| section.size_of_raw_data != 0)
                            .cloned()
                            .collect();

                        // 10. Using the PointerToRawData field (offset 20) in the referenced SectionHeader
                        //     structure as a key, arrange the table's elements in ascending order. In
                        //     other words, sort the section headers in ascending order according to the
                        //     disk-file offset of the sections.
                        sections
                            .make_contiguous()
                            .sort_by_key(|section| section.pointer_to_raw_data);

                        self.sections = sections;

                        self.state = IterState::Sections {
                            tail: end_image_header,
                            sum_of_bytes_hashed,
                        };
                    }
                    IterState::Sections {
                        mut tail,
                        mut sum_of_bytes_hashed,
                    } => {
                        // 11. Walk through the sorted table, load the corresponding section into memory,
                        //     and hash the entire section. Use the SizeOfRawData field in the SectionHeader
                        //     structure to determine the amount of data to hash.
                        if let Some(section) = self.sections.pop_front() {
                            let start = section.pointer_to_raw_data as usize;
                            let end = start + section.size_of_raw_data as usize;
                            tail = end;

                            // 12. Add the section’s SizeOfRawData value to SUM_OF_BYTES_HASHED.
                            sum_of_bytes_hashed += section.size_of_raw_data as usize;

                            debug!("hashing {start:#x} {:#x}", end - start);
                            let buf = &bytes[start..end];

                            // 13. Repeat steps 11 and 12 for all of the sections in the sorted table.
                            self.state = IterState::Sections {
                                tail,
                                sum_of_bytes_hashed,
                            };

                            return Some(buf);
                        } else {
                            self.state = IterState::Final {
                                sum_of_bytes_hashed,
                            };
                        }
                    }
                    IterState::Final {
                        sum_of_bytes_hashed,
                    } => {
                        // 14. Create a value called FILE_SIZE, which is not part of the signature.
                        //     Set this value to the image’s file size, acquired from the underlying
                        //     file system. If FILE_SIZE is greater than SUM_OF_BYTES_HASHED, the
                        //     file contains extra data that must be added to the hash. This data
                        //     begins at the SUM_OF_BYTES_HASHED file offset, and its length is:
                        //       (File Size) - ((Size of AttributeCertificateTable) + SUM_OF_BYTES_HASHED)
                        //
                        // Note: The size of Attribute Certificate Table is specified in the second
                        //       ULONG value in the Certificate Table entry (32 bit: offset 132,
                        //       64 bit: offset 148) in Optional Header Data Directories.
                        let file_size = bytes.len();

                        // If FILE_SIZE is not a multiple of 8 bytes, the data added to the hash must
                        // be appended with zero padding of length (8 – (FILE_SIZE % 8)) bytes
                        let pad_size = (8 - file_size % 8) % 8;
                        self.state = IterState::Padding(pad_size);

                        if file_size > sum_of_bytes_hashed {
                            let extra_data_start = sum_of_bytes_hashed;
                            let len =
                                file_size - sections.certificate_table_size - sum_of_bytes_hashed;

                            debug!("hashing {extra_data_start:#x} {len:#x}",);
                            let buf = &bytes[extra_data_start..extra_data_start + len];

                            return Some(buf);
                        }
                    }
                    IterState::Padding(pad_size) => {
                        self.state = IterState::Done;

                        if pad_size != 0 {
                            debug!("hashing {pad_size:#x}");

                            // NOTE (safety): pad size will be at most 7, and PADDING has a size of 7
                            //                pad_size is computed ~10 lines above.
                            debug_assert!(pad_size <= 7);
                            debug_assert_eq!(PADDING.len(), 7);

                            return Some(&PADDING[..pad_size]);
                        }
                    }
                    IterState::Done => return None,
                }
            }
        } else {
            loop {
                match self.state {
                    IterState::Initial => {
                        self.state = IterState::Done;
                        return Some(bytes);
                    }
                    IterState::Done => return None,
                    _ => {
                        self.state = IterState::Done;
                    }
                }
            }
        }
    }
}
