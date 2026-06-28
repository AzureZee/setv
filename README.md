# Setv
一个 Windows 环境变量管理工具

## Note

`main` 分支用于 wsl 环境下开发,\
`win` 分支用于 windows 环境.

## Build & Install

>Require rust toolchain

构建时会在cargo安装目录下创建`setv`和`setm`的符号链接

```sh
git clone https://github.com/AzureZee/setv
cd setv
cargo build --release
```

## Usage

`setv` modifies user variables.\
To modify system variables, please run as administrator,\
and use the command `setm` instead of `setv`.

```sh
setv <var-name> [value]           设置变量；不带 value 则删除该变量
setv -a, -append     <paths...>   将路径追加到 PATH 末尾
setv -p, -prepend    <paths...>   将路径加到 PATH 开头
setv -d, -delete     <paths...>   从 PATH 中移除指定路径
setv -e, -edit-path  <editor>     用编辑器打开 PATH，保存后写回
setv -l, -list                    列出所有环境变量
setv -h, -help                    显示帮助
```

## Example

```sh
setv JAVA_HOME 'C:\Program Files\Java\jdk-21'
setv -a '%USERPROFILE%\bin' 'C:\tools'
setv -p 'C:\Program Files\Go\bin'
setv -d 'C:\old\tool'
setv -e code
```

## License

[MIT](LICENSE.txt)
