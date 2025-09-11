# cce-fhir-gen

The original purpose of this tool was to generate synthetic XML data which conforms to the CCE FHIR profiles.

However, while working on the CCE Explorer (the UI for the CCE VDC project), it became clear that some data (like a `catalogue.json`) expected by the CCE Explorer can be generated using the same code. Similarly, various FHIR profiles (like CodeSystems) can also be generated.

Hence, the original scope has expanded and newer command-line arguments have been added.

> **NOTE**: If you, as a developer are only interested in synthetic data (and you don't work on the CCE Explorer UI, or the CCE FHIR profiles), then you can safely ignore the `catalogue` and the `fhir-resources` cmd-line options and use only the `synthetic-data` option.

This repository contains [Rust](https://www.rust-lang.org/) code to -
- generate synthetic XML data for [CCE FHIR profiles](https://simplifier.net/cce) 
- generate a `catalogue.json` (to be used by the CCE Explorer UI)
- or, to generate various FHIR profiles themselves

## FHIR library

There are a couple of crates that support FHIR:

- [fhir-sdk](https://docs.rs/fhir-sdk/latest/fhir_sdk/) - is being used in other projects, but it only supports serialization and deserialization to and from JSON, and **XML is not supported yet**.

- Hence, [fhirbolt](https://github.com/lschmierer/fhirbolt) is being used in this project as it supports XML.

## Profiles

There are, a total of 10 profiles -

- 4 Observation profiles (Histology, TNMc, TNMp, VitalStatus)
- 2 Procedure profiles (Operation, Radiotherapy)
- and 1 each of Patient, Condition, Specimen & MedicationStatement

## Usage

This repository implements a command line tool, to be run from the command prompt. It accepts the following command line arguments -

```sh
A program to generate synthetic XML data (conforming to CCE FHIR profiles), or catalogue JSON for the CCE explorer (UI) or FHIR resources for all supported resource types

Usage: cce-fhir-gen <COMMAND>

Commands:
  synthetic-data  Generate synthetic XML data conforming to CCE FHIR profiles
  catalogue       Create catalogue JSON for the CCE explorer (UI)
  fhir-resources  Generate FHIR resources for all supported resource types
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Generate synthetic data

To check, which options are supported by `synthetic-data`, please run the below command (or `cargo run -- synthetic-data -h` in dev mode) -

```sh
Generate synthetic XML data conforming to CCE FHIR profiles

Usage: cce-fhir-gen synthetic-data [OPTIONS]

Options:
  -n, --number <NUMBER>                Number of resources to generate [default: 1]
  -r, --resource-type <RESOURCE_TYPE>  Type of resource to generate [default: bundle] [possible values: bundle, patient, condition, specimen, observation-histology, observation-vital-status, observation-tn-mc, procedure-radiotherapy, procedure-operation, systemic-therapy-medication-statement]
  -o, --output-mode <OUTPUT_MODE>      Where to store the resources [default: screen] [possible values: screen, file, api-call]
  -h, --help                           Print help (see more with '--help')
```

#### Parameters and their default values

| Parameter | Default value | Meaning |
|-----------|---------------|---------|
| n | 1 | a value greater than 1 generates a bundle containing multiple resources of the resource type specified by `r` |
| r | bundle | generates a bundle containing one each of the other resources |
| o | screen | displays the generated data on the screen |

### Generate catalogue.json

To check, which options are supported by `catalogue`, please run the below command (or `cargo run -- catalogue -h` in dev mode) -

```sh
Create catalogue JSON for the CCE explorer (UI)

Usage: cce-fhir-gen catalogue [OPTIONS]

Options:
  -o, --output-mode <OUTPUT_MODE>  Where to store the catalogue.json [default: screen] [possible values: screen, file, api-call]
  -h, --help                       Print help (see more with '--help')
```

### Generate (supported) FHIR profiles

To check, which options are supported by `fhir-resources`, please run the below command (or `cargo run -- fhir-resources -h` in dev mode) -

```sh
Generate FHIR resources for all supported resource types

Usage: cce-fhir-gen fhir-resources

Options:
  -h, --help  Print help
```

### How to run

#### Development mode

In development mode, we tend to run `cargo run` command for running the application. In this case, you can use:

```sh
cargo run -- synthetic-data -n 100 -r patient
```

## License

Copyright © 2023 The Samply Community

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
