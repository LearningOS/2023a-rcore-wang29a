
![实验结果](https://raw.githubusercontent.com/wang29a/image/master/20231108192217.png)


## 实现思路

rcore原本的命令行参数初始化和这次实验要求的实现完全是反过来的。

在rcore代码的基础上修改，倒序将`args`字符放在用户栈上，使用`vec`暂存对应的指针，接着倒序将`vec`中的指针放入用户栈，注意有一个`NULL`放在结尾，`argc`放在开始

`x11`返回值改为`user_sp`

## 代码

```rust
let mut argv = Vec::new();
argv.push(args.len());
for i in (0..args.len()).rev() {
    user_sp -= args[i].len() + 1;
    argv.push(user_sp);
    let mut p = user_sp;
    for c in args[i].as_bytes() {
        *translated_refmut(memory_set.token(), p as *mut u8) = *c;
        p += 1;
    }
    *translated_refmut(memory_set.token(), p as *mut u8) = 0;
}
argv.push(0);
for i in (0..argv.len()).rev() {
    user_sp -= core::mem::size_of::<usize>();
    let p = user_sp;
    *(translated_refmut(memory_set.token(), p as *mut usize)) = argv[i];
}
```


## 问答作业

### elf 文件和 bin 文件有什么区别？

elf文件比bin文件多了一些描述信息，bin文件是单纯的二进制数据

ELF 文件中有一些用于帮助系统加载和运行程序的数据，比如程序的入口地址、数据的初始值

BIN文件纯二进制文件

![](https://raw.githubusercontent.com/wang29a/image/master/20231108193255.png)

