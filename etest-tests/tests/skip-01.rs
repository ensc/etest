use std::process::ExitCode;

use etest::prelude::*;

#[etest(skip=false)]
fn test_0() -> () {
}

#[etest(skip=true)]
fn test_1() -> () {
}

#[etest(skip=false)]
fn test_2() -> ExitCode {
    ExitCode::SUCCESS
}

#[etest(skip=true)]
fn test_3() -> ExitCode {
    ExitCode::FAILURE
}

#[etest(skip=true)]
fn test_4() -> ! {
    loop {
    }
}

#[etest(skip=true)]
fn test_5() -> Result<(), ()> {
    Err(())
}

#[etest(skip=true)]
fn test_6() {
    unreachable!()
}
