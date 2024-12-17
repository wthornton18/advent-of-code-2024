use std::ops::{Add, AddAssign, Mul, MulAssign, Rem};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<K: Clone + Copy> {
    pub x: K,
    pub y: K,
}

impl<K: Clone + Copy> Vec2<K> {
    pub fn new(x: K, y: K) -> Vec2<K> {
        Vec2 { x, y }
    }

    pub fn as_tuple(&self) -> (K, K) {
        (self.x, self.y)
    }
}

impl<K> Mul<K> for Vec2<K>
where
    K: Clone + Copy + Mul<K, Output = K>,
{
    type Output = Self;

    fn mul(self, rhs: K) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<K> MulAssign<K> for Vec2<K>
where
    K: Clone + Copy + MulAssign<K>,
{
    fn mul_assign(&mut self, rhs: K) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<K> Add<Vec2<K>> for Vec2<K>
where
    K: Clone + Copy + Add<K, Output = K>,
{
    type Output = Self;
    fn add(self, rhs: Vec2<K>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<K> AddAssign<Vec2<K>> for Vec2<K>
where
    K: Clone + Copy + AddAssign<K>,
{
    fn add_assign(&mut self, rhs: Vec2<K>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<K> Rem<K> for Vec2<K>
where
    K: Clone + Copy + Rem<K, Output = K>,
{
    type Output = Self;
    fn rem(self, rhs: K) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl<K> Rem<Vec2<K>> for Vec2<K>
where
    K: Clone + Copy + Rem<K, Output = K>,
{
    type Output = Self;
    fn rem(self, rhs: Vec2<K>) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}
