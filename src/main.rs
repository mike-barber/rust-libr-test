fn main() -> Result<(), extendr_api::Error> {
    low_level::start_r();
    low_level::test_eval_low_level();
    test_eval_api()?;

    Ok(())
}

/// Test using the extendr api to call into R after we've
/// started the engine.
fn test_eval_api() -> Result<(), extendr_api::Error> {
    use extendr_api::*;
    unsafe {
        let val1 = r!(1);
        let val2 = r!(1.234);
        let res = call!("+", val1, val2)?;
        println!("scalar result: {:?}", res);

        let vec1 = r!(vec![1, 2, 3, 4, 5]);
        let vec_res = call!("*", vec1, r!(3.14159))?;
        println!("vector result: {:?}", vec_res);
    }
    Ok(())
}

mod low_level {

    use libR_sys::*;
    use std::os::raw;

    // Generate constant static strings.
    // Much more efficient than CString.
    // Generates asciiz.
    macro_rules! cstr {
        ($s: expr) => {
            concat!($s, "\0").as_ptr() as *const raw::c_char
        };
    }

    // Generate mutable static strings.
    // Much more efficient than CString.
    // Generates asciiz.
    macro_rules! cstr_mut {
        ($s: expr) => {
            concat!($s, "\0").as_ptr() as *mut raw::c_char
        };
    }

    
    /// Start the R engine; lifted directly from the libR-sys docs
    /// with some small modifications
    pub fn start_r() {
        unsafe {
            // if std::env::var("R_HOME").is_err() {
            //     // env! gets the build-time R_HOME made in build.rs
            //     std::env::set_var("R_HOME", env!("R_HOME"));
            // }

            // use the default home directory on linux
            let default_home = "/usr/lib/R";
            if std::env::var("R_HOME").is_err() {
                std::env::set_var("R_HOME", default_home);
            }

            // Due to Rf_initEmbeddedR using __libc_stack_end
            // We can't call Rf_initEmbeddedR.
            // Instead we must follow rustr's example and call the parts.

            //let res = unsafe { Rf_initEmbeddedR(1, args.as_mut_ptr()) };
            if cfg!(target_os = "windows") && cfg!(target_arch = "x86") {
                Rf_initialize_R(
                    4,
                    [
                        cstr_mut!("R"),
                        cstr_mut!("--arch=i386"),
                        cstr_mut!("--slave"),
                        cstr_mut!("--no-save"),
                    ]
                    .as_mut_ptr(),
                );
            } else {
                Rf_initialize_R(
                    3,
                    [cstr_mut!("R"), cstr_mut!("--slave"), cstr_mut!("--no-save")].as_mut_ptr(),
                );
            }

            // In case you are curious.
            // Maybe 8MB is a bit small.
            // eprintln!("R_CStackLimit={:016x}", R_CStackLimit);
            if cfg!(not(target_os = "windows")) {
                R_CStackLimit = usize::max_value();
            }

            setup_Rmainloop();
        }
    }

    /// Low-level call into R after starting the engine, lifted
    /// directly from the libR-sys docs
    pub fn test_eval_low_level() {
        unsafe {
            // In an ideal world, we would do the following.
            //   let res = R_ParseEvalString(cstr!("1"), R_NilValue);
            // But R_ParseEvalString is only in recent packages.

            let s = Rf_protect(Rf_mkString(cstr!("1")));
            let mut status: ParseStatus = 0;
            let status_ptr = &mut status as *mut ParseStatus;
            let ps = Rf_protect(R_ParseVector(s, -1, status_ptr, R_NilValue));
            let val = Rf_eval(VECTOR_ELT(ps, 0), R_GlobalEnv);
            Rf_PrintValue(val);
            assert_eq!(TYPEOF(val) as u32, REALSXP);
            assert_eq!(*REAL(val), 1.);
            Rf_unprotect(2);
        }
    }

}
