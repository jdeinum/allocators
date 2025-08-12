use anyhow::{Context, Result};
use std::{
    alloc::{GlobalAlloc, System},
    sync::atomic::AtomicUsize,
};

/// The same as the standard allocator, but we keep track of the memory we are allocating
pub struct TracingAllocator {
    pub allocator: System,
    pub current_bytes: AtomicUsize,
    pub num_allocations: AtomicUsize,
}

impl TracingAllocator {
    pub const fn new() -> Self {
        Self {
            allocator: System,
            current_bytes: AtomicUsize::new(0),
            num_allocations: AtomicUsize::new(0),
        }
    }
}

// TODO: If I try to print using either printf, or write!, it hangs forever. I assume there is an
// infinite loop because when writing the string, we allocate somewhere. There has to be a way to
// print allocations as they happen without needing to allocate any memory. Not super important,
// but would be nice
unsafe impl GlobalAlloc for TracingAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        self.current_bytes
            .fetch_add(layout.size(), std::sync::atomic::Ordering::SeqCst);
        self.num_allocations
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        unsafe { self.allocator.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        unsafe { self.allocator.dealloc(ptr, layout) }
    }
}

#[global_allocator]
pub static ALLOCATOR: TracingAllocator = TracingAllocator::new();

#[tokio::main]
async fn main() -> Result<()> {
    allocators::run::run()
        .await
        .context("run server to completion")?;

    println!(
        "Allocated {} times with a total of {} bytes allocated",
        ALLOCATOR
            .num_allocations
            .load(std::sync::atomic::Ordering::SeqCst),
        ALLOCATOR
            .current_bytes
            .load(std::sync::atomic::Ordering::SeqCst)
    );

    Ok(())
}
