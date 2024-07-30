//! # A simple recursive iterator over a fixed-size array.
//! 
//! Define a new sequence, called t-scale-sequence. It measures the relationship between a number of continuous factors. Below is an example of it.
//! 
//! > Assume that the scale t represents the year and a represents the population born in a certain year. From the current t we can infer the age group of a.
//! >
//! > Suppose that the impact of each age group on the birth population is a fixed coefficient. For example, the impact coefficient of a 32-year-old person on the birth population in the next year is 0.1. Assume that there are 100 32-year-olds now, then next year The birth population will be affected to 10 more people.
//! >
//! > Through the t-scale sequence, we can use the influence coefficient to roughly evaluate the relationship between the birth rate of the previous year and the birth rate of the next year.
//! 
//! 
//! # Example
//! 
//! if you want to get fibonacci sequence, you can use tscale_sequence
//! 
//! ```rust
//! # use tscale_sequence::tscale_sequence::compute_rate_with_data;
//! let array = [0., 1.0];
//! let weight = [1., 1.];
//! let scale=compute_rate_with_data(50,array, weight);
//!
//! // generate 50 values,you must use take to limit the number of values
//! // if you don't use take, it will generate a big array
//! scale.into_iter().take(50).for_each(|v| println!("{}",v));
//! ```
//! 
//! if you want to get the limit rate, you can use tscale_rate
//!
//! ```rust
//! # use tscale_sequence::tscale_rate::compute_limit_rate;
//! let weight = [0.5,0.6,0.7];
//! let rate=compute_limit_rate(&weight);
//! println!("{}",rate);
//! ```
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(clippy::all, clippy::nursery)]

pub mod tscale_sequence;
pub mod tscale_rate;