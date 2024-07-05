fn main() {
    let days = vec![10, 11, 12, 9, 10]; // example 1
    let prog = is_progress(days);
    println!("{}", prog);
}

fn is_progress(days: Vec<i32>) -> i32 {
    let mut progress = 0;
    let mut progress_days = 0;
    for x in 0..days.len() {
        if progress != 0 {
            let cur = progress - 1;
            if days[x] > days[cur] {
                progress_days += 1;
            }
            progress += 1;
        }
        else {
            progress += 1;
        }
    }
    return progress_days;
}