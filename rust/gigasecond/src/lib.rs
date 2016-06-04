extern crate chrono;

use std::ops::Add;

pub fn after(start: chrono::DateTime<chrono::UTC>) -> chrono::DateTime<chrono::UTC> {
    start.add(chrono::Duration::seconds(1_000_000_000))
}