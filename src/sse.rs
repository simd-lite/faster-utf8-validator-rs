#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// // SSE definitions. We require at least SSE4.1 for _mm_test_all_zeros()

// #   define z_validate_utf8  z_validate_utf8_sse4
// #   define z_validate_vec   z_validate_vec_sse4

// #   define V_LEN            (16)
pub(crate) const V_LEN: usize = 16;

// #   define vec_t            __m128i
pub(crate) type VecT = __m128i;

// #   define vmask_t          uint16_t
pub(crate) type VMaskT = u16;

// #   define vmask2_t         uint32_t
pub(crate) type VMask2T = u32;

// #   define v_load(x)        _mm_lddqu_si128((vec_t *)(x))
#[inline(always)]
pub(crate) unsafe fn v_load(x: *const __m128i) -> __m128i {
    _mm_lddqu_si128(x)
}
// #   define v_set1           _mm_set1_epi8
#[inline(always)]
pub(crate) unsafe fn v_set1(x: u8) -> __m128i {
    _mm_set1_epi8(x as i8)
}
// #   define v_and            _mm_and_si128
#[inline(always)]
pub(crate) unsafe fn v_and(a: __m128i, b: __m128i) -> __m128i {
    _mm_and_si128(a, b)
}
// #   define v_testz          _mm_test_all_zeros
#[inline(always)]
pub(crate) unsafe fn v_testz(a: __m128i, b: __m128i) -> i32 {
    _mm_test_all_zeros(a, b)
}

// We have to seperate them since rust requires the second argument
// of _mm_slli_epi16 to be a constant
// #   define v_test_bit(input, bit)                                           \
//         _mm_movemask_epi8(_mm_slli_epi16((input), (uint8_t)(7 - (bit))))
#[inline(always)]
pub(crate) unsafe fn v_test_bit_7(input: __m128i) -> u16 {
    _mm_movemask_epi8(_mm_slli_epi16(input, 7 - 7)) as u16
}

#[inline(always)]
pub(crate) unsafe fn v_test_bit_6(input: __m128i) -> u16 {
    _mm_movemask_epi8(_mm_slli_epi16(input, 7 - 6)) as u16
}

#[inline(always)]
pub(crate) unsafe fn v_test_bit_5(input: __m128i) -> u16 {
    _mm_movemask_epi8(_mm_slli_epi16(input, 7 - 5)) as u16
}

#[inline(always)]
pub(crate) unsafe fn v_test_bit_4(input: __m128i) -> u16 {
    _mm_movemask_epi8(_mm_slli_epi16(input, 7 - 4)) as u16
}

// #   define v_lookup(table, index, shift)                                    \
//         _mm_shuffle_epi8((table),                                           \
//                 v_and(_mm_srli_epi16((index), (shift)), v_set1(0x0F)))
#[inline(always)]
pub(crate) unsafe fn v_lookup_4(table: __m128i, index: __m128i) -> __m128i {
    _mm_shuffle_epi8(table, v_and(_mm_srli_epi16(index, 4), v_set1(0x0F)))
}

#[inline(always)]
pub(crate) unsafe fn v_lookup_0(table: __m128i, index: __m128i) -> __m128i {
    _mm_shuffle_epi8(table, v_and(_mm_srli_epi16(index, 0), v_set1(0x0F)))
}

// #   define V_TABLE_16(...)  _mm_setr_epi8(__VA_ARGS__)
#[inline(always)]
pub(crate) unsafe fn v_table_16(
    v0: i8,
    v1: i8,
    v2: i8,
    v3: i8,
    v4: i8,
    v5: i8,
    v6: i8,
    v7: i8,
    v8: i8,
    v9: i8,
    v10: i8,
    v11: i8,
    v12: i8,
    v13: i8,
    v14: i8,
    v15: i8,
) -> __m128i {
    _mm_setr_epi8(
        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
    )
}
// #   define v_shift_lanes_left v_shift_lanes_left_sse4
// static inline vec_t v_shift_lanes_left(vec_t top) {
//     return _mm_alignr_epi8(top, v_set1(0), 15);
// }

pub(crate) unsafe fn v_shift_lanes_left(top: __m128i) -> __m128i {
    _mm_alignr_epi8(top, v_set1(0), 15)
}
