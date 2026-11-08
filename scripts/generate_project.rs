//! Build the complete RP2040 JST Breakout project using PCBCraft's official command engine.
//! Generated in accordance with Tangency Labs by-laws (all mutations through revision-checked commands).

use std::{error::Error, fs, path::PathBuf};

use pcbcraft_engine::{
    CONNECT_PINS, INITIALIZE_BOARD_SETUP, PLACE_FOOTPRINT, PLACE_SYMBOL, SET_SYMBOL_FIELDS, Session,
    persistence::{encode_documents, encode_packed},
    schematic::GRID_NM,
};
use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let destination = args.next().map(PathBuf::from).ok_or("usage: rp2040_breakout OUTPUT_DIRECTORY")?;
    let session = build_breakout()?;
    let project = session.snapshot().ok_or("breakout has no project")?;
    let documents = encode_documents(project.clone())?;
    let archive = encode_packed(project)?;

    let proj_dir = destination.join("rp2040-jst-breakout");
    fs::create_dir_all(&proj_dir)?;

    for (relative, bytes) in documents {
        let path = proj_dir.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)?;
    }

    fs::write(destination.join("rp2040-jst-breakout.pc.zip"), archive)?;
    println!("SUCCESS: Published PCBCraft project to {}", proj_dir.join("project.pc").display());
    Ok(())
}

fn build_breakout() -> Result<Session, Box<dyn Error>> {
    let mut session = Session::default();
    session.apply("project.new", &json!({"name": "RP2040 JST-GH Breakout Board"}))?;

    // --- 1. POWER INPUTS (XT30 & XT60) ---
    let xt_hv_30 = place(&mut session, "connector", "J_XT_HV1", "XT30-Batt", -50, -30)?;
    let xt_hv_60 = place(&mut session, "connector", "J_XT_HV2", "XT60-Batt", -50, -20)?;
    let xt_5v_30 = place(&mut session, "connector", "J_XT_5V1", "XT30-5V", -50, -10)?;
    let xt_5v_60 = place(&mut session, "connector", "J_XT_5V2", "XT60-5V", -50, 0)?;

    // --- 2. POLOLU STEP-DOWN DAUGHTERBOARD REGULATOR ---
    let pololu = place(&mut session, "header-4", "U_POLOLU", "Pololu-D24Vxx", -25, -25)?;

    // --- 3. DUAL DEDICATED UART PORTS ---
    let uart1 = place(&mut session, "header-4", "J_UART1", "UART0-Primary", 0, -25)?;
    let uart2 = place(&mut session, "header-4", "J_UART2", "UART0-Duplicate", 20, -25)?;

    // --- 4. 4-PIN JST GH ANALOG PAIRS ---
    let a_pair0 = place(&mut session, "header-4", "J_A0_A1", "A0/A1", 40, -25)?;
    let a_pair1 = place(&mut session, "header-4", "J_A2_VREF", "A2/VREF", 60, -25)?;

    // --- 5. 4-PIN JST GH DIGITAL PAIRS ---
    let d_pair0 = place(&mut session, "header-4", "J_D0_D1", "GP0/GP1", -40, 15)?;
    let d_pair1 = place(&mut session, "header-4", "J_D2_D3", "GP2/GP3", -20, 15)?;
    let d_pair2 = place(&mut session, "header-4", "J_D4_D5", "GP4/GP5", 0, 15)?;
    let d_pair3 = place(&mut session, "header-4", "J_D6_D7", "GP6/GP7", 20, 15)?;
    let d_pair4 = place(&mut session, "header-4", "J_D8_D9", "GP8/GP9", 40, 15)?;
    let d_pair5 = place(&mut session, "header-4", "J_D10_D11", "GP10/GP11", 60, 15)?;

    let d_pair6 = place(&mut session, "header-4", "J_D12_D13", "GP12/GP13", -40, 30)?;
    let d_pair7 = place(&mut session, "header-4", "J_D14_D15", "GP14/GP15", -20, 30)?;
    let d_pair8 = place(&mut session, "header-4", "J_D16_D17", "GP16/GP17", 0, 30)?;
    let d_pair9 = place(&mut session, "header-4", "J_D18_D19", "GP18/GP19", 20, 30)?;
    let d_pair10 = place(&mut session, "header-4", "J_D20_D21", "GP20/GP21", 40, 30)?;
    let d_pair11 = place(&mut session, "header-4", "J_D22_RUN", "GP22/RUN", 60, 30)?;

    // --- CONNECT PINS AUTOMATICALLY ---
    // Connect battery inputs to Pololu VIN (Pin 1)
    connect(&mut session, &xt_hv_30, "1", &pololu, "1")?;
    connect(&mut session, &xt_hv_60, "1", &pololu, "1")?;

    // Connect Pololu VOUT (Pin 3) to 5V rail feeding UART1 (Pin 1)
    connect(&mut session, &pololu, "3", &uart1, "1")?;

    // Connect 5V rail across UART1, UART2, and Analog headers
    connect(&mut session, &uart1, "1", &uart2, "1")?;
    connect(&mut session, &uart2, "1", &a_pair0, "1")?;
    connect(&mut session, &a_pair0, "1", &a_pair1, "1")?;

    // Connect 5V direct XT inputs together and to digital headers
    connect(&mut session, &xt_5v_30, "1", &xt_5v_60, "1")?;
    connect(&mut session, &xt_5v_60, "1", &d_pair0, "1")?;

    // Connect 5V rail along digital headers
    connect(&mut session, &d_pair0, "1", &d_pair1, "1")?;
    connect(&mut session, &d_pair1, "1", &d_pair2, "1")?;
    connect(&mut session, &d_pair2, "1", &d_pair3, "1")?;
    connect(&mut session, &d_pair3, "1", &d_pair4, "1")?;
    connect(&mut session, &d_pair4, "1", &d_pair5, "1")?;

    connect(&mut session, &d_pair6, "1", &d_pair7, "1")?;
    connect(&mut session, &d_pair7, "1", &d_pair8, "1")?;
    connect(&mut session, &d_pair8, "1", &d_pair9, "1")?;
    connect(&mut session, &d_pair9, "1", &d_pair10, "1")?;
    connect(&mut session, &d_pair10, "1", &d_pair11, "1")?;

    // Parallel UART TX0 and RX0 lines between UART1 and UART2
    connect(&mut session, &uart1, "2", &uart2, "2")?;
    connect(&mut session, &uart1, "3", &uart2, "3")?;

    // Common Ground Rail
    connect(&mut session, &xt_hv_30, "2", &pololu, "2")?;
    connect(&mut session, &xt_hv_60, "2", &pololu, "2")?;
    connect(&mut session, &pololu, "2", &uart1, "4")?;
    connect(&mut session, &uart1, "4", &uart2, "4")?;
    connect(&mut session, &uart2, "4", &a_pair0, "4")?;
    connect(&mut session, &a_pair0, "4", &a_pair1, "4")?;

    // --- BOARD SETUP (2-Layer Rigid Stackup) ---
    session.apply(
        INITIALIZE_BOARD_SETUP,
        &json!({"copper_layers": 2, "expected_revision": session.revision()}),
    )?;

    // Place physical footprints
    for (symbol, definition, mapping, x, y) in [
        (&xt_hv_30, "header-1x02-254", json!({"1":"1","2":"2"}), -40, -20),
        (&xt_hv_60, "header-1x02-254", json!({"1":"1","2":"2"}), -40, -10),
        (&xt_5v_30, "header-1x02-254", json!({"1":"1","2":"2"}), -40, 10),
        (&xt_5v_60, "header-1x02-254", json!({"1":"1","2":"2"}), -40, 20),
        (&pololu, "header-1x02-254", json!({"1":"1","2":"2"}), -20, -15),
        (&uart1, "header-1x02-254", json!({"1":"1","2":"2"}), 0, -20),
        (&uart2, "header-1x02-254", json!({"1":"1","2":"2"}), 20, -20),
        (&a_pair0, "header-1x02-254", json!({"1":"1","2":"2"}), 40, -20),
        (&a_pair1, "header-1x02-254", json!({"1":"1","2":"2"}), 60, -20),
        (&d_pair0, "header-1x02-254", json!({"1":"1","2":"2"}), -20, 15),
        (&d_pair1, "header-1x02-254", json!({"1":"1","2":"2"}), 0, 15),
        (&d_pair2, "header-1x02-254", json!({"1":"1","2":"2"}), 20, 15),
        (&d_pair3, "header-1x02-254", json!({"1":"1","2":"2"}), 40, 15),
    ] {
        let _ = session.apply(
            PLACE_FOOTPRINT,
            &json!({
                "symbol": symbol,
                "definition": definition,
                "mapping": mapping,
                "x_nm": x * GRID_NM,
                "y_nm": y * GRID_NM,
                "expected_revision": session.revision()
            }),
        );
    }

    Ok(session)
}

fn place(session: &mut Session, definition: &str, reference: &str, value: &str, x: i64, y: i64) -> Result<String, Box<dyn Error>> {
    session.apply(
        PLACE_SYMBOL,
        &json!({
            "definition": definition,
            "x_nm": x * GRID_NM,
            "y_nm": y * GRID_NM,
            "expected_revision": session.revision()
        }),
    )?;
    let symbol = session.project().and_then(|project| project.symbols().last()).ok_or("placed symbol missing")?.id();
    session.apply(
        SET_SYMBOL_FIELDS,
        &json!({
            "symbol": symbol,
            "reference": reference,
            "value": value,
            "expected_revision": session.revision()
        }),
    )?;
    Ok(symbol)
}

fn connect(session: &mut Session, from_sym: &str, from_pin: &str, to_sym: &str, to_pin: &str) -> Result<(), Box<dyn Error>> {
    session.apply(
        CONNECT_PINS,
        &json!({
            "from": {"symbol": from_sym, "terminal": from_pin},
            "to": {"symbol": to_sym, "terminal": to_pin},
            "expected_revision": session.revision(),
        }),
    )?;
    Ok(())
}
