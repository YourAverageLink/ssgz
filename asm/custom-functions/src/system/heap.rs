#![allow(non_snake_case)]
use core::{
    alloc::{AllocError, Allocator, GlobalAlloc},
    ffi::{c_char, c_void},
    ptr::NonNull,
    slice,
};

use super::printf;
pub struct Node<T> {
    pub prev: *mut T,
    pub next: *mut T,
}

#[repr(C)]
pub struct List<T> {
    pub head:   *mut T,
    pub tail:   *mut T,
    pub count:  u16,
    pub offset: u16,
}

impl<T> List<T> {
    pub fn get_idx(&self, idx: u16) -> Option<&'static T> {
        if idx >= self.count {
            return None;
        }

        let mut entry = self.head;

        for _ in 0..idx {
            let node = unsafe {
                (((entry as *const u8).add(self.offset as _)) as *const Node<T>).as_ref()
            };
            if let Some(node) = node {
                entry = node.next;
            } else {
                break;
            }
        }

        return unsafe { entry.as_ref() };
    }
}

#[repr(C)]
pub struct MEMiHeapHead {
    pub signature: [u8; 4],
    pub memLink:   [u32; 2],
    pub memList:   List<c_void>,
    pub heapStart: u32,
    pub heapEnd:   u32,
    // more
}

#[repr(C)]
pub struct HeapVtbl {
    pub field_0x00:         u32,
    pub field_0x04:         u32,
    pub dtor:               unsafe extern "C" fn(This: *mut Heap),
    pub getHeapKind:        unsafe extern "C" fn(This: *mut Heap),
    pub initAllocator:      unsafe extern "C" fn(This: *mut Heap, allocator: u32, alignment: i32),
    pub alloc:              unsafe extern "C" fn(This: *const Heap, u32, u32) -> *mut u8,
    pub free:               unsafe extern "C" fn(This: *const Heap, *const u8),
    pub destroy:            unsafe extern "C" fn(This: *mut Heap),
    pub resizeForMBlock:    unsafe extern "C" fn(This: *mut Heap),
    pub getTotalSize:       unsafe extern "C" fn(This: *const Heap) -> u32,
    pub getAllocatableSize: unsafe extern "C" fn(This: *const Heap, alignment: u32) -> u32,
    pub adjust:             unsafe extern "C" fn(This: *mut Heap),
}

#[repr(C)]
pub struct Heap {
    pub vtable:       &'static HeapVtbl,
    pub contain_heap: *mut Heap,
    pub link:         [u32; 2], // node
    pub heap_handle:  *const MEMiHeapHead,
    pub parent_block: *mut c_void,
    pub flag:         u16,
    pub __pad:        u16,
    pub node:         [u32; 2], // node
    pub children:     List<Heap>,
    pub name:         *const c_char,
}

impl Heap {
    pub fn get_total_size(&self) -> u32 {
        unsafe { (*self.heap_handle).heapEnd - (*self.heap_handle).heapStart }
    }
    pub fn get_free_size(&self) -> u32 {
        unsafe { ((*self.vtable).getAllocatableSize)(self, 4) }
    }
    pub fn get_name(&self) -> &'static str {
        let mut num_bytes = 0;
        while unsafe { *self.name.add(num_bytes) } != 0 {
            num_bytes += 1;
        }
        unsafe {
            core::str::from_utf8_unchecked(slice::from_raw_parts(self.name as *const u8, num_bytes))
        }
    }
}

extern "C" {
    static CURRENT_HEAP: *const Heap;
    static HEAP_MEM1: *mut Heap;
    static HEAP_MEM2: *mut Heap;
    static HEAP_LIST: List<Heap>;
    // fn Heap__alloc(size: u32, align: u32, heap: *const Heap) -> *mut c_void;

}

struct HeapAllocator;
#[global_allocator]
static GLOBAL_ALLOCATOR: HeapAllocator = HeapAllocator;

unsafe impl Sync for HeapAllocator {}
unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe {
            printf(
                b"Allocating on: %s\0".as_ptr() as *const i8,
                get_heap_idx(17).unwrap().get_name().as_ptr(),
            )
        };
        get_heap_idx(17)
            .unwrap()
            .allocate(layout)
            .unwrap()
            .as_mut_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        get_heap_idx(17)
            .unwrap()
            .deallocate(NonNull::new(ptr).unwrap(), layout)
    }
}

unsafe impl Allocator for Heap {
    fn allocate(&self, layout: core::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        let data_ptr = unsafe {
            NonNull::new_unchecked((self.vtable.alloc)(
                self,
                layout.size() as u32,
                layout.align() as u32,
            ))
        };

        Ok(NonNull::slice_from_raw_parts(data_ptr, layout.size()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: core::alloc::Layout) {
        let _ = layout;
        unsafe { (self.vtable.free)(self, ptr.as_ptr()) }
    }
}

pub fn get_num_heaps() -> u16 {
    unsafe { HEAP_LIST.count }
}

pub fn get_heap_idx(idx: u16) -> Option<&'static Heap> {
    unsafe { HEAP_LIST.get_idx(idx) }
}

pub fn get_root_heap_mem1() -> *mut Heap {
    unsafe { HEAP_MEM1 }
}

pub fn get_root_heap_mem2() -> *mut Heap {
    unsafe { HEAP_MEM2 }
}
