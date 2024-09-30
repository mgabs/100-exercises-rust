//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
use std::rc::{Rc, Weak as RcWeak};
use std::sync::{Arc, Mutex, Weak as ArcWeak};

use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example_rc());
    tokio::spawn(example_arc());
    tokio::spawn(example_mutex());
}

async fn example_rc() {
    {
        // `Rc` is not send, if we scope it; It will be dropped before calling `yield_now`
        let non_send = Rc::new(1);
        println!("{}", non_send);
    }
    yield_now().await;
}
async fn example_arc() {
    // `Arc` is send.
    let non_send = Arc::new(1);
    println!("{}", non_send);
    yield_now().await;
}
async fn example_mutex() {
    // `Mutex` is send.
    let non_send = Mutex::new(1);
    println!("{:#?}", non_send);
    yield_now().await;
}
