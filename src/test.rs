use crate::{self as etest, ResourceId};

#[track_caller]
fn foo() -> bool {
//    use std::panic::Location;

    // println!("xx={:?}", Location::caller());

    true
}

enum ResId {
    X,
    Y,
}

impl From<ResId> for ResourceId {
    fn from(value: ResId) -> Self {
        match value {
            ResId::X => "X",
            ResId::Y => "Y",
        }.into()
    }
}

#[etest_derive::etest(skip=foo(), timeout=2_000,
                      consumes={"A"})]
fn test0() {
    std::thread::sleep(std::time::Duration::from_secs(3));
}

#[etest_derive::etest(skip=foo(), timeout=2_000,
                      uses={"A", "B"}, consumes={"C", "D", "E", ResId::X})]
fn test0_0() {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

#[cfg(feature = "tokio")]
mod test_tokio {
    use super::*;

    #[etest_derive::etest(skip=foo(), timeout=2_000, uses="A")]
    async fn test1_0() -> ! {
        todo!()
    }
}

#[cfg(not(feature = "tokio"))]
mod test_no_tokio {
    use super::*;

    #[etest_derive::etest(skip=foo(), test_fn=(), timeout=2_000, uses="A")]
    async fn test1_1() -> ! {
        todo!()
    }
}

#[etest_derive::etest(skip=foo(), timeout=2_000, uses="C", consumes={ResId::Y})]
fn test2() -> () {
    todo!()
}

#[etest_derive::etest(skip=foo(), timeout=2_000, consumes="D")]
fn test3() -> std::result::Result<(), ()> {
    todo!()
}
