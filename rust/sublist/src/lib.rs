#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    let (list_b, list_a) = if _first_list.len() >= _second_list.len() {
        (_first_list, _second_list)
    } else {
        (_second_list, _first_list)
    };


    // a is the possibly shorter list

    for a in list_a {
        for b in list_b {
            if a == b {
                
            }
        }
    }

    
    return Comparison::Unequal
        
}
