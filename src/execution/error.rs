// Copyright 2018 Grove Enterprises LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Error types

use std::io::Error;
use std::result;

use arrow::error::ArrowError;

use sqlparser::sqlparser::ParserError;

pub type Result<T> = result::Result<T, ExecutionError>;

#[derive(Debug)]
pub enum ExecutionError {
    IoError(Error),
    ParserError(ParserError),
    General(String),
    InvalidColumn(String),
    NotImplemented(String),
    InternalError(String),
    ArrowError(ArrowError),
    ExecutionError(String),
}

impl From<Error> for ExecutionError {
    fn from(e: Error) -> Self {
        ExecutionError::IoError(e)
    }
}

impl From<String> for ExecutionError {
    fn from(e: String) -> Self {
        ExecutionError::General(e)
    }
}

impl From<&'static str> for ExecutionError {
    fn from(e: &'static str) -> Self {
        ExecutionError::General(e.to_string())
    }
}

impl From<ArrowError> for ExecutionError {
    fn from(e: ArrowError) -> Self {
        ExecutionError::ArrowError(e)
    }
}

impl From<ParserError> for ExecutionError {
    fn from(e: ParserError) -> Self {
        ExecutionError::ParserError(e)
    }
}
