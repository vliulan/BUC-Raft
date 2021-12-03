// Copyright 2017 PingCAP, Inc.
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

use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::{error, io, str};
use tipb::expression::ScalarFuncSig;
use tipb::select;

use coprocessor::codec::mysql::Res;
use util;
use util::codec::Error as CError;

pub const ERR_UNKNOWN: i32 = 1105;
pub const ERR_TRUNCATED: i32 = 1265;
pub const ERR_UNKNOWN_TIMEZONE: i32 = 1298;
pub const ERR_DATA_OUT_OF_RANGE: i32 = 1690;
pub const ERR_TRUNCATE_WRONG_VALUE: i32 = 1292;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        Type { has: &'static str, expected: &'static str } {
            description("type error")
            display("type error: cannot get {:?} result from {:?} expression", expected, has)
        }
        Codec(err: util::codec::Error) {
            from()
            description("codec error")
            display("codec error: {}", err)
            cause(err)
        }
        ColumnOffset(offset: usize) {
            description("column offset not found")
            display("illegal column offset: {}", offset)
        }
        UnknownSignature(sig: ScalarFuncSig) {
            description("Unknown signature")
            display("Unknown signature: {:?}", sig)
        }
        Truncated(s:String) {
            description("Truncated")
            display("{}",s)
        }
        Eval(s: String,code:i32) {
            description("evaluation failed")
            display("{}", s)
        }
        Other(err: Box<error::Error + Send + Sync>) {
            from()
            cause(err.as_ref())
            description(err.description())
            display("unknown error {:?}", err)
        }
    }
}

impl Error {
    pub fn overflow(data: &str, expr: &str) -> Error {
        let msg = format!("{} value is out of range in {:?}", data, expr);
        Error::Eval(msg, ERR_DATA_OUT_OF_RANGE)
    }

    pub fn truncated_wrong_val(data_type: &str, val: &str) -> Error {
        let msg = format!("Truncated incorrect {} value: '{}'", data_type, val);
        Error::Eval(msg, ERR_TRUNCATE_WRONG_VALUE)
    }

    pub fn cast_neg_int_as_unsigned() -> Error {
        let msg = "Cast to unsigned converted negative integer to it's positive complement";
        Error::Eval(msg.into(), ERR_UNKNOWN)
    }

    pub fn cast_as_signed_overflow() -> Error {
        let msg =
            "Cast to signed converted positive out-of-range integer to it's negative complement";
        Error::Eval(msg.into(), ERR_UNKNOWN)
    }

    pub fn unknown_timezone(tz: i64) -> Error {
        let msg = format!("unknown or incorrect time zone: {}", tz);
        Error::Eval(msg, ERR_UNKNOWN_TIMEZONE)
    }

    pub fn code(&self) -> i32 {
        match *self {
            Error::Truncated(_) => ERR_TRUNCATED,
            Error::Eval(_, code) => code,
            _ => ERR_UNKNOWN,
        }
    }
}

impl Into<select::Error> for Error {
    fn into(self) -> select::Error {
        let mut err = select::Error::new();
        err.set_code(self.code());
        err.set_msg(format!("{:?}", self));
        err
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Codec(CError::Encoding(err.utf8_error()))
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Codec(CError::Encoding(err))
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl<T> Into<Result<T>> for Res<T> {
    fn into(self) -> Result<T> {
        match self {
            Res::Ok(t) => Ok(t),
            Res::Truncated(_) => Err(Error::Truncated("Data Truncated".into())),
            Res::Overflow(_) => Err(Error::overflow("", "")),
        }
    }
}
