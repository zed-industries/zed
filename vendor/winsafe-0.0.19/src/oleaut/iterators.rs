use crate::decl::*;
use crate::prelude::*;

pub(in crate::oleaut) struct IpropertystoreIter<'a, I>
	where I: oleaut_IPropertyStore,
{
	prop_st: &'a I,
	count: u32,
	current: u32,
}

impl<'a, I> Iterator for IpropertystoreIter<'a, I>
	where I: oleaut_IPropertyStore,
{
	type Item = HrResult<PROPERTYKEY>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.prop_st.GetAt(self.current) {
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(ppk) => {
				self.current += 1;
				Some(Ok(ppk))
			},
		}
	}
}

impl<'a, I> IpropertystoreIter<'a, I>
	where I: oleaut_IPropertyStore,
{
	pub(in crate::oleaut) fn new(prop_st: &'a I) -> HrResult<Self> {
		let count = prop_st.GetCount()?;
		Ok(Self { prop_st, count, current: 0 })
	}
}
