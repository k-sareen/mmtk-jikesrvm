use std::usize;

use mmtk::util::metadata::side_metadata::GLOBAL_SIDE_METADATA_VM_BASE_ADDRESS;
use mmtk::vm::*;

use crate::java_header::AVAILABLE_BITS_OFFSET;
pub(crate) use mmtk::util::constants::{LOG_BITS_IN_BYTE, LOG_MIN_OBJECT_SIZE};

const FORWARDING_BITS_OFFSET: isize = AVAILABLE_BITS_OFFSET << LOG_BITS_IN_BYTE;

// Global MetadataSpecs - Start

/// Global logging bit metadata spec
/// 1 bit per object
pub(crate) const LOGGING_SIDE_METADATA_SPEC: VMGlobalLogBitSpec = VMGlobalLogBitSpec::side(
    GLOBAL_SIDE_METADATA_VM_BASE_ADDRESS.as_usize(),
    LOG_MIN_OBJECT_SIZE as usize,
);

// Global MetadataSpecs - End

// PolicySpecific MetadataSpecs - Start

/// PolicySpecific forwarding pointer metadata spec
/// 1 word per object
pub(crate) const FORWARDING_POINTER_METADATA_SPEC: VMLocalForwardingPointerSpec =
    VMLocalForwardingPointerSpec::in_header(FORWARDING_BITS_OFFSET);

/// PolicySpecific object forwarding status metadata spec
/// 2 bits per object
pub(crate) const FORWARDING_BITS_METADATA_SPEC: VMLocalForwardingBitsSpec =
    VMLocalForwardingBitsSpec::in_header(FORWARDING_BITS_OFFSET);

/// PolicySpecific mark bit metadata spec
/// 1 bit per object
pub(crate) const MARKING_METADATA_SPEC: VMLocalMarkBitSpec =
    VMLocalMarkBitSpec::in_header(FORWARDING_BITS_OFFSET);

/// PolicySpecific mark-and-nursery bits metadata spec
/// 2-bits per object
pub(crate) const LOS_METADATA_SPEC: VMLocalLOSMarkNurserySpec =
    VMLocalLOSMarkNurserySpec::in_header(FORWARDING_BITS_OFFSET);

// PolicySpecific MetadataSpecs - End
