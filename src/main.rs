use std::str::FromStr;
use malachite::Integer;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use malachite::num::arithmetic::traits::Pow;
use primal_sieve;
use mathru::algebra::abstr::Polynomial;

fn main() {
    round_1(vec![269,271,277,281,283,293,307,311,313,317,331,337,347,349,353,359], 47055833460, 8);
    round_1(vec![367,373,379,383,389,397,401,409, 419,421,431,433,439,443,449,457], 47055833460, 8);
    round_1(vec![461,463,467,479,487,491,499,503, 509,521,523,541,547,557,563,569], 47055833460, 8);
    round_1(vec![571,577,587,593,599,601,607,613, 617,619,631,641,643,647,653,659], 47055833460, 8);
    round_1(vec![661,673,677,683,691,701,709,719, 727,733,739,743,751,757,761,769], 47055833460, 8);
    round_1(vec![773,787,797,809,811,821,823,827, 829,839,853,857,859,863,877,881], 47055833460, 8);
    round_1(vec![883,887,907,911,919,929,937,941, 947,953,967,971,977,983,991,997], 47055833460, 8);
    round_1(vec![1009,1013,1019,1021,1031,1033, 1039,1049,1051,1061,1063,1069,1087,1091,1093,1097], 47055833460, 8);
    round_1(vec![1103,1109,1117,1123,1129,1151, 1153,1163,1171,1181,1187,1193,1201,1213,1217,1223], 47055833460, 8);
    round_1(vec![1229,1231,1237,1249,1259,1277, 1279,1283,1289,1291,1297,1301,1303,1307,1319,1321], 47055833460, 4);
    round_1(vec![1327,1361,1367,1373,1381,1399, 1409,1423,1427,1429,1433,1439,1447,1451,1453,1459], 47055833460, 4);
}

fn round_1(vec_of_primes: Vec<usize>, ub: usize, x: isize) -> () {

    // vec_of_primes = a list (or vector) of primes being evaluated.
    // ub = upper bound for mod q.
    // x = cyclotomic polynomial evaluated at x.

        vec_of_primes
        .into_par_iter()
        .for_each(|n: usize| {
            
            /* */

            let mut s: isize = (n - 1) as isize;
            let mut value_0: Integer = Integer::from(0 as usize);

            while s >= 0 {
                let a: Integer = Integer::from(x).pow(s as u64);
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

            let value_1: Integer = Integer::from_str(&number_string)
            .unwrap();

            if prime.0 == 0 {
            println!("Calculations started for n = {}.", n);
            }
            else if prime.0 % 1000000000  == 0 {
            println!("Done 50% of the range for n = {}.", n);
            }

            let divisor = Integer::from(prime.1);
            if (value_1 % divisor) == Integer::from(0 as u32) {
                println!("Collision found for (p, q) = ({}, {}) at x = {}", prime.1, n, x);
            }
        });
    });
}

fn round_2(vec_of_primes: Vec<usize>, ub: usize, x: isize) -> () {
    // vec_of_primes = a list (or vector) of primes being evaluated.
    // ub = upper bound for q, i.e. for the order of the field.
    // x = cyclotomic polynomial evaluated at x.

        vec_of_primes
        .into_par_iter()
        .for_each(|n: usize| {

            // Evaluate the polynomial.

            let p: usize = n;

            let chebyshev_t_4: isize = Polynomial::from_chebyshev_t(4).eval((x/2) as f64) as isize;
            let chebyshev_t_2: isize = Polynomial::from_chebyshev_t(2).eval((x/2) as f64) as isize;

            let p_div_2: u32 = ((p-1)/2).try_into().unwrap();

            let chebyshev_u_p_minus_1_pol: Polynomial<f64> = Polynomial::from_chebyshev_u(p_div_2 - 1);
            let chebyshev_u_p_pol: Polynomial<f64> = Polynomial::from_chebyshev_u(p_div_2);

            let final_pol: Polynomial<f64> = &chebyshev_u_p_minus_1_pol + &chebyshev_u_p_pol;

            let mut value_0: Integer = Integer::from(0 as usize);
            let mut value_1: Integer = Integer::from(0 as usize);

            // First, evaluate the quotient.

            let mut s_1: isize = final_pol.degree() as isize;
            let mut s_2: isize = final_pol.degree() as isize;

            let coeffs: Vec<f64> = final_pol.coef.clone();

            while s_1 >= 0 {
                let b: Integer = Integer::from(coeffs[s_1 as usize] as isize);
                let a: Integer = Integer::from(chebyshev_t_4);
                value_0 += &b * &a.pow(s_1 as u64);
                s_1 -= 1;
            }

            // Second, evaluate the dividend.

            while s_2 >= 0 {
                let b: Integer = Integer::from(coeffs[s_2 as usize] as isize);
                let a: Integer = Integer::from(chebyshev_t_2);
                value_1 += &b * &a.pow(s_2 as u64);
                s_2 -= 1;
            }
            
            // Proceed with calculations.

            let number_string: String = (&value_0 / &value_1).to_string();
            let q: usize = (n - 1)*(n - 1);
            let sieve = primal_sieve::Sieve::new(ub);

            sieve.primes_from(1)
            .skip_while(|f| *f < q)
            .enumerate()
            .for_each(|prime: (usize, usize)| {

            let value_1: Integer = Integer::from_str(&number_string)
            .unwrap();

            if prime.0 == 0 {
            println!("Calculations started for n = {}.", 4*n);
            }
            else if prime.0 % 1000000000  == 0 {
            println!("Done 50% of the range for n = {}.", 4*n);
            }

            let divisor = Integer::from(prime.1);
            if (value_1 % divisor) == Integer::from(0 as u32) {
                println!("Collision found for (p, q) = ({}, {}) at x = {}", prime.1, 4*n, x);
            }
        });
    });
}
