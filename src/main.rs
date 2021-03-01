use amiya_say::{build, list_roles};
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use clap::{Arg, SubCommand};
use std::io::{stdin, Read};

fn main() {
    let app = app_from_crate!()
        .subcommand(
            SubCommand::with_name("say")
                .about("将一句话配入对话框，并与角色一同显示")
                .arg(
                    Arg::with_name("role")
                        .short("r")
                        .long("role")
                        .default_value("amiya.term"),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("列出已找到的角色名"));
    let args = app.get_matches();
    match args.subcommand() {
        ("say", Some(say_args)) => {
            let mut text = String::new();
            stdin().read_to_string(&mut text).unwrap();
            println!("{}", build(text.trim(), say_args.value_of("role").unwrap()))
        }
        ("list", _) => {
            let roles = list_roles();
            for role in roles {
                println!("{}", role);
            }
        }
        _ => panic!(),
    }
}

fn _test() {
    let text= build("博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~", "amiya.term");
    println!("{}", text);
}
