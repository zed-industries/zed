use std::{cmp::Ordering, ops::Range};

use collections::HashMap;
use rope::Rope;

#[derive(Eq,PartialEq,Ord,PartialOrd,Hash,Debug,Clone,Copy)]
struct ReplicaId(u16);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Seq(u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct LocalVersion(u32);
struct RawVersion  {
    version: Seq,
    replica_id: ReplicaId,
}

#[derive(Default)]
struct CausalGraph {
    heads: Vec<LocalVersion>,
    entries: Vec<CausalGraphEntry>,
    observed_versions: HashMap<ReplicaId, Vec<ClientEntry>>
}

struct CausalGraphEntry {
    version: LocalVersion,
    end: LocalVersion, // > version.

    replica_id: ReplicaId,
    seq: Seq, // Seq for version.

    parents: Vec<LocalVersion> // Parents for version
}

struct ClientEntry {
    seq: Seq,
    seq_end: Seq,
    /// Version of the first item in this run
    version: LocalVersion,
}

impl CausalGraph {
    fn lv_to_raw_list(&self, heads: Vec<LocalVersion>) -> Vec<RawVersion> {
        heads.into_iter().map(|head| {
            let (e, offset) =  self.find_entry_containing(head);
            RawVersion {replica_id: e.replica_id, version: e.seq + offset}
        }).collect()
    }

    fn find_entry_containing(head: LocalVersion) -> (CausalGraphEntry, u32) {
        // Find the entry containing the given local version
        // Returns the entry and the offset of the version within the entry
        unimplemented!()
    }

    fn find_client_entry_raw(&self, replica_id: ReplicaId, seq: Seq) -> Option<ClientEntry> {
        let Some(agent_versions) = self.observed_versions.get(&replica_id) else {
            return None
        };

        let result = agent_versions.binary_search_by(|x| {
            if x.seq < seq {
                Ordering::Less
            } else if x.seq_end >= seq {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        match result {
            Ok(i) => Some(agent_versions[i]),
            _ => None
        }
    }

    fn find_client_entry(&self, replica_id: ReplicaId, seq: Seq) -> Option<(ClientEntry, Seq)> {
        self.find_client_entry_raw(replica_id, seq)
            .map(|client_entry| (client_entry, seq.0 - client_entry.seq.0))
    }

    fn find_client_entry_trimmed(&self, replica_id: ReplicaId, seq: Seq) -> Option<ClientEntry> {
        self.find_client_entry(replica_id, seq)
            .map(|(entry, offset)| {
                if offset.0 == 0 {
                    entry
                } else {
                    ClientEntry {
                        seq: offset,
                        seq_end: entry.seq_end,
                        // TODO: These are the same thing????
                        version: LocalVersion(entry.version.0 + offset.0)
                    }
                }

            })
    }

    fn add(&self, replica_id: ReplicaId) {
        let next_version = LocalVersion(self.entries.last().map(|_| -> u32 {
            todo!()
        }).unwrap_or(0));



        fn find_client_entry() -> Option<CausalGraphEntry> {
            todo!();
        }

        loop {

            let existing_entry = self.find_client_entry_trimmed(replica_id, Seq(next_version.0));
            let Some(existing_entry) = existing_entry else {
                break
            };

            // if existing_entry.end

        }
    }
}

#[derive(Clone)]
enum Op {
    Insert(usize, char),
    Delete(usize)
}

#[derive(Default)]
struct OpLog {
    ops: Vec<Op>,
    graph: CausalGraph,
}

impl OpLog {
    fn get_latest_version(&self) -> Vec<RawVersion> {
        self.graph.lv_to_raw_list(self.graph.heads)
    }
}

#[derive(Default)]
struct Buffer {
    text: Rope,
    op_log: OpLog,
}

// struct EditContext {
//     items: [],
//     del_targets: Vec<()>,
//     items_by_lv: Vec<()>,
//     cur_version: [],
// }

impl Buffer {
    fn new(replica_id: u16, text: &str) -> Self {
        let mut buffer = Buffer {
            text: Rope::from(text),
            op_log: OpLog::default(),
        };
        buffer.edit(0..0, text);
        buffer
    }

    pub fn edit(&mut self, range: Range<usize>, text: &str) -> Vec<Op>
    {
        let mut operations = Vec::new();
        for _ in range.clone() {
            operations.push(Op::Delete(range.start));

        }
        for (ix, char) in text.chars().enumerate() {
            operations.push(Op::Insert(range.start + ix, char))
        }
        self.op_log.ops.extend(operations.clone());
        return operations

    }

    pub fn apply_op(&mut self, op: Vec<Op>) -> anyhow::Result<()> {
        // Here's the magic
        self.op_log.ops.extend(op);
        Ok(())
    }

    pub fn text(&self) -> String {
        let mut final_text = String::new();
        for op in self.op_log.ops.iter() {
            match op {
                Op::Insert(ix, char) => {final_text.insert(*ix, *char);},
                Op::Delete(ix) => {final_text.remove(*ix);},
            }
        }
        final_text
    }
}


#[test]
fn test_edit() {
    let mut buffer = Buffer::new(0, "abc");
    assert_eq!(buffer.text(), "abc");
    buffer.edit(3..3, "def");
    assert_eq!(buffer.text(), "abcdef");
    buffer.edit(0..0, "ghi");
    assert_eq!(buffer.text(), "ghiabcdef");
    buffer.edit(5..5, "jkl");
    assert_eq!(buffer.text(), "ghiabjklcdef");
    buffer.edit(6..7, "");
    assert_eq!(buffer.text(), "ghiabjlcdef");
    buffer.edit(4..9, "mno");
    assert_eq!(buffer.text(), "ghiamnoef");
}
#[test]
fn test_concurrent_edits() {
    let text = "abcdef";

    let mut buffer1 = Buffer::new(1, text);
    let mut buffer2 = Buffer::new(2, text);
    let mut buffer3 = Buffer::new(3, text);

    let buf1_op = buffer1.edit(1..2, "12");
    assert_eq!(buffer1.text(), "a12cdef");
    let buf2_op = buffer2.edit(3..4, "34");
    assert_eq!(buffer2.text(), "abc34ef");
    let buf3_op = buffer3.edit(5..6, "56");
    assert_eq!(buffer3.text(), "abcde56");

    buffer1.apply_op(buf2_op.clone()).unwrap();
    buffer1.apply_op(buf3_op.clone()).unwrap();
    buffer2.apply_op(buf1_op.clone()).unwrap();
    buffer2.apply_op(buf3_op).unwrap();
    buffer3.apply_op(buf1_op).unwrap();
    buffer3.apply_op(buf2_op).unwrap();

    assert_eq!(buffer1.text(), "a12c34e56");
    assert_eq!(buffer2.text(), "a12c34e56");
    assert_eq!(buffer3.text(), "a12c34e56");
}
