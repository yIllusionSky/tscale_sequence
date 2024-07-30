//! This is a population model in a game.
//! Assume the world's population is divided into five age groups: 0-20, 20-40, 40-60, 60-80, and 80-100 years.
//! Each age group's population affects the birth rate,
//! with influence coefficients of 0.4, 1.2, 0.3, 0.1, and 0.0, respectively.
//! Each iteration represents a passage of 20 years.
//! After each iteration, the population in each age group shifts to the next older age group.
//!
//! Set the initial population to all ones.
//!
//! The goal is to obtain the newborn sequence and roughly evaluate the coefficient relationship between the number of newborns and the previous year's newborns.

use tscale_sequence::{tscale_rate::compute_limit_rate, tscale_sequence:: TScale};

fn main() {
    let start_people = [1.0, 1.0, 1.0, 1.0, 1.0];
    let weight = [0.4, 1.2, 0.3, 0.1, 0.0];
    let rate = compute_limit_rate(&weight);
    println!("limit rate:{rate:?}");

    let tscale = TScale::new_with_config(start_people, weight);

    let population=tscale
        .into_iter()
        .take(100)
        .last().unwrap();

    println!("the 50 round population is {population}");
    // if you want to know after 100 round population
    println!("the 100 round population is {}",rate.powf(50.0)*population);

}
