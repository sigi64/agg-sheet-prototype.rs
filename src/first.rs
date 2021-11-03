use core::marker::PhantomData;

use crate::{Aggregation, MyAgg, Zone};

pub struct Sheet<A, F> {
    _a: PhantomData<A>,
    _f: PhantomData<F>,
}

impl<A, E> Sheet<A, E>
where
    A: Aggregation,
    E: EventMapper,
    <E as EventMapper>::Type: Iterator<Item = <A as Aggregation>::Event>, // tie Aggregation with Transform
{
    pub fn new() -> Self {
        Self {
            _a: PhantomData,
            _f: PhantomData,
        }
    }

    pub fn process(&self, data: Vec<u32>) {
        let iter = E::new(data);
        let mut zone = Zone::<A>::new();

        A::update(&mut zone, iter);
    }
}

// Filter & Transform data from Vec<u32> to Aggregation::Event
pub trait EventMapper {
    type Type: Iterator;
    fn new(data: Vec<u32>) -> Self::Type;
}

pub struct TransformX {
    data: Vec<u32>,
    index: usize,
    // consider using iterator directly and to deal with lifetimes (pin?) -> inner: std::slice::Iter<'a, u32>,
}

impl EventMapper for TransformX {
    type Type = Self;

    fn new(data: Vec<u32>) -> Self::Type {
        Self { data, index: 0 }
    }
}

impl Iterator for TransformX {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.data.len() {
            // SAFETY: We just checked that index is smaller than len of data
            let val = unsafe { self.data.get_unchecked(self.index) };
            self.index += 1;

            // filter
            if (*val % 2) == 0 {
                // map
                return Some((*val, *val * 2));
            }
        }

        None

        // self.inner
        //     .filter(|i| (**i % 2) == 0)
        //     .map(|i| (*i, *i * 2))
        //     .next()
    }
}

pub type SheetX = Sheet<MyAgg, TransformX>;
