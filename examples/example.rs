use std::error::Error;
use yang_rs::parser::YangParser;

fn main() -> Result<(), Box<dyn Error>> {
    let _file = YangParser::parse_file("../../yang/vendor/cisco/xr/771/Cisco-IOS-XR-um-router-bgp-cfg.yang").unwrap();

    // dbg!(file);

    Ok(())
}
