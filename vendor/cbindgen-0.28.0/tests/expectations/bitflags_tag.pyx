from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # Constants shared by multiple CSS Box Alignment properties
  #
  # These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
  cdef struct AlignFlags:
    uint8_t bits;
  # 'auto'
  const AlignFlags AlignFlags_AUTO # = <AlignFlags>{ <uint8_t>0 }
  # 'normal'
  const AlignFlags AlignFlags_NORMAL # = <AlignFlags>{ <uint8_t>1 }
  # 'start'
  const AlignFlags AlignFlags_START # = <AlignFlags>{ <uint8_t>(1 << 1) }
  # 'end'
  const AlignFlags AlignFlags_END # = <AlignFlags>{ <uint8_t>(1 << 2) }
  const AlignFlags AlignFlags_ALIAS # = <AlignFlags>{ <uint8_t>(AlignFlags_END).bits }
  # 'flex-start'
  const AlignFlags AlignFlags_FLEX_START # = <AlignFlags>{ <uint8_t>(1 << 3) }
  const AlignFlags AlignFlags_MIXED # = <AlignFlags>{ <uint8_t>(((1 << 4) | (AlignFlags_FLEX_START).bits) | (AlignFlags_END).bits) }
  const AlignFlags AlignFlags_MIXED_SELF # = <AlignFlags>{ <uint8_t>(((1 << 5) | (AlignFlags_FLEX_START).bits) | (AlignFlags_END).bits) }

  cdef struct DebugFlags:
    uint32_t bits;
  # Flag with the topmost bit set of the u32
  const DebugFlags DebugFlags_BIGGEST_ALLOWED # = <DebugFlags>{ <uint32_t>(1 << 31) }

  cdef struct LargeFlags:
    uint64_t bits;
  # Flag with a very large shift that usually would be narrowed.
  const LargeFlags LargeFlags_LARGE_SHIFT # = <LargeFlags>{ <uint64_t>(1ull << 44) }
  const LargeFlags LargeFlags_INVERTED # = <LargeFlags>{ <uint64_t>~(LargeFlags_LARGE_SHIFT).bits }

  cdef struct OutOfLine:
    uint32_t _0;
  const OutOfLine OutOfLine_A # = <OutOfLine>{ <uint32_t>1 }
  const OutOfLine OutOfLine_B # = <OutOfLine>{ <uint32_t>2 }
  const OutOfLine OutOfLine_AB # = <OutOfLine>{ <uint32_t>((OutOfLine_A)._0 | (OutOfLine_B)._0) }

  void root(AlignFlags flags,
            DebugFlags bigger_flags,
            LargeFlags largest_flags,
            OutOfLine out_of_line);
