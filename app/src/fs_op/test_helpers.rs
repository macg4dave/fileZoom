//! Test-only helpers for exercising failure paths and synchronization in
//! the filesystem operations tests.
//!
//! This module exposes a small set of functions (enabled behind the
//! `test-helpers` feature) that let tests force specific failure paths
//! (for example, to simulate a rename failure) and acquire a global
//! test mutex to serialize operations that would otherwise race in
//! tests. When the feature is disabled the functions remain available
//! as safe no-op fallbacks so callers need not be conditional.
//!
//! Exported symbols are `pub(crate)` because these helpers are internal
//! to the crate's test-suite support.

#[cfg(feature = "test-helpers")]
mod inner {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Mutex, MutexGuard, OnceLock};

    // Three distinct, globally-visible flags used to force failure
    // behaviour in different code paths. `AtomicBool::new` is const so
    // these statics are safe to initialize at compile time.
    static FORCE_RENAME_FAIL_IN_COPY: AtomicBool = AtomicBool::new(false);
    static FORCE_RENAME_FAIL_IN_WRITE: AtomicBool = AtomicBool::new(false);
    static FORCE_RENAME_FAIL_IN_RENAME_OR_COPY: AtomicBool = AtomicBool::new(false);

    // A singleton mutex used to serialize test actions that would
    // otherwise race (for example temporary file cleanup checks).
    static TEST_HOOK_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    /// Set whether rename should be forced to fail during `copy` tests.
    pub(crate) fn set_force_rename_fail_in_copy(value: bool) {
        FORCE_RENAME_FAIL_IN_COPY.store(value, Ordering::SeqCst);
    }

    /// Query whether rename is forced to fail during `copy` tests.
    pub(crate) fn should_force_rename_fail_in_copy() -> bool {
        FORCE_RENAME_FAIL_IN_COPY.load(Ordering::SeqCst)
    }

    /// Set whether rename should be forced to fail during `write` tests.
    pub(crate) fn set_force_rename_fail_in_write(value: bool) {
        FORCE_RENAME_FAIL_IN_WRITE.store(value, Ordering::SeqCst);
    }

    /// Query whether rename is forced to fail during `write` tests.
    pub(crate) fn should_force_rename_fail_in_write() -> bool {
        FORCE_RENAME_FAIL_IN_WRITE.load(Ordering::SeqCst)
    }

    /// Set whether rename should be forced to fail for rename-or-copy
    /// code paths.
    pub(crate) fn set_force_rename_fail_in_rename_or_copy(value: bool) {
        FORCE_RENAME_FAIL_IN_RENAME_OR_COPY.store(value, Ordering::SeqCst);
    }

    /// Query whether rename is forced to fail for rename-or-copy paths.
    pub(crate) fn should_force_rename_fail_in_rename_or_copy() -> bool {
        FORCE_RENAME_FAIL_IN_RENAME_OR_COPY.load(Ordering::SeqCst)
    }

    /// Acquire the global test lock. This function returns a
    /// `MutexGuard<'static, ()>` which releases the lock when dropped.
    ///
    /// The function will panic if the mutex has been poisoned; this is
    /// acceptable for test scaffolding where a poisoned mutex indicates
    /// a prior test failure.
    pub(crate) fn acquire_test_lock() -> MutexGuard<'static, ()> {
        TEST_HOOK_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("failed to acquire test hook mutex")
    }
}

#[cfg(not(feature = "test-helpers"))]
#[allow(dead_code)]
mod inner {
    use std::sync::{Mutex, MutexGuard, OnceLock};

    /// No-op setter when `test-helpers` feature is disabled.
    pub(crate) fn set_force_rename_fail_in_copy(_value: bool) {}
    /// No-op query when feature disabled.
    pub(crate) fn should_force_rename_fail_in_copy() -> bool {
        false
    }

    pub(crate) fn set_force_rename_fail_in_write(_value: bool) {}
    pub(crate) fn should_force_rename_fail_in_write() -> bool {
        false
    }

    pub(crate) fn set_force_rename_fail_in_rename_or_copy(_value: bool) {}
    pub(crate) fn should_force_rename_fail_in_rename_or_copy() -> bool {
        false
    }

    /// Provide a dummy mutex guard when feature is disabled so callers
    /// can hold a lock without branching on the feature.
    pub(crate) fn acquire_test_lock() -> MutexGuard<'static, ()> {
        static DUMMY: OnceLock<Mutex<()>> = OnceLock::new();
        DUMMY.get_or_init(|| Mutex::new(())).lock().expect("failed to acquire dummy mutex")
    }
}

// Re-export the internal implementations with tightened visibility so
// other modules inside the crate can use them while keeping the public
// API surface minimal.
// The `inner` module provides the test helpers (real implementations when
// `test-helpers` feature enabled, no-op fallbacks otherwise). Re-export
// the intended API at crate-internal visibility so other `fs_op` helpers
// (and tests) can reference `crate::fs_op::test_helpers::{...}`.
//
// Some builds (tests without the feature enabled) can trigger ``unused
// import`` lint noise for these re-exports; silence that lint here while
// keeping the tidy internal API stable.
#[allow(unused_imports)]
pub(crate) use inner::acquire_test_lock;
#[allow(unused_imports)]
pub(crate) use inner::set_force_rename_fail_in_copy;
#[allow(unused_imports)]
pub(crate) use inner::set_force_rename_fail_in_rename_or_copy;
#[allow(unused_imports)]
pub(crate) use inner::set_force_rename_fail_in_write;
#[allow(unused_imports)]
pub(crate) use inner::should_force_rename_fail_in_copy;
#[allow(unused_imports)]
pub(crate) use inner::should_force_rename_fail_in_rename_or_copy;
#[allow(unused_imports)]
pub(crate) use inner::should_force_rename_fail_in_write;

#[cfg(test)]
mod tests {
    use super::inner;

    #[test]
    fn acquire_lock_multiple_times() {
        // We can lock multiple times across scopes; dropping releases the lock.
        {
            let _g = inner::acquire_test_lock();
        }
        // second acquire should succeed
        let _g2 = inner::acquire_test_lock();
        drop(_g2);
    }

    #[cfg(feature = "test-helpers")]
    #[test]
    fn feature_flags_toggle() {
        // Ensure each flag can be set and cleared.
        inner::set_force_rename_fail_in_copy(true);
        assert!(inner::should_force_rename_fail_in_copy());
        inner::set_force_rename_fail_in_copy(false);
        assert!(!inner::should_force_rename_fail_in_copy());

        inner::set_force_rename_fail_in_write(true);
        assert!(inner::should_force_rename_fail_in_write());
        inner::set_force_rename_fail_in_write(false);
        assert!(!inner::should_force_rename_fail_in_write());

        inner::set_force_rename_fail_in_rename_or_copy(true);
        assert!(inner::should_force_rename_fail_in_rename_or_copy());
        inner::set_force_rename_fail_in_rename_or_copy(false);
        assert!(!inner::should_force_rename_fail_in_rename_or_copy());
    }

    #[cfg(not(feature = "test-helpers"))]
    #[test]
    fn non_feature_defaults() {
        // When the feature is disabled the query functions are stable no-ops.
        assert!(!inner::should_force_rename_fail_in_copy());
        assert!(!inner::should_force_rename_fail_in_write());
        assert!(!inner::should_force_rename_fail_in_rename_or_copy());
    }
}


