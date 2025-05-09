use interoptopus::{function, Inventory, InventoryBuilder};

use crate::{getCounterPositions, createCounter, decrementCounter, decrementCounterBy, decrementCounterByMany, destroyCounter, getCounterData, getCounterValue, incrementCounter, incrementCounterBy, incrementCounterByMany};

pub fn build_binding_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(createCounter))
        .register(function!(getCounterData))
        .register(function!(getCounterValue))
        .register(function!(incrementCounter))
        .register(function!(decrementCounter))
        .register(function!(incrementCounterBy))
        .register(function!(decrementCounterBy))
        .register(function!(incrementCounterByMany))
        .register(function!(decrementCounterByMany))
        .register(function!(destroyCounter))
        .register(function!(getCounterPositions))
        .inventory()
}
