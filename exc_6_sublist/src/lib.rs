#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
// Community Solution  i found to way better than my
// pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
// use Comparison::*;
// match (a.len(), b.len()) {
//     (0, 0) => Equal,
//     (0, _) => Sublist,
//     (_, 0) => Superlist,
//     (m, n) if m > n => if a.windows(n).any(|v| v == b) {Superlist} else {Unequal},
//     (m, n) if m < n => if b.windows(m).any(|v| v == a) {Sublist} else {Unequal},
//     (_, _) => if a == b {Equal} else {Unequal},
//     by John8790909's solution
// }

// My solution  I know its worst i kinnda applied the same logic but my code is not clean in any
// way
enum Bigger {
    A,
    B,
    Equal,
}

struct sublist;

use Comparison::{Equal, Sublist, Superlist, Unequal};
impl sublist {
    pub fn sublist<T: PartialEq + Ord>(_first_list: &[T], _second_list: &[T]) -> Comparison {
        let first_list_len = _first_list.len();
        let _second_list_len = _second_list.len();
        if first_list_len == 0 || _second_list_len == 0 {
            if first_list_len == _second_list_len {
                return Equal;
            } else if _second_list_len > first_list_len {
                return Sublist;
            } else {
                return Superlist;
            }
        }

        let bigger_list: &[T];
        let small_list: &[T];
        let bigger;
        if first_list_len > _second_list_len {
            bigger_list = _first_list;
            small_list = _second_list;
            bigger = Bigger::A;
        } else if first_list_len < _second_list_len {
            bigger_list = _second_list;
            small_list = _first_list;
            bigger = Bigger::B;
        } else {
            bigger_list = _second_list;
            small_list = _first_list;
            bigger = Bigger::Equal;
        }

        let small_list_len = small_list.len();
        // let bigger_list_len = bigger_list.len();

        let windows: Vec<_> = bigger_list.windows(small_list_len).collect();
        for window in windows {
            if window == small_list {
                match bigger {
                    Bigger::A => {
                        return Superlist;
                    }
                    Bigger::B => {
                        return Sublist;
                    }
                    Bigger::Equal => {
                        return Equal;
                    }
                }
            }
        }
        return Unequal;
    }
}
