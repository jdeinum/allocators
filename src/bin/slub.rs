// The kernel slub allocator works by providing preallocated objects of a particular size. The
// first time you request an object of a particular size, a cache is started for objects of that
// size. The next time you require an object of that size, it is served from the cache instead,
// which means we save ourselves a few pages of allocated memory.
//
// Similar to the glic allocator, when we are all done with our memory, we just return it to the
// userspace cache rather than returning the memory to the operating system.
//
// Because all of our messages are the same size, the Slub allocator should reduce the number of
// allocations since we are never really dealing with more than one message at a time. Not very
// representative i know... but if the cache is large enough the principle should be the same.
//
// In applications where there are many objects with distinct sizes, the slub allocator would be a
// poor choice because we'd be wasting a bunch of space.
//
// One final thing to note is that the kernel typically uses an arena allocator for really large
// objects, so the amount of fragmentation decreases if we aren't actually using all of the large
// objects in the cache.

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
        // check the size

        // if we have seen the size before, serve it from the cache

        // if not, start a cache, and serve from there
        todo!()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {

        // return the item to the cache
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
