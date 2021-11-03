use std::marker::PhantomData;

use crate::{Aggregation, MyAgg, Zone};

pub struct Sheet<E> {
    _f: PhantomData<E>,
}

impl<E> Sheet<E>
where
    E: EventMapper,
{
    pub fn new() -> Self {
        Self { _f: PhantomData }
    }

    pub fn process(&self, data: Vec<u32>) {
        let mut zone = Zone::<<E as EventMapper>::A>::new();
        E::update(&mut zone, data);
    }
}

pub trait EventMapper {
    type A: Aggregation;

    /// Transforms `data` and updates zone
    fn update(zone: &mut Zone<Self::A>, data: Vec<u32>);
}

pub struct MyEventMapper<A> {
    _a: PhantomData<A>,
}

impl<A> EventMapper for MyEventMapper<A>
where
    A: Aggregation<Event = (u32, u32)>,
{
    type A = A;
    fn update(zone: &mut Zone<Self::A>, data: Vec<u32>) {
        let iter = data
            .into_iter()
            .filter(|i| (*i % 2) == 0)
            .map(|i| (i, i * 2));

        A::update(zone, iter);
    }
}

pub type SheetX = Sheet<MyEventMapper<MyAgg>>;
