#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// #   define z_validate_utf8  z_validate_utf8_avx2
// #   define z_validate_vec   z_validate_vec_avx2

// #   define V_LEN            (32)

pub(crate) const V_LEN: usize = 32;

// // Vector and vector mask types. We use #defines instead of typedefs so this
// // header can be included multiple times with different configurations

// #   define vec_t            __m256i
pub(crate) type VecT = __m256i;

// #   define vmask_t          uint32_t
pub(crate) type VMaskT = u32;
// #   define vmask2_t         uint64_t
pub(crate) type VMask2T = u64;

// #   define v_load(x)        _mm256_loadu_si256((vec_t *)(x))
#[inline(always)]
pub(crate) unsafe fn v_load(x: *const __m256i) -> __m256i {
    _mm256_loadu_si256(x)
}

// #   define v_set1           _mm256_set1_epi8
#[inline(always)]
pub(crate) unsafe fn v_set1(x: u8) -> __m256i {
    _mm256_set1_epi8(x as i8)
}
// #   define v_and            _mm256_and_si256
#[inline(always)]
pub(crate) unsafe fn v_and(a: __m256i, b: __m256i) -> __m256i {
    _mm256_and_si256(a, b)
}

// #   define v_test_bit(input, bit)                                           \
//         _mm256_movemask_epi8(_mm256_slli_epi16((input), 7 - (bit)))
#[inline(always)]
pub(crate) unsafe fn v_test_bit_7(input: __m256i) -> u32 {
    _mm256_movemask_epi8(_mm256_slli_epi16(input, 7 - 7)) as u32
}

#[inline(always)]
pub(crate) unsafe fn v_test_bit_6(input: __m256i) -> u32 {
    _mm256_movemask_epi8(_mm256_slli_epi16(input, 7 - 6)) as u32
}

#[inline(always)]
pub(crate) unsafe fn v_test_bit_5(input: __m256i) -> u32 {
    _mm256_movemask_epi8(_mm256_slli_epi16(input, 7 - 5)) as u32
}

#[inline(always)]
pub(crate) unsafe fn v_test_bit_4(input: __m256i) -> u32 {
    _mm256_movemask_epi8(_mm256_slli_epi16(input, 7 - 4)) as u32
}

// Parallel table lookup for all bytes in a vector. We need to AND with 0x0F
// for the lookup, because vpshufb has the neat "feature" that negative values
// in an index byte will result in a zero.

// #   define v_lookup(table, index, shift)                                    \
//         _mm256_shuffle_epi8((table),                                        \
//                 v_and(_mm256_srli_epi16((index), (shift)), v_set1(0x0F)))
#[inline(always)]
pub(crate) unsafe fn v_lookup_4(table: __m256i, index: __m256i) -> __m256i {
    _mm256_shuffle_epi8(table, v_and(_mm256_srli_epi16(index, 4), v_set1(0x0F)))
}

#[inline(always)]
pub(crate) unsafe fn v_lookup_0(table: __m256i, index: __m256i) -> __m256i {
    _mm256_shuffle_epi8(table, v_and(_mm256_srli_epi16(index, 0), v_set1(0x0F)))
}

// #   define v_testz          _mm256_testz_si256
#[inline(always)]
pub(crate) unsafe fn v_testz(a: __m256i, b: __m256i) -> i32 {
    _mm256_testz_si256(a, b)
}

// Simple macro to make a vector lookup table for use with vpshufb. Since
// AVX2 is two 16-byte halves, we duplicate the input values.

// #   define V_TABLE_16(...)    _mm256_setr_epi8(__VA_ARGS__, __VA_ARGS__)
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
) -> __m256i {
    _mm256_setr_epi8(
        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v0, v1, v2, v3, v4,
        v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
    )
}

// #   define v_shift_lanes_left v_shift_lanes_left_avx2

// Move all the bytes in "input" to the left by one and fill in the first byte
// with zero. Since AVX2 generally works on two separate 16-byte vectors glued
// together, this needs two steps. The permute2x128 takes the middle 32 bytes
// of the 64-byte concatenation v_zero:input. The align then gives the final
// result in each half:
//      top half: input_L:input_H --> input_L[15]:input_H[0:14]
//   bottom half:  zero_H:input_L -->  zero_H[15]:input_L[0:14]
// static inline vec_t v_shift_lanes_left(vec_t input) {
//     vec_t zero = v_set1(0);
//     vec_t shl_16 = _mm256_permute2x128_si256(input, zero, 0x03);
//     return _mm256_alignr_epi8(input, shl_16, 15);
// }
#[inline(always)]

pub(crate) unsafe fn v_shift_lanes_left(input: __m256i) -> __m256i {
    let zero = v_set1(0);
    let shl_16 = _mm256_permute2x128_si256(input, zero, 0x03);
    _mm256_alignr_epi8(input, shl_16, 15)
}
