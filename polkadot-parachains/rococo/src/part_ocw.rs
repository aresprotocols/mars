use super::*;
use pallet_ocw;
use sp_runtime::{traits, MultiAddress};
use sp_runtime::generic::Era;
use frame_support::pallet_prelude::Encode;
use sp_runtime::traits::StaticLookup;



parameter_types! {
	pub const GracePeriod: BlockNumber = 5;
	pub const UnsignedInterval: u32 = 10;
	pub const UnsignedPriority: u64 = 1 << 20;
    pub const OcwVersion: u32 = 101;
}

impl pallet_ocw::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type AuthorityId = pallet_ocw::crypto::OcwAuthId ;
    type AuthorityAres = pallet_ocw::sr25519::AuthorityId;
    type GracePeriod = GracePeriod;
    type UnsignedInterval = UnsignedInterval;
    type UnsignedPriority = UnsignedPriority;
    type PalletVersion = OcwVersion;
}

impl frame_system::offchain::SigningTypes for Runtime {
    type Public = <Signature as traits::Verify>::Signer;
    type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
    where
        Call: From<C>,
{
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = Call;
}

// An index to a block.
// pub type BlockNumber = u64;
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
    where
        Call: From<LocalCall>,
{
    //
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call,
        public: <Signature as traits::Verify>::Signer,
        account: AccountId,
        nonce: Index,
    ) -> Option<(Call, <UncheckedExtrinsic as traits::Extrinsic>::SignaturePayload)> {
        let tip = 0;
        // take the biggest period possible.
        let period =
            BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
        let current_block = System::block_number()
            .saturated_into::<u64>()
            .saturating_sub(1);
        let era = Era::mortal(period, current_block);
        let extra = (
            frame_system::CheckSpecVersion::<Runtime>::new(),
            // frame_system::CheckTxVersion::<Runtime>::new(),
            frame_system::CheckGenesis::<Runtime>::new(),
            frame_system::CheckEra::<Runtime>::from(era),
            frame_system::CheckNonce::<Runtime>::from(nonce),
            frame_system::CheckWeight::<Runtime>::new(),
            pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
        );

        // TODO::Sign one of your own data, the signed data is called raw_payload
        let raw_payload = SignedPayload::new(call, extra)
            .map_err(|e| {
                log::warn!("Unable to create signed payload: {:?}", e);
            })
            .ok()?;
        let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
        // TODO::这里的 address 不能通过 Indices::unlookup() 创建因为它限定，MultiAddress<_, u32>， 而系统中使用的是 MultiAddress<_, ()>
        // let address =  Indices::unlookup(account);
        let address= MultiAddress::Id(account);
        let (call, extra, _) = raw_payload.deconstruct();
        // Create Multiddress<_ ,u32>

        Some((call, (address, signature.into(), extra)))
    }
}