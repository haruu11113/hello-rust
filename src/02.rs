use proconio::input;


fn addKakko(n: i32, now: &str, num_open: i32) {
    if n == 0 {
        println!("{}", &now);
        return;
    }
    let n_1: i32 = (n - 1) as i32;
    if num_open == 0 {
        addKakko(n_1, &format!("{}{}", now, "("), num_open + 1);
    } else if num_open < n_1 {
        addKakko(n_1, &format!("{}{}", now, "("), num_open + 1);
        addKakko(n_1, &format!("{}{}", now, ")"), num_open - 1);
    } else {
        addKakko(n_1, &format!("{}{}", now, ")"), num_open - 1);
    }
}

fn main() {
    input! {
        n: i32,
    }
    if n % 2 == 0 {
        addKakko(n, "", 0);
    } else {
        println!("");
    }
}
