# MAX32660

[![Documentation](https://docs.rs/max32660/badge.svg)](https://docs.rs/max32660)
[![Crates.io](https://img.shields.io/crates/v/max32660.svg)](https://crates.io/crates/max32660)

Register mappings for the Analog Devices MAX32660 Cortex-M4 microcontroller generated with the `svd2rust` tool.

## User Guide

The complete user guide for this microcontroller may be found at [UG6659][1] and is a useful supplement to this crate. Note that in this crate "SPI17Y" has been renamed to "SPI0" and "SPIMSS" has been renamed to "SPI1" in the interest of sanity as not even the user guide is using consistent naming for these peripherals. Some minor deviations from the user guide were also necessary to address various obvious typos and in some cases to significantly improve the ergonomics of the generated peripheral access crate.

## Contribute

Install the `svd2rust` and `form` tools and run the `generate.sh` script to generate the crate from the SVD file. A line is also prepended to the `lib.rs` to prevent Clippy from checking the crate because the generated code does not lint well. Do not make manual edits to the `src` folder or the `build.rs` and `device.x` files which are all auto-generated.

## Changelog

### 0.2.0

 - Added some helpful enumerated values to the DMA burst size field
 - Added some missing oneToClear attributes on some SPI0.INT_FL fields
 - Fixed some RTC.CTRL fields with incorrect access attributes
 - Added a number of convenient `writeConstraint` attributes
 - Regenerated crate contents using `svd2rust` version `0.30.1`

### 0.1.0

 - Added SVD from Maxim Micros SDK with various modifications and improvements

[1]: https://www.analog.com/media/en/technical-documentation/user-guides/max32660-user-guide.pdf
