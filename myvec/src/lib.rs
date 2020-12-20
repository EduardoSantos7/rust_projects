use std::alloc;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.len
    }

    pub fn len(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");

        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).expect("Could not allocate");
            // SAFETY: The layout is hardcoded to be 4 * size_of <T> and
            // size_of <T> is > 0
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr: NonNull<T> = NonNull::new(ptr).expect("Could not allocate memory");
            // SAFETY: ptr is non-null and we have just allocated enough space for
            // this item and 3 more. The memory previously at ptr is not read
            unsafe { ptr.as_ptr().write(item) };

            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Cannot reach memory location");
            assert!(offset < isize::MAX as usize, "Wrapped isize");
            // Offset cannot wrap around and pointer is pointing to valid memory
            // and writing to an offset at self.len iis valid
            unsafe { self.ptr.as_ptr().add(self.len).write(item) }
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);
            let new_capacity = self.capacity.checked_mul(2).expect("Capacity");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align).expect("Can't allocate");

            let ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.ptr = ptr;
            self.capacity = new_capacity;
            self.len += 1;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe {
            &*self.ptr.as_ptr().add(index)
        })
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::size_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let vec: MyVec<usize> = MyVec::new();

        vec.push(1usize);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
    }
}
