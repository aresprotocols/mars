
pub mod currency {
	// use node_primitives::Balance;
	pub type CurrencyBalance = u128;

	/// The existential deposit. Set to 1/10 of its parent Relay Chain.
	pub const EXISTENTIAL_DEPOSIT: CurrencyBalance = 100 * AMAS_MILLICENTS;

	pub const AMAS_UNITS: CurrencyBalance = 1_000_000_000_000; // 1
	pub const AMAS_CENTS: CurrencyBalance = AMAS_UNITS / 100; // 0.01
	pub const AMAS_MILLI_UNITS: CurrencyBalance = AMAS_UNITS / 1_000; // 0.001
	pub const AMAS_MILLICENTS: CurrencyBalance = AMAS_CENTS / 1_000; // 0.00001

	// pub const MILLICENTS: Balance = CENTS / 1_000; // 0.00001
	// pub const GRAND: Balance = CENTS * 100_000; // 1000

	// pub const fn deposit(items: u32, bytes: u32) -> Balance {
	// 	// 1/10 of Westend testnet
	// 	(items as Balance * 100 * CENTS + (bytes as Balance) * 5 * MILLICENTS) / 10
	// 	// TODO:: LIN:: test equal.
    //  // (items as Balance * UNITS + (bytes as Balance) * 5 * MILLICENTS) / 10
	// }
}

// pub mod fee {
// 	use node_primitives::Balance;
// 	pub use sp_runtime::Perbill;
// 	use frame_support::weights::{
// 		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
// 		WeightToFeePolynomial,
// 	};
// 	use smallvec::smallvec;
//
// 	/// The block saturation level. Fees will be updates based on this value.
// 	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);
//
// 	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
// 	/// node's balance type.
// 	///
// 	/// This should typically create a mapping between the following ranges:
// 	///   - [0, MAXIMUM_BLOCK_WEIGHT]
// 	///   - [Balance::min, Balance::max]
// 	///
// 	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
// 	///   - Setting it to `0` will essentially disable the weight fee.
// 	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
// 	pub struct WeightToFee;
// 	impl WeightToFeePolynomial for WeightToFee {
// 		type Balance = Balance;
// 		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
// 			// in Polkadot, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
// 			// in Statemint, we map to 1/10 of that, or 1/100 CENT
// 			let p = super::currency::CENTS;
// 			let q = 100 * Balance::from(ExtrinsicBaseWeight::get());
// 			smallvec![WeightToFeeCoefficient {
// 				degree: 1,
// 				negative: false,
// 				coeff_frac: Perbill::from_rational(p % q, q),
// 				coeff_integer: p / q,
// 			}]
// 		}
// 	}
// }
