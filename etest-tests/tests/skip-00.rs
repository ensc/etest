use std::process::ExitCode;

use etest::prelude::*;


fn is_false() -> bool {
    false
}

#[etest(skip=!is_false(), skip_result=0.into())]
fn test_skipped_0() -> ExitCode {
    ExitCode::FAILURE
}

#[etest(skip=is_false(), skip_result=ExitCode::SUCCESS)]
fn test_not_skipped_0() -> ExitCode {
    ExitCode::SUCCESS
}

#[etest(skip=true, skip_result=ExitCode::SUCCESS)]
fn test_skipped_1() -> ExitCode {
    ExitCode::FAILURE
}

#[etest(skip=false, skip_result=ExitCode::FAILURE)]
fn test_not_skipped_1() -> ExitCode {
    ExitCode::SUCCESS
}
