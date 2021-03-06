// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

fn main() {
    println!("Hello, world!");
}

// TODO build a generation executable, autocxxbridge,
// which
// (1) Reads an existing .rs file to tokens
// (2) Finds include_cpp! macros and runs them through include_cpp
//     above to convert them to cxx::bridge sections
// (3) Calls cxx_gen::generate_header_and_cc(input) on the resultant
//     token stream.
// (4) Writes the output .cc and .h to files.
// Using the existing facilities within the IncludeCpp engine
// as far as possible.
