use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use std::thread::sleep;

use etest::prelude::*;

static RES_VIDEO:   AtomicU64 = AtomicU64::new(0);
static RES_AUDIO:   AtomicU64 = AtomicU64::new(0);

fn fetch_inc(v: &AtomicU64) -> u64 {
    v.fetch_add(1, Ordering::Relaxed)
}

fn fetch_dec(v: &AtomicU64) -> u64 {
    v.fetch_sub(1, Ordering::Relaxed)
}

#[etest(consumes=["video", "audio"], uses="network", no_default_uses)]
fn test0_0() {
    assert_eq!(fetch_inc(&RES_VIDEO), 0);
    assert_eq!(fetch_inc(&RES_AUDIO), 0);

    sleep(Duration::from_secs(2));

    assert_eq!(fetch_dec(&RES_AUDIO), 1);
    assert_eq!(fetch_dec(&RES_VIDEO), 1);
}

#[etest(consumes=["video", "audio"], uses="network", no_default_uses)]
fn test0_1() {
    assert_eq!(fetch_inc(&RES_VIDEO), 0);
    assert_eq!(fetch_inc(&RES_AUDIO), 0);

    sleep(Duration::from_secs(2));

    assert_eq!(fetch_dec(&RES_AUDIO), 1);
    assert_eq!(fetch_dec(&RES_VIDEO), 1);
}

#[etest(uses=["video"])]
fn test0_2() {
    fetch_inc(&RES_VIDEO);

    sleep(Duration::from_secs(2));

    fetch_dec(&RES_VIDEO);
}

#[etest(uses=["audio"])]
fn test0_3() {
    fetch_inc(&RES_AUDIO);

    sleep(Duration::from_secs(2));

    fetch_dec(&RES_AUDIO);
}

#[etest(consumes=["video"])]
fn test0_4() {
    assert_eq!(fetch_inc(&RES_VIDEO), 0);

    sleep(Duration::from_secs(2));

    assert_eq!(fetch_dec(&RES_VIDEO), 1);
}

#[etest(consumes=["audio"])]
fn test0_5() {
    assert_eq!(fetch_inc(&RES_AUDIO), 0);

    sleep(Duration::from_secs(2));

    assert_eq!(fetch_dec(&RES_AUDIO), 1);
}
