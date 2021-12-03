// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

#![crate_type = "lib"]
#![cfg_attr(test, feature(test))]
#![feature(fnbox)]
#![feature(alloc)]
#![feature(slice_patterns)]
#![feature(box_syntax)]
#![feature(integer_atomics)]
#![feature(entry_or_default)]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(not(feature = "dev"), allow(unknown_lints))]
#![recursion_limit = "200"]
#![feature(ascii_ctype)]
#![allow(module_inception)]
#![allow(should_implement_trait)]
#![allow(large_enum_variant)]
#![allow(needless_pass_by_value)]
#![allow(unreadable_literal)]
#![allow(new_without_default_derive)]
#![allow(verbose_bit_mask)]
#![allow(implicit_hasher)]
// Currently this raises some false positives, so we allow it:
// https://github.com/rust-lang-nursery/rust-clippy/issues/2638
#![allow(nonminimal_bool)]

extern crate alloc;
extern crate backtrace;
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate chrono;
extern crate crc;
extern crate crossbeam_channel;
#[macro_use]
extern crate fail;
extern crate flat_map;
extern crate fnv;
extern crate fs2;
extern crate futures;
extern crate futures_cpupool;
extern crate grpcio as grpc;
extern crate indexmap;
extern crate kvproto;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate mio;
extern crate murmur3;
extern crate num;
#[macro_use]
extern crate prometheus;
extern crate protobuf;
#[macro_use]
extern crate quick_error;
extern crate raft;
extern crate rand;
extern crate regex;
extern crate rocksdb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sys_info;
extern crate tempdir;
#[cfg(test)]
extern crate test;
extern crate time;
extern crate tipb;
extern crate tokio_core;
extern crate tokio_timer;
#[cfg(test)]
extern crate toml;
extern crate url;
#[cfg(test)]
extern crate utime;
extern crate uuid;
extern crate zipf;

#[macro_use]
pub mod util;
pub mod config;
pub mod coprocessor;
pub mod import;
pub mod pd;
pub mod raftstore;
pub mod server;
pub mod storage;

pub use storage::Storage;
