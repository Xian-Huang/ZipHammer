# ZipHammer（没写完呢.....）
Hammer explosion zip


```
Usage: ZipHammer.exe [OPTIONS] --path <PATH> --length <LENGTH>

Options:
  -p, --path <PATH>              文件路径
  -l, --length <LENGTH>          密码长度
      --min-length <MIN_LENGTH>  密码最大长度，设置该参数后必须设置 最小长度 且 length无效 [default: 0]
      --max-length <MAX_LENGTH>  密码最大长度，设置该参数后必须设置 最大长度 且 length无效 [default: 0]
  -n, --number                   密码中是否包含数字[0-9],默认包含
      --isletter                 密码中是否包含字母[a-z],默认包含
  -c, --capital                  字母是否开启大小写
  -s, --special                  密码中是否包含特殊字符 默认不包含
  -h, --help                     Print help
  -V, --version                  Print version
```
