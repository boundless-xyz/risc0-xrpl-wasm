// Copyright 2026 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::groth16::bn254::{G1Point, Pair, Scalar};

pub(crate) fn add_g1(_p1: G1Point, _p2: G1Point) -> G1Point {
    todo!()
}

pub(crate) fn mult_g1(_p1: G1Point, _scalar: Scalar) -> G1Point {
    todo!()
}

/// Evaluates prod [ e(pairs[i].0, pairs[i].1) == 1 ]
pub(crate) fn pairing_check(_pairs: &[Pair]) -> bool {
    todo!()
}
