const ON: u8 = b'1';
const OFF: u8 = b'0';
pub const CONSERVATION_FILE_PATH: &str =
    "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";

pub fn read_status() -> Result<bool, std::io::Error> {
    std::fs::read(CONSERVATION_FILE_PATH)
        .map_err(|e| {
            std::io::Error::new(e.kind(), format!("Failed to read conservation mode: {e}"))
        })
        .map(|b| b[0] == ON)
}

pub fn set_status(status: bool) -> Result<(), std::io::Error> {
    std::fs::write(
        CONSERVATION_FILE_PATH,
        [if status { ON } else { OFF }, b'\n'],
    )?;
    Ok(())
}

pub fn toggle_status() -> Result<bool, std::io::Error> {
    let new = !read_status()?;
    set_status(new)?;
    Ok(new)
}
