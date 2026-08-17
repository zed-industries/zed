// We need a build script to use `link = "rayon-core"`.  But we're not
// *actually* linking to anything, just making sure that we're the only
// rayon-core in use.
fn main() {
    // we don't need to rebuild for anything else
    println!("cargo:rerun-if-changed=build.rs");
}
