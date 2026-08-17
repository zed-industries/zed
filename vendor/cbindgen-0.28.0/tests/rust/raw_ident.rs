#[repr(u8)]
pub enum r#Enum {
    r#a,
    r#b,
}

#[repr(C)]
pub struct r#Struct {
    r#field: r#Enum,
}

#[no_mangle]
pub extern "C" fn r#fn(r#arg: r#Struct) {
    println!("Hello world");
}

pub mod r#mod {
    #[no_mangle]
    pub static r#STATIC: r#Enum = r#Enum::r#b;
}
