# cce-fhir-gen

Fake XML data generator for CCE FHIR profiles.

This repository contains [Rust](https://www.rust-lang.org/) code to generate fake XML data for [CCE FHIR models](https://simplifier.net/cce).

## FHIR library

There are a couple of creates that support FHIR:

- [fhir-sdk](https://docs.rs/fhir-sdk/latest/fhir_sdk/) - is being used in other projects, but it only supports serialization and deserialization to and from JSON, and **XML is not supported yet**.

- Hence, [fhirbolt](https://github.com/lschmierer/fhirbolt) is being used in this project as it supports XML.

## Profiles

There are, a total of 10 profiles -

- 4 Observation profiles (Histology, TNMc, TNMp, VitalStatus)
- 2 Procedure profiles (Operation, Radiotherapy)
- and 1 each of Patient, Condition, Specimen & MedicationStatement
