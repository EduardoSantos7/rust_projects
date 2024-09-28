use std::alloc::{GlobalAlloc, Layout};
use std::ptr;

struct BumpAllocator {
    heap_start: *mut u8,
    heap_end: *mut u8,
    current: *mut u8,
}

impl BumpAllocator {
    /// Creates a new BumpAllocator with a given heap size
    fn new(heap_size: usize) -> Self {
        // Allocate memory using Rust's global allocator
        let layout = Layout::from_size_align(heap_size, 8).unwrap();
        let heap_start = unsafe { std::alloc::alloc(layout) };
        if heap_start.is_null() {
            panic!("Heap allocation failed");
        }

        let heap_end = unsafe { heap_start.add(heap_size) };
        BumpAllocator {
            heap_start,
            heap_end,
            current: heap_start,
        }
    }

    /// Allocates a block of memory with the given layout
    fn allocate(&mut self, layout: Layout) -> *mut u8 {
        let alloc_start = self.current;
        let alloc_end = unsafe { alloc_start.add(layout.size()) };

        if alloc_end > self.heap_end {
            ptr::null_mut() // Out of memory
        } else {
            self.current = alloc_end;
            alloc_start
        }
    }

    /// Resets the allocator to reuse the memory
    fn reset(&mut self) {
        self.current = self.heap_start;
    }

    /// Helper function to align up an address.
    fn align_up(addr: usize, align: usize) -> usize {
        (addr + align - 1) & !(align - 1)
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        let heap_size = unsafe { self.heap_end.offset_from(self.heap_start) as usize };
        let layout = Layout::from_size_align(heap_size, 8).unwrap();
        unsafe {
            std::alloc::dealloc(self.heap_start, layout);
        }
    }
}

fn main() {
    // Define the size of the heap buffer, e.g., 1 MB.
    const HEAP_SIZE: usize = 1024 * 1024; // 1 MB

    // Initialize the bump allocator.
    let mut allocator = BumpAllocator::new(HEAP_SIZE);

    // Example: Allocate memory for an array of 10 integers.
    let layout = Layout::array::<i32>(10).expect("Invalid layout for i32 array");
    let ptr = allocator.allocate(layout) as *mut i32;

    if !ptr.is_null() {
        // Initialize the allocated memory.
        for i in 0..10 {
            unsafe {
                ptr.add(i).write(i as i32);
            }
        }

        // Read and print the values.
        for i in 0..10 {
            unsafe {
                let value = ptr.add(i).read();
                println!("Value {}: {}", i, value);
            }
        }

        // Reset the allocator to reuse the memory.
        allocator.reset();
    } else {
        println!("Allocation failed");
    }
}