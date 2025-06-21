use regex::Regex;
use std::process::Command;

pub struct CpuInfo {
    constructor: String,
    codename: String,
}
impl CpuInfo {
    pub fn new() -> Result<CpuInfo, String> {
        let output = match Command::new("cpuid").output() {
            Ok(out) => out,
            Err(err) => return Err(err.to_string()),
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        let cpu: &str = match stdout
            .trim()
            .split('\n')
            .rfind(|s| s.trim_start().starts_with("(synth)"))
        {
            Some(s) => s,
            None => return Err(String::from("cpu info not found")),
        }
        .trim_start()
        .strip_prefix("(synth)")
        .unwrap()
        .trim();
        let pattern_constructor = Regex::new(r"AMD|Intel").unwrap();
        let pattern_codename = Regex::new(r"\(.*?\)").unwrap();
        let constr: String = match pattern_constructor.find(cpu) {
            Some(s) => s.as_str().to_lowercase(),
            None => return Err(String::from("Unknow CPU Constructor")),
        };
        let code: String = match pattern_codename.find(cpu) {
            Some(s) => s
                .as_str()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .to_lowercase(),
            None => return Err(String::from("Impossible to parse codename")),
        };

        Ok(CpuInfo {
            constructor: constr,
            codename: code,
        })
    }

    pub fn get_constructor(&self) -> &str {
        return &self.constructor;
    }

    pub fn get_codename(&self) -> &str {
        return &self.codename;
    }
}
