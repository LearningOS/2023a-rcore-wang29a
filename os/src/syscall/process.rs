//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, insert,
    }, mm::{user_data, MapPermission, VirtAddr}, timer::get_time_us, syscall::TASK_INFO,
};

#[repr(C)]
#[derive(Debug)]
///
pub struct TimeVal {
    ///
    pub sec: usize,
    ///
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let ptr = user_data(current_user_token(), ts);
    let us = get_time_us();
    unsafe {
        *ptr = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        }; 
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let ptr = user_data(current_user_token(), ti);
    unsafe {
        match TASK_INFO{
            Some(mut x) => {
                (*ptr).status = TaskStatus::Running;
                (*ptr).time = (*x).time;
                (*ptr).syscall_times = (*x).syscall_times;
                x = ti;
                assert_eq!(x, ti);
            },
            _ => {
                panic!("CAN'T GOT THRER")
            }
        }
    }
    0
}

// YOUR JOB: Implement mmap.
///
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let mut ret: isize = 1;
    if (start &(4096-1)) !=0 || port & !0x7 != 0 || port & 0x7 == 0 {
        return ret
    }
    let permission: MapPermission = 
        match port{
            1 => MapPermission::R,
            2 => MapPermission::W,
            3 => MapPermission::R |MapPermission::W,
            4 => MapPermission::X ,
            5 => MapPermission::X | MapPermission::R,
            6 => MapPermission::X | MapPermission::W,
            7 => MapPermission::X | MapPermission::W | MapPermission::R,
            _ => return ret
        };

    insert(VirtAddr::from(start), VirtAddr::from(start+len), permission);
    ret = 0;
    ret
}

// YOUR JOB: Implement munmap.
///
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
