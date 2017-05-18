# futures-error-chain
This library is a workaround until `futures-rs` and `error-chain` crates works out-of-box with each other

[![Crates.io](https://img.shields.io/crates/v/futures-error-chain.svg?maxAge=2592000)](https://crates.io/crates/futures-error-chain)

## Usage

Just use like the `error-chain` crate, you run the `future_chain!` macro that will create the code needed to work with `error-chain` crate.

Example:
```rust
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate futures;
extern crate futures_cpupool;

#[macro_use]
extern crate futures_error_chain;

mod foo {
    mod errors {
        error_chain! {
            errors {
                Foo(err: String) {
                    description("Foo error")
                    display("Foo error: {}", err)
                }
            }
        }

        future_chain!{}
    }

    pub use self::errors::*;

    use futures::future;

    fn bar() -> FutureChain<String> {
        future::err(ErrorKind::Foo("bar".to_owned()).into()).boxed()
    }

    fn bar2() -> FutureChain<String> {
        future::ok("bar2".to_owned()).boxed()
    }

    pub fn foo() -> FutureChain<String> {
        bar().and_then(|_| bar2()).chain_err(|| "foo")
    }
}

mod errors {
    error_chain! {
        links {
            Foo(::foo::Error, ::foo::ErrorKind);
        }
    }
}

use errors::*;

fn my_main() -> Result<()> {
    use futures_cpupool::CpuPool;
    use futures::Future;

    let pool = CpuPool::new_num_cpus();

    let f = foo::foo();
    let f2 = pool.spawn(f);

    f2.wait()?;

    Ok(())
}

quick_main!(my_main);
```
