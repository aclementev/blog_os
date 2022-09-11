use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{
        FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map
    ///
    /// # Safety
    /// The caller must guarantee that the passed memory map is valid.
    /// The main requirement is that all frames that are marked as `USABLE`
    /// in it are really unused
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // transform into an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/// Initialize a new OffsetPageTable
///
/// # Safety
///
/// The caller must guarantee that the complete physical memory is mapped
/// to virtual memory at the passed `physical_memory_offset`. Also, this
/// function must be only called once to avoid aliaing `&mut` references
/// (which is undefined behaviour)
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table
///
/// # Safety
///
/// The caller must guarantee that the complete physical memory is mapped
/// to virtual memory at the passed `physical_memory_offset`. Also, this
/// function must be only called once to avoid aliaing `&mut` references
/// (which is undefined behaviour)
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    // TODO(alvaro): Can there be a way that we enforce that this function
    // is only called once? can we store somewhere that it was called and
    // check it before panicking? could we enforce that in a multithreaded
    // environment?
    use x86_64::registers::control::Cr3;

    let (level_4_page_table_frame, _) = Cr3::read();

    let phys = level_4_page_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();

    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    // This is unsafe
    &mut *page_table_ptr
}
