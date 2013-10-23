// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[feature(managed_boxes)];

struct foo {
    z: Option<@Invokable>,
}

struct Thing {
    w: @mut foo,
}

trait Invokable {
    fn f(&self);
}

impl Invokable for Thing {
    fn f(&self) {
        nop_foo(self.w);
    }
}

fn nop() { }
fn nop_foo(_x : @mut foo) { }

pub fn main() {
    let w = @mut foo {
        z: None,
    };
    let x = @Thing {
        w: w,
    } as @Invokable;
    w.z = Some(x);
}
