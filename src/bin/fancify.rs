#[macro_use]
extern crate log;
extern crate loggerv;

#[macro_use]
extern crate error_chain;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

/// Do fancy things
#[derive(StructOpt, Debug)]
#[structopt(name = "fancify")]
struct Cli {
    /// Enable logging, use multiple `v`s to increase verbosity
    #[structopt(short = "v", long = "verbose")]
    verbosity: u64,
}

error_chain! {
    foreign_links {
        Log(::log::SetLoggerError);
    }
}

quick_main!(|| -> Result<()> {
    let args = Cli::from_args();

    loggerv::init_with_verbosity(args.verbosity)?;

    // ...
    let thing = "foobar";
    debug!("Thing happened: {}", thing);
    // ...

    info!("It's all good!");
    Ok(())
});

