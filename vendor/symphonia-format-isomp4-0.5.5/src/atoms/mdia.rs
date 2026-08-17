// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, HdlrAtom, MdhdAtom, MinfAtom};

#[allow(dead_code)]
#[derive(Debug)]
pub struct MdiaAtom {
    header: AtomHeader,
    pub mdhd: MdhdAtom,
    pub hdlr: HdlrAtom,
    pub minf: MinfAtom,
}

impl Atom for MdiaAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);

        let mut mdhd = None;
        let mut hdlr = None;
        let mut minf = None;

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::MediaHeader => {
                    mdhd = Some(iter.read_atom::<MdhdAtom>()?);
                }
                AtomType::Handler => {
                    hdlr = Some(iter.read_atom::<HdlrAtom>()?);
                }
                AtomType::MediaInfo => {
                    minf = Some(iter.read_atom::<MinfAtom>()?);
                }
                _ => (),
            }
        }

        if mdhd.is_none() {
            return decode_error("isomp4: missing mdhd atom");
        }

        if hdlr.is_none() {
            return decode_error("isomp4: missing hdlr atom");
        }

        if minf.is_none() {
            return decode_error("isomp4: missing minf atom");
        }

        Ok(MdiaAtom { header, mdhd: mdhd.unwrap(), hdlr: hdlr.unwrap(), minf: minf.unwrap() })
    }
}
