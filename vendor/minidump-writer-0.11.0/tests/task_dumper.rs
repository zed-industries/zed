//! All of these tests are specific to the MacOS task dumper
#![cfg(target_os = "macos")]

use {
    minidump_writer::{mach::LoadCommand, task_dumper::TaskDumper},
    std::fmt::Write,
};

fn call_otool(args: &[&str]) -> String {
    let mut cmd = std::process::Command::new("otool");
    cmd.args(args);

    let exe_path = std::env::current_exe().expect("unable to retrieve test executable path");
    cmd.arg(exe_path);

    let output = cmd.output().expect("failed to spawn otool");

    assert!(output.status.success());

    String::from_utf8(output.stdout).expect("stdout was invalid utf-8")
}

/// Validates we can iterate the load commands for all of the images in the task
#[test]
fn iterates_load_commands() {
    let lc_str = call_otool(&["-l"]);

    let mut expected = String::new();
    let mut lc_index = 0;

    expected.push('\n');

    while let Some(nlc) = lc_str[lc_index..].find("Load command ") {
        lc_index += nlc;

        let block = match lc_str[lc_index + 13..].find("Load command ") {
            Some(ind) => &lc_str[lc_index + 13..lc_index + 13 + ind],
            None => &lc_str[lc_index..],
        };

        // otool prints the load command index for each command, but we only
        // handle the small subset of the available load commands we care about
        // so just ignore that
        let block = &block[block.find('\n').unwrap() + 1..];

        // otool also prints all the sections for LC_SEGMENT_* commands, but
        // we don't care about those, so ignore them
        let block = match block.find("Section") {
            Some(ind) => &block[..ind],
            None => block,
        };

        lc_index += 13;

        let cmd = block
            .find("cmd ")
            .expect("load commnd didn't specify cmd kind");
        let cmd_end = block[cmd..]
            .find('\n')
            .expect("load cmd didn't end with newline");
        if matches!(
            &block[cmd + 4..cmd + cmd_end],
            "LC_SEGMENT_64" | "LC_UUID" | "LC_ID_DYLIB" | "LC_LOAD_DYLINKER"
        ) {
            expected.push_str(block);
        }
    }

    let task_dumper = TaskDumper::new(
        // SAFETY: syscall
        unsafe { mach2::traps::mach_task_self() },
    );

    let mut actual = String::new();

    // Unfortunately, Apple decided to move dynamic libs into a shared cache,
    // removing them from the file system completely, and unless I'm missing it
    // there is no way to get the load commands for the dylibs since otool
    // only understands file paths? So we just get the load commands for the main
    // executable instead, this means that we miss the `LC_ID_DYLIB` commands
    // since they only apply to dylibs, but this test is more that we can
    // correctly iterate through the load commands themselves, so this _should_
    // be fine...
    let exe_img = task_dumper
        .read_executable_image()
        .expect("failed to read executable image");

    {
        let lcmds = task_dumper
            .read_load_commands(&exe_img)
            .expect("failed to read load commands");

        for lc in lcmds.iter() {
            match lc {
                LoadCommand::Segment(seg) => {
                    let segname = std::str::from_utf8(&seg.segment_name).unwrap();
                    let segname = &segname[..segname.find('\0').unwrap()];
                    write!(
                        &mut actual,
                        "
      cmd LC_SEGMENT_64
  cmdsize {}
  segname {}
   vmaddr 0x{:016x}
   vmsize 0x{:016x}
  fileoff {}
 filesize {}
  maxprot 0x{:08x}
 initprot 0x{:08x}
   nsects {}
    flags 0x{:x}",
                        seg.cmd_size,
                        segname,
                        seg.vm_addr,
                        seg.vm_size,
                        seg.file_off,
                        seg.file_size,
                        seg.max_prot,
                        seg.init_prot,
                        seg.num_sections,
                        seg.flags,
                    )
                    .unwrap();
                }
                LoadCommand::Dylib(_dylib) => {
                    unreachable!();
                }
                LoadCommand::Uuid(uuid) => {
                    let id = uuid::Uuid::from_bytes(uuid.uuid);
                    let mut uuid_buf = [0u8; uuid::fmt::Hyphenated::LENGTH];
                    let uuid_str = id.hyphenated().encode_upper(&mut uuid_buf);

                    write!(
                        &mut actual,
                        "
     cmd LC_UUID
 cmdsize {}
    uuid {uuid_str}
",
                        uuid.cmd_size,
                    )
                    .unwrap();
                }
                LoadCommand::DylinkerCommand(dy_cmd) => {
                    write!(
                        &mut actual,
                        "
          cmd LC_LOAD_DYLINKER
      cmdsize {}
         name {} (offset {})",
                        dy_cmd.cmd_size, dy_cmd.name, dy_cmd.name_offset,
                    )
                    .unwrap();
                }
            }
        }
    }

    similar_asserts::assert_eq!(expected, actual);
}
