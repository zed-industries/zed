use std::fs::read_to_string;
use std::mem::swap;
use std::path::PathBuf;

use expect_test::{expect, expect_file};
// use git::bstr::BStr;
// use git_repository as git;

use crate::intern::InternedInput;
use crate::unified_diff::BasicLineDiffPrinter;
use crate::{Algorithm, Diff, UnifiedDiffConfig};

#[test]
fn postprocess() {
    let before = r#"
       /*
        * Stay on the safe side. if read_directory() has run once on
        * "dir", some sticky flag may have been left. Clear them all.
        */
       clear_sticky(dir);

       /*
        * exclude patterns are treated like positive ones in
        * create_simplify. Usually exclude patterns should be a
        * subset of positive ones, which has no impacts on
        * foo
        * bar
        * test
        */
        foo
    "#;
    let after = r#"
       /*
        * exclude patterns are treated like positive ones in
        * create_simplify. Usually exclude patterns should be a
        * subset of positive ones, which has no impacts on
        * foo
        * bar
        * test
        */
        foo
    "#;

    let input = InternedInput::new(before, after);
    for algorithm in [Algorithm::Histogram, Algorithm::Myers] {
        println!("{algorithm:?}");
        let mut diff = Diff::compute(algorithm, &input);
        diff.postprocess_lines(&input);
        let diff = diff
            .unified_diff(
                &BasicLineDiffPrinter(&input.interner),
                UnifiedDiffConfig::default(),
                &input,
            )
            .to_string();
        println!("{diff:?}");
        expect![[r#"
            @@ -1,10 +1,4 @@
             
            -       /*
            -        * Stay on the safe side. if read_directory() has run once on
            -        * "dir", some sticky flag may have been left. Clear them all.
            -        */
            -       clear_sticky(dir);
            -
                    /*
                     * exclude patterns are treated like positive ones in
                     * create_simplify. Usually exclude patterns should be a
        "#]]
        .assert_eq(&diff);
    }
}

#[test]
fn replace() {
    let before = r#"fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
    println!("hello world")        
}
"#;

    let after = r#"const TEST: i32 = 0;
fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
    println!("hello world");        
    println!("hello foo {TEST}");        
}
    
"#;
    let input = InternedInput::new(before, after);
    for algorithm in Algorithm::ALL {
        println!("{algorithm:?}");
        let mut diff = Diff::compute(algorithm, &input);
        diff.postprocess_lines(&input);
        expect![[r#"
            @@ -1,5 +1,8 @@
            +const TEST: i32 = 0;
             fn foo() -> Bar{
                 let mut foo = 2.0;
                 foo *= 100 / 2;
            -    println!("hello world")        
            +    println!("hello world");        
            +    println!("hello foo {TEST}");        
             }
            +    
        "#]]
        .assert_eq(
            &diff
                .unified_diff(
                    &BasicLineDiffPrinter(&input.interner),
                    UnifiedDiffConfig::default(),
                    &input,
                )
                .to_string(),
        );
    }
}

#[test]
fn identical_files() {
    let file = r#"fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
}"#;

    for algorithm in Algorithm::ALL {
        println!("{algorithm:?}");
        let input = InternedInput::new(file, file);
        let mut diff = Diff::compute(algorithm, &input);
        diff.postprocess_lines(&input);
        assert_eq!(
            diff.unified_diff(
                &BasicLineDiffPrinter(&input.interner),
                UnifiedDiffConfig::default(),
                &input,
            )
            .to_string(),
            ""
        );
    }
}

#[test]
fn simple_insert() {
    let before = r#"fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
}"#;

    let after = r#"fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
    println("hello world")
}"#;

    let mut input = InternedInput::new(before, after);
    for algorithm in Algorithm::ALL {
        println!("{algorithm:?}");
        let mut diff = Diff::compute(algorithm, &input);
        diff.postprocess_lines(&input);
        expect![[r#"
          @@ -1,4 +1,5 @@
           fn foo() -> Bar{
               let mut foo = 2.0;
               foo *= 100 / 2;
          +    println("hello world")
           }
          "#]]
        .assert_eq(
            &diff
                .unified_diff(
                    &BasicLineDiffPrinter(&input.interner),
                    UnifiedDiffConfig::default(),
                    &input,
                )
                .to_string(),
        );

        swap(&mut input.before, &mut input.after);

        let mut diff = Diff::compute(algorithm, &input);
        diff.postprocess_lines(&input);
        expect![[r#"
            @@ -1,5 +1,4 @@
             fn foo() -> Bar{
                 let mut foo = 2.0;
                 foo *= 100 / 2;
            -    println("hello world")
             }
            "#]]
        .assert_eq(
            &diff
                .unified_diff(
                    &BasicLineDiffPrinter(&input.interner),
                    UnifiedDiffConfig::default(),
                    &input,
                )
                .to_string(),
        );
        swap(&mut input.before, &mut input.after);
    }
}

pub fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    let mut res = PathBuf::from(dir);
    while !res.join("README.md").exists() {
        res = res
            .parent()
            .expect("reached fs root without finding project root")
            .to_owned()
    }
    res
}

#[test]
#[cfg(not(miri))]
fn hand_checked_udiffs() {
    for algorithm in Algorithm::ALL {
        println!("{algorithm:?}");
        let test_dir = project_root().join("tests");
        let file = "helix_syntax.rs";
        let path_before = test_dir.join(format!("{file}.before"));
        let path_after = test_dir.join(format!("{file}.after"));
        let path_diff = test_dir.join(format!("{file}.{algorithm:?}.diff"));
        let before = read_to_string(path_before).unwrap();
        let after = read_to_string(path_after).unwrap();
        let input = InternedInput::new(&*before, &*after);
        let mut diff = Diff::compute(algorithm, &input);
        diff.postprocess_lines(&input);
        expect_file![path_diff].assert_eq(
            &diff
                .unified_diff(
                    &BasicLineDiffPrinter(&input.interner),
                    UnifiedDiffConfig::default(),
                    &input,
                )
                .to_string(),
        );
    }
}

#[test]
#[cfg(not(miri))]
fn complex_diffs() {
    for algorithm in Algorithm::ALL {
        println!("{algorithm:?}");
        let test_dir = project_root().join("tests");
        for (file1, file2) in [
            ("test1.json", "test2.json"),
            ("helix_syntax.rs.Histogram.diff", "helix_syntax.rs.after"),
        ] {
            let path_before = test_dir.join(file1);
            let path_diff = test_dir.join(file2);
            let before = read_to_string(path_before).unwrap();
            let after = read_to_string(path_diff).unwrap();
            let input = InternedInput::new(&*before, &*after);
            let mut diff = Diff::compute(algorithm, &input);
            println!("start postprocess {file1}");
            diff.postprocess_lines(&input);
            println!("-{} +{}", diff.count_removals(), diff.count_additions())
        }
    }
}

// fn git_diff(
//     algo: Algorithm,
//     repo: &git::Repository,
//     rev1: &git::bstr::BStr,
//     rev2: &git::bstr::BStr,
// ) -> String {
//     let commit1 = repo
//         .rev_parse_single(rev1)
//         .unwrap()
//         .object()
//         .unwrap()
//         .peel_to_kind(git::object::Kind::Commit)
//         .unwrap()
//         .into_commit();
//     let commit2 = repo
//         .rev_parse_single(rev2)
//         .unwrap()
//         .object()
//         .unwrap()
//         .peel_to_kind(git::object::Kind::Commit)
//         .unwrap()
//         .into_commit();
//     let mut res = String::new();
//     commit1
//         .tree()
//         .unwrap()
//         .changes()
//         .track_path()
//         .for_each_to_obtain_tree(
//             &commit2.tree().unwrap(),
//             |change| -> Result<_, fmt::Error> {
//                 match change.event {
//                     git::object::tree::diff::change::Event::Addition { id, .. } => {
//                         let blob = id
//                             .object()
//                             .unwrap()
//                             .peel_to_kind(git::objs::Kind::Blob)
//                             .unwrap();
//                         writeln!(&mut res, "@@")?;
//                         for line in blob.data.as_slice().tokenize() {
//                             write!(&mut res, "+{}", BStr::new(line))?;
//                         }
//                     }
//                     git::object::tree::diff::change::Event::Deletion { id, .. } => {
//                         let blob = id
//                             .object()
//                             .unwrap()
//                             .peel_to_kind(git::objs::Kind::Blob)
//                             .unwrap();
//                         writeln!(&mut res, "@@")?;
//                         for line in blob.data.as_slice().tokenize() {
//                             write!(&mut res, "-{}", BStr::new(line))?;
//                         }
//                         if !res.ends_with('\n') {
//                             writeln!(&mut res)?;
//                         }
//                     }
//                     git::object::tree::diff::change::Event::Modification {
//                         previous_id,
//                         id,
//                         ..
//                     } => {
//                         let prev_blob = previous_id
//                             .object()
//                             .unwrap()
//                             .peel_to_kind(git::objs::Kind::Blob)
//                             .unwrap();
//                         let blob = id
//                             .object()
//                             .unwrap()
//                             .peel_to_kind(git::objs::Kind::Blob)
//                             .unwrap();
//                         let mut input = InternedInput::default();
//                         input.reserve(
//                             prev_blob.data.as_slice().estimate_tokens(),
//                             blob.data.as_slice().estimate_tokens(),
//                         );
//                         input.update_before(prev_blob.data.as_slice().tokenize().map(BStr::new));
//                         input.update_after(blob.data.as_slice().tokenize().map(BStr::new));
//                         let mut diff = Diff::compute(algo, &input);
//                         diff.postprocess(&input);
//                         write!(
//                             &mut res,
//                             "{}",
//                             diff.unified_diff(
//                                 &SimpleLineDiff(&input.interner),
//                                 UnifiedDiffConfig::default(),
//                                 &input,
//                             )
//                         )?;
//                     }
//                 }
//                 if !res.ends_with('\n') {
//                     writeln!(&mut res)?;
//                 }

//                 Ok(git::object::tree::diff::Action::Continue)
//             },
//         )
//         .unwrap();

//     res
// }
