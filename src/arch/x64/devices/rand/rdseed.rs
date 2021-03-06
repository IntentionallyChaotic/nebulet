// Copyright © 2014, Simonas Kazlauskas <rdseed@kazlauskas.me>
//
// Permission to use, copy, modify, and/or distribute this software for any purpose with or without
// fee is hereby granted, provided that the above copyright notice and this permission notice
// appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
// SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
// AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
// NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.

//! Expose `rdseed` instructions.

use raw_cpuid::CpuId;
use rand_core;


extern "platform-intrinsic" {
    fn x86_rdseed32_step() -> (u32, i32);
    fn x86_rdseed64_step() -> (u64, i32);
}

macro_rules! loop_rand {
    ($f:ident) => {
        loop {
            let (val, succ) = ($f)();
            if succ != 0 { return val; }
        }
    }
}

fn has_rdseed() -> bool {
    // https://github.com/rust-lang-nursery/stdsimd/issues/464
    //core::is_x86_feature_detected!("rdseed")

    CpuId::new()
        .get_extended_feature_info()
        .map_or(false, |v| v.has_rdseet())
}

#[target_feature(enable = "rdseed")]
unsafe fn rdseed_next_u32() -> u32 {
    loop_rand!(x86_rdseed32_step);
}

#[target_feature(enable = "rdseed")]
unsafe fn rdseed_next_u64() -> u64 {
    loop_rand!(x86_rdseed64_step);
}


#[derive(Debug)]
pub struct RdSeed(());

impl RdSeed {
    pub fn new() -> Option<RdSeed> {
        if has_rdseed() {
            Some(RdSeed(()))
        } else {
            None
        }
    }
}

impl rand_core::RngCore for RdSeed {
    fn next_u32(&mut self) -> u32 {
        unsafe {
            rdseed_next_u32()
        }
    }
    fn next_u64(&mut self) -> u64 {
        unsafe {
            rdseed_next_u64()
        }
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

