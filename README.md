编程作业，需要完成以下要求并且提交代码链接：
1.为 template 模块的 do_something 添加 benchmark 用例（也可以是其它自选模块的可调用函数），并且将 benchmark 运行的结果转换为对应的权重定义；
（1）编写 benchmarking.rs【pallers/template/src/benchmarking.rs】
（2）cargo build --features runtime-benchmarks --release
（3）生成 weights.rs 文件，执行命令：
target/release/node-template benchmark \
--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet pallet_benchmark_demo \
--extrinsic do_something \
--steps 20 \
--repeat 50 \
--template=.maintain/frame-weight-template.hbs \
--output=./pallets/benchmark-demo/src/weights_test.rs


```rust

...省略
pub mod weights;
...省略

#[frame_support::pallet]
pub mod pallet {
  ...省略
	pub use crate::weights::WeightInfo;
  ...省略

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: WeightInfo;
	}
  #[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a single value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(T::WeightInfo::do_something(*something))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}
    ...省略
  }
```

2.选择 node-template 或者其它节点程序，生成 Chain Spec 文件（两种格式都需要）；