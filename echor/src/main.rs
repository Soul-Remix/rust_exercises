use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Soul-Remix <soulremix-dev@pm.me>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .help("o not print newline")
                .takes_value(false)
                .short("n"),
        )
        .get_mathces();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}
