package info.rgomes

import net.jcip.annotations.NotThreadSafe

import scala.reflect.ClassTag


/** A ring buffer which is simple, zero-cost, branchless and not thread, for people who know what they are doing.
  * <p/>
  * Design Principles
  *<ul>
  * <li>Never provokes garbage collections, except when the ring buffer itself becomes unreachable.</li>
  * <li>Zero cost abstractions, meaning that you are not going to pay for features you do not need.</li>
  * <li>Branchless programming means that the ring buffer always assumes the "happy path", assuming that
  * the programmer knows what he/she is doing. It's a developer responsibility to make sure that there
  * ring buffer is not full before putting elements on it, or ensure that the ring buffer is not empty
  * before taking elements from it. 
  * <li>Not thread-safe, lock-free implementation: leave this concern to the caller;</li>
  * <li>Does not throw exceptions, since exceptions are expensive</li>
  * <li>Compatible with Apache Arrow, which means that you can index the ring buffer with and unsigned long.</li>
  *</ul>
  */
@NotThreadSafe
class RingBuffer[T:ClassTag](val ring: Array[T]) extends RingLike[T]:
  def this(capacity: Int) = this(Array.ofDim[T](capacity))

  private var _head: Int = 0
  private var _tail: Int = 0
  private var _len:  Int = 0

  @inline
  override def capacity: Int = ring.length

  @inline
  override def length: Int = this._len

  @inline
  override def is_index_in_range(index: Int): Boolean =
    (this._tail <= index) && (index < this._head) && (this._len > 0)

  @inline
  override def is_empty: Boolean =
    this._len == 0

  @inline
  override def is_full: Boolean =
    this._len >= capacity

  @inline
  override def get(index: Int): T =
    ring(index % capacity)

  @inline
  override def put(index: Int, o: T): T =
    ring(index % capacity) = o
    o

  @inline
  override def head: T = get(this._head)

  @inline
  override def tail: T = get(this._tail)

  @inline
  override def push(o: T): T =
    put(this._head, o)
    this._len = this._len+1
    this._head = (this._head+1) % capacity
    o

  @inline
  override def pop: T =
    val result = get(this._tail)
    this._len = this._len-1
    this._tail = (this._tail+1) % capacity
    result


@NotThreadSafe
class CheckedRingBuffer[T:ClassTag](override val ring: Array[T]) extends RingBuffer[T](ring):
  @inline
  override def get(index: Int): T =
    if is_index_in_range(index) then
      ring(index % capacity)
    else
      throw new ArithmeticException(s"invalid index: ${index}")

  @inline
  override def put(index: Int, o: T): T =
    if is_index_in_range(index) then
      ring(index % capacity) = o
      o
    else
      throw new ArithmeticException(s"invalid index: ${index}")
