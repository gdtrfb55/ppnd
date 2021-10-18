pub fn read() -> Result<String, String> {
    use std::fs;
    
    const PATH: &str = r"/proc/net/dev";
    
    if let Ok(s) = fs::read_to_string(PATH) { return Ok(s) };
    Err(format!("could not read {}", PATH))
}