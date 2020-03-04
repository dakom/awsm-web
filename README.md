[![Build Status](https://github.com/dakom/awsm-web/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/awsm-web/actions)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/awsm_web.svg)](https://crates.io/crates/awsm_web)
[![Documentation](https://docs.rs/awsm_web/badge.svg)](https://docs.rs/awsm_web)
[![Demo](https://img.shields.io/badge/demo-launch-yellow)](https://dakom.github.io/awsm-web)

## About

awsm_web is primarily used as a building block for other crates in the [awsm](https://github.com/dakom/awsm) ecosystem.

## Description 

The approach with this library is similar in spirit to [gloo](https://github.com/rustwasm/gloo) - that is to say, it bridges the gap between the auto-generated WebIDL-powered bindings web-sys provides, and the type of code we'd typically consider a real starting point in web apps.

The goal is to keep it very low level and low-cost abstraction that is _almost_ universal. However, _almost_ universal is not without opinions. For example, the webgl wrapper does a ton of up-front caching and stores local state to avoid making gl calls unnecessarily - something most projects would do, but not all (e.g. if all locations are hardcoded). 

## Features

Features are heavily gated keep dependencies minimal. The default is for no features to be enabled, but the `all` feature will turn them all on (except those that are only meant for debugging like `debug_log`)