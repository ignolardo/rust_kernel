use x86_64::{structures::paging::{mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB}, VirtAddr};
use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();


pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_alloc: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>>
{
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_alloc
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_alloc)?.flush();
        };
    }

    unsafe {
        let heap_start: *mut u8 = VirtAddr::new(HEAP_START as u64).as_mut_ptr();
        ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
    }

    Ok(())
}