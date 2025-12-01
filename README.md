# Advent of Code in Bazel

## Goals
The goal of this repo is to demonstrate how to use Bazel in a number of effective ways
through one of the greatest teaching tools known to engineers: Advent of Code!

AoC is an amazing resource for developing programming skills, learning a new language,
or honing existing principles. This repo serves as demonstration (and personal) practice
for how to use Bazel to handle external dependencies and build targets for use in "real"
stuations.

## To use
1. Navigate to [https://adventofcode.com](https://adventofcode.com) and log in using one
of the provided oauth methods
1. Go to a year and get your puzzle input: [2024 day 1 example](https://https://adventofcode.com/2024/day/1/input)
1. Open the Network tab in your browser and reload the page
1. Scroll to the "cookies" section and copy the `session=abcd12345...` section of the cookie
and save it to a file called `.aoc_cookie` at the base of this repo.

## The MODULE.bazel setup
Add the following to your `MODULE.bazel` file to pull in your input as external repository
files:
```python
aoc_input = use_extension("//:extensions.bzl", "aoc_puzzle_input")
aoc_input.use_year(
    cookie_file = "//:.aoc_cookie",
    year = 2024,
)
```

Run `bazel mod tidy` which should update the `MODULE.bazel` file to include a `use_repo`
call with the years you've selected:

```python
aoc_input = use_extension("//:extensions.bzl", "aoc_puzzle_input")
aoc_input.use_year(
    cookie_file = "//:.aoc_cookie",
    year = 2024,
)
use_repo(aoc_input, "aoc2024")
```

To use multiple years, add more `aoc_input.use_year()` calls:
```python
aoc_input = use_extension("//:extensions.bzl", "aoc_puzzle_input")
aoc_input.use_year(
    cookie_file = "//:.aoc_cookie",
    year = 2024,
)
aoc_input.use_year(
    cookie_file = "//:.aoc_cookie",
    year = 2025,
)
use_repo(aoc_input, "aoc2024", "aoc2025")
```

