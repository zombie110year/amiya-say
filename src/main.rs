use amiya_say::build;
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use clap::{Arg, SubCommand};

fn main() {
    let app = app_from_crate!().subcommand(
        SubCommand::with_name("say")
            .about("将一句话配入对话框，并与角色一同显示")
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
