//! Process management syscalls
use crate::{
    task::{
        curr_task_read_data, curr_task_write_data, exit_current_and_run_next,
        get_curr_task_syscall_cnt, suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            let pos = id as *const u8;
            curr_task_read_data(pos) as isize
        }
        1 => {
            let pos = id as *const u8;
            curr_task_write_data(pos, (data & 0xFF) as u8);
            0
        }
        2 => get_curr_task_syscall_cnt(id),
        _ => {
            // unknown trace request
            -1
        }
    }
}
