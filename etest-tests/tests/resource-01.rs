use etest::prelude::*;

enum Output {
    Hdmi,
    Lvds,
}

impl From<Output> for ResourceId {
    fn from(o: Output) -> Self {
        match o {
            Output::Hdmi	=> "hdmi",
            Output::Lvds	=> "lvds",
        }.into()
    }
}

#[etest(consumes=[Output::Hdmi])]
fn test0() {
}

#[etest(uses=[Output::Lvds])]
fn test1() {
}

#[etest(uses=[Output::Lvds, Output::Hdmi])]
fn test2() {
}

#[etest(consumes=[Output::Lvds, Output::Hdmi])]
fn test3() {
}
