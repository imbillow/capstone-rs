// Contains code common to the build script and main crate
//
// Needs to be included with include! macro

extern crate std;
use std::string::{String, ToString};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
/// Information specific to architecture
pub struct CapstoneArchInfo<'a> {
    /// name of C header
    header_name: &'a str,

    /// name used within capstone C library
    cs_name: &'a str,
    real_cs_name: Option<&'a str>,
}

impl<'a> CapstoneArchInfo<'a> {
    /// Get the name of the C header
    pub fn header_name(&self) -> &str {
        self.header_name
    }

    /// Get the arch name used in Capstone types
    pub fn cs_name(&self) -> &str {
        self.cs_name
    }

    pub fn real_cs_name(&self) -> String {
        self.real_cs_name
            .map_or(self.cs_name().to_uppercase(), |x| x.to_string())
    }
}

pub static ARCH_INCLUDES: &[CapstoneArchInfo<'static>] = &[
    CapstoneArchInfo {
        header_name: "arm.h",
        cs_name: "arm",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "aarch64.h",
        cs_name: "aarch64",
        real_cs_name: Some("AArch64"),
    },
    CapstoneArchInfo {
        header_name: "evm.h",
        cs_name: "evm",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "m680x.h",
        cs_name: "m680x",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "m68k.h",
        cs_name: "m68k",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "mips.h",
        cs_name: "mips",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "ppc.h",
        cs_name: "ppc",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "riscv.h",
        cs_name: "riscv",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "sparc.h",
        cs_name: "sparc",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "systemz.h",
        cs_name: "sysz",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "tms320c64x.h",
        cs_name: "tms320c64x",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "x86.h",
        cs_name: "x86",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "xcore.h",
        cs_name: "xcore",
        real_cs_name: None,
    },
    CapstoneArchInfo {
        header_name: "tricore.h",
        cs_name: "tricore",
        real_cs_name: None,
    },
];

pub static BINDINGS_FILE: &str = "capstone.rs";
pub static BINDINGS_IMPL_FILE: &str = "capstone_archs_impl.rs";
