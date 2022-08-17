fn main() {
    // Create the `WasiEnv`.
    let mut wasi_env = wasmer_wasi::WasiState::new("command-name")
        .finalize()
        .unwrap();
    let store = wasmer::Store::default();
    let module = wasmer::Module::from_file(
        &store,
        "../wasi-plugin/target/wasm32-wasi/debug/wasi-plugin.wasm",
    )
    .unwrap();
    let imports = wasi_env.import_object(&module).unwrap();
    let instance = wasmer::Instance::new(&module, &imports).unwrap();
    let start = instance.exports.get_function("_start").unwrap();
    start.call(&[]).unwrap();
}
