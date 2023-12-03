# changelog

all notable changes to this project will be documented here.

the format is inspired by [keep a changelog][changelog] with entries logged as
[conventional commits][commits] and marked with [calendar versioning][calver].

## [unreleased]

- chore(2023): make a makefile that makes development commands easier
- ci(tests): always check formatting and validity of code via tests
- ci(tests): install the rust toolchain to the shared runner directory
- ci(deps): configure dependabot for automatic dependency updates
- fix(2023.1.2): remove zero from the possibilities for replacement
- fix(2023.1.2): attempt to swap spellings for digits in apperance order
- fix(2023.1.2): repeat replacements to capture overlapping spellings
- fix(2023.1.1): separate changes to calibration made for problem two
- style(2023): follow recommendations of formatting changes from clippy
- test(2023.1): perform tests on actual input with the expected answers

## [0.2023.1]

- docs(changelog): log the first commit to the changelog
- docs(readme): begin adventures into code for the season
- docs(license): share code under the mit license
- docs(release): tag efforts of the day before continuing
- feat(2023.1.1): calibrate sums from digits for problem one of day one
- feat(2023.1.2): naively replace spellings of numbers with digits
- test(2023.1): include input values of the day one puzzle

<!-- a collection of links -->
[calver]: https://calver.org
[changelog]: https://keepachangelog.com/en/1.1.0/
[commits]: https://www.conventionalcommits.org/en/v1.0.0/

<!-- a collection of releases -->
[unreleased]: https://github.com/zimeg/advent-of-code/compare/v0.2023.1...HEAD
[0.2023.1]: https://github.com/zimeg/advent-of-code/releases/tag/v0.2023.1
