#[export_name = "do_the_thing_with_export_name"]
pub extern "C" fn do_the_thing() {
  println!("doing the thing!");
}

#[unsafe(export_name = "do_the_thing_with_unsafe_export_name")]
pub extern "C" fn unsafe_do_the_thing() {
  println!("doing the thing!");
}
