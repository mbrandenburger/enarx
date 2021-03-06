// Copyright 2020 Red Hat, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Page SecInfo (Section 38.11)
//! These structs specify metadata about en enclave page.

use bitflags::bitflags;

bitflags! {
    /// The `Flags` of a page
    ///
    /// Section 38.11.1
    pub struct Flags: u8 {
        /// The page can be read from inside the enclave.
        const R = 1 << 0;

        /// The page can be written from inside the enclave.
        const W = 1 << 1;

        /// The page can be executed from inside the enclave.
        const X = 1 << 2;

        /// The page is in the PENDING state.
        const PENDING = 1 << 3;

        /// The page is in the MODIFIED state.
        const MODIFIED = 1 << 4;

        /// A permission restriction operation on the page is in progress.
        const PR = 1 << 5;
    }
}

/// The `Class` of a page
///
/// The `Class` type is the `PAGE_TYPE` data structure, merely renamed
/// due to the collision with the Rust `type` keyword.
///
/// Section 38.11.2
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Class {
    /// Page is an SECS.
    Secs = 0,
    /// Page is a TCS.
    Tcs = 1,
    /// Page is a regular page.
    Reg = 2,
    /// Page is a Version Array.
    Va = 3,
    /// Page is in trimmed state.
    Trim = 4,
}

/// The security information (`SecInfo`) about a page
///
/// Note that the `FLAGS` field from the SGX documentation is here
/// divided into two fields (`flags` and `class`) for easy manipulation.
///
/// Section 38.11
#[derive(Copy, Clone, Debug)]
#[repr(C, align(64))]
pub struct SecInfo {
    /// Section 38.11.1
    pub flags: Flags,
    /// Section 38.11.2
    pub class: Class,
    reserved: [u16; 31],
}

impl AsRef<[u8]> for SecInfo {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of_val(self),
            )
        }
    }
}

impl SecInfo {
    /// Creates a SecInfo (page) of class type Regular.
    pub const fn reg(flags: Flags) -> Self {
        Self {
            flags,
            class: Class::Reg,
            reserved: [0; 31],
        }
    }

    /// Creates a SecInfo (page) of class type TCS.
    pub const fn tcs() -> Self {
        Self {
            flags: Flags::empty(),
            class: Class::Tcs,
            reserved: [0; 31],
        }
    }
}

testaso! {
    struct SecInfo: 64, 64 => {
        flags: 0,
        class: 1
    }
}
