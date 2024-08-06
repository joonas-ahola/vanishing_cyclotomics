use std::time::Instant;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;
use malachite::Integer;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use malachite::num::arithmetic::traits::Pow;
use primal_sieve;

#[derive(Clone)]
pub struct Polynomial {
    pub coef: Vec<Integer>,
}

impl Polynomial {
    fn chebyshev_t(n: u32) -> Polynomial {
        match n {
            0 => Polynomial {
                coef: vec![Integer::from(1 as usize)],
            },
            1 => Polynomial {
                coef: vec![Integer::from(0 as usize), Integer::from(1 as usize)],
            },
            _ => {
                let p_1: Polynomial = Polynomial {
                    coef: vec![Integer::from(0 as usize), Integer::from(2 as usize)]
                };
                p_1 * Polynomial::chebyshev_t(n - 1) - Polynomial::chebyshev_t(n - 2)
            }
        }
    }
    fn chebyshev_u(n: usize) -> Polynomial {
        let _n: usize = 
        if n < 4 
           {4} 
        else {n};

        let mut table: Vec<Polynomial> = vec![Polynomial{coef: vec![Integer::from(10 as usize)]}; (_n + 1) as usize];
        let u_2_m_1: Polynomial = Polynomial{coef: vec![Integer::from(-2 as isize), Integer::from(0 as usize), Integer::from(4 as usize)]};

        table[0] = Polynomial{coef: vec![Integer::from(1 as usize)]};
        table[1] = Polynomial{coef: vec![Integer::from(0 as usize), Integer::from(2 as usize)]};
        table[2] = Polynomial{coef: vec![Integer::from(-1 as isize), Integer::from(0 as usize), Integer::from(4 as usize)]};
        table[3] = Polynomial{coef: vec![Integer::from(0 as isize), Integer::from(-4 as isize), Integer::from(0 as usize), Integer::from(8 as usize)]};
        table[4] = Polynomial{coef: vec![Integer::from(1 as isize), Integer::from(0 as usize), Integer::from(-12 as isize), Integer::from(0 as usize), Integer::from(16 as usize)]};

        if n >= 5 {
            for k in 5..=n {
                table[k] = table[k - 2].clone()*u_2_m_1.clone() - table[k - 4].clone();
            }
        }

        table[n].clone()

    }
    fn eval(self, x: Integer) -> Integer {
        let mut s: isize = self.degree() as isize;
        let mut value: Integer = Integer::from(0 as usize);

        while s >= 0 {
            let _x: Integer = x.clone();
            let b: Integer = self.coef[s as usize].clone();
            value += b * _x.pow(s as u64);
            s -= 1;
        }

        value
    }
    fn degree(&self) -> usize {
        self.coef.len() - 1
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut sum: Vec<Integer> = if self.coef.len() > rhs.coef.len() {
            self.coef.clone()
        } else {
            rhs.coef.clone()
        };

        
        for (i, (a_i, b_i)) in self.coef.iter().zip(rhs.coef.iter()).enumerate() {
            sum[i] = a_i + b_i;
        } 

        Polynomial { coef: sum }
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut diff: Vec<Integer> = 
        if self.coef.len() > rhs.coef.len() {
            self.coef.clone()
        } else {
            rhs.coef
            .clone()
            .into_iter()
            .map(|x| -x)
            .collect()
        };

        for (i, (a_i, b_i)) in self.coef.iter().zip(rhs.coef.iter()).enumerate() {
            diff[i] = a_i - b_i;
        }
        Polynomial { coef: diff }
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let deg_lhs: usize = self.degree();
        let deg_rhs: usize = rhs.degree();

        let deg_res: usize = deg_lhs + deg_rhs;
        let mut res: Vec<Integer> = vec![Integer::from(0 as usize); deg_res + 1];

        for (i, v_i) in self.coef.iter().enumerate() {
            for (j, v_j) in rhs.coef.iter().enumerate() {
                res[i + j] += v_i.clone() * v_j.clone();
            }
        }

        Polynomial { coef: res }
    }
}

impl Div for Polynomial {
    type Output = Self;

    fn div(self, rhs: Polynomial) -> Self {
        if rhs.degree() > self.degree() {
            return Polynomial{coef: vec![Integer::from(0 as usize)]}
        }

        let mut remainder: Vec<Integer> = self.coef.clone();
        let quotient_degree: usize = self.degree() - rhs.degree();
        let mut quotient = vec![Integer::from(0 as usize); quotient_degree + 1];

        for i in (0..(quotient_degree + 1)).rev() {
            let q: Integer = remainder[rhs.degree() + i].clone() / rhs.coef[rhs.degree()].clone();

            quotient[i] = q.clone();

            for (k, v_k) in rhs.coef.iter().enumerate() {
                remainder[k + i] -= v_k.clone() * q.clone();
            }
        }

        Polynomial { coef: quotient }
    }
}



fn main() {
    round_2(vec![269,271,277,281,283,293,307,311, 313,317,331,337,347,349,353,359], 47055833460, 4);
    round_2(vec![367,373,379,383,389,397,401,409, 419,421,431,433,439,443,449,457], 47055833460, 4);
    round_2(vec![461,463,467,479,487,491,499,503, 509,521,523,541,547,557,563,569], 47055833460, 4);
    round_2(vec![571,577,587,593,599,601,607,613, 617,619,631,641,643,647,653,659], 47055833460, 4);
    round_2(vec![661,673,677,683,691,701,709,719, 727,733,739,743,751,757,761,769], 47055833460, 4);
    round_2(vec![773,787,797,809,811,821,823,827, 829,839,853,857,859,863,877,881], 47055833460, 4);
    round_2(vec![883,887,907,911,919,929,937,941, 947,953,967,971,977,983,991,997], 47055833460, 4);
    round_2(vec![1009,1013,1019,1021,1031,1033, 1039,1049,1051,1061,1063,1069,1087,1091,1093,1097], 47055833460, 4);
    round_2(vec![1103,1109,1117,1123,1129,1151, 1153,1163,1171,1181,1187,1193,1201,1213,1217,1223], 47055833460, 4);
    round_2(vec![1229,1231,1237,1249,1259,1277, 1279,1283,1289,1291,1297,1301,1303,1307,1319,1321], 47055833460, 4);
    round_2(vec![1327,1361,1367,1373,1381,1399, 1409,1423,1427,1429,1433,1439,1447,1451,1453,1459], 47055833460, 4);
}

fn round_1(vec_of_primes: Vec<usize>, ub: usize, x: isize) -> () {

    // vec_of_primes = a list (or vector) of primes being evaluated.
    // ub = upper bound for mod q.
    // x = integer value at which poylomial is evaluated.

        vec_of_primes
        .into_par_iter()
        .for_each(|n: usize| {

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
    let now = Instant::now();
    
    // vec_of_primes = a list (or vector) of primes being evaluated.
    // ub = upper bound for mod q.
    // x = integer value at which poylomial is evaluated.

        vec_of_primes
        .into_par_iter()
        .for_each(|n: usize| {

            // Evaluate the polynomial.
            let x_1: Integer = Integer::from(x/2);
            let x_2: Integer = x_1.clone();

            let p: usize = n;

            let chebyshev_t_2_pol: Polynomial = Polynomial::chebyshev_t(2);
            let chebyshev_t_4_pol: Polynomial = Polynomial::chebyshev_t(4);

            let chebyshev_t_2: Integer = chebyshev_t_2_pol.eval(x_1);
            let chebyshev_t_4: Integer = chebyshev_t_4_pol.eval(x_2);

            let p_div_2: usize = ((p-1)/2).try_into().unwrap();

            let chebyshev_u_p_minus_1_pol: Polynomial = Polynomial::chebyshev_u(p_div_2 - 1);
            let chebyshev_u_p_pol: Polynomial = Polynomial::chebyshev_u(p_div_2);

            let final_pol: Polynomial = chebyshev_u_p_minus_1_pol + chebyshev_u_p_pol;

            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);

            // Evaluate the divisor, and divided.

            let final_pol_clone: Polynomial = Polynomial{ coef: final_pol.coef.clone() };

            let value_0: Integer = final_pol.eval(chebyshev_t_4);
            let value_1: Integer = final_pol_clone.eval(chebyshev_t_2);
            
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
