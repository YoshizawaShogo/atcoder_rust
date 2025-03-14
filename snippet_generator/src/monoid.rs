use cargo_snippet::snippet;

#[snippet("monoid_xor")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct MonoidXor<T>(T);

#[snippet("monoid_xor")]
impl<T> MonoidXor<T>
where
    T: Copy,
{
    fn val(&self) -> T {
        self.0
    }
}

#[snippet("monoid_xor")]
impl<T> std::ops::Add for MonoidXor<T>
where
    T: std::ops::BitXor<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

#[snippet("monoid_xor")]
impl<T> std::ops::AddAssign for MonoidXor<T>
where
    T: std::ops::BitXorAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[snippet("monoid_xor")]
impl<T> std::ops::Sub for MonoidXor<T>
where
    T: std::ops::BitXor<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

#[snippet("monoid_xor")]
impl<T> std::ops::SubAssign for MonoidXor<T>
where
    T: std::ops::BitXorAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
