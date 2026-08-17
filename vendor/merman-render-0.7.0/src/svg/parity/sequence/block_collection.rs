use super::model::SequenceSvgModel;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub(super) struct AltSection<'a> {
    pub(super) raw_label: &'a str,
    pub(super) message_ids: Vec<&'a str>,
}

#[derive(Debug, Clone)]
pub(super) enum SequenceBlock<'a> {
    Alt {
        control_id: &'a str,
        sections: Vec<AltSection<'a>>,
    },
    Opt {
        control_id: &'a str,
        raw_label: &'a str,
        message_ids: Vec<&'a str>,
    },
    Break {
        control_id: &'a str,
        raw_label: &'a str,
        message_ids: Vec<&'a str>,
    },
    Par {
        control_id: &'a str,
        sections: Vec<AltSection<'a>>,
    },
    Loop {
        control_id: &'a str,
        raw_label: &'a str,
        message_ids: Vec<&'a str>,
    },
    Critical {
        control_id: &'a str,
        sections: Vec<AltSection<'a>>,
    },
}

#[derive(Debug, Clone)]
enum BlockStackEntry<'a> {
    Alt {
        raw_labels: Vec<&'a str>,
        sections: Vec<Vec<&'a str>>,
    },
    Loop {
        raw_label: &'a str,
        messages: Vec<&'a str>,
    },
    Opt {
        raw_label: &'a str,
        messages: Vec<&'a str>,
    },
    Break {
        raw_label: &'a str,
        messages: Vec<&'a str>,
    },
    Par {
        raw_labels: Vec<&'a str>,
        sections: Vec<Vec<&'a str>>,
    },
    Critical {
        raw_labels: Vec<&'a str>,
        sections: Vec<Vec<&'a str>>,
    },
}

pub(super) fn collect_sequence_blocks<'a>(
    model: &'a SequenceSvgModel,
) -> (FxHashMap<&'a str, Vec<usize>>, Vec<SequenceBlock<'a>>) {
    // Mermaid renders block frames (`alt`, `loop`, ...) as `<g>` elements before message lines.
    // Use layout-derived message y-coordinates for separator placement to avoid visual artifacts
    // like dashed lines ending in a gap right before the frame border.
    let mut blocks_by_end_id: FxHashMap<&str, Vec<usize>> =
        FxHashMap::with_capacity_and_hasher(model.messages.len(), Default::default());
    let mut blocks: Vec<SequenceBlock> = Vec::new();
    let mut stack: Vec<BlockStackEntry> = Vec::new();

    for msg in &model.messages {
        let raw_label = msg.message_text();
        match msg.message_type {
            // notes
            2 => {
                // Notes inside blocks must contribute to block frame bounds and section separators.
                // Track them in the active block scopes, similar to message edges.
                for entry in stack.iter_mut() {
                    push_item_to_block_stack_entry(entry, &msg.id);
                }
                continue;
            }
            // loop start/end
            10 => stack.push(BlockStackEntry::Loop {
                raw_label,
                messages: Vec::new(),
            }),
            11 => {
                if let Some(BlockStackEntry::Loop {
                    raw_label,
                    messages,
                }) = stack.pop()
                {
                    push_block(
                        &mut blocks_by_end_id,
                        &mut blocks,
                        msg.id.as_str(),
                        SequenceBlock::Loop {
                            control_id: msg.id.as_str(),
                            raw_label,
                            message_ids: messages,
                        },
                    );
                }
            }
            // opt start/end
            15 => stack.push(BlockStackEntry::Opt {
                raw_label,
                messages: Vec::new(),
            }),
            16 => {
                if let Some(BlockStackEntry::Opt {
                    raw_label,
                    messages,
                }) = stack.pop()
                {
                    push_block(
                        &mut blocks_by_end_id,
                        &mut blocks,
                        msg.id.as_str(),
                        SequenceBlock::Opt {
                            control_id: msg.id.as_str(),
                            raw_label,
                            message_ids: messages,
                        },
                    );
                }
            }
            // break start/end
            30 => stack.push(BlockStackEntry::Break {
                raw_label,
                messages: Vec::new(),
            }),
            31 => {
                if let Some(BlockStackEntry::Break {
                    raw_label,
                    messages,
                }) = stack.pop()
                {
                    push_block(
                        &mut blocks_by_end_id,
                        &mut blocks,
                        msg.id.as_str(),
                        SequenceBlock::Break {
                            control_id: msg.id.as_str(),
                            raw_label,
                            message_ids: messages,
                        },
                    );
                }
            }
            // alt start/else/end
            12 => stack.push(BlockStackEntry::Alt {
                raw_labels: vec![raw_label],
                sections: vec![Vec::new()],
            }),
            13 => {
                if let Some(BlockStackEntry::Alt {
                    raw_labels,
                    sections,
                    ..
                }) = stack.last_mut()
                {
                    raw_labels.push(raw_label);
                    sections.push(Vec::new());
                }
            }
            14 => {
                if let Some(BlockStackEntry::Alt {
                    raw_labels,
                    sections,
                }) = stack.pop()
                {
                    let idx = blocks.len();
                    blocks.push(SequenceBlock::Alt {
                        control_id: msg.id.as_str(),
                        sections: into_alt_sections(raw_labels, sections),
                    });
                    blocks_by_end_id
                        .entry(msg.id.as_str())
                        .or_default()
                        .push(idx);
                }
            }
            // par start/and/end
            19 | 32 => stack.push(BlockStackEntry::Par {
                raw_labels: vec![raw_label],
                sections: vec![Vec::new()],
            }),
            20 => {
                if let Some(BlockStackEntry::Par {
                    raw_labels,
                    sections,
                    ..
                }) = stack.last_mut()
                {
                    raw_labels.push(raw_label);
                    sections.push(Vec::new());
                }
            }
            21 => {
                if let Some(BlockStackEntry::Par {
                    raw_labels,
                    sections,
                }) = stack.pop()
                {
                    let idx = blocks.len();
                    blocks.push(SequenceBlock::Par {
                        control_id: msg.id.as_str(),
                        sections: into_alt_sections(raw_labels, sections),
                    });
                    blocks_by_end_id
                        .entry(msg.id.as_str())
                        .or_default()
                        .push(idx);
                }
            }
            // critical start/option/end
            27 => stack.push(BlockStackEntry::Critical {
                raw_labels: vec![raw_label],
                sections: vec![Vec::new()],
            }),
            28 => {
                if let Some(BlockStackEntry::Critical {
                    raw_labels,
                    sections,
                    ..
                }) = stack.last_mut()
                {
                    raw_labels.push(raw_label);
                    sections.push(Vec::new());
                }
            }
            29 => {
                if let Some(BlockStackEntry::Critical {
                    raw_labels,
                    sections,
                }) = stack.pop()
                {
                    let idx = blocks.len();
                    blocks.push(SequenceBlock::Critical {
                        control_id: msg.id.as_str(),
                        sections: into_alt_sections(raw_labels, sections),
                    });
                    blocks_by_end_id
                        .entry(msg.id.as_str())
                        .or_default()
                        .push(idx);
                }
            }
            _ => {
                // If this is a "real" message edge, attach it to all active block scopes.
                if msg.from.is_some() && msg.to.is_some() {
                    for entry in stack.iter_mut() {
                        push_item_to_block_stack_entry(entry, &msg.id);
                    }
                }
            }
        }
    }

    (blocks_by_end_id, blocks)
}

fn push_block<'a>(
    blocks_by_end_id: &mut FxHashMap<&'a str, Vec<usize>>,
    blocks: &mut Vec<SequenceBlock<'a>>,
    end_id: &'a str,
    block: SequenceBlock<'a>,
) {
    let idx = blocks.len();
    blocks.push(block);
    blocks_by_end_id.entry(end_id).or_default().push(idx);
}

fn push_item_to_block_stack_entry<'a>(entry: &mut BlockStackEntry<'a>, item_id: &'a str) {
    match entry {
        BlockStackEntry::Alt { sections, .. }
        | BlockStackEntry::Par { sections, .. }
        | BlockStackEntry::Critical { sections, .. } => {
            if let Some(cur) = sections.last_mut() {
                cur.push(item_id);
            }
        }
        BlockStackEntry::Loop { messages, .. }
        | BlockStackEntry::Opt { messages, .. }
        | BlockStackEntry::Break { messages, .. } => {
            messages.push(item_id);
        }
    }
}

fn into_alt_sections<'a>(
    raw_labels: Vec<&'a str>,
    sections: Vec<Vec<&'a str>>,
) -> Vec<AltSection<'a>> {
    let mut out_sections = Vec::new();
    let mut sections = sections.into_iter();
    for raw_label in raw_labels {
        let message_ids = sections.next().unwrap_or_default();
        out_sections.push(AltSection {
            raw_label,
            message_ids,
        });
    }
    out_sections
}
