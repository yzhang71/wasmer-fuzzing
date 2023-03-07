#![no_main]

use libfuzzer_sys::{arbitrary, arbitrary::Arbitrary, fuzz_target};
use wasm_smith::{Config, ConfiguredModule};
use wasmer::{imports, CompilerConfig, EngineBuilder, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;

use wasmer::{wat2wasm, Value};

#[derive(Arbitrary, Debug, Default, Copy, Clone)]
struct NoImportsConfig;
impl Config for NoImportsConfig {
    fn max_imports(&self) -> usize {
        0
    }
    // fn max_memory_pages(&self) -> u32 {
    fn max_memory_pages(&self, is_64: bool) -> u64 {
        // https://github.com/wasmerio/wasmer/issues/2187
        65535
    }
    fn allow_start_export(&self) -> bool {
        false
    }
}
#[derive(Arbitrary)]
struct WasmSmithModule(ConfiguredModule<NoImportsConfig>);
impl std::fmt::Debug for WasmSmithModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_str(&wasmprinter::print_bytes(self.0.to_bytes()).unwrap())
        f.write_str(&wasmprinter::print_bytes(self.0.module.to_bytes()).unwrap())
    }
}

// use core::slice::SlicePattern;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::fs;
use std::str;

// |data: &[u8]|
fuzz_target!(|module: WasmSmithModule| {
    // let wasm_bytes = module.0.to_bytes();
    let wasm_bytes = module.0.module.to_bytes();

    // println!("The module is: {:?}", &module);

    if let Ok(path) = std::env::var("DUMP_TESTCASE") {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path).unwrap();
        file.write_all(&wasm_bytes).unwrap();
        return;
    }

    let mut compiler = Cranelift::default();
    compiler.canonicalize_nans(true);
    compiler.enable_verifier();
    let mut store = Store::new(compiler);

    // println!("The content is: {:?}", module.0);

    // let module = Module::new(&store, &wasm_bytes);

    match Module::new(&store, &wasm_bytes){
        Ok(_) => {}
        Err(e) => {
            // let error_message = format!("{}", e);
            // if error_message.starts_with("RuntimeError: ")
            //     && error_message.contains("out of bounds")
            // {
            //     return;
            // }
            panic!("{}", e);
            // println!("{}", e);
        }
    }
});


// fuzz_target!(|module: WasmSmithModule| {
//     let wasm_bytes = module.0.to_bytes();

//     if let Ok(path) = std::env::var("DUMP_TESTCASE") {
//         use std::fs::File;
//         use std::io::Write;
//         let mut file = File::create(path).unwrap();
//         file.write_all(&wasm_bytes).unwrap();
//         return;
//     }

//     let mut compiler = Cranelift::default();
//     compiler.canonicalize_nans(true);
//     compiler.enable_verifier();
//     let mut store = Store::new(compiler);
//     let module = Module::new(&store, &wasm_bytes).unwrap();
//     match Instance::new(&mut store, &module, &imports! {}) {
//         Ok(_) => {}
//         Err(e) => {
//             let error_message = format!("{}", e);
//             if error_message.starts_with("RuntimeError: ")
//                 && error_message.contains("out of bounds")
//             {
//                 return;
//             }
//             panic!("{}", e);
//         }
//     }
// });
