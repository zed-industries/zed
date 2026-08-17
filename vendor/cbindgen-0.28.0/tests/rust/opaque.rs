/// Fast hash map used internally.
type FastHashMap<K, V> =
    std::collections::HashMap<K, V, std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>;

pub type Foo = FastHashMap<i32, i32>;

pub type Bar = Result<Foo, ()>;

#[no_mangle]
pub extern "C" fn root(a: &Foo, b: &Bar) {}
