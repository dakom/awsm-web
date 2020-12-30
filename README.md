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

## Features

Features are heavily gated keep dependencies minimal. The default is for no features to be enabled, but the `all` feature will turn them all on (except those that are only meant for debugging like `debug_log`)
## WebGL


### Cache

The goal is to keep it very low level and low-cost abstraction that is _almost_ universal. However, _almost_ universal is not without opinions. For example, the webgl wrapper provides a lazy caching mechanism for all string lookups (including ubo offsets and texture samplers) and stores local state to avoid making unnecessary gl calls.

In terms of whether to use strings or precalculated integers:

1. `uniforms`: you can't know those in advance, you have to get them via a web api call that takes a string
2. `uniform buffer objects`: you can sorta set and calculate it in advance, but it's very error prone and fragile
3. `sampler index`: can be set in advance but requires syncing with the uniform value
4. `attributes`: can be set in advance, either through shader code or web api

Therefore, for everything other than attributes, it's best to use strings everywhere - after the first call, it's merely a local cache lookup.

Note also that there is a global registry for specifying ubo and attribute locations in advance of any shader code

### Api

It happens in a couple levels

The first is very thin wrappers over the native web_sys types, added as extension traits on the contexts (both webgl1 and 2). These can be used directly on the contexts and are prefixed `awsm_*`. At this level, no additional state is maintained.

Then, the WebGlRenderer wrappers (also for 1 and 2) contain all the state and caching stuff mentiond above. For ease of use, many of the functions that could be called on the context directly are re-exported on the renderer without the `awsm_` prefix.
