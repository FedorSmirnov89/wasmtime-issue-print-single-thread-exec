# Description

Minimal example of a current issue with `wasmtime` where running a module which uses printing (which is compiled to the usage of the `fd_write` function) (a) in asynchronous context and (b) on a single-thread executor causes a panic.

## Building the example

Build the module

```
cd module
cargo build --target wasm32-wasi
```

Building and running the code using the runtime

```
cd runtime
cargo run
```

## Reproducing the issue

- Running with a multi-threaded executor results in the expected behavior and the `Hello, world!` output.
- Running with a single-thread executor -- `#[tokio::main(flavor=current_thread)]` -- results in a panic with the output

```
thread 'main' panicked at 'can call blocking only when running on the multi-threaded runtime', /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasi-tokio-10.0.1/src/lib.rs:123:5
```

Running the code with `RUST_BACKTRACE=1` results in the following stack trace:

```
thread 'main' panicked at 'can call blocking only when running on the multi-threaded runtime', /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasi-tokio-10.0.1/src/lib.rs:123:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:578:5
   1: core::panicking::panic_fmt
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panicking.rs:67:14
   2: core::panicking::panic_display
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panicking.rs:150:5
   3: tokio::runtime::scheduler::multi_thread::worker::block_in_place
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/multi_thread/worker.rs:427:9
   4: tokio::task::blocking::block_in_place
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/task/blocking.rs:78:9
   5: wasi_tokio::block_on_dummy_executor
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasi-tokio-10.0.1/src/lib.rs:123:5
   6: <wasi_tokio::file::Stdout as wasi_common::file::WasiFile>::write_vectored::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasi-tokio-10.0.1/src/file.rs:144:17
   7: <core::pin::Pin<P> as core::future::future::Future>::poll
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/future/future.rs:125:9
   8: wasi_common::snapshots::preview_1::<impl wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1 for wasi_common::ctx::WasiCtx>::fd_write::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasi-common-10.0.1/src/snapshots/preview_1.rs:466:56
   9: <core::pin::Pin<P> as core::future::future::Future>::poll
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/future/future.rs:125:9
  10: wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_write::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasi-common-10.0.1/src/snapshots/preview_1.rs:27:1
  11: <tracing::instrument::Instrumented<T> as core::future::future::Future>::poll
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-0.1.37/src/instrument.rs:272:9
  12: wasmtime_wasi::tokio::snapshots::preview_1::add_wasi_snapshot_preview1_to_linker::{{closure}}::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-wasi-10.0.1/src/lib.rs:34:5
  13: wasmtime::store::AsyncCx::block_on
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/store.rs:1876:17
  14: wasmtime::linker::Linker<T>::func_wrap4_async::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/linker.rs:165:32
  15: <F as wasmtime::func::IntoFunc<T,(wasmtime::func::Caller<T>,A1,A2,A3,A4),R>>::into_func::native_call_shim::{{closure}}::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func.rs:1982:41
  16: core::ops::function::FnOnce::call_once
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/ops/function.rs:250:5
  17: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panic/unwind_safe.rs:271:9
  18: std::panicking::try::do_call
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:485:40
  19: __rust_try
  20: std::panicking::try
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:449:19
  21: std::panic::catch_unwind
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panic.rs:140:14
  22: <F as wasmtime::func::IntoFunc<T,(wasmtime::func::Caller<T>,A1,A2,A3,A4),R>>::into_func::native_call_shim::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func.rs:1977:29
  23: wasmtime::func::Caller<T>::with::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func.rs:1777:13
  24: wasmtime_runtime::instance::Instance::from_vmctx
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-runtime-10.0.1/src/instance.rs:217:9
  25: wasmtime::func::Caller<T>::with
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func.rs:1775:9
  26: <F as wasmtime::func::IntoFunc<T,(wasmtime::func::Caller<T>,A1,A2,A3,A4),R>>::into_func::native_call_shim
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func.rs:1966:34
  27: <unknown>
  28: <unknown>
  29: <unknown>
  30: <unknown>
  31: <unknown>
  32: <unknown>
  33: <unknown>
  34: <unknown>
  35: <unknown>
  36: <unknown>
  37: <unknown>
  38: <unknown>
  39: <unknown>
  40: <unknown>
  41: <unknown>
  42: <unknown>
  43: <() as wasmtime::func::typed::WasmParams>::invoke::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func/typed.rs:587:21
  44: <(A1,) as wasmtime::func::HostAbi>::call
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func.rs:1684:18
  45: <() as wasmtime::func::typed::WasmParams>::invoke
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func/typed.rs:586:17
  46: wasmtime::func::typed::TypedFunc<Params,Results>::call_raw::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func/typed.rs:181:17
  47: wasmtime_runtime::traphandlers::catch_traps::call_closure
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-runtime-10.0.1/src/traphandlers.rs:281:18
  48: wasmtime_setjmp
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-runtime-10.0.1/src/helpers.c:55:3
  49: wasmtime_runtime::traphandlers::catch_traps::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-runtime-10.0.1/src/traphandlers.rs:263:9
  50: wasmtime_runtime::traphandlers::<impl wasmtime_runtime::traphandlers::call_thread_state::CallThreadState>::with::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-runtime-10.0.1/src/traphandlers.rs:389:44
  51: wasmtime_runtime::traphandlers::tls::set
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-runtime-10.0.1/src/traphandlers.rs:740:13
  52: wasmtime_runtime::traphandlers::<impl wasmtime_runtime::traphandlers::call_thread_state::CallThreadState>::with
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-runtime-10.0.1/src/traphandlers.rs:389:19
  53: wasmtime_runtime::traphandlers::catch_traps
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-runtime-10.0.1/src/traphandlers.rs:262:18
  54: wasmtime::func::invoke_wasm_and_catch_traps
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func.rs:1363:22
  55: wasmtime::func::typed::TypedFunc<Params,Results>::call_raw
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func/typed.rs:177:22
  56: wasmtime::func::typed::TypedFunc<Params,Results>::call_async::{{closure}}::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func/typed.rs:126:26
  57: wasmtime::store::<impl wasmtime::store::context::StoreContextMut<T>>::on_fiber::{{closure}}::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/store.rs:1589:34
  58: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/alloc/src/boxed.rs:1973:9
  59: wasmtime_fiber::Suspend<Resume,Yield,Return>::execute::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-fiber-10.0.1/src/lib.rs:166:62
  60: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panic/unwind_safe.rs:271:9
  61: std::panicking::try::do_call
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:485:40
  62: __rust_try
  63: std::panicking::try
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:449:19
  64: std::panic::catch_unwind
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panic.rs:140:14
  65: wasmtime_fiber::Suspend<Resume,Yield,Return>::execute
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-fiber-10.0.1/src/lib.rs:166:22
  66: wasmtime_fiber::unix::fiber_start
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-fiber-10.0.1/src/unix.rs:137:9
  67: wasmtime_fiber_start
  68: wasmtime_fiber::unix::Fiber::resume
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-fiber-10.0.1/src/unix.rs:163:13
  69: wasmtime_fiber::Fiber<Resume,Yield,Return>::resume
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-fiber-10.0.1/src/lib.rs:119:9
  70: wasmtime::store::<impl wasmtime::store::context::StoreContextMut<T>>::on_fiber::{{closure}}::FiberFuture::resume
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/store.rs:1705:28
  71: <wasmtime::store::<impl wasmtime::store::context::StoreContextMut<T>>::on_fiber::{{closure}}::FiberFuture as core::future::future::Future>::poll
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/store.rs:1752:27
  72: wasmtime::store::<impl wasmtime::store::context::StoreContextMut<T>>::on_fiber::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/store.rs:1604:15
  73: wasmtime::func::typed::TypedFunc<Params,Results>::call_async::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-10.0.1/src/func/typed.rs:128:13
  74: runtime::run_wasm::{{closure}}
             at ./src/main.rs:50:9
  75: runtime::main::{{closure}}
             at ./src/main.rs:9:19
  76: <core::pin::Pin<P> as core::future::future::Future>::poll
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/future/future.rs:125:9
  77: tokio::runtime::scheduler::current_thread::CoreGuard::block_on::{{closure}}::{{closure}}::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/current_thread.rs:651:57
  78: tokio::runtime::coop::with_budget
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/coop.rs:107:5
  79: tokio::runtime::coop::budget
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/coop.rs:73:5
  80: tokio::runtime::scheduler::current_thread::CoreGuard::block_on::{{closure}}::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/current_thread.rs:651:25
  81: tokio::runtime::scheduler::current_thread::Context::enter
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/current_thread.rs:410:19
  82: tokio::runtime::scheduler::current_thread::CoreGuard::block_on::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/current_thread.rs:650:36
  83: tokio::runtime::scheduler::current_thread::CoreGuard::enter::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/current_thread.rs:729:68
  84: tokio::runtime::context::scoped::Scoped<T>::set
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/context/scoped.rs:40:9
  85: tokio::runtime::context::set_scheduler::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/context.rs:176:26
  86: std::thread::local::LocalKey<T>::try_with
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/thread/local.rs:252:16
  87: std::thread::local::LocalKey<T>::with
             at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/thread/local.rs:228:9
  88: tokio::runtime::context::set_scheduler
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/context.rs:176:9
  89: tokio::runtime::scheduler::current_thread::CoreGuard::enter
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/current_thread.rs:729:27
  90: tokio::runtime::scheduler::current_thread::CoreGuard::block_on
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/current_thread.rs:638:19
  91: tokio::runtime::scheduler::current_thread::CurrentThread::block_on::{{closure}}
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/current_thread.rs:175:28
  92: tokio::runtime::context::runtime::enter_runtime
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/context/runtime.rs:65:16
  93: tokio::runtime::scheduler::current_thread::CurrentThread::block_on
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/current_thread.rs:167:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```