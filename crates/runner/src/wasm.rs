use std::collections::HashMap;

use anyhow::anyhow;

use wasmtime::{Engine, Func, Instance, Memory, MemoryType, Module, Store, TypedFunc};

use crate::*;

pub struct Wasm<T> {
    engine: Engine,
    module: Module,
    store: Store<T>,
    instance: Instance,
    alloc_buffer: TypedFunc<i32, i32>,
    free_buffer: TypedFunc<(i32, i32), ()>,
}

pub struct WasmPlugin<T> {
    pub source_bytes: Vec<u8>,
    pub store_data: T,
}

impl<T> Wasm<T> {
    pub fn dump_memory(data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            if i % 32 == 0 {
                println!();
            }
            if i % 4 == 0 {
                print!("|");
            }
            if *byte == 0 {
                print!("__")
            } else {
                print!("{:02x}", byte);
            }
        }
        println!();
    }
}

impl<S> Runtime for Wasm<S> {
    type Plugin = WasmPlugin<S>;
    type Error = anyhow::Error;

    fn init(plugin: WasmPlugin<S>) -> Result<Self, Self::Error> {
        let engine = Engine::default();
        let module = Module::new(&engine, plugin.source_bytes)?;
        let mut store: Store<S> = Store::new(&engine, plugin.store_data);
        let instance = Instance::new(&mut store, &module, &[])?;

        let alloc_buffer = instance.get_typed_func(&mut store, "__alloc_buffer")?;
        let free_buffer = instance.get_typed_func(&mut store, "__free_buffer")?;

        Ok(Wasm {
            engine,
            module,
            store,
            instance,
            alloc_buffer,
            free_buffer,
        })
    }

    fn constant<T: DeserializeOwned>(&mut self, handle: &Handle) -> Result<T, Self::Error> {
        let export = self
            .instance
            .get_export(&mut self.store, handle.inner())
            .ok_or_else(|| anyhow!("Could not get export"))?;

        todo!()
    }

    // TODO: dont' use as for conversions
    fn call<A: Serialize, R: DeserializeOwned>(
        &mut self,
        handle: &Handle,
        arg: A,
    ) -> Result<R, Self::Error> {
        // serialize the argument using bincode
        let arg = bincode::serialize(&arg)?;
        let arg_buffer_len = arg.len();

        // allocate a buffer and write the argument to that buffer
        let arg_buffer_ptr = self
            .alloc_buffer
            .call(&mut self.store, arg_buffer_len as i32)?;
        let plugin_memory = self
            .instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow!("Could not grab slice of plugin memory"))?;
        plugin_memory.write(&mut self.store, arg_buffer_ptr as usize, &arg)?;

        // get the webassembly function we want to actually call
        let fun = self
            .instance
            .get_typed_func::<(i32, i32), i32, _>(&mut self.store, handle.inner())?;

        // call the function, passing in the buffer and its length
        // this should return a pointer to a (ptr, lentgh) pair
        let result_buffer = fun.call(&mut self.store, (arg_buffer_ptr, arg_buffer_len as i32))?;
        dbg!(result_buffer);

        // panic!();
        // dbg!()

        // create a buffer to read the (ptr, length) pair into
        // this is a total of 4 + 4 = 8 bytes.
        let buffer = &mut [0; 8];
        plugin_memory.read(&mut self.store, result_buffer as usize, buffer)?;

        // use these bytes (wasm stores things little-endian)
        // to get a pointer to the buffer and its length
        let b = buffer;
        let result_buffer_ptr = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize;
        let result_buffer_len = u32::from_le_bytes([b[4], b[5], b[6], b[7]]) as usize;
        let result_buffer_end = result_buffer_ptr + result_buffer_len;

        dbg!(result_buffer_ptr);
        dbg!(result_buffer_len);

        // read the buffer at this point into a byte array
        let result = &plugin_memory.data(&mut self.store)[result_buffer_ptr..result_buffer_end];

        // deserialize the byte array into the provided serde type
        let result = bincode::deserialize(result)?;
        return Ok(result);
    }

    fn register_handle<T: AsRef<str>>(&mut self, name: T) -> bool {
        self.instance
            .get_export(&mut self.store, name.as_ref())
            .is_some()
    }
}
