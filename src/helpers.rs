use crate::Location;

pub fn mark_skipped(loc: &Location) {
    eprintln!("{}: SKIPPED", loc);
}

// from https://github.com/rust-lang/rfcs/issues/2798#issuecomment-552949300
pub fn panic_after<T, F>(loc: &Location, d: std::time::Duration, f: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T,
    F: Send + 'static,
{
    use std::sync::mpsc::RecvTimeoutError as E;
    use std::sync::Arc;

    let (done_tx, done_rx) = std::sync::mpsc::channel();

    // Use `Arc` for refcounting: master thread (which might panic on timeouts)
    // holds the strong count, the test thread a weak reference.
    //
    // Master drops its strong reference on exit or shortly before the panic!().
    // Child thread will send the completion signal only when there are strong
    // references.
    let is_alive = Arc::new(());

    let mut t_builder = std::thread::Builder::new();

    // atm, there is no way to retrieve the current test name.  Some users may
    // rely on the hack that it is assigned to the thread name.  Copy it when
    // possible (which might enhance diagnosts too).
    if let Some(name) = std::thread::current().name() {
        t_builder = t_builder.name(name.to_string());
    }

    let handle = t_builder.spawn({
        let is_alive = Arc::downgrade(&is_alive);

        move || {
            let val = f();

            if is_alive.strong_count() > 0 {
                done_tx.send(()).expect("Unable to send completion signal");
            }

            val
        }
    }).unwrap();

    match done_rx.recv_timeout(d) {
        Err(E::Timeout)	=> {
            drop(is_alive);
            panic!("{}: TIMEOUT", loc);
        },

        _		=> match handle.join() {
            Ok(r)	=> r,
            Err(e)	=> std::panic::resume_unwind(e),
        }
    }
}
