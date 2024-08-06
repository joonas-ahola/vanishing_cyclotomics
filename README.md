# vanishing_cyclotomics

Functions for calculating pth cyclotomic polynomials and 4p real cyclotomic polynomials over finite field F_q for some prime q.
Polynomials are evaluated at some integer value, and checked whether they vanish over F_q.

-------------------------------------------------------------------------------------------
fn round_1 = function for calculation pth cyclotomic polynomials at x over F_q

Inputs: 
- vec_pf_primes, a vector of primes q for F_q.
- ub: value for the upper bound for prime p.
- x: value at which cyclotomic polynomial is evaluated

-------------------------------------------------------------------------------------------
fn round_2 = function for 4p real cyclotomic polynomials at x over F_q

Inputs: 
- vec_pf_primes, a vector of primes q for F_q.
- ub: value for the upper bound for prime p.
- x: value at which cyclotomic polynomial is evaluated, the variable accepts only even values.
-------------------------------------------------------------------------------------------
