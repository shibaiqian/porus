#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut n : isize = default();

    scanf!(stdin, " %d", &mut n);

    if n <= 0 {
        return;
    }

    let a = array![0isize; n];

    for i in 0..n {
        scanf!(stdin, " %d", &mut a[i]);
    }

    printf!(stdout, "%d", a[n-1]);
    for i in 1..n {
        printf!(stdout, " %d", a[n-1-i]);
    }
    printf!(stdout, "\n");
}