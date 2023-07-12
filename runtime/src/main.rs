use anyhow::Error;
use std::sync::Arc;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::{tokio::WasiCtxBuilder, WasiCtx};

// #[tokio::main] -- no issues when running with the multi-threaded executor
#[tokio::main(flavor = "current_thread")] // setting causing the issue
async fn main() -> Result<(), Error> {
    let env = Environment::new()?;
    run_wasm(&env).await?;
    Ok(())
}

#[derive(Clone)]
struct Environment {
    engine: Engine,
    module: Module,
    linker: Arc<Linker<WasiCtx>>,
}

impl Environment {
    pub fn new() -> Result<Self, Error> {
        let mut config = Config::new();
        config.async_support(true);
        config.consume_fuel(true);
        let engine = Engine::new(&config)?;
        let module = Module::from_file(&engine, "../module/target/wasm32-wasi/debug/module.wasm")?;
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::tokio::add_to_linker(&mut linker, |cx| cx)?;
        Ok(Self {
            engine,
            module,
            linker: Arc::new(linker),
        })
    }
}

async fn run_wasm(env: &Environment) -> Result<(), Error> {
    let wasi = WasiCtxBuilder::new()
        .inherit_stdout()
        .build();
    let mut store = Store::new(&env.engine, wasi);
    store.out_of_fuel_async_yield(u64::MAX, 10000);
    let instance = env
        .linker
        .instantiate_async(&mut store, &env.module)
        .await?;
    instance
        .get_typed_func::<(), i32>(&mut store, "__main_void")?
        .call_async(&mut store, ())
        .await?;
    Ok(())
}