// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{TupleType, Type};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A future type consisting of the type of the inputs.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FutureType {
    // Optional type specification of inputs.
    pub inputs: Vec<Type>,
}

impl FutureType {
    /// Initialize a new future type.
    pub fn new(inputs: Vec<Type>) -> Self {
        Self { inputs }
    }
    /// Returns the inputs of the future type.
    pub fn inputs(&self) -> &[Type] {
        &self.inputs
    }
}

impl Default for FutureType {
    fn default() -> Self {
        Self { inputs: Vec::new() }
    }
}
impl fmt::Display for crate::FutureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Future<{}>", self.inputs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}