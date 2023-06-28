use core::marker::PhantomData;

use codec::{Decode, Encode};
use frame_support::{
	traits::{Get, OnRuntimeUpgrade},
	weights::Weight,
	Twox64Concat,
};
use scale_info::TypeInfo;
use xcm::latest::prelude::*;

use crate::{Config, XcmTransactInfo, XcmFlow};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

/// Pre-migrations storage struct
#[derive(Clone, Copy, Debug, Encode, Decode, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OldXcmAssetConfig {
	pub fee_per_second: u128,
	/// The UnitWeightCost of a single instruction on the target chain
	pub instruction_weight: Weight,
	/// The desired instruction flow for the target chain
	pub flow: XcmFlow,
}

impl From<OldXcmAssetConfig> for XcmTransactInfo {
	fn from(data: OldXcmAssetConfig) -> Self {
		XcmTransactInfo { flow: data.flow }
	}
}

#[frame_support::storage_alias]
pub type DestinationAssetConfig =
	StorageMap<XcmpHandler, Twox64Concat, MultiLocation, OldXcmAssetConfig>;

pub struct MigrateToTransactInfo<T>(PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for MigrateToTransactInfo<T> {
	fn on_runtime_upgrade() -> Weight {
		log::info!(target: "xcmp-handler", "MigrateToTransactInfo migration");

		let mut migrated_transact_infos = 0u32;
		DestinationAssetConfig::iter().for_each(|(location, asset_config)| {
			let migrated_transact_info: XcmTransactInfo = asset_config.into();
			crate::TransactInfo::<T>::insert(location, migrated_transact_info);
			migrated_transact_infos += 1;
		});

		log::info!(target: "xcmp-handler", "MigrateToTransactInfo successful! Migrated {} object.", migrated_transact_infos);

		T::DbWeight::get()
			.reads_writes(migrated_transact_infos as u64, migrated_transact_infos as u64)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		let prev_count = DestinationAssetConfig::iter().count() as u32;
		Ok(prev_count.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(prev_count: Vec<u8>) -> Result<(), &'static str> {
		let prev_count: u32 = Decode::decode(&mut prev_count.as_slice())
			.expect("the state parameter should be something that was generated by pre_upgrade");
		let post_count = crate::TransactInfo::<T>::iter().count() as u32;
		assert!(post_count == prev_count);

		log::info!(
			target: "xcmp-handler",
			"MigrateToTransactInfo try-runtime checks complete"
		);

		Ok(())
	}
}