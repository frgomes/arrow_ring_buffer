use core::ptr::NonNull;
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use alloc::alloc::{Layout, alloc, handle_alloc_error};


#[cfg(feature = "compat")]
type Index = i32;

#[cfg(not(feature = "compat"))]
type Index = u64;



/// Underlying, low level API for a queue with window support and direct access to elements.
///
/// You can think that this queue maintains pointers to elements allocated and kept somewhere else.
///
/// The lower level API does not perform any range check. The caller code must perform validations and
/// keep invariants in order to guarateee the validity of operations performed by the low level API.
pub struct RingBuffer<'a, T: Sized, const N: Index> {
    memory: NonNull<&'a T>,
    _marker: PhantomData<T>,
    capacity: Index,
    len: Index,
    head: Index,
    tail: Index,
}

impl<'a, T, const N: Index> RingBuffer<'a,T,N> {
    
    /// Creates a ring buffer of size N
    pub fn new() -> Self {
        RingBuffer::<T,N>::with_capacity(N)
    }    

    fn with_capacity(capacity: Index) -> RingBuffer<'a,T,N> {
        let layout = Layout::from_size_align((capacity as usize) * size_of::<T>(), align_of::<T>()).unwrap();
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            handle_alloc_error(layout);
        } else {
            RingBuffer {
            	memory: NonNull::new(ptr).unwrap().cast(),
            	_marker: PhantomData::<T>,
            	len: 0,
            	capacity,
            	head: 0,
            	tail: 0
            }
        }
    }

    /// Returns the maximum capacity of the ring buffer.
    #[inline]
    pub fn capacity(&mut self) -> Index { self.capacity }

    /// Returns the actual number of elements in the ring buffer.
    #[inline]
    pub fn len(&mut self) -> Index { self.len }

    /// Returns whether or not a given index is in range, i.e.: satisfies the conditions:
    ///  1. tail <= index < head
    ///  2. len > 0
    #[inline]
    pub fn is_index_in_range(&self, index: Index) -> bool {
        (self.tail <= index) && (index < self.head) && (self.len > 0)
    }

    /// Returns `true` if the ring buffer is empty. Returns `false` otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool { self.len == 0 }            

    /// Returns `true` if the ring buffer is full. Returns `false` otherwise.
    #[inline]
    pub fn is_full(&self) -> bool { self.len >= self.capacity }

    /// Returns an element (which is a reference &T) at the last (or the newest) position in the queue.
    /// The actual element position is calculated by performing `head` modulus divide `capacity`.
    /// It does not perform any validations before retrieving the element, eventually leading to UB
    /// (undefined behavior) in case the underlying memory location was never initialized.
    #[inline]
    pub fn head(&mut self) -> &'a T {
        self.get(self.head)
    }

    /// Returns an element (which is a reference &T) at the first (or the oldest) position in the queue.
    /// The actual element position is calculated by performing `tail` modulus divide `capacity`.
    /// It does not perform any validations before retrieving the element, eventually leading to UB
    /// (undefined behavior) in case the underlying memory location was never initialized.
    #[inline]
    pub fn tail(&mut self) -> &'a T {
        self.get(self.tail)
    }

    /// Returns an element (which is a reference &T) at absolute position `index`.
    /// The actual element position is calculated by performing `index` modulus divide `capacity`.
    /// Passing an index which is out of range results in undefined behavior.
    #[inline]
    pub fn get(&mut self, index: Index) -> &'a T {
        unsafe { self.memory.as_ptr().add((index % N) as usize).read() }
    }

    /// Stores an element (which is a reference &T) at absolute position `index`.
    /// The actual element position is calculated by performing `index` modulus divide `capacity`.
    /// Passing an index which is out of range results in undefined behavior.
    #[inline]
    pub fn put(&mut self, index: Index, elem: &'a T) -> &'a T {
        unsafe { self.memory.as_ptr().add((index % N) as usize).write(elem); }
        elem
    }

    /// Pushes the given element value to the end of the queue, possibly overwriting elements.
    /// It does not perform any validations before pushing elements.
    /// It may `panic!` if you try to insert more than `Index::MAX` elements.
    #[inline]
    pub fn push(&mut self, elem: &'a T) {
        self.put(self.head, elem);
        self.len  = self.len + 1;  // this may panic!
        self.head = self.head + 1; // this may panic!
    }

    /// Removes an element from the front of the queue.
    /// It does not perform any validations before removing elements.
    /// It may `panic!` if you try to remove elements when the queue is empty.
    /// It may `panic!` if you try to remove more than `Index::MAX` elements.
    /// It may even lead to `tail` ahead of `head`, which break invariant assumptions.
    #[inline]
    pub fn pop(&mut self) -> &'a T {
        let result = self.get(self.tail);
        self.len  = self.len - 1;  // this may panic!
        self.tail = self.tail + 1; // this may panic!
        result
    }

}


#[cfg(test)]
mod tests {
    use super::{Index, RingBuffer};

    #[test]
    fn check_index_size() {
        use core::mem::size_of;
        assert!(4 == size_of::<Index>());
    }

    #[test]
    fn ability_to_overflow() {
        let p = Index::MAX;
        let q = p.wrapping_add(1);
        assert!(Index::MIN == q);
    }

    #[test]
    fn ability_to_underflow() {
        let p = Index::MIN;
        let q = p.wrapping_sub(1);
        assert!(Index::MAX == q);
    }

    #[test]
    fn ability_to_create_size_zero() {
        let mut r: RingBuffer<u32, 0> = RingBuffer::new();
        assert!(0 == r.len());
        assert!(!r.is_index_in_range(0));
    }

    #[test]
    fn ability_to_create_size_one() {
        let mut r: RingBuffer<u32, 1> = RingBuffer::new();
        let v0: u32 = 0;
        let p0: &u32 = &v0;
        assert!(0 == r.len());
        r.push(p0);
        assert!(1 == r.len());
        assert!(r.is_index_in_range(0));
        assert!(p0 == r.get(0));
        assert!(!r.is_index_in_range(1));
    }

    #[test]
    fn ability_to_fill() {
        let mut r: RingBuffer<u32, 2> = RingBuffer::new();
        let v0: u32 = 0;
        let v1: u32 = 1;
        let v2: u32 = 2;
        let p0: &u32 = &v0;
        let p1: &u32 = &v1;
        let p2: &u32 = &v2;
        assert!(0 == r.len());
        r.push(p0);
        assert!(1 == r.len());
        assert!(r.is_index_in_range(0));
        assert!(p0 == r.get(0));
        r.push(p1);
        assert!(2 == r.len());
        assert!(r.is_index_in_range(1));
        assert!(p1 == r.get(1));
        r.push(p2);
        assert!(3 == r.len());
        assert!(r.is_index_in_range(2));
        assert!(p2 == r.get(2));
        assert!(!r.is_index_in_range(3));
    }

    #[test]
    fn ability_to_push_and_pop() {
        let mut r: RingBuffer<u32, 3> = RingBuffer::new();
        let v0: u32 = 0;
        let v1: u32 = 1;
        let v2: u32 = 2;
        let p0: &u32 = &v0;
        let p1: &u32 = &v1;
        let p2: &u32 = &v2;
        //--
        assert!(3 == r.capacity());
        assert!(0 == r.len());
        assert!(r.is_empty());
        assert!(!r.is_full());
        //---
        r.push(p0);
        assert!(3 == r.capacity());
        assert!(1 == r.len());
        assert!(!r.is_empty());
        assert!(!r.is_full());
        //--
        r.push(p1);
        assert!(3 == r.capacity());
        assert!(2 == r.len());
        assert!(!r.is_empty());
        assert!(!r.is_full());
        //--
        r.push(p2);
        assert!(3 == r.capacity());
        assert!(3 == r.len());
        assert!(!r.is_empty());
        assert!(r.is_full());
        //--
        assert!(3 == r.len());
        assert!(r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.pop());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p1 == r.pop());
        assert!(1 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p2 == r.pop());
        assert!(0 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
    }

    #[test]
    fn ability_to_read_from_tail() {
        let mut r: RingBuffer<u32, 3> = RingBuffer::new();
        let v0: u32 = 0;
        let v1: u32 = 1;
        let v2: u32 = 2;
        let p0: &u32 = &v0;
        let p1: &u32 = &v1;
        let p2: &u32 = &v2;
        //--
        r.push(p0);
        r.push(p1);
        r.push(p2);
        assert!(3 == r.len());
        assert!(r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.tail());
        assert!(3 == r.len());
        assert!(r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.pop());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p1 == r.tail());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p1 == r.pop());
        assert!(1 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p2 == r.tail());
        assert!(1 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p2 == r.pop());
        assert!(0 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.tail()); // it is reading from an empty buffer here
        assert!(0 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
    }

    #[test]
    fn ability_to_read_from_head() {
        let mut r: RingBuffer<u32, 3> = RingBuffer::new();
        let v0: u32 = 0;
        let v1: u32 = 1;
        let v2: u32 = 2;
        let p0: &u32 = &v0;
        let p1: &u32 = &v1;
        let p2: &u32 = &v2;
        //--
        r.push(p0);
        r.push(p1);
        r.push(p2);
        assert!(3 == r.len());
        assert!(r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.head());
        assert!(3 == r.len());
        assert!(r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.pop());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.head());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p1 == r.pop());
        assert!(1 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.head());
        assert!(1 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p2 == r.pop());
        assert!(0 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.head()); // it is reading from an empty buffer here
        assert!(0 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
    }

    #[test]
    fn ability_to_go_around() {
        let mut r: RingBuffer<u32, 3> = RingBuffer::new();
        let v0: u32 = 0;
        let v1: u32 = 1;
        let v2: u32 = 2;
        let v3: u32 = 3;
        let v4: u32 = 4;
        let v5: u32 = 5;
        let p0: &u32 = &v0;
        let p1: &u32 = &v1;
        let p2: &u32 = &v2;
        let p3: &u32 = &v3;
        let p4: &u32 = &v4;
        let p5: &u32 = &v5;
        //--
        r.push(p0);
        r.push(p1);
        r.push(p2);
        assert!(3 == r.len());
        assert!(r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.tail());
        assert!(3 == r.len());
        assert!(r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p0 == r.pop());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        assert!(p1 == r.tail());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        //--
        r.push(p3);
        assert!(3 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(!r.is_index_in_range(4));
        //--
        assert!(p1 == r.tail());
        assert!(3 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(!r.is_index_in_range(4));
        //--
        assert!(p1 == r.pop());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(!r.is_index_in_range(4));
        //--
        assert!(p2 == r.tail());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(!r.is_index_in_range(4));
        //--
        r.push(p4);
        assert!(3 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(r.is_index_in_range(4));
        assert!(!r.is_index_in_range(5));
        //--
        assert!(p2 == r.tail());
        assert!(3 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(r.is_index_in_range(4));
        assert!(!r.is_index_in_range(5));
        //--
        assert!(p2 == r.pop());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(r.is_index_in_range(4));
        assert!(!r.is_index_in_range(5));
        //--
        assert!(p3 == r.tail());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(r.is_index_in_range(4));
        assert!(!r.is_index_in_range(5));
        //--
        r.push(p5);
        assert!(3 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(r.is_index_in_range(4));
        assert!(r.is_index_in_range(5));
        assert!(!r.is_index_in_range(6));
        //--
        assert!(p3 == r.tail());
        assert!(3 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(r.is_index_in_range(3));
        assert!(r.is_index_in_range(4));
        assert!(r.is_index_in_range(5));
        assert!(!r.is_index_in_range(6));
        //--
        assert!(p3 == r.pop());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        assert!(r.is_index_in_range(4));
        assert!(r.is_index_in_range(5));
        assert!(!r.is_index_in_range(6));
        //--
        assert!(p4 == r.tail());
        assert!(2 == r.len());
        assert!(!r.is_index_in_range(0));
        assert!(!r.is_index_in_range(1));
        assert!(!r.is_index_in_range(2));
        assert!(!r.is_index_in_range(3));
        assert!(r.is_index_in_range(4));
        assert!(r.is_index_in_range(5));
        assert!(!r.is_index_in_range(6));
    }

}

