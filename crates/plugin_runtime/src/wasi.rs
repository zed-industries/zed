use std::{collections::HashMap, fs::File, path::Path};

use anyhow::{anyhow, Error};
use serde::{de::DeserializeOwned, Serialize};

use wasmtime::{Engine, Func, Instance, Linker, Memory, MemoryType, Module, Store, TypedFunc};
use wasmtime_wasi::{dir, Dir, WasiCtx, WasiCtxBuilder};

pub struct Wasi {
    engine: Engine,
    module: Module,
    store: Store<WasiCtx>,
    instance: Instance,
    alloc_buffer: TypedFunc<i32, i32>,
    // free_buffer: TypedFunc<(i32, i32), ()>,
}

pub struct WasiPlugin {
    pub module: Vec<u8>,
    pub wasi_ctx: WasiCtx,
}

impl Wasi {
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

impl Wasi {
    pub fn default_ctx() -> WasiCtx {
        WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build()
    }

    pub fn init(plugin: WasiPlugin) -> Result<Self, Error> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        println!("linking");
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        println!("linked");
        let mut store: Store<_> = Store::new(&engine, plugin.wasi_ctx);
        println!("moduling");
        let module = Module::new(&engine, plugin.module)?;
        println!("moduled");

        linker.module(&mut store, "", &module)?;
        println!("linked again");
        let instance = linker.instantiate(&mut store, &module)?;
        println!("instantiated");

        let alloc_buffer = instance.get_typed_func(&mut store, "__alloc_buffer")?;
        // let free_buffer = instance.get_typed_func(&mut store, "__free_buffer")?;
        println!("can alloc");

        Ok(Wasi {
            engine,
            module,
            store,
            instance,
            alloc_buffer,
            // free_buffer,
        })
    }

    pub fn attach_file<T: AsRef<Path>>(&mut self, path: T) -> Result<(), Error> {
        let ctx = self.store.data_mut();
        let file = File::open(&path).unwrap();
        let dir = Dir::from_std_file(file);
        // this is a footgun and a half.
        let dir = dir::Dir::from_cap_std(dir);
        ctx.push_preopened_dir(Box::new(dir), path)?;
        Ok(())
    }

    // pub fn remove_file<T: AsRef<Path>>(&mut self, path: T) -> Result<(), Error> {
    //     let ctx = self.store.data_mut();
    //     ctx.remove
    //     Ok(())
    // }

    // So this call function is kinda a dance, I figured it'd be a good idea to document it.
    // the high level is we take a serde type, serialize it to a byte array,
    // (we're doing this using bincode for now)
    // then toss that byte array into webassembly.
    // webassembly grabs that byte array, does some magic,
    // and serializes the result into yet another byte array.
    // we then grab *that* result byte array and deserialize it into a result.
    //
    // phew...
    //
    // now the problem is, webassambly doesn't support buffers.
    // only really like i32s, that's it (yeah, it's sad. Not even unsigned!)
    // (ok, I'm exaggerating a bit).
    //
    // the Wasm function that this calls must have a very specific signature:
    //
    // fn(pointer to byte array: i32, length of byte array: i32)
    //     -> pointer to (
    //            pointer to byte_array: i32,
    //            length of byte array: i32,
    //     ): i32
    //
    // This pair `(pointer to byte array, length of byte array)` is called a `Buffer`
    // and can be found in the cargo_test plugin.
    //
    // so on the wasm side, we grab the two parameters to the function,
    // stuff them into a `Buffer`,
    // and then pray to the `unsafe` Rust gods above that a valid byte array pops out.
    //
    // On the flip side, when returning from a wasm function,
    // we convert whatever serialized result we get into byte array,
    // which we stuff into a Buffer and allocate on the heap,
    // which pointer to we then return.
    // Note the double indirection!
    //
    // So when returning from a function, we actually leak memory *twice*:
    //
    // 1) once when we leak the byte array
    // 2) again when we leak the allocated `Buffer`
    //
    // This isn't a problem because Wasm stops executing after the function returns,
    // so the heap is still valid for our inspection when we want to pull things out.

    // TODO: dont' use as for conversions
    pub fn call<A: Serialize, R: DeserializeOwned>(
        &mut self,
        handle: &str,
        arg: A,
    ) -> Result<R, Error> {
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
        // TODO: precompute handle
        let fun_name = format!("__{}", handle);
        let fun = self
            .instance
            .get_typed_func::<(i32, i32), i32, _>(&mut self.store, &fun_name)?;

        // call the function, passing in the buffer and its length
        // this should return a pointer to a (ptr, lentgh) pair
        let arg_buffer = (arg_buffer_ptr, arg_buffer_len as i32);
        let result_buffer = fun.call(&mut self.store, arg_buffer)?;

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

        // read the buffer at this point into a byte array
        // deserialize the byte array into the provided serde type
        let result = &plugin_memory.data(&mut self.store)[result_buffer_ptr..result_buffer_end];
        let result = bincode::deserialize(result)?;

        // TODO: this is handled wasm-side, but I'd like to double-check
        // // deallocate the argument buffer
        // self.free_buffer.call(&mut self.store, arg_buffer);

        return Ok(result);
    }
}
