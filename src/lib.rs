#[cfg(target_feature = "avx2")]
mod avx2;
#[cfg(target_feature = "avx2")]
use avx2::*;

#[cfg(not(target_feature = "avx2"))]
mod sse;
#[cfg(not(target_feature = "avx2"))]
use sse::*;

/// Validate one vectletor's worth of input bytes
#[inline(always)]
unsafe fn z_validate_vec(bytes: VecT, shifted_bytes: VecT, last_cont: &mut VMaskT) -> bool {
    // Error lookup tables for the first, second, and third nibbles
    let error_1: VecT = v_table_16(
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x06,
        0x38,
    );
    let error_2: VecT = v_table_16(
        0x0B, 0x01, 0x00, 0x00, 0x10, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x24, 0x20,
        0x20,
    );
    let error_3: VecT = v_table_16(
        0x29, 0x29, 0x29, 0x29, 0x29, 0x29, 0x29, 0x29, 0x2B, 0x33, 0x35, 0x35, 0x31, 0x31, 0x31,
        0x31,
    );

    // Quick skip for ascii-only input. If there are no bytes with the high bit
    // set, we don't need to do any more work. We return either valid or
    // invalid based on whether we expected any continuation bytes here.
    let high: VMaskT = v_test_bit_7(bytes);
    if high == 0 {
        return *last_cont == 0;
    }

    // Which bytes are required to be continuation bytes
    let mut req: VMask2T = (*last_cont) as VMask2T;
    // A bitmask of the actual continuation bytes in the input
    let cont: VMaskT;

    // Compute the continuation byte mask by finding bytes that start with
    // 11x, 111x, and 1111. For each of these prefixes, we get a bitmask
    // and shift it forward by 1, 2, or 3. This loop should be unrolled by
    // the compiler, and the (n == 1) branch inside eliminated.
    let mut set: VMaskT = high;

    // n = 1
    set &= v_test_bit_6(bytes);
    // Mark continuation bytes: those that have the high bit set but
    // not the next one

    cont = high ^ set;

    // We add the shifted mask here instead of ORing it, which would
    // be the more natural operation, so that this line can be done
    // with one lea. While adding could give a different result due
    // to carries, this will only happen for invalid UTF-8 sequences,
    // and in a way that won't cause it to pass validation. Reasoning:
    // Any bits for required continuation bytes come after the bits
    // for their leader bytes, and are all contiguous. For a carry to
    // happen, two of these bit sequences would have to overlap. If
    // this is the case, there is a leader byte before the second set
    // of required continuation bytes (and thus before the bit that
    // will be cleared by a carry). This leader byte will not be
    // in the continuation mask, despite being required. QEDish.
    req += (set as VMask2T) << 1;

    // the same with n = 2
    set &= v_test_bit_5(bytes);

    req += (set as VMask2T) << 2;

    // the same with n = 3
    set &= v_test_bit_4(bytes);

    req += (set as VMask2T) << 3;

    // Check that continuation bytes match. We must cast req from vmask2_t
    // (which holds the carry mask in the upper half) to vmask_t, which
    // zeroes out the upper bits
    if cont != (req as VMaskT) {
        return false;
    }

    // Look up error masks for three consecutive nibbles.
    let e_1: VecT = v_lookup_4(error_1, shifted_bytes);
    let e_2: VecT = v_lookup_0(error_2, shifted_bytes);
    let e_3: VecT = v_lookup_4(error_3, bytes);

    // Check if any bits are set in all three error masks
    if v_testz(v_and(e_1, e_2), e_3) == 0 {
        return false;
    }

    // Save continuation bits and input bytes for the next round
    *last_cont = (req >> V_LEN) as VMaskT;
    return true;
}

pub fn validate(data: &[u8]) -> bool {
    let mut bytes: VecT;
    let mut shifted_bytes: VecT;
    let len = data.len();
    let data = data.as_ptr();

    // Keep continuation bits from the previous iteration that carry over to
    // each input chunk vector
    let mut last_cont: VMaskT = 0;

    let mut offset: usize = 0;
    // Deal with the input up until the last section of bytes
    if len >= V_LEN {
        unsafe {
            // We need a vector of the input byte stream shifted forward one byte.
            // Since we don't want to read the memory before the data pointer
            // (which might not even be mapped), for the first chunk of input just
            // use vector instructions.
            shifted_bytes = v_shift_lanes_left(v_load(data as *const VecT));

            // Loop over input in V_LEN-byte chunks, as long as we can safely read
            // that far into memory
            while offset + V_LEN < len {
                bytes = v_load(data.add(offset) as *const VecT);
                if !z_validate_vec(bytes, shifted_bytes, &mut last_cont) {
                    return false;
                }
                shifted_bytes = v_load(data.add(offset + V_LEN - 1) as *const VecT);
                offset += V_LEN
            }
        }
    }
    // Deal with any bytes remaining. Rather than making a separate scalar path,
    // just fill in a buffer, reading bytes only up to len, and load from that.
    if offset < len {
        let mut buffer: [u8; V_LEN + 1] = [0; V_LEN + 1];

        unsafe {
            // use copy?
            if offset > 0 {
                buffer[0] = *data.add(offset - 1);
            }
            for i in 0..(len - offset) {
                buffer[i + 1] = *data.add(offset + i);
            }

            bytes = v_load(buffer.as_ptr().add(1) as *const VecT);
            shifted_bytes = v_load(buffer.as_ptr() as *const VecT);
            if !z_validate_vec(bytes, shifted_bytes, &mut last_cont) {
                return false;
            }
        }
    }

    // The input is valid if we don't have any more expected continuation bytes
    last_cont == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            // Setting both fork and timeout is redundant since timeout implies
            // fork, but both are shown for clarity.
            // Disabled for code coverage, enable to track bugs
            // fork: true,
            .. ProptestConfig::default()
        })]

        #[test]
        fn prop_valid_utf8(s in ".*") {
            assert!(validate(s.as_bytes()))
        }

        #[test]
        fn prop_invalid_utf8(mut s in ".*") {
            if s.len() > 0 {
                let s = unsafe{ s.as_bytes_mut() };
                s[0] = 0;
                let is_valid = validate(s);

                assert!(is_valid == std::str::from_utf8(s).is_ok())
            }
        }
    }
}
