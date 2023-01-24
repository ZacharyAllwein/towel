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
pub fn duplication<F: FnOnce(A, A) -> B, A: Clone, B>(f: F, a: A) -> B {
    f(a.clone(), a)
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
