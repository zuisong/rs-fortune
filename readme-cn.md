# rs-fortune

`rs-fortune`是一个简单的Rust命令行工具,用于打印随机的fortune条目。  

## 安装  

- 可以从 Release 页面下载安装
- 可以通过Cargo安装:  

    ```bash
    cargo install --git https://github.com/zuisong/rs-fortune
    ```

- MacOS 可以通过homebrew 安装

    ```bash
    brew install zuisong/tap/rs-fortune
    ```

## 用法

```txt
rs-fortune [选项] <fortune文件>
选项:

--help - 打印帮助信息
--completion <shell> - 生成shell补全脚本,<shell>可以是bash、zsh、fish等
```

`<fortune文件>` 是包含 fortune 条目的文本文件, 每条 fortune 之间用独占一行的一个 % 分隔。

`rs-fortune` 也支持从 `FORTUNE_FILE` 环境变量中读取 `<fortune文件>` 路径:

```bash
FORTUNE_FILE=fortunes.txt rs-fortune
```

如果同时传入了命令行参数和环境变量,命令行参数的优先级更高。

例如,一个fortune文件fortunes.txt的内容可以是:

```txt
Fortune favors the bold.
%
The early bird gets the worm.
%
Slow and steady wins the race.
```

然后你可以这样使用:

```bash
rs-fortune fortunes.txt
# 可能输出  
# Fortune favors the bold  

### 也可以从环境变量读取 fortune 文件路径
FORTUNE_FILE=fortunes.txt rs-fortune  

### 也可以从管道中读取 fortunes
cat fortunes.txt | rs-fortune  

# 打印帮助信息
rs-fortune --help  
```

如果不传入 `<fortune文件>` 或 `FORTUNE_FILE` 环境变量,将打印一条默认的 fortune

这是一个简单实用的命令行工具,可以用来打印随机的名言警句等。欢迎提出改进意见和建议!
