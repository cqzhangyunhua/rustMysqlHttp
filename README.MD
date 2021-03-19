## Rust 项目的基础例子
包含http服务器,mysql连接池、json返回数据结构、配置文件读取 多目录结构

### 运行 
```
建conf目录 
再建mysql.ini文件
[DEFAULT]
linkmysql = mysql://root:root@xxxx.xxx.xxx:3306/test

```
### 交叉编译 .cargo下是用于交叉编译，当前为ubuntu 编译成windows
.cargo/config 用于配置交叉编译
cargo build --release --target=x86_64-pc-windows-gnu
