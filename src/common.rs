use std::ops::{Add, AddAssign, Sub, SubAssign};

pub struct BarValue<T>
where T: Add + Sub + PartialOrd + Copy {
    pub min: T,
    pub max: T,
    value: T,
}

impl<T: Add + Sub + PartialOrd + Copy> BarValue<T> {

    pub fn new(min: T, max: T, value: T) -> Self {

        if value < min || value > max {
            panic!("value must more than min and less then max.")
        }

        Self {
            min,
            max,
            value
        }
    }


    pub fn value(&self) -> T {
        self.value
    }
}


impl<T: Add<Output = T> + Sub + PartialOrd + Copy> AddAssign<T> for BarValue<T>
{
    fn add_assign(&mut self, rhs: T) {
        self.value = if self.value + rhs > self.max { self.max } else { self.value + rhs };
    }
}

impl<T: Sub<Output = T> + Add + PartialOrd + Copy> SubAssign<T> for BarValue<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.value = if self.value - rhs < self.min {
            panic!("value can not less than min");
        } else {
            self.value - rhs
        };
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test() {
//         let mut a: BarValue = BarValue::new(0, 100, 10);
//         a -= 3;
//         println!("current: {}", a.value);
//     }
// }

