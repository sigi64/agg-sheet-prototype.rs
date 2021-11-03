use std::{fmt::Debug, marker::PhantomData};

use crate::first::SheetX;

mod first;
mod second;

pub trait Aggregation {
    /// Type of data Aggregation accepts to update zone
    type Event: Debug;

    /// We require Iterator to item to avoid large memory allocation
    /// during filtering and transformation of Vec<u32> to Event type
    fn update<I>(zone: &mut Zone<Self>, items: I)
    where
        I: Iterator<Item = Self::Event>;
}

pub struct MyAgg;

impl Aggregation for MyAgg {
    type Event = (u32, u32);

    fn update<I>(zone: &mut Zone<Self>, mut items: I)
    where
        I: Iterator<Item = Self::Event>,
    {
        while let Some(item) = items.next() {
            // update zone e.g.
            zone.set(item);
            // println!("Item: {:?}", item);
        }
    }
}

// Container for data
pub struct Zone<A>
where
    A: Aggregation + ?Sized,
{
    _a: PhantomData<A>,
}

impl<A> Zone<A>
where
    A: Aggregation,
{
    pub fn new() -> Self {
        Self { _a: PhantomData }
    }

    pub fn set(&mut self, item: <A as Aggregation>::Event) {
        println!("Set {:?}", item);
    }
}

fn transform_x(data: Vec<u32>) -> impl Iterator<Item = (u32, u32)> {
    data.into_iter()
        .filter(|i| (*i % 2) == 0)
        .map(|i| (i, i * 2))
}

fn main() {
    // What I want to achieve in Sheet::process is
    let data = vec![1, 2, 3];
    let iter = transform_x(data);
    let mut zone = Zone::<MyAgg>::new();
    MyAgg::update(&mut zone, iter);

    // -- Requires implement Iterator and use Agg directly to update Zone
    let data = vec![1, 2, 3];
    let c = SheetX::new();
    c.process(data);

    // Use indirection in EventMapper to Agg to update Zone. More generic
    let data = vec![1, 2, 3];
    let c = second::SheetX::new();
    c.process(data);
}
