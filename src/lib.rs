#![allow(clippy::redundant_field_names)]

//! Controls execution of `#[test]`
//!
//! Crate supports
//!
//! - [skipping](#conditional-execution) of tests based on dynamic conditions
//!
//! - [serial execution](#limiting-parallel-execution) of tests
//!
//! - [scheduling timeouts](#timeout) of tests
//!
//! See [etest-tests](../../etest_tests/) crate for more examples.
//!
//! ## Conditional execution
//!
//! Related attributes:
//!
//! - `skip`: an expression which when evaluates to true will skip the test.
//!   This expression is executed at the runtime of the test
//!
//! - `skip_result`: allows to explicitly state the return value of the
//!   function when it is skipped.  It should not be needed in most cases;
//!   when not given the return type of the test function must implement
//!   [`DefaultReturn`]
//!
//! ### Examples
//!
//! ```
//! # use etest::etest;
//! fn is_arch_arm() -> bool {
//!     /* ... */
//! # false
//! }
//!
//! #[etest(skip=!is_arch_arm(), skip_result=42.into())]
//! fn test_arm() -> ExitCode {
//!     /* ... */
//! # 0
//! }
//! ```
//!
//! ```
//! # use etest::etest;
//! #[etest(skip=true, skip_result=Err(()))]
//! fn test() -> Result<(), ()> {
//!     Ok(())
//! }
//! ```
//!
//! ### Limitations
//!
//! `etest` still uses the default `#[test]` harnish which does not support
//! skipping of tests.  Hence there is no special indication that a test is
//! skipped: it will be reported as "ok" or (when `skip_result` forces it) to
//! "failed".
//!
//! Only the test output (`--show-output`) will tell the reason
//!
//! ```text
//! ---- test::test0 stdout ----
//! src/test.rs:26:1 (test::test0): SKIPPED
//! ```
//!
//! ## Limiting parallel execution
//!
//! Parallel execution of tests can be prevented by consuming resources for
//! the runtime of the test.  A "resource" can be specified by something which
//! implements [`Into<ResourceId>`](ResourceId).
//!
//! Related attributes:
//!
//! - `uses`: a list of resources which are used; multiple tests can "use" the
//!   same resource at the same time.  It can be compared to a shared lock.
//!
//! - `consumes`: a list of resources which are consumed by the test; only a
//!   single test can own such a resource and all other ones which try to
//!   "use" or "consume" it are blocked until the test finishes.
//!
//!   It can be compared to an exclusive lock.
//!
//! - `no_default_uses`: by default, every test uses implicitly a default
//!   resource. This tag prevents it.  See also `notparallel` below.
//!
//! - `notparallel`: with this tag the default ressource (see
//!   `no_default_uses` above) is consumed so that the test does not run with
//!   other ones in parallel.
//!
//! Both the `uses` and `consumes` resources can be specified as
//!
//! - a single literal (e.g. `"video"`)
//!
//! - a bracket, comma separated list of expressions which are evaluated at
//!   runtime of the test
//!
//! Resources will be allocated **after** checking whether test shall be
//! skipped.
//!
//! ### Examples
//!
//! ```
//! # use etest::etest;
//! #[etest(consumes=["video", "audio"], uses="network", no_default_uses)]
//! fn test0() { /* ... */ }
//! ```
//!
//! ```
//! # use etest::etest;
//! # fn is_hdmi_connected() -> bool { true }
//! fn output() -> &'static str {
//!     if is_hdmi_connected() { "hdmi" } else { "lvds" }
//! }
//!
//! #[etest(consumes=[output()])]
//! fn test1() { /* ... */ }

//! #[etest(notparallel)]
//! fn test2() { /* ... */ }
//! ```
//!
//! ```
//! # use etest::{ etest, ResourceId };
//! enum Output {
//!     Hdmi,
//!     Lvds,
//! }
//!
//! impl From<Output> for ResourceId {
//! # fn from(_: Output) -> Self { "hdmi".into() }
//!     /* ... */
//! }
//!
//! #[etest(consumes=[Output::Hdmi])]
//! fn test2() { /* ... */ }
//! ```
//!
//! ## Timeout
//!
//! Related attributes:
//!
//! - `timeout`: timeout which represents the maximum runtime of the test.
//!   When test is still active after this time, it will be aborted by a
//!   `panic!`.
//!
//!   The timeout is a value which implements [`Into<Timeout>`](Timeout);
//!   plain numbers will mean milliseconds.
//!
//! Clock will start to tick **after** resources have been allocated.
//!
//! ### Examples
//!
//! ```
//! # use etest::etest;
//! #[etest(timeout=20_000)]
//! fn test() { /* ... */ }
//! ```


// declares macros for use in crate; must be on top of file
#[doc(hidden)]
mod logging;

pub use etest_derive::etest;

mod resource;
mod location;
mod timeout;
mod default_return;
mod helpers;

#[doc(hidden)]
pub use location::Location;

#[doc(inline)]
pub use resource::ResourceId;

// TODO: this is only public to allow rustdoc to generate the related
// documentation
pub use resource::ResourceIdImpl;

#[doc(hidden)]
pub use resource::{ ResourceBuilder, RESOURCES };

#[doc(inline)]
pub use default_return::DefaultReturn;

#[doc(inline)]
pub use timeout::Timeout;

#[doc(hidden)]
pub use helpers::*;

#[doc(hidden)]
pub mod prelude {
    pub use crate::DefaultReturn;
    pub use crate::ResourceId;
    pub use crate::Timeout;
    pub use crate::etest;
}
