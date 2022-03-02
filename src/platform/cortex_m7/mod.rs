mod scheduler;
mod ipc;

pub type PidType = u32;

pub fn platform_heap_allocator() {

}

pub fn platform_target_init() {

}

pub fn platform_init() -> (scheduler::Scheduler, ipc::Ipc) {
  let sched = scheduler::Scheduler::new();
  let ip = ipc::Ipc::new();

  (sched, ip)
}
