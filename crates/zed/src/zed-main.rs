pub fn main() {
    // separated out so that the file containing the main function can be imported by other crates,
    // while having all gpui resources that are registered in main (primarily actions) initialized
    zed::main();
}
