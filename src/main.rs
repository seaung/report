mod options;
mod utils;
mod helper;

fn main(){
    utils::Banner::show();
    options::Options::run();
}
