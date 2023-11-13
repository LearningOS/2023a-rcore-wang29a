
![](https://raw.githubusercontent.com/wang29a/image/master/20231109203645.png)


## 实现思路

一共需要添加三个`syscall`，根据错误信息查看对应`syscall`文档

### syscall_ioctl

根据文档描述，我的判断是这一个系统调用是不需要实现的，直接返回0

因为rcore当前还没有设备去控制

### syscall_writev

向给出的文件描述符写入数据

参数提供的是 Vector I/O data structure，以及对应的元素个数


### syscall_exit_group

直接调用`syscall_exit`

## 代码

```rust
pub fn sys_writev(fd: usize, buf: *const IoVec, len: usize) -> isize {
    let token = current_user_token();
    let buf = user_data(token, buf);
    trace!("kernel:pid[{}] sys_write", current_task().unwrap().pid.0);
    info!("{} {}", fd, len);
    let mut ret = 0isize;
    for i in 0usize..len{
        info!("{}", i);
        unsafe {
            let t = sys_write(
                fd,
                (*buf.wrapping_add(i)).iov_base as *const u8,
                (*buf.wrapping_add(i)).iov_len
            );
            if t == -1{
                return -1
            }
            ret += t;
        }
    }
    ret
}
```

## 问答题目

**TODO**