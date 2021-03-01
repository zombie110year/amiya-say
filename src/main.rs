use amiya_say::build;
use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new("amiya-say").subcommand(
        SubCommand::with_name("say")
            .arg(
                Arg::with_name("role")
                    .short("r")
                    .long("role")
                    .default_value("amiya.term"),
            )
            .arg(Arg::with_name("text")),
    );
    let args = app.get_matches();
    match args.subcommand() {
        ("say", Some(say_args)) => {
            println!(
                "{}",
                build(
                    say_args.value_of("text").unwrap(),
                    say_args.value_of("role").unwrap()
                )
            )
        }
        _ => panic!(),
    }
}

fn _test() {
    let text= build("博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~", "amiya.term");
    println!("{}", text);
}
