use heim_common::prelude::{Future, Result};

use crate::sys;

/// Returns future which will resolve into a amount of logical CPUs.
pub fn logical_count() -> impl Future<Output = Result<u64>> {
    sys::logical_count()
}
