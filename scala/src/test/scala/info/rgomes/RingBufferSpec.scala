package info.rgomes

import utest._

// @formatter:off


object RingBufferSpec extends TestSuite {
  val tests = this {
    "Ability to create a zero sized ring buffer"-{
      val ring = new RingBuffer[Int](0)
      assert(ring.length == 0)
    }

    "Ability to create a one sized ring buffer"-{
      val ring = new RingBuffer[Int](1)
      assert(ring.length   == 0)
      assert(ring.push(10) == 10)
      assert(ring.length == 1)
      assert(ring.pop    == 10)
      assert(ring.length == 0)
      assert(ring.pop    == 10)
      assert(ring.length == -1)
    }

    "Ability to put elements until it is full"-{
      val ring = new RingBuffer[Int](4)
      assert(ring.push(10) == 10)
      assert(ring.length   == 1)
      assert(ring.push(20) == 20)
      assert(ring.length   == 2)
      assert(ring.push(30) == 30)
      assert(ring.length   == 3)
      assert(ring.push(40) == 40)
      assert(ring.length   == 4)
      assert(ring.push(50) == 50)
      assert(ring.length   == 5)
      assert(ring.push(60) == 60)
      assert(ring.length   == 6)
    }

    "Ability to take elements until it is empty"-{
      val ring = new RingBuffer[Int](4)
      assert(ring.push(10) == 10)
      assert(ring.push(20) == 20)
      assert(ring.push(30) == 30)
      assert(ring.push(40) == 40)
      assert(ring.length == 4)
      assert(ring.pop    == 10)
      assert(ring.length == 3)
      assert(ring.pop    == 20)
      assert(ring.length == 2)
      assert(ring.pop    == 30)
      assert(ring.length == 1)
      assert(ring.pop    == 40)
      assert(ring.length == 0)
      assert(ring.pop    == 10)
      assert(ring.length == -1)
      assert(ring.pop    == 20)
      assert(ring.length == -2)
    }

    "Ability to roll over the internal boundaries."-{
      // i.e: ability to manage ``head`` and ``tail`` properly.
      "First we put two elements and take one in each test."-{
        val ring = new RingBuffer[Int](4)
        assert(ring.push(10) == 10)
        assert(ring.push(20) == 20)
        assert(ring.pop    == 10)
        assert(ring.length == 1)

        assert(ring.push(30) == 30)
        assert(ring.push(40) == 40)
        assert(ring.pop    == 20)
        assert(ring.length == 2)

        assert(ring.push(50) == 50)
        assert(ring.push(60) == 60)
        assert(ring.pop    == 30)
        assert(ring.length == 3)

        assert(ring.push(70) == 70)
        assert(ring.push(80) == 80)
        assert(ring.pop    == 80) // rollover
        assert(ring.length == 4)
      }
    }
  }
}
