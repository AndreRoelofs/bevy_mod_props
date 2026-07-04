# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 3.0.0 - 2026-07-04

- Updated to bevy `v0.19`.
- Split the ECS roles of `Props` into the `EntityProps` component and the
  `GlobalProps` resource, as required by bevy `v0.19` (`Resource` now implies
  `Component`). `Props` itself is now a plain data type; both wrappers
  dereference to it, and the extension traits work unchanged.

## 2.0.0 - 2026-01-28

- Moved from `ustr` to `estr` (a fork of `ustr`).
- Updated to bevy `v0.18`.
- Improved link following.

## 1.0.0 - 2025-11-23

- Factored out of another project and open-sourced.
