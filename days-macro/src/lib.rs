use proc_macro::TokenStream;

#[proc_macro]
pub fn days(day_n: TokenStream) -> TokenStream {
    let n: usize = day_n.to_string().parse().unwrap();

    let mut code: Vec<String> = vec![];

    for i in 1..=n {
        code.push(gen_day(i));
    }

    code.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn day(day_n: TokenStream) -> TokenStream {
    let n: usize = day_n.to_string().parse().unwrap();

    let code = gen_day(n);

    code.parse().unwrap()
}

fn gen_day(n: usize) -> String {
    let msg = "\"day {}: part1 = {}, part2 = {}\"";

    "
    let i = day{day_n}::input();

    println!(
        {msg},
        {day_n},
        day{day_n}::part1(&i),
        day{day_n}::part2(&i)
    );
    "
    .replace("{day_n}", &n.to_string())
    .replace("{msg}", msg)
}

#[proc_macro]
pub fn benches(day_n: TokenStream) -> TokenStream {
    let n: usize = day_n.to_string().parse().unwrap();

    let mut code: Vec<String> = vec![];

    for i in 1..=n {
        code.push(gen_bench(i));
    }

    code.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn bench(day_n: TokenStream) -> TokenStream {
    let n: usize = day_n.to_string().parse().unwrap();

    let code = gen_bench(n);

    code.parse().unwrap()
}

fn gen_bench(n: usize) -> String {
    "
    let data = day{day_n}::input();

    c.bench_function(\"day{day_n}::part1\", |b| {
        b.iter(|| black_box(day{day_n}::part1(black_box(&data))))
    });

    c.bench_function(\"day{day_n}::part2\", |b| {
        b.iter(|| black_box(day{day_n}::part2(black_box(&data))))
    });
    "
    .replace("{day_n}", &n.to_string())
    .parse()
    .unwrap()
}
