use towel::prelude::{Either, Right};
use towel::data_structures::OptionT;

#[test]
fn option_t_init(){
    let x = OptionT::<Either<Option<i32>, Option<i32>>, i32>::new(3);
    let y = OptionT::<Either<Option<i32>, Option<i32>>, i32>::new_manual(Right(Some(3)));

    assert_eq!(x, y);
}
