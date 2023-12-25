use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut arr: [[i32; w]; h],
    }

    // wxh行列の各行・各列の合計値を取得しW[i], H[i]に格納する
    let mut h_sum: Vec<i32> = vec![0; h];
    let mut w_sum: Vec<i32> = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            h_sum[i] += arr[i][j];
            w_sum[j] += arr[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{} ", h_sum[i] + w_sum[j] - arr[i][j]);
        }
        println!("");
    }

}
