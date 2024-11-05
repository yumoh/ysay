# 简单语音播放器

### 使用

```bash
say 0.3.0

USAGE:
    say [FLAGS] [OPTIONS] [text]

FLAGS:
        --sf         输入音频流数据
    -h, --help       Prints help information
    -s               使用流输入文本内容
    -V, --version    Prints version information

OPTIONS:
        --server <tts-server>      使用特定tts服务器,默认自建服务器
        --speaker <tts-speaker>    说话人1-173 默认: 5
    -f, --file <vedio-file>        播放音频文件

ARGS:
    <text>    消息主体文本
```

- say --sf < 1.wav
- say "雨墨世界红尘，一道线隔绝了阴阳生死。"
- say -s < speech.txt

### 其它

1.[tts服务](https://git.yumolab.cn:8088/ai-go/tts-serve2)
2. on linux do `sudo apt-get install libasound2-dev`
3. on macos do `brew install llvm@15 llvm@16 llvm@18`
