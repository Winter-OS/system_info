use std::path::Path;
use std::process::Command;
use std::{fs, io};

#[derive(Debug)]
pub struct ComputerInfo {
    vendor: String,
    product_family: String,
    product_name: String,
    disk: Vec<String>,
}
impl ComputerInfo {
    const HARDWARE_VENDOR_REPLACMENT: [(&'static str, &'static str); 2] =
        [("Hewlett-Packard", "hp"), ("Hewlett Packard", "hp")];

    const FAMILY_EXCEPTION_RULES: [(&'static str, &[(&'static str, &'static str)]); 1] = [(
        "framework",
        &[("13in laptop", "13inch"), ("16in laptop", "16inch")],
    )];

    fn grep_vendor() -> Result<String, String> {
        let vendor = match fs::read_to_string("/sys/devices/virtual/dmi/id/sys_vendor") {
            Ok(vendor) => vendor.trim().to_string(),
            Err(e) => return Err(format!("Impossible to get vendor : {}", e.to_string())),
        };
        match Self::HARDWARE_VENDOR_REPLACMENT
            .iter()
            .position(|(s, _)| s.contains(&vendor))
        {
            Some(i) => Ok(Self::HARDWARE_VENDOR_REPLACMENT[i].1.to_string()),
            None => Ok(vendor.to_lowercase()),
        }
    }

    fn grep_product_family(vendor: &str) -> Result<String, String> {
        let family = match fs::read_to_string("/sys/devices/virtual/dmi/id/product_family") {
            Ok(f) => Ok(f.trim().to_string()),
            Err(e) => Err(format!(
                "Impossible to get product family : {}",
                e.to_string()
            )),
        }?
        .to_lowercase();
        let pos_vendor = Self::FAMILY_EXCEPTION_RULES
            .iter()
            .position(|(s, _)| s.eq(&vendor));
        if let Some(pos) = pos_vendor {
            let pos_rule = Self::FAMILY_EXCEPTION_RULES[pos]
                .1
                .iter()
                .position(|(s, _)| s.eq(&family));
            if let Some(posr) = pos_rule {
                return Ok(Self::FAMILY_EXCEPTION_RULES[pos].1[posr].1.to_string());
            }
        }
        return Ok(family);
    }

    fn grep_product_name() -> Result<String, String> {
        match fs::read_to_string("/sys/devices/virtual/dmi/id/product_name") {
            Ok(vendor) => Ok(vendor.trim().to_string().to_lowercase()),
            Err(e) => Err(format!(
                "Impossible to get product name : {}",
                e.to_string()
            )),
        }
    }

    pub fn new() -> Result<ComputerInfo, String> {
        let v = Self::grep_vendor()?;
        Ok(ComputerInfo {
            product_family: Self::grep_product_family(&v)?,
            product_name: Self::grep_product_name()?,
            vendor: v,
            disk: match Self::list_block_device() {
                Ok(l) => l,
                Err(_) => return Err(String::from("Error read block devies")),
            },
        })
    }

    pub fn get_vendor(&self) -> &str {
        return &self.vendor;
    }

    pub fn get_product_family(&self) -> &str {
        return &self.product_family;
    }

    pub fn get_product_name(&self) -> &str {
        return &self.product_name;
    }

    pub fn has_iio_device() -> bool {
        let path = Path::new("/sys/bus/iio/devices");
        if path.exists()
            && path.is_dir()
            && match path.read_dir() {
                Ok(read_dir) => read_dir,
                Err(_) => return false,
            }
            .next()
            .is_some()
        {
            return true;
        }
        return false;
    }

    pub fn has_fingerprint_device() -> bool {
        let output = match Command::new("lsusb").output() {
            Ok(out) => out,
            Err(_) => return false,
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.trim().split('\n').collect();

        for line in lines {
            if line.to_lowercase().contains("fingerprint") {
                return true;
            }
        }
        return false;
    }

    pub fn is_laptop() -> bool {
        let path = Path::new("/sys/class/power_supply");
        if path.exists()
            && path.is_dir()
            && match path.read_dir() {
                Ok(read_dir) => read_dir,
                Err(_) => return false,
            }
            .next()
            .is_some()
        {
            return true;
        }
        return false;
    }

    fn list_block_device() -> io::Result<Vec<String>> {
        let mut devices = Vec::new();
        for entry in fs::read_dir("/sys/block")? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            // Optionnel : ignorer les périphériques loop ou ram
            if name.starts_with("sd") || name.starts_with("nvme") {
                devices.push(name);
            }
        }
        Ok(devices)
    }

    fn is_hdd(device: &str) -> io::Result<bool> {
        let path = format!("/sys/block/{}/queue/rotational", device);
        let contents = fs::read_to_string(path)?;
        Ok(contents.trim() == "1")
    }

    fn is_ssd(device: &str) -> io::Result<bool> {
        let path = format!("/sys/block/{}/queue/rotational", device);
        let contents = fs::read_to_string(path)?;
        Ok(contents.trim() == "0")
    }

    pub fn has_hdd(&self) -> bool {
        for device in &self.disk {
            match Self::is_hdd(&device) {
                Ok(true) => return true,
                Ok(false) => continue,
                Err(_) => panic!("Error: impossible to read disk info"),
            }
        }
        return false;
    }

    pub fn has_ssd(&self) -> bool {
        for device in &self.disk {
            match Self::is_ssd(&device) {
                Ok(true) => return true,
                Ok(false) => continue,
                Err(_) => panic!("Error: impossible to read disk info"),
            }
        }
        return false;
    }
}
