use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
  let mut signals = Signals::new(&[SIGINT])?;

  thread::spawn(move || {
    for sig in signals.forever() {
      println!("Recieved signal {:?}", sig);
    }
  });
  thread::sleep(Duration::from_secs(2));
  Ok(())
}