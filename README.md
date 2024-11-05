# 简单语音播放器

### 使用

```bash
say 0.3.1

USAGE:
    ysay [FLAGS] [OPTIONS] [text]

FLAGS:
        --sf         输入音频流数据
    -h, --help       Prints help information
    -s               使用流输入文本内容
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config-path>                 config file path
        --generate-config <gen-config-path>    generate config to file
    -o, --output <output-file>                 保存到音频文件
        --speaker <tts-speaker>                alex,benjamin,anna,diana,default: diana
    -f, --file <vedio-file>                    播放音频文件

ARGS:
    <text>    消息主体文本
```

- say --sf < 1.wav
- say "雨墨世界红尘，一道线隔绝了阴阳生死。"
- say -s < speech.txt

### 其它

1. [tts服务](https://cloud.siliconflow.cn/playground/text-to-speech)
2. on linux do `sudo apt-get install libasound2-dev`
3. on macos do `brew install llvm@15 llvm@16 llvm@18 sox`
4. 安装: `cargo install ysay`
