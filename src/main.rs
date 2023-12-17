extern crate indicatif;

use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time};

fn main() {
    let total_items = 100;
    let progress_bar = ProgressBar::new(total_items as u64);

    //Customization of the progress bar
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
        .unwrap().progress_chars("##-"));

    for item in 0..total_items {

        thread::sleep(time::Duration::from_millis(50));
        //Increment the progress bar
        progress_bar.inc(1);
    }
    //Finish the progress bar
    progress_bar.finish();

}
