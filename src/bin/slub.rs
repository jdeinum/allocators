use anyhow::{Context, Result};
use std::{
    alloc::{GlobalAlloc, System},
    sync::atomic::AtomicUsize,
};

/// The same as the standard allocator, but we keep track of the memory we are allocating
pub struct SlubAllocator {
    pub current_bytes: AtomicUsize,
    pub num_allocations: AtomicUsize,
}

impl SlubAllocator {
    pub const fn new() -> Self {
        Self {
            current_bytes: AtomicUsize::new(0),
            num_allocations: AtomicUsize::new(0),
        }
    }
}

// TODO: If I try to print using either printf, or write!, it hangs forever. I assume there is an
// infinite loop because when writing the string, we allocate somewhere. There has to be a way to
// print allocations as they happen without needing to allocate any memory. Not super important,
// but would be nice
unsafe impl GlobalAlloc for SlubAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        todo!()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        todo!()
    }
}

#[global_allocator]
pub static ALLOCATOR: SlubAllocator = SlubAllocator::new();

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
