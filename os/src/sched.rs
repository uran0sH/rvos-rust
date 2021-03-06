#![allow(unused)]

use crate::config::RegT;
use crate::config::MAX_TASKS;
use crate::config::STACK_SIZE;
use crate::riscv::*;
use crate::uart::uart_puts;
use crate::platform::*;
use spin::Mutex;
use crate::lock::*;

extern "C" {
    fn switch_to(next: *const context);
    fn printf(s: *const u8, ...);
}

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/* task management */
pub struct context {
    /* ignore x0 */
    ra: RegT,
    sp: RegT,
    gp: RegT,
    tp: RegT,
    t0: RegT,
    t1: RegT,
    t2: RegT,
    s0: RegT,
    s1: RegT,
    a0: RegT,
    a1: RegT,
    a2: RegT,
    a3: RegT,
    a4: RegT,
    a5: RegT,
    a6: RegT,
    a7: RegT,
    s2: RegT,
    s3: RegT,
    s4: RegT,
    s5: RegT,
    s6: RegT,
    s7: RegT,
    s8: RegT,
    s9: RegT,
    s10: RegT,
    s11: RegT,
    t3: RegT,
    t4: RegT,
    t5: RegT,
    t6: RegT,
    pc: RegT,
}

impl context {
    pub const fn new() -> context {
        context {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            pc: 0,
        }
    }
}

// lazy_static!{
//     pub static ref task_stack: Mutex<[[u8; STACK_SIZE]; MAX_TASKS]> = Mutex::new([[0; STACK_SIZE]; MAX_TASKS]);
//     pub static ref ctx_tasks: Mutex<[context; MAX_TASKS]> = Mutex::new([context::new(); MAX_TASKS]);
//     pub static ref top: Mutex<usize> = Mutex::new(0);
// }

// static mut task_stack: Mutex<[[u8; STACK_SIZE]; MAX_TASKS]> = Mutex::new([[0; STACK_SIZE]; MAX_TASKS]);
// static mut _top: Mutex<usize> = Mutex::new(0);
// static mut _current: Mutex<isize> = Mutex::new(-1);
// static mut ctx_tasks: Mutex<[context; MAX_TASKS]> = Mutex::new([context::new(); MAX_TASKS]);
static mut task_stack: [[u8; STACK_SIZE]; MAX_TASKS] = [[0; STACK_SIZE]; MAX_TASKS];
static mut _top: usize = 0;
static mut _current: isize = -1;
static mut ctx_tasks: [context; MAX_TASKS] = [context::new(); MAX_TASKS];

pub fn sched_init() {
    w_mscratch(0);

    /* enable machine-mode software interrupts. */
	w_mie(r_mie() | (1 << 3));
}

pub fn schedule() {
    // unsafe {
    //     if *_top.lock() <= 0 {
    //         return;
    //     }

    //     *_current.lock() = (*_current.lock() + 1) % (*_top.lock() as isize);
    //     let next = &(*ctx_tasks.lock())[*_current.lock() as usize] as *const context;
    //     switch_to(next);
    // }
    unsafe {
        if _top <= 0 {
            return;
        }

        _current = (_current + 1) % (_top as isize);
        let next = &ctx_tasks[_current as usize] as *const context;
        switch_to(next);
    }
}

pub fn task_delay(mut count: i32) {
    count = count * 50000;
    while count > 0 {
        count -= 1;
    }
}


pub fn task_yield() {
    let id = r_mhartid(); // This line will cause illegal instruction. I have no idea about this.
    unsafe {
        *(clint_msip(id as usize) as *mut u32) = 1;
    }
}

pub fn task_create(start_routine: fn()) -> i32 {
    unsafe {
        if _top < MAX_TASKS {
            ctx_tasks[_top].sp = (&task_stack[_top][STACK_SIZE - 1]) as *const u8 as RegT;
            ctx_tasks[_top].pc = start_routine as fn() as RegT;
            _top += 1;
            return 0;
        }
    }
    // unsafe {
    //     if *_top.lock() < MAX_TASKS {
    //         (*ctx_tasks.lock())[*_top.lock()].sp =  (&(*task_stack.lock())[*_top.lock()][STACK_SIZE - 1]) as *const u8 as RegT;
    //         (*ctx_tasks.lock())[*_top.lock()].ra = start_routine as fn() as RegT;
    //         *_top.lock() += 1;
    //         return 0;
    //     }
    // }
    return -1;
}
