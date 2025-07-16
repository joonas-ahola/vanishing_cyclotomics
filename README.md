# vanishing_cyclotomics

Functions for calculating p-th cyclotomic and 4p-th real cyclotomic polynomials.
Polynomials are evaluated at some integer value, and checked whether the evaluation vanishes in F_q.

-------------------------------------------------------------------------------------------
fn round_1 = function for evaluating pth cyclotomic polynomials at x in F_q.

Inputs: 
- vec_of_primes: a vector of primes for p.
- ub: upper bound of q.
- x: value at which cyclotomic polynomial is evaluated
-------------------------------------------------------------------------------------------
fn round_2 = function for evaluating 4p real cyclotomic polynomials at even x in F_q.

Inputs: 
- vec_of_primes: a vector of primes p for 4p.
- ub: upper bound of q.
- x: value at which cyclotomic polynomial is evaluated.
-------------------------------------------------------------------------------------------
fn round_3 = function for evaluating 4p real cyclotomic polynomials at all integer x in F_q.

Inputs: 
- vec_of_primes: a vector of primes p for 4p.
- ub: upper bound of q.
- x: value at which cyclotomic polynomial is evaluated.
-------------------------------------------------------------------------------------------

