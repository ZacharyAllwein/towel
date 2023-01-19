use towel::prelude::*;

#[test]
fn state_functor() {
    let s1 = State::new(Box::new(|_| ((), 1)));

    //s1 is now borrowed;
    let s2 = s1.fmap(|_| 1);

    assert_eq!(s2.eval(0), (1, 1))
}

#[test]
fn state_applicative() {
    let s1 = <State<'static, Vec<i32>, i32> as Applicative<i32>>::pure(1);
    let s2: State<'static, Vec<i32>, fn(&i32) -> i32> = State::new(Box::new(|s| (|&x| x + 1, s)));
    let s3 = s1.app(&s2);

    assert_eq!(s3.eval(vec![1]), (2, vec![1]));
}

#[test]
fn state_monad() {
    let s1 = <State<'static, Vec<i32>, i32> as Monad<i32>>::ret(1);

    let s2 = s1.bind(|&x| {
        State::new(Box::new(move |mut s| {
            let a = x + 1;
            s.push(a);
            (a, s)
        }))
    });

    assert_eq!(s2.eval(vec![1]), (2, vec![1, 2]))
}
