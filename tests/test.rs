use qx_rs_err::err::*;
use qx_rs_md_env::md_env;

#[test]  
fn test() -> Result<()> {

    md_env::setup(Some("dev"))?;

    let v = md_env::str("TEST");
    println!("v: {:?}", v);
    
    Ok(())

}
