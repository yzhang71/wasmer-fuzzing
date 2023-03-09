#![no_main]

use libfuzzer_sys::{arbitrary, arbitrary::Arbitrary, fuzz_target};
use wasm_smith::{Config, ConfiguredModule};
use wasmer::{imports, CompilerConfig, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fuzz_target!(|data: &[u8]| {
    if let Ok(path) = std::env::var("DUMP_TESTCASE") {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path).unwrap();
        file.write_all(&data).unwrap();
        return;
    }

    let mut compiler = Cranelift::default();
    let engine = Universal::new(compiler).engine();

    let store = Store::new(&engine);
    let module = Module::new(&store, &data).unwrap();
    match Instance::new(&module, &imports! {}) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
});
