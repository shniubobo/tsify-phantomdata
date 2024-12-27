use std::marker::PhantomData;

use serde::Serialize;
use tsify_next::Tsify;

#[derive(Debug, Serialize, Tsify)]
pub struct Foo<T> {
    bar: PhantomData<T>,
}
