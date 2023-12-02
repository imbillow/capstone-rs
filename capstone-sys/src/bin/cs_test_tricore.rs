use capstone_sys::cs_arch::CS_ARCH_TRICORE;
use capstone_sys::cs_opt_type::CS_OPT_DETAIL;
use capstone_sys::cs_opt_value::CS_OPT_ON;
use capstone_sys::{cs_close, cs_disasm, cs_free, cs_insn, cs_mode, cs_open, cs_option, csh};
use std::ffi::CStr;
// use rayon::current_thread_index;
use rayon::prelude::*;
// use std::ffi::CStr;
use rayon::current_thread_index;
use std::mem::MaybeUninit;

fn f(addr: u64, inp: u32) {
    let mut handle: csh = 0;
    unsafe {
        let err = cs_open(CS_ARCH_TRICORE, cs_mode::CS_MODE_TRICORE_162, &mut handle);
        if err > 0 {
            panic!("Failed cs_open")
        }
        cs_option(handle, CS_OPT_DETAIL, CS_OPT_ON as usize);

        let mut pinsns = MaybeUninit::<*mut cs_insn>::zeroed();
        let x = inp.to_le_bytes();
        let count = cs_disasm(handle, x.as_ptr(), x.len(), addr, 0, pinsns.as_mut_ptr());

        // let insns = std::slice::from_raw_parts(*pinsns.as_ptr(), count);
        // for i in insns {
        //     println!(
        //         "{:#010x}:#{}:\t{:?}\t{:?}",
        //         inp,
        //         current_thread_index().unwrap(),
        //         CStr::from_ptr(i.mnemonic.as_ptr()),
        //         CStr::from_ptr(i.op_str.as_ptr())
        //     );
        // }
        cs_free(*pinsns.as_mut_ptr(), count);
        cs_close(&mut handle);
    }
}

fn main() {
    (0..u32::MAX).into_par_iter().for_each(|x| f(0, x));
    (0..u32::MAX)
        .into_par_iter()
        .for_each(|x| f(0xffff_ffff, x));
}
