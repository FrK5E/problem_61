use num_traits::pow;

fn generate(f: fn(i32) -> i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut i = 1;
    let mut f1 = f(i);
    while f1 < 10000 {
        if f1 >= 1000 {
            result.push(f1);
        }

        i += 1;
        f1 = f(i);
    }

    result
}

fn getDigit(a: u32, k: u8) -> u8 {
    let static_data: [u32; 4] = core::array::from_fn(|i| pow(10, i+1));
    let data2: [u32; 4] = core::array::from_fn(|i| pow(10, i));

    ((a % static_data[k as usize]) / data2[k as usize]) as u8
}

//fn numbers_match(a: i32, b: i32) -> bool {}

fn main() {
    println!("Hello, world!");

    let t = generate(|i| i * (i + 1) / 2);
    let s = generate(|i| i * i);

    for i in s {
        println!("{}", i)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_digit() {
        let a: u32 = 9876;

        assert_eq!(getDigit(a, 0), 6);
        assert_eq!(getDigit(a, 1), 7);
        assert_eq!(getDigit(a, 2), 8);
        assert_eq!(getDigit(a, 3), 9);
    }
}
