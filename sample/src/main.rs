use ::windows_app_sdk::Microsoft::Windows::System::Power::*;

fn main() -> ::windows::core::Result<()> {
    let charge = PowerManager::RemainingChargePercent()?;
    println!("Remaining charge: {charge}%");
    Ok(())
}
