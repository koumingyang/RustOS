use bit_allocator::BitAlloc;
use consts::KERNEL_OFFSET;
// Depends on kernel
use memory::{FRAME_ALLOCATOR, init_heap};
use super::{BootInfo, MemoryRegionType};
use ucore_memory::PAGE_SIZE;
use ucore_memory::paging::PageTable;

/* *
 * init - initialize the memory
 * @brief:  initialize the physical memory management(PageData struct, page table, frame allocator) and heap
 * @param:  
    boot_info:  {BootInfo, MemoryRegionType}, from bootloader
 * @retval: none
 * */
pub fn init(boot_info: &BootInfo) {
    assert_has_not_been_called!("memory::init must be called only once");
    init_frame_allocator(boot_info);
    init_heap();
    info!("memory: init end");
}

/* *
 * init_frame_allocator
 * @brief:  Init FrameAllocator and insert all 'Usable' regions from BootInfo.
 * @param:  
    boot_info:  {BootInfo, MemoryRegionType}, from bootloader
 * @retval: none
 * */
fn init_frame_allocator(boot_info: &BootInfo) {
    let mut ba = FRAME_ALLOCATOR.lock();
    for region in boot_info.memory_map.iter() {
        if region.region_type == MemoryRegionType::Usable {
            ba.insert(region.range.start_frame_number as usize..region.range.end_frame_number as usize);
        }
    }
}
