//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::get_time_us, syscall::TASK_INFO,
};

#[repr(C)]
#[derive(Debug)]
/// TimeVal
pub struct TimeVal {
    /// TimeVal
    pub sec: usize,
    /// TimeVal
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

impl TaskInfo {
    ///
    pub fn sys_times_plus(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
    }
}
/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    unsafe {
        match TASK_INFO{
            Some(mut x) => {
                // if (*ti).syscall_times[SYSCALL_TASK_INFO] == 0 {
                    (*ti).status = TaskStatus::Running;
                    (*ti).time = (*x).time;
                    (*ti).syscall_times = (*x).syscall_times;
                    x = ti;
                    assert_eq!(x, ti);
                // }
                // assert_eq!(x, ti);   
                // info!("sys_task_info:{}", (*ti).syscall_times[SYSCALL_TASK_INFO]);
                // info!("sys_task_info:{}", (*x).syscall_times[SYSCALL_TASK_INFO]);
                // info!("sys_get_time:{}", (*ti).syscall_times[SYSCALL_GET_TIME]);
                // info!("sys_get_time:{}", (*x).syscall_times[SYSCALL_GET_TIME]);
                // info!("time:{}", x.time);
            },
            _ => {
                panic!("CAN'T GOT THRER")
            }
        }
    }
    0
}
