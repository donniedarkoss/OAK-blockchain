
//! Autogenerated weights for `pallet_automation_time`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-20, STEPS: `1`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/neumann-collator
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_automation_time
// --extrinsic
// "*"
// --repeat
// 64
// --steps
// 100
// --raw
// --output
// ./pallets/automation-time/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_automation_time.
pub trait WeightInfo {
	fn schedule_notify_task_empty() -> Weight;
	fn schedule_notify_task_full() -> Weight;
	fn schedule_native_transfer_task_empty() -> Weight;
	fn schedule_native_transfer_task_full() -> Weight;
	fn cancel_scheduled_task() -> Weight;
	fn cancel_scheduled_task_full() -> Weight;
	fn cancel_overflow_task() -> Weight;
	fn force_cancel_scheduled_task() -> Weight;
	fn force_cancel_scheduled_task_full() -> Weight;
	fn force_cancel_overflow_task() -> Weight;
	fn run_notify_task() -> Weight;
	fn run_native_transfer_task() -> Weight;
	fn run_missed_tasks_many_found(v: u32, ) -> Weight;
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight;
	fn run_tasks_many_found(v: u32, ) -> Weight;
	fn run_tasks_many_missing(v: u32, ) -> Weight;
	fn update_task_queue_overhead() -> Weight;
	fn append_to_missed_tasks(v: u32, ) -> Weight;
	fn update_scheduled_task_queue() -> Weight;
	fn shift_missed_tasks() -> Weight;
}

/// Weight functions for `pallet_automation_time`.
pub struct AutomationWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AutomationWeight<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn schedule_notify_task_empty() -> Weight {
		(38_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn schedule_notify_task_full() -> Weight {
		(49_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn schedule_native_transfer_task_empty() -> Weight {
		(38_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn schedule_native_transfer_task_full() -> Weight {
		(49_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn cancel_scheduled_task() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn cancel_scheduled_task_full() -> Weight {
		(23_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:0)
	// Storage: AutomationTime TaskQueue (r:1 w:1)
	fn cancel_overflow_task() -> Weight {
		(23_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn force_cancel_scheduled_task_full() -> Weight {
		(23_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:0)
	// Storage: AutomationTime TaskQueue (r:1 w:1)
	fn force_cancel_overflow_task() -> Weight {
		(23_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn run_notify_task() -> Weight {
		(7_000_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(26_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 70_000
			.saturating_add((10_187_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime Tasks (r:1 w:0)
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 65_000
			.saturating_add((8_844_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 89_000
			.saturating_add((29_531_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime Tasks (r:1 w:0)
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((9_000_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(2_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueue (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(1_396_000 as Weight)
			// Standard Error: 113_000
			.saturating_add((1_812_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime TaskQueue (r:1 w:1)
	// Storage: AutomationTime MissedQueue (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(15_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(3_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
