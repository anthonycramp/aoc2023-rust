# [2023 Day 01](https://adventofcode.com/2023/day/1)

Couple of things learned in this solution:

- BTreeMap provides for iteration in-order of its keys. I've written code in the
  past that used some kind of hash map and then extracted the keys, sorted them,
  and then collected the values in key order. All that is replaced with
  BTreeMap.
- [str::match_indices](https://doc.rust-lang.org/std/primitive.str.html#method.match_indices)
  returns pairs of index and match for all matches in the source string.
