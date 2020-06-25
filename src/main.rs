use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    let quit = Arc::new(Mutex::new(false));
    let (quit1, quit2, quit3) = (quit.clone(), quit.clone(), quit.clone());

    let read_handle = thread::spawn(move || read::read_loop(&infile, quit1));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit2));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, quit3));

    // crash if any threads have crashed
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;



    // let mut total_bytes = 0;
    // loop {
    //     let buffer = match read::read(&args.infile) {
    //         Ok(x) if x.is_empty() => break,
    //         Ok(x) => x,
    //         Err(_) => break,
    //     };

    //     stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
    //     if !write::write(&args.outfile, &buffer)? {
    //         break;
    //     }
    // }

    // stats::stats(args.silent, 0, &mut total_bytes, true);
    Ok(())
}

