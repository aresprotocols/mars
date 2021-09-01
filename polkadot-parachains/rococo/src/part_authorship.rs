use super::*;
use pallet_authorship;

parameter_types! {
	pub const UncleGenerations: BlockNumber = 0;
}

// impl pallet_authorship::Config for Runtime {
//     // type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
//     type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
//     type UncleGenerations = UncleGenerations;
//     type FilterUncle = ();
//     type EventHandler = ();
// }

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = (CollatorSelection,);
}