#![feature(test, bench_black_box)]
extern crate test;

use keypath::Keyable;
use keypath2::{
    dyn_trait_based,
    lens_but_dyn::{self, Lens},
};

use std::hint::black_box;

use test::Bencher;

#[derive(Keyable)]
struct Foo {
    x: i32,
}

#[bench]
fn dyn_trait_based_get(b: &mut Bencher) {
    let path = black_box(dyn_trait_based::keypath!(Foo::x));
    let foo = black_box(Foo { x: 234 });
    b.iter(move || *path.get(&foo))
}

#[bench]
fn path_based_get(b: &mut Bencher) {
    let path = black_box(keypath::keypath!(Foo.x));
    let foo = black_box(Foo { x: 234 });
    b.iter(move || foo[&path])
}

#[bench]
fn lens_but_dyn_get(b: &mut Bencher) {
    let lens = black_box(&lens_but_dyn::lens!(Foo::x) as &'static dyn Lens<Foo, i32>);
    let foo = black_box(Foo { x: 234 });
    b.iter(move || lens.with(&foo, |x| *x))
}

#[bench]
fn dyn_trait_based_put(b: &mut Bencher) {
    let path = black_box(dyn_trait_based::keypath!(Foo::x));
    let mut foo = black_box(Foo { x: 234 });
    b.iter(move || path.put(&mut foo, 452))
}

#[bench]
fn path_based_put(b: &mut Bencher) {
    let path = black_box(keypath::keypath!(Foo.x));
    let mut foo = black_box(Foo { x: 234 });
    b.iter(move || foo[&path] = 452)
}

#[bench]
fn lens_but_dyn_put(b: &mut Bencher) {
    let lens = black_box(&lens_but_dyn::lens!(Foo::x) as &'static dyn Lens<Foo, i32>);
    let mut foo = black_box(Foo { x: 234 });
    b.iter(move || lens.with_mut(&mut foo, |x| *x = 452))
}
