# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - [2025-11-10 Mon]
- add the GH action to create releases for multiple OSes

## [0.4.0] - [2025-11-10 Mon]
- ability to generate a (Lens specific) catalogue (meant for CCE Explorer) and fhir-profiles (in addition to generating synthetic data)
- various bug fixes (like generating random data for previously hard-coded ones)

## [0.3.0] - [2025-07-23 Wed]

### Bug Fixes
- fix the `build` GH action to run on pushing of tags (instead of on main branches)

## [0.2.0] - [2025-07-03 Thu]

### Added
- GH actions to build (on push or pull request to main) & create release (on creating a tag), and a dependabot file

### Bug Fixes
- rename MedicationStatementSystemicTherapy to SystemicTherapyMedicationStatement
- comment the unnecessary SampleMaterialType enum values
- add a LICENSE file, and update README with license info
- even a single resource should be wrapped in a bundle
- patient deceased date to only have date and no time component

## [0.1.0] - [2025-04-30 Wed]

### Added
- ability to parse cmd line args
- generate a single bundle containing one resource of each type
- generate a single bundle containing n resources of a single type
