use crate::co;
use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

pub(in crate::dxgi) struct IdxgiadapterEnumoutputsIter<'a, I>
	where I: dxgi_IDXGIAdapter,
{
	adapter: &'a I,
	cur_index: u32,
}

impl<'a, I> Iterator for IdxgiadapterEnumoutputsIter<'a, I>
	where I: dxgi_IDXGIAdapter,
{
	type Item = HrResult<IDXGIOutput>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_index == 0xffff_ffff {
			None
		} else {
			let mut queried = unsafe { IDXGIOutput::null() };
			match ok_to_hrresult(
				unsafe {
					(vt::<IDXGIAdapterVT>(self.adapter).EnumOutputs)(
						self.adapter.ptr(),
						self.cur_index,
						queried.as_mut(),
					)
				},
			) {
				Err(err) => {
					self.cur_index = 0xffff_ffff; // no further iterations will be made
					match err {
						co::HRESULT::DXGI_ERROR_NOT_FOUND => None, // no more entries
						_ => Some(Err(err)), // actual error
					}
				},
				Ok(_) => {
					self.cur_index += 1;
					Some(Ok(queried))
				},
			}
		}
	}
}

impl<'a, I> IdxgiadapterEnumoutputsIter<'a, I>
	where I: dxgi_IDXGIAdapter,
{
	pub(in crate::dxgi) const fn new(adapter: &'a I) -> Self {
		Self { adapter, cur_index: 0 }
	}
}

//------------------------------------------------------------------------------

pub(in crate::dxgi) struct IdxgifactoryEnumadaptersIter<'a, I>
	where I: dxgi_IDXGIFactory,
{
	fact: &'a I,
	cur_index: u32,
}

impl<'a, I> Iterator for IdxgifactoryEnumadaptersIter<'a, I>
	where I: dxgi_IDXGIFactory,
{
	type Item = HrResult<IDXGIAdapter>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_index == 0xffff_ffff {
			None
		} else {
			let mut queried = unsafe { IDXGIAdapter::null() };
			match ok_to_hrresult(
				unsafe {
					(vt::<IDXGIFactoryVT>(self.fact).EnumAdapters)(
						self.fact.ptr(),
						self.cur_index,
						queried.as_mut(),
					)
				},
			) {
				Err(err) => {
					self.cur_index = 0xffff_ffff; // no further iterations will be made
					match err {
						co::HRESULT::DXGI_ERROR_NOT_FOUND => None, // no more entries
						_ => Some(Err(err)), // actual error
					}
				},
				Ok(_) => {
					self.cur_index += 1;
					Some(Ok(queried))
				},
			}
		}
	}
}

impl<'a, I> IdxgifactoryEnumadaptersIter<'a, I>
	where I: dxgi_IDXGIFactory,
{
	pub(in crate::dxgi) const fn new(fact: &'a I) -> Self {
		Self { fact, cur_index: 0 }
	}
}
