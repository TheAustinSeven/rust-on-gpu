// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{Target, TargetOptions};

pub fn target() -> Target {
    let opts = TargetOptions {
        linker: "".to_string(),
        ar: "".to_string(),
        cpu: "fiji".to_string(),

        dynamic_linking: false,
        executables: false,
        no_compiler_rt: true,
        allow_asm: false,
        .. Default::default()
    };
    Target {
        llvm_target: "amdgcn-unknown-unknown".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_os: "none".to_string(),
        target_env: "".to_string(),
        target_vendor: "unknown".to_string(),
        data_layout: "e-p:32:32-p1:64:64-p2:64:64-p3:32:32-p4:64:64-p5:32:32-p24:\
                      64:64-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:\
                      256-v512:512-v1024:1024-v2048:2048-n32:64".to_string(),
        arch: "amdgcn".to_string(),
        options: opts,
    }
}