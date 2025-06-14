/*
Copyright 2025 The Hyperlight Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
 */

// This, unlike the rest of hyperlight, isn't really a library (since
// it's only used by our own build-time tools), so the reasons not to
// panic don't really apply.
#![allow(clippy::unwrap_used)]
// "Needless" lifetimes are useful for clarity
#![allow(clippy::needless_lifetimes)]

// Typechecking and elaboration
pub mod component;
pub mod elaborate;
pub mod etypes;
pub mod structure;
pub mod substitute;
pub mod subtype;
pub mod tv;
pub mod wf;

// Generally useful for code emit
pub mod emit;
pub mod hl;
pub mod resource;
pub mod rtypes;
pub mod util;

// Specific code emit
pub mod guest;
pub mod host;
