// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use crate::{SubSysAttrItem, SubSystemAttributes};
use assert_matches::assert_matches;
use quote::quote;
use syn::{parse::Parse, parse_quote};

mod attr {
	use super::*;

	#[test]
	fn attr_full_works() {
		let attr: AttrArgs = parse_quote! {
			gen=AllMessage, event=::some::why::ExternEvent, signal=SigSigSig, signal_capacity=111, message_capacity=222,
			error=OverseerError,
		};
		assert_matches!(attr, AttrArgs {
			message_channel_capacity,
			signal_channel_capacity,
			..
		} => {
			assert_eq!(message_channel_capacity, 222);
			assert_eq!(signal_channel_capacity, 111);
		});
	}

	#[test]
	fn attr_partial_works() {
		let attr: AttrArgs = parse_quote! {
			gen=AllMessage, event=::some::why::ExternEvent, signal=::foo::SigSigSig,
			error=OverseerError,
		};
		assert_matches!(attr, AttrArgs {
			message_channel_capacity: _,
			signal_channel_capacity: _,
			..
		} => {
		});
	}
}

mod strukt {

	use super::*;

	#[test]
	fn parse_subsystem_attr_item_works_00_wip() {
		assert_matches!(
		syn::parse2::<SubSysAttrItem>(quote! {
			wip
		}), Ok(SubSysAttrItem::Wip(_)) => {
		});
	}

	#[test]
	fn parse_subsystem_attr_item_works_01_no_dispatch() {
		assert_matches!(
		syn::parse2::<SubSysAttrItem>(quote! {
			no_dispatch
		}), Ok(SubSysAttrItem::NoDispatch(_)) => {
		});
	}

	#[test]
	fn parse_subsystem_attr_item_works_02_sends() {
		assert_matches!(
		syn::parse2::<SubSysAttrItem>(quote! {
			sends: [A, B, C]
		}), Ok(SubSysAttrItem::Sends(sends)) => {
			assert_eq!(sends.sends.len(), 3);
		});
	}

	#[test]
	fn parse_subsystem_attr_item_works_03_sends() {
		assert_matches!(
		syn::parse2::<SubSysAttrItem>(quote! {
			sends: [A]
		}), Ok(SubSysAttrItem::Sends(sends)) => {
			assert_eq!(sends.sends.len(), 1);
		});
	}

	#[test]
	fn parse_subsystem_attr_item_works_04_sends() {
		assert_matches!(
		syn::parse2::<SubSysAttrItem>(quote! {
			sends: [A,]
		}), Ok(SubSysAttrItem::Sends(sends)) => {
			assert_eq!(sends.sends.len(), 1);
		});
	}

	#[test]
	fn parse_subsystem_attr_item_works_05_sends() {
		assert_matches!(
		syn::parse2::<SubSysAttrItem>(quote! {
			sends: []
		}), Ok(SubSysAttrItem::Sends(sends)) => {
			assert_eq!(sends.sends.len(), 0);
		});
	}

	#[test]
	fn parse_subsystem_attr_item_works_06_consumes() {
		assert_matches!(
		syn::parse2::<SubSysAttrItem>(quote! {
			consumes: Foo
		}), Ok(SubSysAttrItem::Consumes(_consumes)) => {
		});
	}

	#[test]
	fn parse_subsystem_attr_item_works_07_consumes() {
		assert_matches!(
		syn::parse2::<SubSysAttrItem>(quote! {
			Foo
		}), Ok(SubSysAttrItem::Consumes(_consumes)) => {
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_00() {
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(wip, no_dispatch, blocking, consumes: Foo, sends: [])]
		})
		.unwrap();
	}

	#[test]
	fn parse_subsystem_attributes_works_01() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(blocking, Foo, sends: [])]
		}), Ok(_) => {
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_02() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(consumes: Foo, sends: [Bar])]
		}), Ok(_) => {
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_03() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(blocking, consumes: Foo, sends: [Bar])]
		}), Ok(_) => {
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_04() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(wip, consumes: Foo, sends: [Bar])]
		}), Ok(_) => {
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_05() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(consumes: Foo)]
		}), Ok(_) => {
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_06() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(sends: [Foo], consumes: Bar)]
		}), Ok(_) => {
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_07() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(sends: [Foo], Bar, Y)]
		}), Err(e) => {
			dbg!(e)
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_08() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(sends: [Foo], consumes: Bar)]
		}), Ok(_) => {
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_09() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(no_dispatch, sends: [])]
		}), Err(e) => {
			// must either consume smth or sends smth, neither is NOK
			dbg!(e)
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_10() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem()]
		}), Err(e) => {
			dbg!(e)
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_11() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem]
		}), Err(e) => {
			dbg!(e)
		});
	}

	#[test]
	fn parse_subsystem_attributes_works_12() {
		assert_matches!(
		syn::parse2::<SubSystemAttributes>(quote! {
			#[subsystem(Foo)]
		}), Ok(_) => {
		});
	}

	#[test]
	fn struct_parse_full() {
		let item: OverseerGuts = parse_quote! {
			pub struct Ooooh<X = Pffffffft> where X: Secrit {
				#[subsystem(no_dispatch, consumes: Foo, sends: [])]
				sub0: FooSubsystem,

				#[subsystem(blocking, consumes: Bar, sends: [])]
				yyy: BaersBuyBilliardBalls,

				#[subsystem(no_dispatch, blocking, consumes: Twain, sends: [])]
				fff: Beeeeep,

				#[subsystem(consumes: Rope)]
				mc: MountainCave,

				metrics: Metrics,
			}
		};
		let _ = dbg!(item);
	}

	#[test]
	fn struct_parse_basic() {
		let item: OverseerGuts = parse_quote! {
			pub struct Ooooh {
				#[subsystem(consumes: Foo, sends: [])]
				sub0: FooSubsystem,
			}
		};
		let _ = dbg!(item);
	}
}
