# cce-fhir-gen

Synthetic XML data generator for CCE FHIR profiles.

This repository contains [Rust](https://www.rust-lang.org/) code to generate synthetic XML data for [CCE FHIR models](https://simplifier.net/cce).

## FHIR library

There are a couple of creates that support FHIR:

- [fhir-sdk](https://docs.rs/fhir-sdk/latest/fhir_sdk/) - is being used in other projects, but it only supports serialization and deserialization to and from JSON, and **XML is not supported yet**.

- Hence, [fhirbolt](https://github.com/lschmierer/fhirbolt) is being used in this project as it supports XML.

## Profiles

There are, a total of 10 profiles -

- 4 Observation profiles (Histology, TNMc, TNMp, VitalStatus)
- 2 Procedure profiles (Operation, Radiotherapy)
- and 1 each of Patient, Condition, Specimen & MedicationStatement

## Usage

This repository implements a command line tool, to be run from the command prompt. It accepts the following command line arguments -

```
Usage: cce-fhir-gen [OPTIONS]

Options:
  -n, --number <NUMBER>
          Number of resources to generate
          
          [default: 1]

  -r, --resource-type <RESOURCE_TYPE>
          Type of resource to generate
          
          [default: bundle]

          Possible values:
          - bundle:                                     Generate whole Bundle
          - patient:                                    Generate Patient
          - condition:                                  Generate Condition
          - specimen:                                   Generate Specimen
          - observation-histology:                      Generate Observation Histology
          - observation-vital-status:                   Generate Observation VitalStatus
          - observation-tn-mc:                          Generate Observation TNMc
          - procedure-radiotherapy:                     Generate Procedure Radiotherapy
          - procedure-operation:                        Generate Procedure Operation
          - medication-statement-systemic-therapy:      Generate Medication Statement

  -o, --output-mode <OUTPUT_MODE>
          Where to store the resources
          
          [default: screen]

          Possible values:
          - screen:   Show the generated XML in the terminal
          - file:     Store the generated XML in a file
          - api-call: Call the given API endpoint (WIP)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Parameters and their default values

| Parameter | Default value | Meaning |
|-----------|---------------|---------|
| n | 1 | a value greater than 1 generates a bundle containing multiple resources of the resource type specified by `r` |
| r | bundle | generates a bundle containing one each of the other resources |
| o | screen | displays the generated data on the screen |

### How to run

#### Development mode

In development mode, we tend to run `cargo run` command for running the application. In this case, you can use:

```
cargo run -- -n 100 -r patient
```
