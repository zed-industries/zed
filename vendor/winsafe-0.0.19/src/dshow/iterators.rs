use crate::decl::*;
use crate::prelude::*;

pub(in crate::dshow) struct IenumfiltersIter<'a, I>
	where I: dshow_IEnumFilters,
{
	enum_filters: &'a I,
}

impl<'a, I> Iterator for IenumfiltersIter<'a, I>
	where I: dshow_IEnumFilters,
{
	type Item = HrResult<IBaseFilter>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.enum_filters.Next() {
			Err(err) => Some(Err(err)),
			Ok(maybe_item) => maybe_item.map(|item| Ok(item)),
		}
	}
}

impl<'a, I> IenumfiltersIter<'a, I>
	where I: dshow_IEnumFilters,
{
	pub(in crate::dshow) const fn new(enum_filters: &'a I) -> Self {
		Self { enum_filters }
	}
}

//------------------------------------------------------------------------------

pub(in crate::dshow) struct IenummediatypesIter<'a, I>
	where I: dshow_IEnumMediaTypes,
{
	enum_mt: &'a I,
	amt: AM_MEDIA_TYPE<'a>,
}

impl<'a, I> Iterator for IenummediatypesIter<'a, I>
	where I: dshow_IEnumMediaTypes,
{
	type Item = HrResult<&'a AM_MEDIA_TYPE<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.enum_mt.Next(&mut self.amt) {
			Err(err) => Some(Err(err)),
			Ok(success) => if success {
				// Returning a reference cannot be done until GATs
				// stabilization, so we simply cheat the borrow checker.
				let ptr = &self.amt as *const AM_MEDIA_TYPE;
				Some(Ok(unsafe { &*ptr }))
			} else {
				None
			},
		}
	}
}

impl<'a, I> IenummediatypesIter<'a, I>
	where I: dshow_IEnumMediaTypes,
{
	pub(in crate::dshow) fn new(enum_mt: &'a I) -> Self {
		Self { enum_mt, amt: AM_MEDIA_TYPE::default() }
	}
}

//------------------------------------------------------------------------------

pub(in crate::dshow) struct IenumpinsIter<'a, I>
	where I: dshow_IEnumPins,
{
	enum_pins: &'a I,
}

impl<'a, I> Iterator for IenumpinsIter<'a, I>
	where I: dshow_IEnumPins,
{
	type Item = HrResult<IPin>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.enum_pins.Next() {
			Err(err) => Some(Err(err)),
			Ok(maybe_item) => maybe_item.map(|item| Ok(item)),
		}
	}
}

impl<'a, I> IenumpinsIter<'a, I>
	where I: dshow_IEnumPins,
{
	pub(in crate::dshow) const fn new(enum_pins: &'a I) -> Self {
		Self { enum_pins }
	}
}
