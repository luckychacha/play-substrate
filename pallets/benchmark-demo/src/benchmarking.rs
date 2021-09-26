//! Benchmark-demo pallet benchmarking.
// 编译标签，表示指定了 runtime-benchmarks 这种形式编译的时候才会引入当前的子模块。
#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks!{
	do_something {
		let b in 1 .. 1000;
		let caller = account("caller", 0, 0);
	}: _ (RawOrigin::Signed(caller), b.into())
	verify {
		let value = Something::<T>::get();
		assert_eq!(value, b.into());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, Test};
	use frame_support::assert_ok;

	#[test]
	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_do_something::<Test>());
		});
	}
}
