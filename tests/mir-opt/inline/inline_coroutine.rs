// skip-filecheck
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// compile-flags: -Zinline-mir-hint-threshold=1000
#![feature(coroutines, coroutine_trait)]

use std::ops::Coroutine;
use std::pin::Pin;

// EMIT_MIR inline_coroutine.main.Inline.diff
fn main() {
    let _r = Pin::new(&mut g()).resume(false);
}

#[inline]
pub fn g() -> impl Coroutine<bool> {
    #[inline]
    |a| { yield if a { 7 } else { 13 } }
}
