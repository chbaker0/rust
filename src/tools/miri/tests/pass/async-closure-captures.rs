// Same as rustc's `tests/ui/async-await/async-closures/captures.rs`, keep in sync

#![feature(async_closure, noop_waker)]

use std::future::Future;
use std::pin::pin;
use std::task::*;

pub fn block_on<T>(fut: impl Future<Output = T>) -> T {
    let mut fut = pin!(fut);
    let ctx = &mut Context::from_waker(Waker::noop());

    loop {
        match fut.as_mut().poll(ctx) {
            Poll::Pending => {}
            Poll::Ready(t) => break t,
        }
    }
}

fn main() {
    block_on(async_main());
}

async fn call<T>(f: &impl async Fn() -> T) -> T {
    f().await
}

async fn call_once<T>(f: impl async FnOnce() -> T) -> T {
    f().await
}

#[derive(Debug)]
#[allow(unused)]
struct Hello(i32);

async fn async_main() {
    // Capture something by-ref
    {
        let x = Hello(0);
        let c = async || {
            println!("{x:?}");
        };
        call(&c).await;
        call_once(c).await;

        let x = &Hello(1);
        let c = async || {
            println!("{x:?}");
        };
        call(&c).await;
        call_once(c).await;
    }

    // Capture something and consume it (force to `AsyncFnOnce`)
    {
        let x = Hello(2);
        let c = async || {
            println!("{x:?}");
            drop(x);
        };
        call_once(c).await;
    }

    // Capture something with `move`, don't consume it
    {
        let x = Hello(3);
        let c = async move || {
            println!("{x:?}");
        };
        call(&c).await;
        call_once(c).await;

        let x = &Hello(4);
        let c = async move || {
            println!("{x:?}");
        };
        call(&c).await;
        call_once(c).await;
    }

    // Capture something with `move`, also consume it (so `AsyncFnOnce`)
    {
        let x = Hello(5);
        let c = async move || {
            println!("{x:?}");
            drop(x);
        };
        call_once(c).await;
    }
}
