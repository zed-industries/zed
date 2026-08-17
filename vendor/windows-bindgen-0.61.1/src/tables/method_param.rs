use super::*;

impl std::fmt::Debug for MethodParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MethodParam").field(&self.name()).finish()
    }
}

impl MethodParam {
    pub fn flags(&self) -> ParamAttributes {
        ParamAttributes(self.usize(0) as u16)
    }

    pub fn sequence(&self) -> u16 {
        self.usize(1) as u16
    }

    pub fn name(&self) -> &'static str {
        self.str(2)
    }
}
