fn main() -> Result<(), extendr_api::Error> {
    // start R engine
    extendr_engine::start_r();
    println!("R started");

    // higher-level api -- try some operations
    api::test_basic()?;
    api::test_dataframe()?;
    api::test_function()?;
    api::test_complex_call()?;

    // stop R engine -- never attempt a restart
    extendr_engine::end_r();
    println!("R stopped.");

    Ok(())
}

mod api {
    use std::time::Instant;

    use extendr_api::prelude::*;

    /// Test using the extendr api to call into R after we've
    /// started the engine.
    pub fn test_basic() -> Result<()> {
        let val1 = r!(1);
        let val2 = r!(1.234);
        let res = call!("+", val1, val2)?;
        println!("scalar result: {:?}", res);

        let vec1 = r!(vec![1, 2, 3, 4, 5]);
        let vec_res = call!("*", vec1, r!(3.14159))?;
        println!("vector result: {:?}", vec_res);
        Ok(())
    }

    /// Test a dataframe
    pub fn test_dataframe() -> Result<()> {
        let basic_df = data_frame!(x = 1, y = 2);

        let vec1: Vec<i32> = (0..100).collect();
        let vec2: Vec<i32> = (10..20).collect();

        let longer_df = data_frame!(first = vec1, second = vec2);

        call!("print", basic_df)?;
        call!("print", longer_df)?;

        Ok(())
    }

    pub fn test_function() -> Result<()> {
        // not sure what to do with this; it looks like as_function and
        // Function exist in extendr_api master, but are not in the current
        // package yet.
        let expr = R!(function(a = 1, b) { a + b })?;
        println!("Is function? {}", expr.is_function());
        let _f = expr.as_func().unwrap();

        R!(myfn <- function(a) a^2)?;
        let res = call!("myfn", 10)?;
        println!("myfn result {:?}", res);

        Ok(())
    }

    pub fn test_complex_call() -> Result<()> {
        for _ in 0..100 {
            let start = Instant::now();
            R!(
                testfn <- function(num) {
                    data.frame(a=rnorm(num), b=rnorm(num), c=rnorm(num))
                }
            )?;
            let res = call!("testfn", 1_000_000)?;
            let end = Instant::now();

            let a = call!("$", &res, "a")?;

            println!(
                "result is: {:?} len {}, took {:?}",
                a.rtype(),
                a.len(),
                end - start
            );
        }
        Ok(())
    }
}
