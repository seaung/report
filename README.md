# Report - 数据库安全检测工具

一个用Rust编写的数据库安全检测工具，支持多种数据库的未授权访问检测和密码暴力破解。

## 功能特点

- 支持多种主流数据库
- 提供未授权访问检测
- 支持密码暴力破解
- 并发检测，提高效率
- 友好的命令行界面

## 支持的数据库

- MySQL
- Redis
- MongoDB
- PostgreSQL
- Elasticsearch
- Memcached
- Oracle
- MSSQL

## 安装

```bash
# 克隆项目
git clone https://github.com/seaung/report.git

# 进入项目目录
cd report

# 编译
cargo build --release

# 运行
./target/release/report
```

## 使用说明

### 基本命令格式

```bash
report <数据库类型> --host <主机地址> --port <端口号> [--dict <密码字典文件>]
```

### 未授权访问检测

不带`--dict`参数时，将进行未授权访问检测：

```bash
# MySQL未授权检测示例
report mysql --host 127.0.0.1 --port 3306

# Redis未授权检测示例
report redis --host 127.0.0.1 --port 6379
```

### 密码暴力破解

带`--dict`参数时，将进行密码暴力破解：

```bash
# MySQL密码暴力破解示例
report mysql --host 127.0.0.1 --port 3306 --dict passwords.txt

# Redis密码暴力破解示例
report redis --host 127.0.0.1 --port 6379 --dict passwords.txt
```

### 密码字典格式

密码字典文件为纯文本文件，每行一个密码。例如：

```text
password123
admin123
123456
root
```

## 注意事项

1. 使用本工具进行安全检测时，请确保您有合法的授权
2. 建议在测试环境中使用，避免对生产环境造成影响
3. 密码暴力破解可能会产生大量的登录失败日志
4. 部分数据库可能会有登录失败次数限制或IP封禁机制

## 常见问题

1. Q: 为什么连接失败？
   A: 请检查目标主机的防火墙设置，以及数据库是否允许远程连接

2. Q: 暴力破解速度较慢？
   A: 工具默认使用了并发检测来提高效率，但仍受限于网络条件和目标服务器的性能

3. Q: 支持自定义用户名吗？
   A: 目前使用的是数据库默认的管理员用户名，如MySQL使用root，MSSQL使用sa等

## 许可证

MIT License