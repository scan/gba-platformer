pub use agb::fixnum::num;

pub type FixedNum = agb::fixnum::FixedNum<8>;
pub type Vector2D = agb::fixnum::Vector2D<FixedNum>;
pub type Rect = agb::fixnum::Rect<FixedNum>;

pub fn lerp(a: FixedNum, b: FixedNum, t: FixedNum) -> FixedNum {
    a + (b - a) * t
}
