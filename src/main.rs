use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut nums: Vec<u32> = (1..10).collect();

    let tries = 500;

    let num = dotimes(tries, &mut nums);

    let ans = 100.00 * num as f64 / tries as f64;

    println!(
        "Out of {} tries, number of successful attempts is {}; resulting percent is {}",
        tries, num, ans
    );
}

fn dotimes(i: u32, nums: &mut Vec<u32>) -> i32 {
    // This function does play i number of times
    let mut ans = 0;

    for _b in 0..i {
        nums.shuffle(&mut thread_rng());

        if play(&nums) {
            ans += 1;
        }
    }

    ans
}

fn play(nums: &Vec<u32>) -> bool {
    for x in nums {
        if !trial(&x, &nums) {
            return false;
        }
    }

    true
}

fn trial(i: &u32, var: &Vec<u32>) -> bool {
    let cc = *i;
    let mut i = *i;
    i -= 1;
    for _a in 1..5 {
        i = var[i as usize];
        if i == cc {
            return true;
        }

        i -= 1;
    }

    false
}
