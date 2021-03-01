//! 对话框排版

use std::fmt::Write;

/// 手机端评论区宽度为 21 个汉字，取 18 个汉字宽度填充字符
const DIALOG_WIDTH: usize = 18 * 2;

pub fn dialog(text: String) -> Result<String, std::fmt::Error> {
    let mut length = 0;
    for c in text.chars() {
        length += if c.is_ascii() { 1 } else { 2 };
    }
    let lines = length / DIALOG_WIDTH;
    let lines = if DIALOG_WIDTH * lines >= length {
        lines
    } else {
        lines + 1
    };
    let mut dialog_box =
        String::with_capacity(DIALOG_WIDTH * lines + 2 * (DIALOG_WIDTH + lines) + 4);

    let mut line_width = 0;
    let mut chars = text.chars();
    if let Some(c) = chars.next() {
        let char_width = if c.is_ascii() { 1 } else { 2 };
        dialog_box.write_char(c)?;
        line_width += char_width;
    }
    for c in chars {
        let char_width = if c.is_ascii() { 1 } else { 2 };
        if line_width + char_width <= DIALOG_WIDTH {
            dialog_box.write_char(c)?;
            line_width += char_width;
        } else {
            // 行末
            dialog_box.write_str("\n")?;
            dialog_box.write_char(c)?;
            line_width = char_width;
        }
    }
    // 最后一行
    dialog_box.write_str("\n")?;
    dialog_box.write_str("   \\")?;
    dialog_box.write_str("\n")?;

    return Ok(dialog_box);
}
