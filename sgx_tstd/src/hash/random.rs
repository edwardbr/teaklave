// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..


//! This module exists to isolate [`RandomState`] and [`DefaultHasher`] outside of the
//! [`collections`] module without actually publicly exporting them, so that parts of that
//! implementation can more easily be moved to the [`alloc`] crate.
//!
//! Although its items are public and contain stability attributes, they can't actually be accessed
//! outside this crate.
//!
//! [`collections`]: crate::collections
#[allow(deprecated)]
use super::{BuildHasher, Hasher, SipHasher13};
use crate::cell::Cell;
use crate::fmt;
use crate::sys;

/// `RandomState` is the default state for [`HashMap`] types.
///
/// A particular instance `RandomState` will create the same instances of
/// [`Hasher`], but the hashers created by two different `RandomState`
/// instances are unlikely to produce the same result for the same values.
///
/// [`HashMap`]: crate::collections::HashMap
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use std::hash::RandomState;
///
/// let s = RandomState::new();
/// let mut map = HashMap::with_hasher(s);
/// map.insert(1, 2);
/// ```
#[derive(Clone)]
pub struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    /// Constructs a new `RandomState` that is initialized with random keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    ///
    /// let s = RandomState::new();
    /// ```
    #[inline]
    #[allow(deprecated)]
    // rand
    #[must_use]
    pub fn new() -> RandomState {
        // Historically this function did not cache keys from the OS and instead
        // simply always called `rand::thread_rng().gen()` twice. In #31356 it
        // was discovered, however, that because we re-seed the thread-local RNG
        // from the OS periodically that this can cause excessive slowdown when
        // many hash maps are created on a thread. To solve this performance
        // trap we cache the first set of randomly generated keys per-thread.
        //
        // Later in #36481 it was discovered that exposing a deterministic
        // iteration order allows a form of DOS attack. To counter that we
        // increment one of the seeds on every RandomState creation, giving
        // every corresponding HashMap a different iteration order.
        thread_local!(static KEYS: Cell<(u64, u64)> = {
            Cell::new(sys::hashmap_random_keys())
        });

        KEYS.with(|keys| {
            let (k0, k1) = keys.get();
            keys.set((k0.wrapping_add(1), k1));
            RandomState { k0, k1 }
        })
    }
}

impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;
    #[inline]
    #[allow(deprecated)]
    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(self.k0, self.k1))
    }
}

/// The default [`Hasher`] used by [`RandomState`].
///
/// The internal algorithm is not specified, and so it and its hashes should
/// not be relied upon over releases.
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct DefaultHasher(SipHasher13);

impl DefaultHasher {
    /// Creates a new `DefaultHasher`.
    ///
    /// This hasher is not guaranteed to be the same as all other
    /// `DefaultHasher` instances, but is the same as all other `DefaultHasher`
    /// instances created through `new` or `default`.
    #[inline]
    #[allow(deprecated)]
    #[must_use]
    pub fn new() -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(0, 0))
    }
}

impl Default for DefaultHasher {
    /// Creates a new `DefaultHasher` using [`new`].
    /// See its documentation for more.
    ///
    /// [`new`]: DefaultHasher::new
    #[inline]
    fn default() -> DefaultHasher {
        DefaultHasher::new()
    }
}

impl Hasher for DefaultHasher {
    // The underlying `SipHasher13` doesn't override the other
    // `write_*` methods, so it's ok not to forward them here.

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.write(msg)
    }

    #[inline]
    fn write_str(&mut self, s: &str) {
        self.0.write_str(s);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}

impl Default for RandomState {
    /// Constructs a new `RandomState`.
    #[inline]
    fn default() -> RandomState {
        RandomState::new()
    }
}

impl fmt::Debug for RandomState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RandomState").finish_non_exhaustive()
    }
}
