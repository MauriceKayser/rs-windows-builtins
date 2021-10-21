/// Returns the 1-based index of the failing test, or 0 if all tests succeeded.
#[inline(never)]
fn test_data() -> u32 {
	#[cfg(target_arch = "x86")]
	let mut error_index: u32 = 1;
	#[cfg(target_arch = "x86_64")]
	let error_index: u32 = 1;

    // Test `chkstk`.
    #[inline(never)]
    fn big_stack() -> [u8; 0x2001] {
        return [0; 0x2001];
    }
    if big_stack()[0] != 0 {
        return error_index;
    }

    // Test 64 bit calculations.
    #[cfg(target_arch = "x86")]
    {
		error_index += 1;

		const VALUES: [i64; 10] = [
			3, 7,
			0x7FFF_FFFF, 0x8000_0000,
			0x1_0000_0000, 0x1_2345_6789,
			0x7FFF_FFFF_FFFF_FFFF, 0x8000_0000_0000_0000u64 as i64,
			-4, -11
		];

		const ALLDIV_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
			1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			2, 1, 0, 0, 0, 0, 0, 0, -1, 0,
			0x2AAAAAAA, 0x12492492, 1, 0, 0, 0, 0, 0, 0xFFFFFFFFE0000001u64 as i64, 0xFFFFFFFFF45D1746u64 as i64,
			0x2AAAAAAA, 0x12492492, 1, 1, 0, 0, 0, 0, 0xFFFFFFFFE0000000u64 as i64, 0xFFFFFFFFF45D1746u64 as i64,
			0x55555555, 0x24924924, 2, 2, 1, 0, 0, 0, 0xFFFFFFFFC0000000u64 as i64, 0xFFFFFFFFE8BA2E8Cu64 as i64,
			0x61172283, 0x299C335C, 2, 2, 1, 1, 0, 0, 0xFFFFFFFFB72EA61Eu64 as i64, 0xFFFFFFFFE58553AEu64 as i64,
			0x2AAAAAAAAAAAAAAA, 0x1249249249249249, 0x100000002, 0xFFFFFFFF, 0x7FFFFFFF, 0x70800000, 1, 0, 0xE000000000000001u64 as i64, 0xF45D1745D1745D18u64 as i64,
			0xD555555555555556u64 as i64, 0xEDB6DB6DB6DB6DB7u64 as i64, 0xFFFFFFFEFFFFFFFEu64 as i64, 0xFFFFFFFF00000000u64 as i64, 0xFFFFFFFF80000000u64 as i64, 0xFFFFFFFF8F800000u64 as i64, -1, 1, 0x2000000000000000, 0xBA2E8BA2E8BA2E8,
			-1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
			-3, -1, 0, 0, 0, 0, 0, 0, 2, 1
		];

		const AULLDIV_RESULTS: [u64; VALUES.len() * VALUES.len()] = [
			1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			2, 1, 0, 0, 0, 0, 0, 0, 0, 0,
			0x2AAAAAAA, 0x12492492, 1, 0, 0, 0, 0, 0, 0, 0,
			0x2AAAAAAA, 0x12492492, 1, 1, 0, 0, 0, 0, 0, 0,
			0x55555555, 0x24924924, 2, 2, 1, 0, 0, 0, 0, 0,
			0x61172283, 0x299C335C, 2, 2, 1, 1, 0, 0, 0, 0,
			0x2AAAAAAAAAAAAAAA, 0x1249249249249249, 0x100000002, 0xFFFFFFFF, 0x7FFFFFFF, 0x70800000, 1, 0, 0, 0,
			0x2AAAAAAAAAAAAAAA, 0x1249249249249249, 0x100000002, 0x100000000, 0x80000000, 0x70800000, 1, 1, 0, 0,
			0x5555555555555554, 0x2492492492492491, 0x200000004, 0x1FFFFFFFF, 0xFFFFFFFF, 0xE1000000, 1, 1, 1, 1,
			0x5555555555555551, 0x2492492492492490, 0x200000003, 0x1FFFFFFFF, 0xFFFFFFFF, 0xE1000000, 1, 1, 0, 1
		];

		const ALLDVRM_RESULTS: [(i64, i64); VALUES.len() * VALUES.len()] = [
			(1, 0), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3),
			(2, 1), (1, 0), (0, 7), (0, 7), (0, 7), (0, 7), (0, 7), (0, 7), (-1, 3), (0, 7),
			(0x2AAAAAAA, 1), (0x12492492, 1), (1, 0), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0xFFFFFFFFE0000001u64 as i64, 3), (0xFFFFFFFFF45D1746u64 as i64, 1),
			(0x2AAAAAAA, 2), (0x12492492, 2), (1, 1), (1, 0), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (0xFFFFFFFFE0000000u64 as i64, 0), (0xFFFFFFFFF45D1746u64 as i64, 2),
			(0x55555555, 1), (0x24924924, 4), (2, 2), (2, 0), (1, 0), (0, 0x100000000), (0, 0x100000000), (0, 0x100000000), (0xFFFFFFFFC0000000u64 as i64, 0), (0xFFFFFFFFE8BA2E8Cu64 as i64, 4),
			(0x61172283, 0), (0x299C335C, 5), (2, 0x2345678B), (2, 0x23456789), (1, 0x23456789), (1, 0), (0, 0x123456789), (0, 0x123456789), (0xFFFFFFFFB72EA61Eu64 as i64, 1), (0xFFFFFFFFE58553AEu64 as i64, 3),
			(0x2AAAAAAAAAAAAAAA, 1), (0x1249249249249249, 0), (0x100000002, 1), (0xFFFFFFFF, 0x7FFFFFFF), (0x7FFFFFFF, 0xFFFFFFFF), (0x70800000, 0x4B7FFFFF), (1, 0), (0, 0x7FFFFFFFFFFFFFFF), (0xE000000000000001u64 as i64, 3), (0xF45D1745D1745D18u64 as i64, 7),
			(0xD555555555555556u64 as i64, -2), (0xEDB6DB6DB6DB6DB7u64 as i64, -1), (0xFFFFFFFEFFFFFFFEu64 as i64, -2), (0xFFFFFFFF00000000u64 as i64, 0), (0xFFFFFFFF80000000u64 as i64, 0), (0xFFFFFFFF8F800000u64 as i64, 0xFFFFFFFFB4800000u64 as i64), (-1, -1), (1, 0), (0x2000000000000000, 0), (0xBA2E8BA2E8BA2E8, -8),
			(-1, -1), (0, -4), (0, -4), (0, -4), (0, -4), (0, -4), (0, -4), (0, -4), (1, 0), (0, -4),
			(-3, -2), (-1, -4), (0, -11), (0, -11), (0, -11), (0, -11), (0, -11), (0, -11), (2, -3), (1, 0)
		];

		const AULLDVRM_RESULTS: [(u64, u64); VALUES.len() * VALUES.len()] = [
			(1, 0), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3), (0, 3),
			(2, 1), (1, 0), (0, 7), (0, 7), (0, 7), (0, 7), (0, 7), (0, 7), (0, 7), (0, 7),
			(0x2AAAAAAA, 1), (0x12492492, 1), (1, 0), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF), (0, 0x7FFFFFFF),
			(0x2AAAAAAA, 2), (0x12492492, 2), (1, 1), (1, 0), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000), (0, 0x80000000),
			(0x55555555, 1), (0x24924924, 4), (2, 2), (2, 0), (1, 0), (0, 0x100000000), (0, 0x100000000), (0, 0x100000000), (0, 0x100000000), (0, 0x100000000),
			(0x61172283, 0), (0x299C335C, 5), (2, 0x2345678B), (2, 0x23456789), (1, 0x23456789), (1, 0), (0, 0x123456789), (0, 0x123456789), (0, 0x123456789), (0, 0x123456789),
			(0x2AAAAAAAAAAAAAAA, 1), (0x1249249249249249, 0), (0x100000002, 1), (0xFFFFFFFF, 0x7FFFFFFF), (0x7FFFFFFF, 0xFFFFFFFF), (0x70800000, 0x4B7FFFFF), (1, 0), (0, 0x7FFFFFFFFFFFFFFF), (0, 0x7FFFFFFFFFFFFFFF), (0, 0x7FFFFFFFFFFFFFFF),
			(0x2AAAAAAAAAAAAAAA, 2), (0x1249249249249249, 1), (0x100000002, 2), (0x100000000, 0), (0x80000000, 0), (0x70800000, 0x4B800000), (1, 1), (1, 0), (0, 0x8000000000000000), (0, 0x8000000000000000),
			(0x5555555555555554, 0), (0x2492492492492491, 5), (0x200000004, 0), (0x1FFFFFFFF, 0x7FFFFFFC), (0xFFFFFFFF, 0xFFFFFFFC), (0xE1000000, 0x96FFFFFC), (1, 0x7FFFFFFFFFFFFFFD), (1, 0x7FFFFFFFFFFFFFFC), (1, 0), (1, 7),
			(0x5555555555555551, 2), (0x2492492492492490, 5), (0x200000003, 0x7FFFFFF8), (0x1FFFFFFFF, 0x7FFFFFF5), (0xFFFFFFFF, 0xFFFFFFF5), (0xE1000000, 0x96FFFFF5), (1, 0x7FFFFFFFFFFFFFF6), (1, 0x7FFFFFFFFFFFFFF5), (0, -11i64 as u64), (1, 0)
		];

		const ALLMUL_AULLMUL_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
			9, 0x15, 0x17FFFFFFD, 0x180000000, 0x300000000, 0x369D0369B, 0x7FFFFFFFFFFFFFFD, 0x8000000000000000u64 as i64, -12, -33,
			0x15, 0x31, 0x37FFFFFF9, 0x380000000, 0x700000000, 0x7F6E5D4BF, 0x7FFFFFFFFFFFFFF9, 0x8000000000000000u64 as i64, -28, -77,
			0x17FFFFFFD, 0x37FFFFFF9, 0x3FFFFFFF00000001, 0x3FFFFFFF80000000, 0x7FFFFFFF00000000, 0x91A2B3C35CBA9877u64 as i64, 0x7FFFFFFF80000001, 0x8000000000000000u64 as i64, 0xFFFFFFFE00000004u64 as i64, 0xFFFFFFFA8000000Bu64 as i64,
			0x180000000, 0x380000000, 0x3FFFFFFF80000000, 0x4000000000000000, 0x8000000000000000u64 as i64, 0x91A2B3C480000000u64 as i64, 0xFFFFFFFF80000000u64 as i64, 0, 0xFFFFFFFE00000000u64 as i64, 0xFFFFFFFA80000000u64 as i64,
			0x300000000, 0x700000000, 0x7FFFFFFF00000000, 0x8000000000000000u64 as i64, 0, 0x2345678900000000, 0xFFFFFFFF00000000u64 as i64, 0, 0xFFFFFFFC00000000u64 as i64, 0xFFFFFFF500000000u64 as i64,
			0x369D0369B, 0x7F6E5D4BF, 0x91A2B3C35CBA9877u64 as i64, 0x91A2B3C480000000u64 as i64, 0x2345678900000000, 0x4B66DC326FB98751, 0x7FFFFFFEDCBA9877, 0x8000000000000000u64 as i64, 0xFFFFFFFB72EA61DCu64 as i64, 0xFFFFFFF37C048D1Du64 as i64,
			0x7FFFFFFFFFFFFFFD, 0x7FFFFFFFFFFFFFF9, 0x7FFFFFFF80000001, 0xFFFFFFFF80000000u64 as i64, 0xFFFFFFFF00000000u64 as i64, 0x7FFFFFFEDCBA9877, 1, 0x8000000000000000u64 as i64, 4, 0x800000000000000Bu64 as i64,
			0x8000000000000000u64 as i64, 0x8000000000000000u64 as i64, 0x8000000000000000u64 as i64, 0, 0, 0x8000000000000000u64 as i64, 0x8000000000000000u64 as i64, 0, 0, 0x8000000000000000u64 as i64,
			-12, -28, 0xFFFFFFFE00000004u64 as i64, 0xFFFFFFFE00000000u64 as i64, 0xFFFFFFFC00000000u64 as i64, 0xFFFFFFFB72EA61DCu64 as i64, 4, 0, 16, 0x2C,
			-33, -77, 0xFFFFFFFA8000000Bu64 as i64, 0xFFFFFFFA80000000u64 as i64, 0xFFFFFFF500000000u64 as i64, 0xFFFFFFF37C048D1Du64 as i64, 0x800000000000000Bu64 as i64, 0x8000000000000000u64 as i64, 0x2C, 0x79
		];

		const ALLREM_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
			0, 3, 3, 3, 3, 3, 3, 3, 3, 3,
			1, 0, 7, 7, 7, 7, 7, 7, 3, 7,
			1, 1, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 3, 1,
			2, 2, 1, 0, 0x80000000, 0x80000000, 0x80000000, 0x80000000, 0, 2,
			1, 4, 2, 0, 0, 0x100000000, 0x100000000, 0x100000000, 0, 4,
			0, 5, 0x2345678B, 0x23456789, 0x23456789, 0, 0x123456789, 0x123456789, 1, 3,
			1, 0, 1, 0x7FFFFFFF, 0xFFFFFFFF, 0x4B7FFFFF, 0, 0x7FFFFFFFFFFFFFFF, 3, 7,
			-2, -1, -2, 0, 0, 0xFFFFFFFFB4800000u64 as i64, -1, 0, 0, -8,
			-1, -4, -4, -4, -4, -4, -4, -4, 0, -4,
			-2, -4, -11, -11, -11, -11, -11, -11, -3, 0
		];

		const AULLREM_RESULTS: [u64; VALUES.len() * VALUES.len()] = [
			0, 3, 3, 3, 3, 3, 3, 3, 3, 3,
			1, 0, 7, 7, 7, 7, 7, 7, 7, 7,
			1, 1, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF,
			2, 2, 1, 0, 0x80000000, 0x80000000, 0x80000000, 0x80000000, 0x80000000, 0x80000000,
			1, 4, 2, 0, 0, 0x100000000, 0x100000000, 0x100000000, 0x100000000, 0x100000000,
			0, 5, 0x2345678B, 0x23456789, 0x23456789, 0, 0x123456789, 0x123456789, 0x123456789, 0x123456789,
			1, 0, 1, 0x7FFFFFFF, 0xFFFFFFFF, 0x4B7FFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF,
			2, 1, 2, 0, 0, 0x4B800000, 1, 0, 0x8000000000000000, 0x8000000000000000,
			0, 5, 0, 0x7FFFFFFC, 0xFFFFFFFC, 0x96FFFFFC, 0x7FFFFFFFFFFFFFFD, 0x7FFFFFFFFFFFFFFC, 0, 7,
			2, 5, 0x7FFFFFF8, 0x7FFFFFF5, 0xFFFFFFF5, 0x96FFFFF5, 0x7FFFFFFFFFFFFFF6, 0x7FFFFFFFFFFFFFF5, -11i64 as u64, 0
		];

		const ALLSHL_AULLSHL_RUST_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
			24, 0x180, 0x8000000000000000u64 as i64, 3, 3, 0x600, 0x8000000000000000u64 as i64, 3, 0x3000000000000000, 0x60000000000000,
			56, 0x380, 0x8000000000000000u64 as i64, 7, 7, 0xE00, 0x8000000000000000u64 as i64, 7, 0x7000000000000000, 0xE0000000000000,
			0x3FFFFFFF8, 0x3FFFFFFF80, 0x8000000000000000u64 as i64, 0x7FFFFFFF, 0x7FFFFFFF, 0xFFFFFFFE00, 0x8000000000000000u64 as i64, 0x7FFFFFFF, 0xF000000000000000u64 as i64, 0xFFE0000000000000u64 as i64,
			0x400000000, 0x4000000000, 0, 0x80000000, 0x80000000, 0x10000000000, 0, 0x80000000, 0, 0,
			0x800000000, 0x8000000000, 0, 0x100000000, 0x100000000, 0x20000000000, 0, 0x100000000, 0, 0,
			0x91A2B3C48, 0x91A2B3C480, 0x8000000000000000u64 as i64, 0x123456789, 0x123456789, 0x2468ACF1200, 0x8000000000000000u64 as i64, 0x123456789, 0x9000000000000000u64 as i64, 0xF120000000000000u64 as i64,
			-8, -0x80, 0x8000000000000000u64 as i64, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, -0x200, 0x8000000000000000u64 as i64, 0x7FFFFFFFFFFFFFFF, 0xF000000000000000u64 as i64, 0xFFE0000000000000u64 as i64,
			0, 0, 0, 0x8000000000000000u64 as i64, 0x8000000000000000u64 as i64, 0, 0, 0x8000000000000000u64 as i64, 0, 0,
			-0x20, -0x200, 0, -4, -4, -0x800, 0, -4, 0xC000000000000000u64 as i64, 0xFF80000000000000u64 as i64,
			-88, -0x580, 0x8000000000000000u64 as i64, -11, -11, -0x1600, 0x8000000000000000u64 as i64, -11, 0x5000000000000000, 0xFEA0000000000000u64 as i64
		];

		const ALLSHL_AULLSHL_INTRIN_RESULTS: [i64; VALUES.len() * VALUES.len()] = [
			24, 0x180, 0 /**/, 3, 3, 0/**/, 0/**/, 3, 0/**/, 0/**/,
			56, 0x380, 0/**/, 7, 7, 0/**/, 0/**/, 7, 0/**/, 0/**/,
			0x3FFFFFFF8, 0x3FFFFFFF80, 0/**/, 0x7FFFFFFF, 0x7FFFFFFF, 0/**/, 0/**/, 0x7FFFFFFF, 0/**/, 0/**/,
			0x400000000, 0x4000000000, 0, 0x80000000, 0x80000000, 0/**/, 0, 0x80000000, 0, 0,
			0x800000000, 0x8000000000, 0, 0x100000000, 0x100000000, 0/**/, 0, 0x100000000, 0, 0,
			0x91A2B3C48, 0x91A2B3C480, 0/**/, 0x123456789, 0x123456789, 0/**/, 0/**/, 0x123456789, 0/**/, 0/**/,
			-8, -0x80, 0/**/, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0/**/, 0/**/, 0x7FFFFFFFFFFFFFFF, 0/**/, 0/**/,
			0, 0, 0, 0x8000000000000000u64 as i64, 0x8000000000000000u64 as i64, 0, 0, 0x8000000000000000u64 as i64, 0, 0,
			-0x20, -0x200, 0, -4, -4, 0/**/, 0, -4, 0/**/, 0/**/,
			-88, -0x580, 0/**/, -11, -11, 0/**/, 0/**/, -11, 0/**/, 0/**/
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
							if x86::$func(*l as $t, *r as $t) != results[i * $values.len() + j] $($($conv)+)? {
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
				0, 0, 0, 7, 7, 0, 0, 7, 0, 0,
				0xFFFFFFF, 0xFFFFFF, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0x3FFFFF, 0, 0x7FFFFFFF, 0, 0,
				0x10000000, 0x1000000, 0, 0x80000000, 0x80000000, 0x400000, 0, 0x80000000, 0, 0,
				0x20000000, 0x2000000, 0, 0x100000000, 0x100000000, 0x800000, 0, 0x100000000, 0, 0,
				0x2468ACF1, 0x2468ACF, 0, 0x123456789, 0x123456789, 0x91A2B3, 0, 0x123456789, 0, 0,
				0x0FFFFFFFFFFFFFFF, 0x00FFFFFFFFFFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x003FFFFFFFFFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 7, 0x3FF,
				0xF000000000000000u64 as i64, 0xFF00000000000000u64 as i64, -1, 0x8000000000000000u64 as i64, 0x8000000000000000u64 as i64, 0xFFC0000000000000u64 as i64, -1, 0x8000000000000000u64 as i64, -8, -0x400,
				-1, -1, -1, -4, -4, -1, -1, -4, -1, -1,
				-2, -1, -1, -11, -11, -1, -1, -11, -1, -1
			]),
			(allshr_, i64, [
				0, 0, 0, 3, 3, 0, 0, 3, 0, 0,
				0, 0, 0, 7, 7, 0, 0, 7, 0, 0,
				0xFFFFFFF, 0xFFFFFF, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0/**/, 0, 0x7FFFFFFF, 0, 0,
				0x10000000, 0x1000000, 0, 0x80000000, 0x80000000, 0/**/, 0, 0x80000000, 0, 0,
				0x20000000, 0x2000000, 0, 0x100000000, 0x100000000, 0/**/, 0, 0x100000000, 0, 0,
				0x2468ACF1, 0x2468ACF, 0, 0x123456789, 0x123456789, 0/**/, 0, 0x123456789, 0, 0,
				0x0FFFFFFFFFFFFFFF, 0x00FFFFFFFFFFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0/**/, 0, 0x7FFFFFFFFFFFFFFF, 0/**/, 0/**/,
				0xF000000000000000u64 as i64, 0xFF00000000000000u64 as i64, -1, 0x8000000000000000u64 as i64, 0x8000000000000000u64 as i64, -1/**/, -1, 0x8000000000000000u64 as i64, -1/**/, -1/**/,
				-1, -1, -1, -4, -4, -1, -1, -4, -1, -1,
				-2, -1, -1, -11, -11, -1, -1, -11, -1, -1
			]),
			// Unexpected: Rust and intrinsic differ, see /**/ markers.
			(aullshr, u64, [
				0, 0, 0, 3, 3, 0, 0, 3, 0, 0,
				0, 0, 0, 7, 7, 0, 0, 7, 0, 0,
				0xFFFFFFF, 0xFFFFFF, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0x3FFFFF, 0, 0x7FFFFFFF, 0, 0,
				0x10000000, 0x1000000, 0, 0x80000000, 0x80000000, 0x400000, 0, 0x80000000, 0, 0,
				0x20000000, 0x2000000, 0, 0x100000000, 0x100000000, 0x800000, 0, 0x100000000, 0, 0,
				0x2468ACF1, 0x2468ACF, 0, 0x123456789, 0x123456789, 0x91A2B3, 0, 0x123456789, 0, 0,
				0x0FFFFFFFFFFFFFFF, 0x00FFFFFFFFFFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x003FFFFFFFFFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 7, 0x3FF,
				0x1000000000000000, 0x100000000000000, 1, 0x8000000000000000, 0x8000000000000000, 0x40000000000000, 1, 0x8000000000000000, 8, 0x400,
				0x1FFFFFFFFFFFFFFF, 0x1FFFFFFFFFFFFFF, 1, -4i64 as u64, -4i64 as u64, 0x7FFFFFFFFFFFFF, 1, -4i64 as u64, 15, 0x7FF,
				0x1FFFFFFFFFFFFFFE, 0x1FFFFFFFFFFFFFF, 1, -11i64 as u64, -11i64 as u64, 0x7FFFFFFFFFFFFF, 1, -11i64 as u64, 15, 0x7FF
			]),
			(aullshr_, u64, [
				0, 0, 0, 3, 3, 0, 0, 3, 0, 0,
				0, 0, 0, 7, 7, 0, 0, 7, 0, 0,
				0xFFFFFFF, 0xFFFFFF, 0, 0x7FFFFFFF, 0x7FFFFFFF, 0/**/, 0, 0x7FFFFFFF, 0, 0,
				0x10000000, 0x1000000, 0, 0x80000000, 0x80000000, 0/**/, 0, 0x80000000, 0, 0,
				0x20000000, 0x2000000, 0, 0x100000000, 0x100000000, 0/**/, 0, 0x100000000, 0, 0,
				0x2468ACF1, 0x2468ACF, 0, 0x123456789, 0x123456789, 0/**/, 0, 0x123456789, 0, 0,
				0x0FFFFFFFFFFFFFFF, 0x00FFFFFFFFFFFFFF, 0, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0/**/, 0, 0x7FFFFFFFFFFFFFFF, 0/**/, 0/**/,
				0x1000000000000000, 0x100000000000000, 0/**/, 0x8000000000000000, 0x8000000000000000, 0/**/, 0/**/, 0x8000000000000000, 0/**/, 0/**/,
				0x1FFFFFFFFFFFFFFF, 0x1FFFFFFFFFFFFFF, 0/**/, -4i64 as u64, -4i64 as u64, 0/**/, 0/**/, -4i64 as u64, 0/**/, 0/**/,
				0x1FFFFFFFFFFFFFFE, 0x1FFFFFFFFFFFFFF, 0/**/, -11i64 as u64, -11i64 as u64, 0/**/, 0/**/, -11i64 as u64, 0/**/, 0/**/
			])
		);
    }

    0
}

#[cfg(target_arch = "x86")]
mod x86 {
	/// Calls `__alldiv`.
	#[inline(never)]
	pub(super) fn alldiv(a: i64, b: i64) -> i64 {
		a / b
	}

	/// Calls `__alldiv`.
	#[inline(never)]
	pub(super) fn alldiv_(a: i64, b: i64) -> i64 {
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

	/// No call to `__alldvrm`, uses `__alldiv` and manual remainder calculation.
	#[inline(never)]
	pub(super) fn alldvrm(a: i64, b: i64) -> (i64, i64) {
		(a / b, a % b)
	}

	/// Calls `__alldvrm`.
	#[inline(never)]
	pub(super) fn alldvrm_(a: i64, b: i64) -> (i64, i64) {
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

	/// No call to `__allmul`, uses `imul` and `mul` instructions.
	#[inline(never)]
	pub(super) fn allmul(a: i64, b: i64) -> i64 {
		a * b
	}

	/// Calls `__allmul`.
	#[inline(never)]
	pub(super) fn allmul_(a: i64, b: i64) -> i64 {
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

	/// Calls `__allrem`.
	#[inline(never)]
	pub(super) fn allrem(a: i64, b: i64) -> i64 {
		a % b
	}

	/// Calls `__allrem`.
	#[inline(never)]
	pub(super) fn allrem_(a: i64, b: i64) -> i64 {
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

	/// No call to `__allshl`, uses `shld` and `shl` instructions.
	#[inline(never)]
	pub(super) fn allshl(a: i64, b: i64) -> i64 {
		a << b
	}

	/// Calls `__allshl`.
	#[inline(never)]
	pub(super) fn allshl_(a: i64, b: i64) -> i64 {
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

	/// No call to `__allshr`, uses `shrd` and `sar` instructions.
	#[inline(never)]
	pub(super) fn allshr(a: i64, b: i64) -> i64 {
		a >> b
	}

	/// Calls `__allshr`.
	#[inline(never)]
	pub(super) fn allshr_(a: i64, b: i64) -> i64 {
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

	/// Calls `__aulldiv`.
	#[inline(never)]
	pub(super) fn aulldiv(a: u64, b: u64) -> u64 {
		a / b
	}

	/// Calls `__aulldiv`.
	#[inline(never)]
	pub(super) fn aulldiv_(a: u64, b: u64) -> u64 {
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

	/// No call to `__aulldvrm`, uses `__aulldiv` and manual remainder calculation.
	#[inline(never)]
	pub(super) fn aulldvrm(a: u64, b: u64) -> (u64, u64) {
		(a / b, a % b)
	}

	/// Calls `__aulldvrm`.
	#[inline(never)]
	pub(super) fn aulldvrm_(a: u64, b: u64) -> (u64, u64) {
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

	/// No call to `__aullmul`, uses `imul` and `mul` instructions.
	#[inline(never)]
	pub(super) fn aullmul(a: u64, b: u64) -> u64 {
		a * b
	}

	/// `__aullmul` does not exist, calls signed `__allmul` instead.
	#[inline(never)]
	pub(super) fn aullmul_(a: u64, b: u64) -> u64 {
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

	/// Calls `__aullrem`.
	#[inline(never)]
	pub(super) fn aullrem(a: u64, b: u64) -> u64 {
		a % b
	}

	/// Calls `__aullrem`.
	#[inline(never)]
	pub(super) fn aullrem_(a: u64, b: u64) -> u64 {
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

	/// No call to `__aullshl`, uses `shld` and `shl` instructions.
	#[inline(never)]
	pub(super) fn aullshl(a: u64, b: u64) -> u64 {
		a << b
	}

	/// `__aullshl` does not exist, calls signed `__allshl` instead.
	#[inline(never)]
	pub(super) fn aullshl_(a: u64, b: u64) -> u64 {
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

	/// No call to `__aullshr`, uses `shrd` and `shr` instructions.
	#[inline(never)]
	pub(super) fn aullshr(a: u64, b: u64) -> u64 {
		a >> b
	}

	/// Calls `__aullshr`.
	#[inline(never)]
	pub(super) fn aullshr_(a: u64, b: u64) -> u64 {
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
}