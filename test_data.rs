/// Returns the 1-based index of the failing test, or 0 if all tests succeeded.
#[inline(never)]
fn test_data() -> u32 {
	let mut error_index: u32 = 1;

    // Test `chkstk`.
    #[inline(never)]
    fn big_stack() -> [u8; 0x2001] {
        return [0; 0x2001];
    }
    if big_stack()[0] != 0 {
        return error_index;
    }
	error_index += 1;

    // Test 64 bit calculations.
	const VALUES: [i64; 10] = [
		3, 37,
		0x7FFF_FFFF, 0x8000_0000,
		0x1_0000_0000, 0x1_2345_6789,
		0x7FFF_FFFF_FFFF_FFFF, 0x8000_0000_0000_0000u64 as i64,
		-4, -38
	];

	const ALLDIV_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
		1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		12, 1, 0, 0, 0, 0, 0, 0, -9, 0,
		0x2AAAAAAA, 0x3759F22, 1, 0, 0, 0, 0, 0, -0x1FFFFFFF, -0x35E50D7,
		0x2AAAAAAA, 0x3759F22, 1, 1, 0, 0, 0, 0, -0x20000000, -0x35E50D7,
		0x55555555, 0x6EB3E45, 2, 2, 1, 0, 0, 0, -0x40000000, -0x6BCA1AF,
		0x61172283, 0x7DF47FC, 2, 2, 1, 1, 0, 0, -0x48D159E2, -0x7AA3F5B,
		0x2AAAAAAAAAAAAAAA, 0x3759F22983759F2, 0x100000002, 0xFFFFFFFF, 0x7FFFFFFF, 0x70800000, 1, 0, -0x1FFFFFFFFFFFFFFF, -0x35E50D79435E50D,
		-0x2AAAAAAAAAAAAAAA, -0x3759F22983759F2, -0x100000002, -0x100000000, -0x80000000, -0x70800000, -1, 1, 0x2000000000000000, 0x35E50D79435E50D,
		-1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
		-12, -1, 0, 0, 0, 0, 0, 0, 9, 1
	];

	const AULLDIV_RESULTS: [u64; VALUES.len() * VALUES.len()] = [
		1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		12, 1, 0, 0, 0, 0, 0, 0, 0, 0,
		0x2AAAAAAA, 0x3759F22, 1, 0, 0, 0, 0, 0, 0, 0,
		0x2AAAAAAA, 0x3759F22, 1, 1, 0, 0, 0, 0, 0, 0,
		0x55555555, 0x6EB3E45, 2, 2, 1, 0, 0, 0, 0, 0,
		0x61172283, 0x7DF47FC, 2, 2, 1, 1, 0, 0, 0, 0,
		0x2AAAAAAAAAAAAAAA, 0x3759F22983759F2, 0x100000002, 0xFFFFFFFF, 0x7FFFFFFF, 0x70800000, 1, 0, 0, 0,
		0x2AAAAAAAAAAAAAAA, 0x3759F22983759F2, 0x100000002, 0x100000000, 0x80000000, 0x70800000, 1, 1, 0, 0,
		0x5555555555555554, 0x6EB3E45306EB3E4, 0x200000004, 0x1FFFFFFFF, 0xFFFFFFFF, 0xE1000000, 1, 1, 1, 1,
		0x5555555555555548, 0x6EB3E45306EB3E3, 0x200000003, 0x1FFFFFFFF, 0xFFFFFFFF, 0xE1000000, 1, 1, 0, 1
	];

	const ALLDVRM_RESULTS: [(i64, i64); VALUES.len() * VALUES.len()] = [
		(1, 0), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3),
		(12, 1), (1, 0), (0, 0x25), (0, 0x25), (0, 0x25), (0, 0x25), (0, 0x25), (0, 0x25), (-9, 1), (0, 0x25),
		(0x2AAAAAAA, 1), (0x3759F22, 0x15), (1, 0), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (-0x1FFFFFFF, 3), (-0x35E50D7, 0x15),
		(0x2AAAAAAA, 2), (0x3759F22, 0x16), (1, 1), (1, 0), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (-0x20000000, 0), (-0x35E50D7, 0x16),
		(0x55555555, 1), (0x6EB3E45, 7), (2, 2), (2, 0), (1, 0), (0, 0x100000000), (0, 0x100000000), (0, 0x100000000), (-0x40000000, 0), (-0x6BCA1AF, 6),
		(0x61172283, 0), (0x7DF47FC, 0x1D), (2, 0x2345678B), (2, 0x23456789), (1, 0x23456789), (1, 0), (0, 0x123456789), (0, 0x123456789), (-0x48D159E2, 1), (-0x7AA3F5B, 7),
		(0x2AAAAAAAAAAAAAAA, 1), (0x3759F22983759F2, 5), (0x100000002, 1), (0xFFFFFFFF, 0x7FFFFFFF), (0x7FFFFFFF, 0xFFFFFFFF), (0x70800000, 0x4B7FFFFF), (1, 0), (0, 0x7FFFFFFFFFFFFFFF), (-0x1FFFFFFFFFFFFFFF, 3), (-0x35E50D79435E50D, 0x11),
		(-0x2AAAAAAAAAAAAAAA, -2), (-0x3759F22983759F2, -6), (-0x100000002, -2), (-0x100000000, 0), (-0x80000000, 0), (-0x70800000, -0x4B800000), (-1, -1), (1, 0), (0x2000000000000000, 0), (0x35E50D79435E50D, -0x12),
		(-1, -1), (0, -4), (0, -4), (0, -4), (0, -4), (0, -4), (0, -4), (0, -4), (1, 0), (0, -4),
		(-12, -2), (-1, -1), (0, -0x26), (0, -0x26), (0, -0x26), (0, -0x26), (0, -0x26), (0, -0x26), (9, -2), (1, 0)
	];

	const AULLDVRM_RESULTS: [(u64, u64); VALUES.len() * VALUES.len()] = [
		(1, 0), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3),
		(12, 1), (1, 0), (0, 0x25), (0, 0x25), (0, 0x25), (0, 0x25), (0, 0x25), (0, 0x25), (0, 0x25), (0, 0x25),
		(0x2AAAAAAA, 1), (0x3759F22, 0x15), (1, 0), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF),
		(0x2AAAAAAA, 2), (0x3759F22, 0x16), (1, 1), (1, 0), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000),
		(0x55555555, 1), (0x6EB3E45, 7), (2, 2), (2, 0), (1, 0), (0, 0x100000000), (0, 0x100000000), (0, 0x100000000), (0, 0x100000000), (0, 0x100000000),
		(0x61172283, 0), (0x7DF47FC, 0x1D), (2, 0x2345678B), (2, 0x23456789), (1, 0x23456789), (1, 0), (0, 0x123456789), (0, 0x123456789), (0, 0x123456789), (0, 0x123456789),
		(0x2AAAAAAAAAAAAAAA, 1), (0x3759F22983759F2, 5), (0x100000002, 1), (0xFFFFFFFF, 0x7FFFFFFF), (0x7FFFFFFF, 0xFFFFFFFF), (0x70800000, 0x4B7FFFFF), (1, 0), (0, 0x7FFFFFFFFFFFFFFF), (0, 0x7FFFFFFFFFFFFFFF), (0, 0x7FFFFFFFFFFFFFFF),
		(0x2AAAAAAAAAAAAAAA, 2), (0x3759F22983759F2, 6), (0x100000002, 2), (0x100000000, 0), (0x80000000, 0), (0x70800000, 0x4B800000), (1, 1), (1, 0), (0, -0x8000000000000000i64 as u64), (0, -0x8000000000000000i64 as u64),
		(0x5555555555555554, 0), (0x6EB3E45306EB3E4, 8), (0x200000004, 0), (0x1FFFFFFFF, 0x7FFFFFFC), (0xFFFFFFFF, 0xFFFFFFFC), (0xE1000000, 0x96FFFFFC), (1, 0x7FFFFFFFFFFFFFFD), (1, 0x7FFFFFFFFFFFFFFC), (1, 0), (1, 0x22),
		(0x5555555555555548, 2), (0x6EB3E45306EB3E3, 11), (0x200000003, 0x7FFFFFDD), (0x1FFFFFFFF, 0x7FFFFFDA), (0xFFFFFFFF, 0xFFFFFFDA), (0xE1000000, 0x96FFFFDA), (1, 0x7FFFFFFFFFFFFFDB), (1, 0x7FFFFFFFFFFFFFDA), (0, -0x26i64 as u64), (1, 0)
	];

	const ALLMUL_AULLMUL_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
		9, 0x6F, 0x17FFFFFFD, 0x180000000, 0x300000000, 0x369D0369B, 0x7FFFFFFFFFFFFFFD, -0x8000000000000000, -12, -0x72,
		0x6F, 0x559, 0x127FFFFFDB, 0x1280000000, 0x2500000000, 0x2A1907F6CD, 0x7FFFFFFFFFFFFFDB, -0x8000000000000000, -0x94, -0x57E,
		0x17FFFFFFD, 0x127FFFFFDB, 0x3FFFFFFF00000001, 0x3FFFFFFF80000000, 0x7FFFFFFF00000000, -0x6E5D4C3CA3456789, 0x7FFFFFFF80000001, -0x8000000000000000, -0x1FFFFFFFC, -0x12FFFFFFDA,
		0x180000000, 0x1280000000, 0x3FFFFFFF80000000, 0x4000000000000000, -0x8000000000000000, -0x6E5D4C3B80000000, -0x80000000, 0, -0x200000000, -0x1300000000,
		0x300000000, 0x2500000000, 0x7FFFFFFF00000000, -0x8000000000000000, 0, 0x2345678900000000, -0x100000000, 0, -0x400000000, -0x2600000000,
		0x369D0369B, 0x2A1907F6CD, -0x6E5D4C3CA3456789, -0x6E5D4C3B80000000, 0x2345678900000000, 0x4B66DC326FB98751, 0x7FFFFFFEDCBA9877, -0x8000000000000000, -0x48D159E24, -0x2B3C4D5E56,
		0x7FFFFFFFFFFFFFFD, 0x7FFFFFFFFFFFFFDB, 0x7FFFFFFF80000001, -0x80000000, -0x100000000, 0x7FFFFFFEDCBA9877, 1, -0x8000000000000000, 4, 0x26,
		-0x8000000000000000, -0x8000000000000000, -0x8000000000000000, 0, 0, -0x8000000000000000, -0x8000000000000000, 0, 0, 0,
		-12, -0x94, -0x1FFFFFFFC, -0x200000000, -0x400000000, -0x48D159E24, 4, 0, 16, 0x98,
		-0x72, -0x57E, -0x12FFFFFFDA, -0x1300000000, -0x2600000000, -0x2B3C4D5E56, 0x26, 0, 0x98, 0x5A4
	];

	const ALLREM_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
		0, 3, 3, 3, 3, 3, 3, 3, 3, 3,
		1, 0, 0x25, 0x25, 0x25, 0x25, 0x25, 0x25, 1, 0x25,
		1, 0x15, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 3, 0x15,
		2, 0x16, 1, 0, 0x80000000, 0x80000000, 0x80000000, 0x80000000, 0, 0x16,
		1, 7, 2, 0, 0, 0x100000000, 0x100000000, 0x100000000, 0, 6,
		0, 0x1D, 0x2345678B, 0x23456789, 0x23456789, 0, 0x123456789, 0x123456789, 1, 7,
		1, 5, 1, 0x7FFFFFFF, 0xFFFFFFFF, 0x4B7FFFFF, 0, 0x7FFFFFFFFFFFFFFF, 3, 0x11,
		-2, -6, -2, 0, 0, -0x4B800000, -1, 0, 0, -0x12,
		-1, -4, -4, -4, -4, -4, -4, -4, 0, -4,
		-2, -1, -0x26, -0x26, -0x26, -0x26, -0x26, -0x26, -2, 0
	];

	const AULLREM_RESULTS: [u64; VALUES.len() * VALUES.len()] = [
		0, 3, 3, 3, 3, 3, 3, 3, 3, 3,
		1, 0, 0x25, 0x25, 0x25, 0x25, 0x25, 0x25, 0x25, 0x25,
		1, 0x15, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF,
		2, 0x16, 1, 0, 0x80000000, 0x80000000, 0x80000000, 0x80000000, 0x80000000, 0x80000000,
		1, 7, 2, 0, 0, 0x100000000, 0x100000000, 0x100000000, 0x100000000, 0x100000000,
		0, 0x1D, 0x2345678B, 0x23456789, 0x23456789, 0, 0x123456789, 0x123456789, 0x123456789, 0x123456789,
		1, 5, 1, 0x7FFFFFFF, 0xFFFFFFFF, 0x4B7FFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF,
		2, 6, 2, 0, 0, 0x4B800000, 1, 0, -0x8000000000000000i64 as u64, -0x8000000000000000i64 as u64,
		0, 8, 0, 0x7FFFFFFC, 0xFFFFFFFC, 0x96FFFFFC, 0x7FFFFFFFFFFFFFFD, 0x7FFFFFFFFFFFFFFC, 0, 0x22,
		2, 11, 0x7FFFFFDD, 0x7FFFFFDA, 0xFFFFFFDA, 0x96FFFFDA, 0x7FFFFFFFFFFFFFDB, 0x7FFFFFFFFFFFFFDA, -0x26i64 as u64, 0
	];

	const ALLSHL_AULLSHL_RUST_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
		0x18, 0x6000000000, -0x8000000000000000, 3, 3, 0x600, -0x8000000000000000, 3, 0x3000000000000000, 0xC000000,
		0x128, 0x4A000000000, -0x8000000000000000, 0x25, 0x25, 0x4A00, -0x8000000000000000, 0x25, 0x5000000000000000, 0x94000000,
		0x3FFFFFFF8, -0x2000000000, -0x8000000000000000, 0x7FFFFFFF, 0x7FFFFFFF, 0xFFFFFFFE00, -0x8000000000000000, 0x7FFFFFFF, -0x1000000000000000, 0x1FFFFFFFC000000,
		0x400000000, 0, 0, 0x80000000, 0x80000000, 0x10000000000, 0, 0x80000000, 0, 0x200000000000000,
		0x800000000, 0, 0, 0x100000000, 0x100000000, 0x20000000000, 0, 0x100000000, 0, 0x400000000000000,
		0x91A2B3C48, 0x68ACF12000000000, -0x8000000000000000, 0x123456789, 0x123456789, 0x2468ACF1200, -0x8000000000000000, 0x123456789, -0x7000000000000000, 0x48D159E24000000,
		-8, -0x2000000000, -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, -0x200, -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, -0x1000000000000000, -0x4000000,
		0, 0, 0, -0x8000000000000000, -0x8000000000000000, 0, 0, -0x8000000000000000, 0, 0,
		-0x20, -0x8000000000, 0, -4, -4, -0x800, 0, -4, -0x4000000000000000, -0x10000000,
		-0x130, -0x4C000000000, 0, -0x26, -0x26, -0x4C00, 0, -0x26, -0x6000000000000000, -0x98000000
	];

	const ALLSHL_AULLSHL_INTRIN_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
		0x18, 0x6000000000, 0/**/, 3, 3, 0/**/, 0/**/, 3, 0/**/, 0/**/,
		0x128, 0x4A000000000, 0/**/, 0x25, 0x25, 0/**/, 0/**/, 0x25, 0/**/, 0/**/,
		0x3FFFFFFF8, -0x2000000000, 0/**/, 0x7FFFFFFF, 0x7FFFFFFF, 0/**/, 0/**/, 0x7FFFFFFF, 0/**/, 0/**/,
		0x400000000, 0, 0, 0x80000000, 0x80000000, 0/**/, 0, 0x80000000, 0, 0/**/,
		0x800000000, 0, 0, 0x100000000, 0x100000000, 0/**/, 0, 0x100000000, 0, 0/**/,
		0x91A2B3C48, 0x68ACF12000000000, 0/**/, 0x123456789, 0x123456789, 0/**/, 0/**/, 0x123456789, 0/**/, 0/**/,
		-8, -0x2000000000, 0/**/, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0/**/, 0/**/, 0x7FFFFFFFFFFFFFFF, 0/**/, 0/**/,
		0, 0, 0, -0x8000000000000000, -0x8000000000000000, 0, 0, -0x8000000000000000, 0, 0,
		-0x20, -0x8000000000, 0, -4, -4, 0/**/, 0, -4, 0/**/, 0/**/,
		-0x130, -0x4C000000000, 0, -0x26, -0x26, 0/**/, 0, -0x26, 0/**/, 0/**/
	];

	macro_rules! assert_results {
		($error_index:ident, $values:ident, $(($func:ident, $t:ty, $results:expr $(, $($conv:tt)+)?)),+) => {{
			$(
				let results = $results;
				if $values.len() * $values.len() != results.len() {
					return i32::MIN as u32 | $error_index;
				}

				for (i, l) in $values.iter().enumerate() {
					for (j, r) in $values.iter().enumerate() {
						if $func(*l as $t, *r as $t) != results[i * $values.len() + j] $($($conv)+)? {
							return $error_index;
						}
						$error_index += 1;
					}
				}
			)+
		}};
	}

	assert_results!(error_index, VALUES,
		// Expected: Rust and intrinsic produce the same results.
		(alldiv, i64, ALLDIV_RESULTS), (alldiv_, i64, ALLDIV_RESULTS),
		// Expected: Rust and intrinsic produce the same results.
		(aulldiv, u64, AULLDIV_RESULTS), (aulldiv_, u64, AULLDIV_RESULTS),
		// Expected: Rust and intrinsic produce the same results.
		(alldvrm, i64, ALLDVRM_RESULTS), (alldvrm_, i64, ALLDVRM_RESULTS),
		// Expected: Rust and intrinsic produce the same results.
		(aulldvrm, u64, AULLDVRM_RESULTS), (aulldvrm_, u64, AULLDVRM_RESULTS),
		// Expected: Rust and intrinsic produce the same results for signed and unsigned.
		(allmul, i64, ALLMUL_AULLMUL_RESULTS, as i64), (allmul_, i64, ALLMUL_AULLMUL_RESULTS, as i64),
		(aullmul, u64, ALLMUL_AULLMUL_RESULTS, as u64), (aullmul_, u64, ALLMUL_AULLMUL_RESULTS, as u64),
		// Expected: Rust and intrinsic produce the same results.
		(allrem, i64, ALLREM_RESULTS), (allrem_, i64, ALLREM_RESULTS),
		// Expected: Rust and intrinsic produce the same results.
		(aullrem, u64, AULLREM_RESULTS), (aullrem_, u64, AULLREM_RESULTS),
		// Unexpected: Rust produces the same results for signed and unsigned, but differs from intrinsic, which produces the same results for signed and unsigned.
		(allshl, i64, ALLSHL_AULLSHL_RUST_RESULTS, as i64), (aullshl, u64, ALLSHL_AULLSHL_RUST_RESULTS, as u64),
		(allshl_, i64, ALLSHL_AULLSHL_INTRIN_RESULTS, as i64), (aullshl_, u64, ALLSHL_AULLSHL_INTRIN_RESULTS, as u64),
		// Unexpected: Rust and intrinsic differ, see /**/ markers.
		(allshr, i64, [
			0, 0, 0, 3, 3, 0, 0, 3, 0, 0,
			4, 0, 0, 0x25, 0x25, 0, 0, 0x25, 0, 0,
			0xFFFFFFF, 0, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0x3FFFFF, 0, 0x7FFFFFFF, 0, 0x1F,
			0x10000000, 0, 0, 0x80000000, 0x80000000, 0x400000, 0, 0x80000000, 0, 0x20,
			0x20000000, 0, 0, 0x100000000, 0x100000000, 0x800000, 0, 0x100000000, 0, 0x40,
			0x2468ACF1, 0, 0, 0x123456789, 0x123456789, 0x91A2B3, 0, 0x123456789, 0, 0x48,
			0xFFFFFFFFFFFFFFF, 0x3FFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x3FFFFFFFFFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 7, 0x1FFFFFFFFF,
			-0x1000000000000000, -0x4000000, -1, -0x8000000000000000, -0x8000000000000000, -0x40000000000000, -1, -0x8000000000000000, -8, -0x2000000000,
			-1, -1, -1, -4, -4, -1, -1, -4, -1, -1,
			-5, -1, -1, -0x26, -0x26, -1, -1, -0x26, -1, -1
		]),
		(allshr_, i64, [
			0, 0, 0, 3, 3, 0, 0, 3, 0, 0,
			4, 0, 0, 0x25, 0x25, 0, 0, 0x25, 0, 0,
			0xFFFFFFF, 0, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0/**/, 0, 0x7FFFFFFF, 0, 0/**/,
			0x10000000, 0, 0, 0x80000000, 0x80000000, 0/**/, 0, 0x80000000, 0, 0/**/,
			0x20000000, 0, 0, 0x100000000, 0x100000000, 0/**/, 0, 0x100000000, 0, 0/**/,
			0x2468ACF1, 0, 0, 0x123456789, 0x123456789, 0/**/, 0, 0x123456789, 0, 0/**/,
			0xFFFFFFFFFFFFFFF, 0x3FFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0/**/, 0, 0x7FFFFFFFFFFFFFFF, 0/**/, 0/**/,
			-0x1000000000000000, -0x4000000, -1, -0x8000000000000000, -0x8000000000000000, -1/**/, -1, -0x8000000000000000, -1/**/, -1/**/,
			-1, -1, -1, -4, -4, -1, -1, -4, -1, -1,
			-5, -1, -1, -0x26, -0x26, -1, -1, -0x26, -1, -1
		]),
		// Unexpected: Rust and intrinsic differ, see /**/ markers.
		(aullshr, u64, [
			0, 0, 0, 3, 3, 0, 0, 3, 0, 0,
			4, 0, 0, 0x25, 0x25, 0, 0, 0x25, 0, 0,
			0xFFFFFFF, 0, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0x3FFFFF, 0, 0x7FFFFFFF, 0, 0x1F,
			0x10000000, 0, 0, 0x80000000, 0x80000000, 0x400000, 0, 0x80000000, 0, 0x20,
			0x20000000, 0, 0, 0x100000000, 0x100000000, 0x800000, 0, 0x100000000, 0, 0x40,
			0x2468ACF1, 0, 0, 0x123456789, 0x123456789, 0x91A2B3, 0, 0x123456789, 0, 0x48,
			0xFFFFFFFFFFFFFFF, 0x3FFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x3FFFFFFFFFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 7, 0x1FFFFFFFFF,
			0x1000000000000000, 0x4000000, 1, -0x8000000000000000i64 as u64, -0x8000000000000000i64 as u64, 0x40000000000000, 1, -0x8000000000000000i64 as u64, 8, 0x2000000000,
			0x1FFFFFFFFFFFFFFF, 0x7FFFFFF, 1, -0x4i64 as u64, -0x4i64 as u64, 0x7FFFFFFFFFFFFF, 1, -0x4i64 as u64, 15, 0x3FFFFFFFFF,
			0x1FFFFFFFFFFFFFFB, 0x7FFFFFF, 1, -0x26i64 as u64, -0x26i64 as u64, 0x7FFFFFFFFFFFFF, 1, -0x26i64 as u64, 15, 0x3FFFFFFFFF
		]),
		(aullshr_, u64, [
			0, 0, 0, 3, 3, 0, 0, 3, 0, 0,
			4, 0, 0, 0x25, 0x25, 0, 0, 0x25, 0, 0,
			0xFFFFFFF, 0, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0/**/, 0, 0x7FFFFFFF, 0, 0/**/,
			0x10000000, 0, 0, 0x80000000, 0x80000000, 0/**/, 0, 0x80000000, 0, 0/**/,
			0x20000000, 0, 0, 0x100000000, 0x100000000, 0/**/, 0, 0x100000000, 0, 0/**/,
			0x2468ACF1, 0, 0, 0x123456789, 0x123456789, 0/**/, 0, 0x123456789, 0, 0/**/,
			0xFFFFFFFFFFFFFFF, 0x3FFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0/**/, 0, 0x7FFFFFFFFFFFFFFF, 0/**/, 0/**/,
			0x1000000000000000, 0x4000000, 0/**/, -0x8000000000000000i64 as u64, -0x8000000000000000i64 as u64, 0/**/, 0/**/, -0x8000000000000000i64 as u64, 0/**/, 0/**/,
			0x1FFFFFFFFFFFFFFF, 0x7FFFFFF, 0/**/, -0x4i64 as u64, -0x4i64 as u64, 0/**/, 0/**/, -0x4i64 as u64, 0/**/, 0/**/,
			0x1FFFFFFFFFFFFFFB, 0x7FFFFFF, 0/**/, -0x26i64 as u64, -0x26i64 as u64, 0/**/, 0/**/, -0x26i64 as u64, 0/**/, 0/**/
		])
	);

    0
}

/// Calls `__alldiv`.
#[inline(never)]
fn alldiv(a: i64, b: i64) -> i64 {
	a / b
}
/// Calls `__alldiv`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn alldiv_(a: i64, b: i64) -> i64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{b} + 4]
		push eax
		mov  eax, [{b}]
		push eax
		mov  eax, [{a} + 4]
		push eax
		mov  eax, [{a}]
		push eax
		call __alldiv
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as i64) << 32) | eax as i64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn alldiv_(a: i64, b: i64) -> i64 {
	a / b
}

/// No call to `__alldvrm`, uses `__alldiv` and manual remainder calculation.
#[inline(never)]
fn alldvrm(a: i64, b: i64) -> (i64, i64) {
	(a / b, a % b)
}
/// Calls `__alldvrm`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn alldvrm_(a: i64, b: i64) -> (i64, i64) {
	let eax: u32;
	let ebx: u32;
	let ecx: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{b} + 4]
		push eax
		mov  eax, [{b}]
		push eax
		mov  eax, [{a} + 4]
		push eax
		mov  eax, [{a}]
		push eax
		call __alldvrm
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("ebx") ebx,
		out("ecx") ecx,
		out("edx") edx
	); }

	(
		((edx as i64) << 32) | eax as i64,
		((ebx as i64) << 32) | ecx as i64,
	)
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn alldvrm_(a: i64, b: i64) -> (i64, i64) {
	(a / b, a % b)
}

/// No call to `__allmul`, uses `imul` and `mul` instructions.
#[inline(never)]
fn allmul(a: i64, b: i64) -> i64 {
	a * b
}
/// Calls `__allmul`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn allmul_(a: i64, b: i64) -> i64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{b} + 4]
		push eax
		mov  eax, [{b}]
		push eax
		mov  eax, [{a} + 4]
		push eax
		mov  eax, [{a}]
		push eax
		call __allmul
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as i64) << 32) | eax as i64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn allmul_(a: i64, b: i64) -> i64 {
	a * b
}

/// Calls `__allrem`.
#[inline(never)]
fn allrem(a: i64, b: i64) -> i64 {
	a % b
}
/// Calls `__allrem`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn allrem_(a: i64, b: i64) -> i64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{b} + 4]
		push eax
		mov  eax, [{b}]
		push eax
		mov  eax, [{a} + 4]
		push eax
		mov  eax, [{a}]
		push eax
		call __allrem
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as i64) << 32) | eax as i64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn allrem_(a: i64, b: i64) -> i64 {
	a % b
}

/// No call to `__allshl`, uses `shld` and `shl` instructions.
#[inline(never)]
fn allshl(a: i64, b: i64) -> i64 {
	a << b
}
/// Calls `__allshl`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn allshl_(a: i64, b: i64) -> i64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{a}]
		mov  edx, [{a} + 4]
		mov  cl,  [{b}]
		call __allshl
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as i64) << 32) | eax as i64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn allshl_(a: i64, b: i64) -> i64 {
	let b = b as u8;
	if b < 64 {
		if b < 32 {
			a << b
		} else {
			(((a as u32) << ((b & (32-1)) as u32)) as i64) << 32
		}
	} else {
		0
	}
}

/// No call to `__allshr`, uses `shrd` and `sar` instructions.
#[inline(never)]
fn allshr(a: i64, b: i64) -> i64 {
	a >> b
}
/// Calls `__allshr`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn allshr_(a: i64, b: i64) -> i64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{a}]
		mov  edx, [{a} + 4]
		mov  cl,  [{b}]
		call __allshr
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as i64) << 32) | eax as i64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn allshr_(a: i64, b: i64) -> i64 {
	let b = b as u8;
	if b < 64 {
		if b < 32 {
			a >> b
		} else {
			let upper = (a >> 32) as i32;
			(((upper >> (32-1)) as i64) << 32) | ((upper >> (b & (32-1))) as i64)
		}
	} else {
		let upper = ((a >> 32) as i32) >> (32-1);
		((upper as i64) << 32) | (upper as i64)
	}
}

/// Calls `__aulldiv`.
#[inline(never)]
fn aulldiv(a: u64, b: u64) -> u64 {
	a / b
}
/// Calls `__aulldiv`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn aulldiv_(a: u64, b: u64) -> u64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{b} + 4]
		push eax
		mov  eax, [{b}]
		push eax
		mov  eax, [{a} + 4]
		push eax
		mov  eax, [{a}]
		push eax
		call __aulldiv
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as u64) << 32) | eax as u64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn aulldiv_(a: u64, b: u64) -> u64 {
	a / b
}

/// No call to `__aulldvrm`, uses `__aulldiv` and manual remainder calculation.
#[inline(never)]
fn aulldvrm(a: u64, b: u64) -> (u64, u64) {
	(a / b, a % b)
}
/// Calls `__aulldvrm`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn aulldvrm_(a: u64, b: u64) -> (u64, u64) {
	let eax: u32;
	let ebx: u32;
	let ecx: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{b} + 4]
		push eax
		mov  eax, [{b}]
		push eax
		mov  eax, [{a} + 4]
		push eax
		mov  eax, [{a}]
		push eax
		call __aulldvrm
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("ebx") ebx,
		out("ecx") ecx,
		out("edx") edx
	); }

	(
		((edx as u64) << 32) | eax as u64,
		((ebx as u64) << 32) | ecx as u64,
	)
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn aulldvrm_(a: u64, b: u64) -> (u64, u64) {
	(a / b, a % b)
}

/// No call to `__aullmul`, uses `imul` and `mul` instructions.
#[inline(never)]
fn aullmul(a: u64, b: u64) -> u64 {
	a * b
}
/// `__aullmul` does not exist, calls signed `__allmul` instead.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn aullmul_(a: u64, b: u64) -> u64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{b} + 4]
		push eax
		mov  eax, [{b}]
		push eax
		mov  eax, [{a} + 4]
		push eax
		mov  eax, [{a}]
		push eax
		call __allmul
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as u64) << 32) | eax as u64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn aullmul_(a: u64, b: u64) -> u64 {
	a * b
}

/// Calls `__aullrem`.
#[inline(never)]
fn aullrem(a: u64, b: u64) -> u64 {
	a % b
}
/// Calls `__aullrem`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn aullrem_(a: u64, b: u64) -> u64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{b} + 4]
		push eax
		mov  eax, [{b}]
		push eax
		mov  eax, [{a} + 4]
		push eax
		mov  eax, [{a}]
		push eax
		call __aullrem
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as u64) << 32) | eax as u64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn aullrem_(a: u64, b: u64) -> u64 {
	a % b
}

/// No call to `__aullshl`, uses `shld` and `shl` instructions.
#[inline(never)]
fn aullshl(a: u64, b: u64) -> u64 {
	a << b
}
/// `__aullshl` does not exist, calls signed `__allshl` instead.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn aullshl_(a: u64, b: u64) -> u64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{a}]
		mov  edx, [{a} + 4]
		mov  cl,  [{b}]
		call __allshl
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as u64) << 32) | eax as u64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn aullshl_(a: u64, b: u64) -> u64 {
	allshl_(a as i64, b as i64) as u64
}

/// No call to `__aullshr`, uses `shrd` and `shr` instructions.
#[inline(never)]
fn aullshr(a: u64, b: u64) -> u64 {
	a >> b
}
/// Calls `__aullshr`.
#[cfg(target_arch = "x86")]
#[inline(never)]
fn aullshr_(a: u64, b: u64) -> u64 {
	let eax: u32;
	let edx: u32;
	unsafe { asm!(
		"
		mov  eax, [{a}]
		mov  edx, [{a} + 4]
		mov  cl,  [{b}]
		call __aullshr
		",

		a = in(reg) &a,
		b = in(reg) &b,

		out("eax") eax,
		out("edx") edx
	); }

	((edx as u64) << 32) | eax as u64
}
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn aullshr_(a: u64, b: u64) -> u64 {
	let b = b as u8;
	if b < 64 {
		if b < 32 {
			a >> b
		} else {
			(((a >> 32) as u32) >> ((b & (32-1)) as u32)) as u64
		}
	} else {
		0
	}
}