use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::lazy::OnceCell;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Sign {
    Top,
    Pos,
    Zero,
    Neg,
    Bot,
}

impl PartialOrd for Sign {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        match (self, other) {
            (Sign::Top, _) => Some(Ordering::Greater),
            (_, Sign::Top) => Some(Ordering::Less),
            (Sign::Bot, _) => Some(Ordering::Less),
            (_, Sign::Bot) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl Sign {
    fn plus(self, other: Self) -> Self {
        use Sign::*;
        let cell = OnceCell::new();
        assert!(cell.get().is_none());

        let mp: &HashMap<(Sign, Sign), Sign> = cell.get_or_init(|| {
            let mut mp = HashMap::new();
            mp.insert((Bot, Bot), Bot);
            mp.insert((Bot, Zero),Bot);
            mp.insert((Bot, Neg), Bot);
            mp.insert((Bot, Pos), Bot);
            mp.insert((Bot, Top), Bot);

            mp.insert((Zero, Bot), Bot);
            mp.insert((Zero, Zero), Zero);
            mp.insert((Zero, Neg), Neg);
            mp.insert((Zero, Pos), Pos);
            mp.insert((Zero, Top), Top);

            mp.insert((Neg, Bot), Bot);
            mp.insert((Neg, Zero), Neg);
            mp.insert((Neg, Neg), Neg);
            mp.insert((Neg, Pos), Top);
            mp.insert((Neg, Top), Top);

            mp.insert((Pos, Bot), Bot);
            mp.insert((Pos, Zero), Pos);
            mp.insert((Pos, Neg), Top);
            mp.insert((Pos, Pos), Pos);
            mp.insert((Pos, Top), Top);

            mp.insert((Top, Bot), Bot);
            mp.insert((Top, Zero), Top);
            mp.insert((Top, Neg), Top);
            mp.insert((Top, Pos), Top);
            mp.insert((Top, Top), Top);
            mp
        });
        mp[&(self, other)]
    }
}

fn check_monotone(f:&dyn Fn(Sign,Sign)->Sign) -> bool {
    todo!();
    true
}


#[cfg(test)]
mod tests {
    use crate::ast_parser::parse;
    use crate::declaration_analysis::DeclarationAnalysis;
    use crate::dfs::Dfs;
    use std::collections::HashMap;
    use std::fs;
    use crate::sign_lattice::check_monotone;
    use crate::sign_lattice::Sign;
    use std::cmp::Ordering;

    #[test]
    fn test_ord() {
        assert_eq!((Sign::Bot,Sign::Top).partial_cmp(&(Sign::Bot,Sign::Top)),Some(Ordering::Equal));
        assert_eq!((Sign::Bot,Sign::Pos).partial_cmp(&(Sign::Bot,Sign::Top)),Some(Ordering::Less));
        assert_eq!((Sign::Bot,Sign::Top).partial_cmp(&(Sign::Top,Sign::Bot)),None);
        assert_eq!((Sign::Top,Sign::Bot).partial_cmp(&(Sign::Bot,Sign::Top)),None);
    }

    #[test]
    fn test_sign_lattice() {
       assert!(check_monotone(&Sign::plus));
    }
}
