use crate::deps::AndroidSdk;
use crate::error::*;

/// Starts installed APK on on emulator or connected device.
/// Runs `adb shell am start ...` command.
pub fn start_apk(sdk: &AndroidSdk, package: &str) -> Result<()> {
    let mut adb = sdk.platform_tool(bin!("adb"))?;
    adb.arg("shell")
        .arg("am")
        .arg("start")
        .arg("-a")
        .arg("android.intent.action.MAIN")
        .arg("-n")
        .arg(format!("{}/android.app.NativeActivity", package));
    adb.output_err(true)?;
    Ok(())
}
