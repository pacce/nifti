use {
    nifti::Header
    , std::{
        fs::File
        , io::Read
    }
};

fn main() -> std::io::Result<()> {
    let args : Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Ok(());
    }

    let name        = args[1].clone();

    let mut file    = File::open(&name)?;
    let h           = Header::decode(&mut file)?;
    eprintln!("{:?}", h);


    Ok(())
}
