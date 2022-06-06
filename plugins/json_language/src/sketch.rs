
use zed_plugin::RopeRef;


// Host
struct Handles {
    items: Vec<Box<dyn Any>>,
}

struct Rope;

impl Link for Rope {
    fn link(linker: &mut Linker) -> Result<()> {
        linker.add(|this: &mut Rope| {
            
        });
        linker.func_wrap("env", "len", |handles, arg| {
            let rope = handles.downcast::<Rope>(arg.0);
            let rope = Arc::from_raw(ptr);
            let result = rope.len();
            Arc::leak(rope);
            result
        });
    }
    
    fn to_handle(self) -> Handle<Rope> {
        todo!()
    }
}

// -- Host

pub fn edit_rope(&mut self) {
    let rope: &mut Rope = self......rope();
    let handle: RopeRef = self.runtime.to_handle(rope);
    self.runtime.call("edit_rope", handle);
}

// Guest

extern "C" long rope__len(u32 handle);

struct RopeRef(u32);

impl RopeRef {
    fn len(&self) -> usize {
        rope__len(self.0);
    }
}

pub fn edit_rope(rope: RopeRef) {
    rope.len()
}

// Host side ---

pub struct Rope { .. }

RopeRef(u32);

impl Link for RopeRef {
    pub fn init(&mut something) {
        something.add("length", |rope| )
    }
}

// ---

extern "C" {
    pub fn length(item: u32) -> u32;
}

struct RopeRef {
    handle: u32,
}

pub fn length(ref: RopeRef) -> u32 {
    ref.length()
}

// Host side

#[plugin_interface]
trait LspAdapter {
    name() -> &'static str;
}

// Guest side
