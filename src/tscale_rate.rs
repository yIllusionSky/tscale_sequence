//! Used to calculate the ratio as time t approaches positive infinity.

use approximately::ApproxEq;

/// rank must be greater than 1
/// 
/// To handle the case where β = 1
/// 
/// The function is defined as:
/// 
/// at = a{t-1} + a{t-2} + ... + a{t-rank}
pub fn compute_limit_normal_rate(rank: usize) -> f64 {
    if rank == 0 {
        panic!("rank must be greater than 1");
    }
    let rank = rank as f64;
    let fx = |x: f64| x + x.powf(-rank) - 2.0;
    let dfdx = |x: f64| rank.mul_add(-x.powf(-rank - 1.0), 1.0);
    let mut x = 2.0;
    loop {
        let x1 = x - fx(x) / dfdx(x);
        if x1.approx(x){
            return x1;
        }
        x = x1;
    }
}

/// The function is defined as:
/// 
/// at = β1*a{t-1} + β2*a{t-2} + ... + βn*a{t-n}
pub fn compute_limit_rate(beta: &[f64]) -> f64 {
    if beta.is_empty() {
        panic!("less than 1 beta is not allowed");
    }
    let beta_iter = beta
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, beta)| (i as f64, *beta));
    let fx = |x: f64| {
        beta_iter
            .clone()
            .fold(beta[0] - x, |sum, (i, beta)| beta.mul_add(x.powf(-i), sum))
    };
    let dfdx = |x: f64| {
        beta_iter.clone().fold(-1.0, |sum, (i, beta)| {
            (-beta * i).mul_add(x.powf(-i - 1.0), sum)
        })
    };
    let mut x = 1.;
    loop {
        let x1 = x - fx(x) / dfdx(x);

        if x1.approx(x) {
            return x1;
        }
        x = x1;
    }
}

#[cfg(test)]
mod tests {
    use crate::tscale_sequence::compute_rate_with_data;

    use super::*;

    fn test_rate<const C:usize>(array:[f64;C],weight:[f64;C]){
        compute_limit_rate(&weight).assert_approx(
            compute_rate_with_data(50,array,weight).last().unwrap()
        );
    }
    #[test]
    fn test_all_rate(){
        // Fibonacci sequence
        let array=[0.0, 1.0];
        let weight = [1.0, 1.0];

        test_rate(array,weight);

        // more
        let array=[[1.0, 1.0, 1.0],[1.0,1.0,1.0],[1.0,1.0,1.0]];
        let weight=[[1.5, 1.0, 1.4],[2.3,2.8,1.0],[1.,1.6,1.5]];

        for (array,weight) in array.iter().zip(weight.iter()){
            test_rate(*array,*weight);
        }
    }
}
