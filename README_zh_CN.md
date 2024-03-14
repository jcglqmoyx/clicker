# 鼠标自动点击器
[English](./README.md)  

1. 支持设置界面语言
2. 支持将记录导出Python代码(PyAutoGUI), 一键复制。
3. 支持启用日志
4. 支持Linux，macOS，Windows等操作系统
5. 支持将操作步骤存入数据库, 以便以后使用
6. 支持对操作进行预测试（只移动光标，不点击鼠标，当用户点击了鼠标的时候，再进行下一步。可以中途结束。只在只有鼠标操作的时候可以使用此功能）
7. change all codes to the following pattern
```rust
panel.add( & click_hotkey_panel);
```