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

fn get_digit(a: u32, k: u8) -> u8 {
    let static_data: [u32; 4] = core::array::from_fn(|i| pow(10, i + 1));
    let data2: [u32; 4] = core::array::from_fn(|i| pow(10, i));

    ((a % static_data[k as usize]) / data2[k as usize]) as u8
}

fn get_last_two_digits(a: u32) -> u32 {
    (10 * get_digit(a, 1) + get_digit(a, 0)).into()
}

fn get_first_two_digits(a: u32) -> u32 {
   (10 * get_digit(a, 3) + get_digit(a, 2)).into()
}

fn numbers_do_chain(a: u32, b: u32) -> bool {
    get_last_two_digits(a) == get_first_two_digits(b)
}

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

        assert_eq!(get_digit(a, 0), 6);
        assert_eq!(get_digit(a, 1), 7);
        assert_eq!(get_digit(a, 2), 8);
        assert_eq!(get_digit(a, 3), 9);
    }

    #[test]
    fn test_last_two() {
        assert_eq!(get_last_two_digits(2356), 56);
        assert_eq!(get_last_two_digits(2300), 0);
        assert_eq!(get_last_two_digits(99), 99);
        assert_eq!(get_last_two_digits(0), 0);
    }

    #[test]
    fn test_first_two() {
        assert_eq!(get_first_two_digits(2356), 23);
        assert_eq!(get_first_two_digits(2300), 23);
        assert_eq!(get_first_two_digits(99), 0);
        assert_eq!(get_first_two_digits(0), 0);
    }


    #[test]
    fn test_chain() {
        assert_eq!(numbers_do_chain(2356, 5600), true );
        assert_eq!(numbers_do_chain( 1256, 5700), false);
    }
}
