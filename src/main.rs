#[macro_use]
extern crate chan;
extern crate chan_signal;

use chan_signal::Signal;

fn main() {
    println!("Starting our application...");
    // Initialization logic
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);

    // We create a channel to be used when application wants to stop itself.
    let (sdone, rdone) = chan::sync(0);

    // Run our application logic in a separate thread.
	::std::thread::spawn(move || run(sdone));

    // Wait for a signal or for application to stop itself.
    chan_select! {
        signal.recv() -> signal => {
            println!("Shutting down... (received signal: {:?})", signal)
        },
        rdone.recv() => {
            println!("Application stopped.");
        }
    }
}

fn run(_sdone: chan::Sender<()>) {
    println!("Application started");
    loop {
    	// Application logic
    }
}
