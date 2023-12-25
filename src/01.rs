use proconio::input;

fn is_possible(cuts: &[i32], l: i32, k: usize, min_length: i32) -> bool {
    let mut count = 0;
    let mut prev_cut = 0;
    for &cut in cuts {
        if cut - prev_cut >= min_length && l - cut >= min_length {
            count += 1;
            prev_cut = cut;
        }
    }
    count >= k
}

fn main() {
    input! {
        n: usize,
        l: i32,
        k: usize,
        a: [i32; n],
    }

    let mut low = 0;
    let mut high = l;
    while low < high {
        let mid = (low + high + 1) / 2;
        if is_possible(&a, l, k, mid) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    println!("{}", low);
}
