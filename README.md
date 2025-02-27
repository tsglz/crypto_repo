# crypto_repo

通过 rust 实现的对于 shellcode 加密算法快速使用

同时，将通过文档和注释的方式协助进行代码的扩充和自定义处理

脚本也能够协助使用者进行批量化处理

本仓库包含以下内容：

1. aes_128 (done)
2. 求最大公约数 (done)
3. Fermat素数测试 (done)
4. aes_256
5. rsa
6. des
7. ecc

本项目使用的库为目前最新版本，同时作者会不定期与依赖仓库的最新版本进行同步更新

因为作者的个人原因，本项目暂时只支持 Windows 平台，后续会考虑支持 Linux 平台

插件直接跨平台使用可能存在问题，有条件的师傅可以考虑自己手动编译

欢迎各位师傅们一起参与进来，共同完善本项目


## aes_128

aes_128 目前支持 16进制(0x00,0x01)，以及机器码(\xfc\f44) 两种形式

1. 16进制形式：直接将 shellcode 输入到 output.txt 文件中，然后运行 aes_128.exe
2. 机器码形式：将shellcode 添加到 divide.py 的指定位置，运行start.bat(需要 python3 环境)，将 shellcode 转换为机器码，然后 bat 脚本会自动运行 aes_128.exe

## 求最大公约数

使用拓展欧几里得算法，求两个数的最大公约数，同时计算出贝祖公式的 s 和 t 值

## Fermat素数测试

### 素数测试

使用 fermat 素性测试，判断给定数是否为素数

### 最大素数

同样使用 fermat 素性测试，判断给定范围内的最大素数