# lazyfs

[![Build Status](https://travis-ci.org/droundy/lazyfs.svg)](https://travis-ci.org/droundy/lazyfs)

This is a tiny crate for rust utility functions that wrap a few
`std::fs` functions.

So far, we only handle `read_dir`, in a way that returns an iterator
to whatever entries it can find.
