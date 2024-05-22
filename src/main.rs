use std::str::FromStr;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use malachite::Natural;
use malachite::num::arithmetic::traits::Pow;
use primal_sieve;

fn main() {
    round_1(vec![2539,2543,2549,2551,2557,2579,2591,2593,2609,2617,2621,2633,2647,2657,2659,2663], 47055833460, 2);
    round_1(vec![2671,2677,2683,2687,2689,2693,2699,2707,2711,2713,2719,2729,2731,2741,2749,2753], 47055833460, 2);
    round_1(vec![2767,2777,2789,2791,2797,2801,2803,2819,2833,2837,2843,2851,2857,2861,2879,2887], 47055833460, 2);
    round_1(vec![2897,2903,2909,2917,2927,2939,2953,2957,2963,2969,2971,2999,3001,3011,3019,3023], 47055833460, 2);
    round_1(vec![3037,3041,3049,3061,3067,3079,3083,3089,3109,3119,3121,3137,3163,3167,3169,3181], 47055833460, 2);
    round_1(vec![3187,3191,3203,3209,3217,3221,3229,3251,3253,3257,3259,3271,3299,3301,3307,3313], 47055833460, 2);
    round_1(vec![3319,3323,3329,3331,3343,3347,3359,3361,3371,3373,3389,3391,3407,3413,3433,3449], 47055833460, 2);
    round_1(vec![3457,3461,3463,3467,3469,3491,3499,3511,3517,3527,3529,3533,3539,3541,3547,3557], 47055833460, 2);
    round_1(vec![3559,3571,3581,3583,3593,3607,3613,3617,3623,3631,3637,3643,3659,3671,3673,3677], 47055833460, 2);
}

fn round_1(vec_of_primes: Vec<usize>, ub: usize, x: usize) -> () {

    // vec_of_primes = a list (or vector) of primes being evaluated.
    // ub = upper bound for mod q.
    // x = cyclotomic polynomial evaluated at x.

        vec_of_primes
        .into_par_iter()
        .for_each(|n: usize| {

            let mut s: isize = (n - 1) as isize;
            let mut value_0: Natural = Natural::from(0 as usize);

            while s >= 0 {
                let a: Natural = Natural::from(x).pow(s as u64);
                value_0 += &a;
                s -= 1;
            }

            let number_string = value_0.to_string();
            let q: usize = (n - 1)*(n - 1);
            let sieve = primal_sieve::Sieve::new(ub);

            sieve.primes_from(1)
            .skip_while(|f| *f < q)
            .enumerate()
            .for_each(|prime: (usize, usize)| {

            let value_1: Natural = Natural::from_str(&number_string)
            .unwrap();

            if prime.0 == 0 {
            println!("Calculations started for n = {}.", n);
            }
            else if prime.0 % 1000000000  == 0 {
            println!("Done 50% of the range for n = {}.", n);
            }

            let divisor = Natural::from(prime.1);
            if (value_1 % divisor) == Natural::from(0 as u32) {
                println!("Collision found for (p, q) = ({}, {}) at x = {}", prime.1, n, x);
            }
           });
        });
}

