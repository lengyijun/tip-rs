use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::lazy::OnceCell;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Sign {
    Top,
    Pos,
    Zero,
    Neg,
    Bot,
}

trait PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
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

impl PartialOrd for (Sign, Sign) {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        match (self.0.partial_cmp(&other.0), self.1.partial_cmp(&other.1)) {
            (None, _) => None,
            (_, None) => None,
            (Some(Ordering::Greater), Some(Ordering::Less)) => None,
            (Some(Ordering::Less), Some(Ordering::Greater)) => None,
            (Some(Ordering::Greater), Some(Ordering::Greater)) => Some(Ordering::Greater),
            (Some(Ordering::Less), Some(Ordering::Less)) => Some(Ordering::Less),
            (t, Some(Ordering::Equal)) => t,
            (Some(Ordering::Equal), t) => t,
            _ => unreachable!(),
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
            mp.insert((Bot, Zero), Bot);
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

    fn minus(self, other: Self) -> Self {
        use Sign::*;
        let cell = OnceCell::new();
        assert!(cell.get().is_none());

        let mp: &HashMap<(Sign, Sign), Sign> = cell.get_or_init(|| {
            let mut mp = HashMap::new();
            mp.insert((Bot, Bot), Bot);
            mp.insert((Bot, Zero), Bot);
            mp.insert((Bot, Neg), Bot);
            mp.insert((Bot, Pos), Bot);
            mp.insert((Bot, Top), Bot);

            mp.insert((Zero, Bot), Bot);
            mp.insert((Zero, Zero), Zero);
            mp.insert((Zero, Neg), Pos);
            mp.insert((Zero, Pos), Neg);
            mp.insert((Zero, Top), Top);

            mp.insert((Neg, Bot), Bot);
            mp.insert((Neg, Zero), Neg);
            mp.insert((Neg, Neg), Top);
            mp.insert((Neg, Pos), Neg);
            mp.insert((Neg, Top), Top);

            mp.insert((Pos, Bot), Bot);
            mp.insert((Pos, Zero), Pos);
            mp.insert((Pos, Neg), Pos);
            mp.insert((Pos, Pos), Top);
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

fn check_monotone(f: &dyn Fn(Sign, Sign) -> Sign) -> bool {
    use Sign::*;
    let v = vec![
        (Bot, Bot),
        (Bot, Zero),
        (Bot, Neg),
        (Bot, Pos),
        (Bot, Top),
        (Zero, Bot),
        (Zero, Zero),
        (Zero, Neg),
        (Zero, Pos),
        (Zero, Top),
        (Neg, Bot),
        (Neg, Zero),
        (Neg, Neg),
        (Neg, Pos),
        (Neg, Top),
        (Pos, Bot),
        (Pos, Zero),
        (Pos, Neg),
        (Pos, Pos),
        (Pos, Top),
        (Top, Bot),
        (Top, Zero),
        (Top, Neg),
        (Top, Pos),
        (Top, Top),
    ];
    for a in &v {
        for b in &v {
            if a.partial_cmp(b) == Some(Ordering::Greater) {
                let t = f(a.0, a.1).partial_cmp(&f(b.0, b.1));
                if t == Some(Ordering::Less) {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::ast_parser::parse;
    use crate::declaration_analysis::DeclarationAnalysis;
    use crate::dfs::Dfs;
    use crate::sign_lattice::check_monotone;
    use crate::sign_lattice::PartialOrd;
    use crate::sign_lattice::Sign;
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_ord() {
        assert_eq!(
            (Sign::Bot, Sign::Top).partial_cmp(&(Sign::Bot, Sign::Top)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            (Sign::Bot, Sign::Pos).partial_cmp(&(Sign::Bot, Sign::Top)),
            Some(Ordering::Less)
        );
        assert_eq!(
            (Sign::Bot, Sign::Top).partial_cmp(&(Sign::Top, Sign::Bot)),
            None
        );
        assert_eq!(
            (Sign::Top, Sign::Bot).partial_cmp(&(Sign::Bot, Sign::Top)),
            None
        );
    }

    #[test]
    fn test_sign_lattice() {
        assert!(check_monotone(&Sign::plus));
        assert!(check_monotone(&Sign::minus));
    }
}
