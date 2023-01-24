/// id returns the same value that was passed into it
/// unchanged
pub fn identity<A>(a: A) -> A {
    a
}

/// constant always returns the first value passed into it
pub fn constant<A, B>(a: A, _: B) -> A {
    a
}

/// takes a fn and a value and applies the fn to value
pub fn apply<F: FnOnce(A) -> B, A, B>(f: F, a: A) -> B {
    f(a)
}

/// takes a value and a fn and applies fn to value
pub fn thrush<A, F: FnOnce(A) -> B, B>(a: A, f: F) -> B {
    f(a)
}

/// passes one arg to a fn that takes two args twice
pub fn duplication<F: Fn(A, A) -> B, A: Clone, B>(f: F) -> impl Fn(A) -> B {
    move |a| f(a.clone(), a)
}

/// flips arguments for a fn around
pub fn flip<F: Fn(A, B) -> C, A, B, C>(f: F) -> impl Fn(B, A) -> C {
    move |b, a| f(a, b)
}

/// composes two function one which takes the output of the other
/// creating a new fn
pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |a| f(g(a))
}

/// like composition but less similar to the mathematical definition
/// works like unix |, passes output to input of next fn
pub fn pipe<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a| g(f(a))
}


/// takes a function with two args and allows the second arg to be
/// substituted with a function application on the first arg
pub fn substitution<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    A: Clone,
    F: Fn(A, B) -> C,
    G: Fn(A) -> B,
{
    move |a| f(a.clone(), g(a))
}

/// allows composition where
/// you need to have some constant environment(A)
/// in a function
pub fn chain<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    A: Clone,
    F: Fn(A) -> B,
    G: Fn(B, A) -> C,
{
    move |a| g(f(a.clone()), a)
}

/// takes two functions that take same arg, but have different outputs
/// and a function that combines their outputs,
/// and returns a fn that takes one input and returns the combined output
pub fn converge<F, G, H, A, B, C, D>(f: F, g: G, h: H) -> impl Fn(A) -> D
where
    A: Clone,
    F: Fn(B, C) -> D,
    G: Fn(A) -> B,
    H: Fn(A) -> C,
{
    move |a| f(g(a.clone()), h(a))
}

/// a sister to converge. returns a fn that takes two args runs them through a
/// unary fn, then merges their outputs with a provided binary fn
pub fn psi<F, G, A, B, C>(f: F, g: G) -> impl Fn(A, A) -> C
where
    F: Fn(B, B) -> C,
    G: Fn(A) -> B,
{
    move |a, b| f(g(a), g(b))
}
