# v0.1.0

1. 模仿 cowsay 将 stdin 输入的文本排版为字符画 + 对话框的格式
2. 提供一批预设的字符画，见仓库 `resources/roles` 目录
3. 可在 `$XDG_DATA_HOME/amiyasay/roles/` 目录下存储自定义字符画。字符画文件名为 `<name>.txt`，前面的 `<name>` 就是 `-r` 参数所接受的角色名
4. 在输错角色名时，提供纠错功能，基于 Levenshtein 编辑距离算法。