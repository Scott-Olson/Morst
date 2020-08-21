extern crate beep;
extern crate dimensioned;

use beep::beep;
use dimensioned::si;
// use std::{thread, time};

fn main() {
    beep(440. * si::HZ);
}
