//! 对话框排版

use std::fmt::Write;

const TABLE_LU: &'static str = "┌";
const TABLE_RU: &'static str = "┐";
const TABLE_LD: &'static str = "└";
const TABLE_RD: &'static str = "┘";
const TABLE_HR: &'static str = "─";
const TABLE_VT: &'static str = "│";
const TABLE_AG: &'static str = "v";
/// 手机端评论区宽度为 21 个汉字，取 18 个汉字宽度填充字符
const DIALOG_WIDTH: usize = 18 * 2;

pub fn dialog_nobox(text: String) -> Result<String, std::fmt::Error> {
    let mut length = 0;
    for c in text.chars() {
        length += if c.is_ascii() { 1 } else { 2 };
    }

    let last_line_remain = length % DIALOG_WIDTH;
    let lines = if last_line_remain == 0 {
        length / DIALOG_WIDTH
    } else {
        length / DIALOG_WIDTH + 1
    };
    let mut dialog_box = String::with_capacity(
        length - last_line_remain + DIALOG_WIDTH + 2 * (DIALOG_WIDTH + lines) + 4,
    );

    let mut chars = text.chars().peekable();
    'outer: loop {
        let mut line_width = 0;
        'inner: loop {
            if let Some(ch) = chars.peek() {
                let char_width = if ch.is_ascii() { 1 } else { 2 };
                if line_width + char_width <= DIALOG_WIDTH {
                    // 消耗迭代器
                    let ch = chars.next().unwrap();
                    dialog_box.write_char(ch)?;
                    line_width += char_width;
                } else {
                    // 一行的末尾
                    break 'inner;
                }
            } else {
                // 最后一行
                let whitespaces = DIALOG_WIDTH - line_width;
                if whitespaces > 0 {
                    dialog_box.write_str(" ".repeat(whitespaces).as_str())?;
                }
                dialog_box.write_str("\n")?;
                break 'outer;
            }
        }
        dialog_box.write_str("\n")?;
    }

    // 框线
    dialog_box.write_str("   \\")?;
    dialog_box.write_str("\n")?;

    return Ok(dialog_box);
}

pub fn dialog_withbox(text: String) -> Result<String, std::fmt::Error> {
    let mut length = 0;
    for c in text.chars() {
        length += if c.is_ascii() { 1 } else { 2 };
    }
    if length <= DIALOG_WIDTH {
        let mut dialog_box = String::with_capacity(length + 2 * length + 6);
        dialog_box.write_str(&TABLE_LU)?;
        dialog_box.write_str(TABLE_HR.repeat(length).as_str())?;
        dialog_box.write_str(&TABLE_RU)?;
        dialog_box.write_str("\n")?;
        dialog_box.write_str(&TABLE_VT)?;
        dialog_box.write_str(text.as_str())?;
        dialog_box.write_str(&TABLE_VT)?;
        dialog_box.write_str("\n")?;
        dialog_box.write_str(&TABLE_LD)?;
        dialog_box.write_str(&TABLE_AG)?;
        dialog_box.write_str(TABLE_HR.repeat(length - 1).as_str())?;
        dialog_box.write_str(&TABLE_RD)?;
        dialog_box.write_str("\n")?;
        return Ok(dialog_box);
    } else {
        let last_line_remain = length % DIALOG_WIDTH;
        let lines = if last_line_remain == 0 {
            length / DIALOG_WIDTH
        } else {
            length / DIALOG_WIDTH + 1
        };
        let mut dialog_box = String::with_capacity(
            length - last_line_remain + DIALOG_WIDTH + 2 * (DIALOG_WIDTH + lines) + 4,
        );

        dialog_box.write_str(&TABLE_LU)?;
        dialog_box.write_str(TABLE_HR.repeat(DIALOG_WIDTH).as_str())?;
        dialog_box.write_str(&TABLE_RU)?;
        dialog_box.write_str("\n")?;

        let mut chars = text.chars().peekable();
        'outer: loop {
            let mut line_width = 0;
            dialog_box.write_str(&TABLE_VT)?;
            'inner: loop {
                if let Some(ch) = chars.peek() {
                    let char_width = if ch.is_ascii() { 1 } else { 2 };
                    if line_width + char_width <= DIALOG_WIDTH {
                        // 消耗迭代器
                        let ch = chars.next().unwrap();
                        dialog_box.write_char(ch)?;
                        line_width += char_width;
                    } else {
                        // 一行的末尾
                        let whitespaces = DIALOG_WIDTH - line_width;
                        if whitespaces > 0 {
                            dialog_box.write_str(" ".repeat(whitespaces).as_str())?;
                        }
                        break 'inner;
                    }
                } else {
                    // 最后一行
                    let whitespaces = DIALOG_WIDTH - line_width;
                    if whitespaces > 0 {
                        dialog_box.write_str(" ".repeat(whitespaces).as_str())?;
                    }
                    dialog_box.write_str(&TABLE_VT)?;
                    dialog_box.write_str("\n")?;
                    break 'outer;
                }
            }
            dialog_box.write_str(&TABLE_VT)?;
            dialog_box.write_str("\n")?;
        }
        dialog_box.write_str(&TABLE_LD)?;
        dialog_box.write_str(&TABLE_AG)?;
        dialog_box.write_str(TABLE_HR.repeat(DIALOG_WIDTH - 1).as_str())?;
        dialog_box.write_str(&TABLE_RD)?;
        dialog_box.write_str("\n")?;

        return Ok(dialog_box);
    }
}
