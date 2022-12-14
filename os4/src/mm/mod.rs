//! Memory management implementation
//! 
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//! 
//! Every task or process has a memory_set to control its virtual memory.


mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}

pub fn current_translated_physcial_address(token:usize,ptr:*const u8) -> usize{
    let page_table = PageTable::from_token(token);
    let mut va = VirtAddr::from(ptr as usize);
    let ppn = page_table.find_pte(va.floor()).unwrap().ppn();
    PhysAddr::from(ppn).0 + va.page_offset()
} 