// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
#![deny(warnings)]
deno_ops_compile_test_runner::prelude!();

#[op2(fast)]
fn op_add(a: u32, b: u32) -> u32 {
  a + b
}
