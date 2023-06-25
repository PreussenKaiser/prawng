pub trait Reverse {
    fn reverse(self) -> Self;
}

impl Reverse for i32 {
    fn reverse(self) -> i32 {
        return self * -1;
    }
}


#[cfg(test)]
mod tests {
    use crate::math::Reverse;

    #[test]
    fn pos_num_reversed() {
        let num = 1;

        let res = num.reverse();

        assert_ne!(num, res);
    }

    #[test]
    fn neg_num_reversed() {
        let num = -1;

        let res = num.reverse();

        assert_ne!(num, res);
    }
}