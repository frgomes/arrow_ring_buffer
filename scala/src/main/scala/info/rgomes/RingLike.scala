package info.rgomes

/** `RingLike` defines a common interface for all ring buffers. */
trait RingLike[@scala.specialized T]:
  /** Returns the maximum capacity of the ring buffer */
  def capacity: Int

  /** Returns the actual number of elements in the ring buffer. */
  def length: Int

  /** Returns whether or not a given index is in range, i.e.: satisfies the conditions:
    *  1. tail <= index < head
    *  2. len > 0
    */
  def is_index_in_range(index: Int): Boolean

  /** Returns `true` if the ring buffer is empty. Returns `false` otherwise. */
  def is_empty: Boolean

  /** Returns `true` if the ring buffer is full. Returns `false` otherwise. */
  def is_full: Boolean

  /** Returns an element at absolute position `index`.
    * The actual element position is calculated by performing `index` modulus divide `capacity`.
    * Passing an index which is out of range results in undefined behavior.
    */
  def get(index: Int): T

  /** Stores an element at absolute position `index`.
    * The actual element position is calculated by performing `index` modulus divide `capacity`.
    * Passing an index which is out of range results in undefined behavior.
    */
  def put(index: Int, o: T): T

  /** Returns an element at the last (or the newest) position in the queue.
    * The actual element position is calculated by performing `head` modulus divide `capacity`.
    */
  def head: T

  /** Returns an element (which is a reference &T) at the first (or the oldest) position in the queue.
    * The actual element position is calculated by performing `tail` modulus divide `capacity`.
    */
  def tail: T

  /** Put one element onto the tail position of the ring buffer.
    * Passing an index which is out of range results in undefined behavior.
    */
  def push(o: T): T

  /** Gets one element from the head position of the ring buffer.
    * Passing an index which is out of range results in undefined behavior.
    */
  def pop: T
