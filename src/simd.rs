//! SIMD-optimized ASCII detection.
//!
//! Works around LLVM codegen issues with `-C target-cpu=native` on AVX512 CPUs.

/// Check if all bytes are ASCII (< 128).
#[inline]
pub fn is_ascii(bytes: &[u8]) -> bool {
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx512bw") {
        return unsafe { is_ascii_avx512(bytes) };
    }
    bytes.is_ascii()
}

/// Check if all characters in the string are ASCII.
#[inline]
pub fn is_ascii_str(s: &str) -> bool {
    is_ascii(s.as_bytes())
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512bw")]
/// # Safety
/// Caller must ensure AVX512BW is available (via `is_x86_feature_detected!`).
unsafe fn is_ascii_avx512(bytes: &[u8]) -> bool {
    use core::arch::x86_64::*;

    let ptr = bytes.as_ptr();
    let len = bytes.len();
    let mut i = 0;

    // Process 64-byte chunks
    // SAFETY: Loop condition ensures i+64 <= len
    while i + 64 <= len {
        if _mm512_movepi8_mask(_mm512_loadu_si512(ptr.add(i).cast())) != 0 {
            return false;
        }
        i += 64;
    }

    // Tail: masked load for remaining bytes
    if i < len {
        // SAFETY: Mask has only `len - i` bits set, so only valid bytes are loaded
        let mask = (1u64 << (len - i)) - 1;
        if _mm512_movepi8_mask(_mm512_maskz_loadu_epi8(mask, ptr.add(i).cast())) != 0 {
            return false;
        }
    }

    true
}
