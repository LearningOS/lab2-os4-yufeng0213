//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,get_current_status,get_syscall_times,get_current_start_time,current_user_token};
use crate::timer::get_time_us;
use crate::mm::{current_translated_physcial_address};
use crate::mm::KERNEL_SPACE;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    
    let _us = get_time_us();
    let ts = current_translated_physcial_address(current_user_token(),_ts as *const u8) as *mut TimeVal;
    unsafe {
         *ts = TimeVal {
             sec: _us / 1_000_000,
             usec: _us % 1_000_000,
         };
     }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    //KERNEL_SPACE.lock().mmap(_start,_len,_port)  //mmap(_start,_len,_port)   
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    //KERNEL_SPACE.lock().munmap(_start,_len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let _ti = current_translated_physcial_address(current_user_token(),ti as *const u8) as*mut TaskInfo;
    unsafe{
        *_ti = TaskInfo{
            status: get_current_status(),
            syscall_times: get_syscall_times(),
            time: (get_time_us() - get_current_start_time())/1000,
        }
    }
    0
}
