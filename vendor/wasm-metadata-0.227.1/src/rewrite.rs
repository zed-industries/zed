use crate::{
    Authors, ComponentNames, Description, Homepage, Licenses, ModuleNames, Producers, Revision,
    Source,
};
use anyhow::Result;
use std::mem;
use wasm_encoder::ComponentSection as _;
use wasm_encoder::{ComponentSectionId, Encode, Section};
use wasmparser::{KnownCustom, Parser, Payload::*};

pub(crate) fn rewrite_wasm(
    add_name: &Option<String>,
    add_producers: &Producers,
    add_author: &Option<Authors>,
    add_description: &Option<Description>,
    add_licenses: &Option<Licenses>,
    add_source: &Option<Source>,
    add_homepage: &Option<Homepage>,
    add_revision: &Option<Revision>,
    add_version: &Option<crate::Version>,
    input: &[u8],
) -> Result<Vec<u8>> {
    let mut producers_found = false;
    let mut names_found = false;
    let mut stack = Vec::new();
    let mut output = Vec::new();
    for payload in Parser::new(0).parse_all(&input) {
        let payload = payload?;

        // Track nesting depth, so that we don't mess with inner producer sections:
        match payload {
            Version { encoding, .. } => {
                output.extend_from_slice(match encoding {
                    wasmparser::Encoding::Component => &wasm_encoder::Component::HEADER,
                    wasmparser::Encoding::Module => &wasm_encoder::Module::HEADER,
                });
            }
            ModuleSection { .. } | ComponentSection { .. } => {
                stack.push(mem::take(&mut output));
                continue;
            }
            End { .. } => {
                let mut parent = match stack.pop() {
                    Some(c) => c,
                    None => break,
                };
                if output.starts_with(&wasm_encoder::Component::HEADER) {
                    parent.push(ComponentSectionId::Component as u8);
                    output.encode(&mut parent);
                } else {
                    parent.push(ComponentSectionId::CoreModule as u8);
                    output.encode(&mut parent);
                }
                output = parent;
            }
            _ => {}
        }

        // Only rewrite the outermost custom sections
        if let CustomSection(c) = &payload {
            if stack.len() == 0 {
                match c.as_known() {
                    KnownCustom::Producers(_) => {
                        producers_found = true;
                        let mut producers = Producers::from_bytes(c.data(), c.data_offset())?;
                        // Add to the section according to the command line flags:
                        producers.merge(&add_producers);
                        // Encode into output:
                        producers.section().append_to(&mut output);
                        continue;
                    }
                    KnownCustom::Name(_) => {
                        names_found = true;
                        let mut names = ModuleNames::from_bytes(c.data(), c.data_offset())?;
                        names.merge(&ModuleNames::from_name(add_name));

                        names.section()?.as_custom().append_to(&mut output);
                        continue;
                    }
                    KnownCustom::ComponentName(_) => {
                        names_found = true;
                        let mut names = ComponentNames::from_bytes(c.data(), c.data_offset())?;
                        names.merge(&ComponentNames::from_name(add_name));
                        names.section()?.as_custom().append_to(&mut output);
                        continue;
                    }
                    KnownCustom::Unknown if c.name() == "author" => {
                        if add_author.is_none() {
                            let author = Authors::parse_custom_section(c)?;
                            author.append_to(&mut output);
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "description" => {
                        if add_description.is_none() {
                            let description = Description::parse_custom_section(c)?;
                            description.append_to(&mut output);
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "licenses" => {
                        if add_licenses.is_none() {
                            let licenses = Licenses::parse_custom_section(c)?;
                            licenses.append_to(&mut output);
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "source" => {
                        if add_source.is_none() {
                            let source = Source::parse_custom_section(c)?;
                            source.append_to(&mut output);
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "homepage" => {
                        if add_source.is_none() {
                            let homepage = Homepage::parse_custom_section(c)?;
                            homepage.append_to(&mut output);
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "revision" => {
                        if add_source.is_none() {
                            let revision = Revision::parse_custom_section(c)?;
                            revision.append_to(&mut output);
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "version" => {
                        if add_version.is_none() {
                            let version = crate::Version::parse_custom_section(c)?;
                            version.append_to(&mut output);
                            continue;
                        }
                    }
                    _ => {}
                }
            }
        }
        // All other sections get passed through unmodified:
        if let Some((id, range)) = payload.as_section() {
            wasm_encoder::RawSection {
                id,
                data: &input[range],
            }
            .append_to(&mut output);
        }
    }
    if !names_found && add_name.is_some() {
        if output.starts_with(&wasm_encoder::Component::HEADER) {
            let names = ComponentNames::from_name(add_name);
            names.section()?.append_to_component(&mut output);
        } else {
            let names = ModuleNames::from_name(add_name);
            names.section()?.append_to(&mut output)
        }
    }
    if !producers_found && !add_producers.is_empty() {
        let mut producers = Producers::empty();
        // Add to the section according to the command line flags:
        producers.merge(add_producers);
        // Encode into output:
        producers.section().append_to(&mut output);
    }
    if let Some(author) = add_author {
        author.append_to(&mut output);
    }
    if let Some(description) = add_description {
        description.append_to(&mut output);
    }
    if let Some(licenses) = add_licenses {
        licenses.append_to(&mut output);
    }
    if let Some(source) = add_source {
        source.append_to(&mut output);
    }
    if let Some(homepage) = add_homepage {
        homepage.append_to(&mut output);
    }
    if let Some(revision) = add_revision {
        revision.append_to(&mut output);
    }
    if let Some(version) = add_version {
        version.append_to(&mut output);
    }
    Ok(output)
}
