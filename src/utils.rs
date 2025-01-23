use std::fs::File;
use std::io::{self, BufRead, BufReader};
use colored::*;
use spinners::{Spinner, Spinners};
use std::thread;
use std::time::Duration;

pub struct Banner;

impl Banner {
    pub fn show() {
        let banner = r#"
        ____                       _   
       |  _ \ ___ _ __   ___  _ __| |_ 
       | |_) / _ \ '_ \ / _ \| '__| __|
       |  _ <  __/ |_) | (_) | |  | |_ 
       |_| \_\___| .__/ \___/|_|   \__|
                 |_|                    
        "#;
        println!("{}", banner.bright_blue());
        println!("{}", "Database Security Scanner Tool".bright_blue());
        println!("{}", "Author: Seaung <https://github.com/seaung>".bright_blue());
        println!("{}", "Version: 1.0.0#dev".bright_blue());
        println!();
    }
}

#[derive(Debug)]
pub struct Credential {
    pub username: String,
    pub password: String
}

pub struct DictReader;

impl DictReader {
    pub fn read_credentials(dict_path: &str) -> io::Result<Vec<Credential>> {
        let file = File::open(dict_path)?;
        let reader = BufReader::new(file);
        let mut credentials = Vec::new();
        
        let mut sp = Spinner::new(Spinners::Dots9, "正在读取密码字典...".into());
        
        for line in reader.lines() {
            let line = line?;
            if let Some((username, password)) = line.split_once(':') {
                credentials.push(Credential {
                    username: username.trim().to_string(),
                    password: password.trim().to_string()
                });
            }
            thread::sleep(Duration::from_millis(10));
        }
        
        sp.stop();
        println!("{}", "密码字典读取完成!".green());
        
        Ok(credentials)
    }
}

pub struct Progress;

impl Progress {
    pub fn start(message: &str) -> Spinner {
        Spinner::new(Spinners::Dots12, message.into())
    }
    
    pub fn stop(mut spinner: Spinner, success: bool) {
        if success {
            spinner.stop_with_message("✓ 完成".green().to_string());
        } else {
            spinner.stop_with_message("✗ 失败".red().to_string());
        }
    }
}