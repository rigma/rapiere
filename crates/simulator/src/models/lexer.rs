use std::{borrow::Cow, fmt, ops::Deref};

#[derive(Clone, Debug)]
pub(crate) struct Fragment<'s>(Cow<'s, [u8]>);

impl<'s> Fragment<'s> {
    #[inline(always)]
    pub(crate) fn new(word: &str) -> Self {
        let bytes = Vec::from(word.as_bytes());

        Self(Cow::Owned(bytes))
    }
}

impl<'s> Deref for Fragment<'s> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Fragment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RawInput<'i> {
    fragments: Vec<Fragment<'i>>,
}

impl<'i> RawInput<'i> {
    #[inline(always)]
    pub(crate) fn new(fragments: Vec<Fragment<'i>>) -> Self {
        Self { fragments }
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.fragments.len());
        for fragment in &self.fragments {
            output.extend(fragment.deref());
        }

        output
    }
}

impl fmt::Display for RawInput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.as_bytes()))
    }
}
