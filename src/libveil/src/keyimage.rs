// Copyright 2020 Veil Rust Developers
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use std::borrow;
use std::cmp;
use std::convert;
use std::fmt;
use std::slice;

pub struct KeyImage(pub [u8; 33]);

impl AsRef<[u8]> for KeyImage {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for KeyImage {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl borrow::Borrow<[u8]> for KeyImage {
    fn borrow(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl borrow::BorrowMut<[u8]> for KeyImage {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

// TODO: TryFrom
//impl convert::TryFrom<&[u8]> for KeyImage {
//    type Error = ();
//
//    fn try_from(slice: &[u8]) -> Result<Self, ()> {
//        let tf = <&Self>::try_from(slice).map(|r| r.0).unwrap();
//        Self(tf)
//    }
//}

impl fmt::Debug for KeyImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&&self.0[..], f)
    }
}

impl<'a> IntoIterator for &'a KeyImage {
    type Item = &'a u8;
    type IntoIter = slice::Iter<'a, u8>;

    fn into_iter(self) -> slice::Iter<'a, u8> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut KeyImage {
    type Item = &'a mut u8;
    type IntoIter = slice::IterMut<'a, u8>;

    fn into_iter(self) -> slice::IterMut<'a, u8> {
        self.0.iter_mut()
    }
}

impl PartialEq<KeyImage> for KeyImage {
    #[inline]
    fn eq(&self, other: &KeyImage) -> bool {
        self.0[..] == other.0[..]
    }
}

impl PartialEq<[u8; 32]> for KeyImage {
    #[inline]
    fn eq(&self, other: &[u8; 32]) -> bool {
        self.0[..] == other[..]
    }
}

impl PartialEq<[u8]> for KeyImage {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.0[..] == other[..]
    }
}

impl<'b> PartialEq<&'b [u8]> for KeyImage {
    #[inline]
    fn eq(&self, other: &&'b [u8]) -> bool {
        self.0[..] == other[..]
    }
}

impl Eq for KeyImage {}

impl<'b> PartialEq<&'b mut [u8]> for KeyImage {
    #[inline]
    fn eq(&self, other: &&'b mut [u8]) -> bool {
        self.0[..] == other[..]
    }
}

impl PartialOrd for KeyImage {
    #[inline]
    fn partial_cmp(&self, other: &KeyImage) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&&self.0[..], &&other.0[..])
    }
    #[inline]
    fn lt(&self, other: &KeyImage) -> bool {
        PartialOrd::lt(&&self.0[..], &&other.0[..])
    }
    #[inline]
    fn le(&self, other: &KeyImage) -> bool {
        PartialOrd::le(&&self.0[..], &&other.0[..])
    }
    #[inline]
    fn gt(&self, other: &KeyImage) -> bool {
        PartialOrd::gt(&&self.0[..], &&other.0[..])
    }
    #[inline]
    fn ge(&self, other: &KeyImage) -> bool {
        PartialOrd::ge(&&self.0[..], &&other.0[..])
    }
}

impl Ord for KeyImage {
    #[inline]
    fn cmp(&self, other: &KeyImage) -> cmp::Ordering {
        Ord::cmp(&&self.0[..], &&other.0[..])
    }
}
