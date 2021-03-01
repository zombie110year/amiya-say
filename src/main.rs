use amiya_say::build;

fn main() {
    _test();
}

fn _test() {
    let text= build("博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~博士还不能休息哦。ko~ko~da~yo~", "amiya.term");
    println!("{}", text);
}