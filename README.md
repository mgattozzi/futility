# futility
Functional Utility types, macros, and functions for common tasks or needs in Rust

## Motivation
I often have found that I have to keep writing similar logic or types or traits
when using Rust through the years and rather than keep rewriting them I'd like
to just collect them all into one crate to reuse. `futility` is that crate.

## How to use
Given this is a grab bag of functions and types you can either peruse the
documentation or look at the examples provided or the tests to have an
understanding of what's possible. Currently these modules exist:

- `termination`: types and functions associated with exiting a program

These macros currently exist:

- `try_`: a macro to use `try/catch` blocks in Rust until they're actually
  implemented in the language

## Versioning
Some of these items are implemented in subcrates like the `try_` proc-macro.
Their versions are tied to the top level `fuitlity` crate which itself follows
semver.

# License
Copyright (C) 2022 Michael Gattozzi

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
