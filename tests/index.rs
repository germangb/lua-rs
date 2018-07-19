extern crate lua;

use lua::Index;

#[test]
fn test_from_absolute() {
    let from_abs_neg = Index::from_absolute(-4);
    let from_abs_pos = Index::from_absolute(2);

    assert_eq!(Index::Top(4), from_abs_neg);
    assert_eq!(Index::Bottom(2), from_abs_pos);
}

#[test]
fn test_to_absolute() {
    let top = Index::Top(4);
    let bottom = Index::Bottom(4);

    assert_eq!(-4, top.as_absolute());
    assert_eq!(4, bottom.as_absolute());
}
