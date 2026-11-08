//! Build the complete Adafruit Feather RP2040 JST-GH Breakout board.

use std::{collections::BTreeMap, error::Error, fs, path::PathBuf};

use pcbcraft_engine::{
    ADD_DRAWING, ASSIGN_FOOTPRINT, CONNECT_PINS, INITIALIZE_BOARD_SETUP, PLACE_ASSIGNED_FOOTPRINT,
    PLACE_SYMBOL, SET_BOARD_SETUP, SET_SYMBOL_FIELDS, Session,
    board::setup::LayerPurpose,
    library::sources::Kind,
    net_labels,
    persistence::{encode_documents, encode_packed},
    schematic::GRID_NM,
};
use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let destination = args.next().map(PathBuf::from).ok_or("usage: build_breakout OUTPUT_DIRECTORY")?;

    let docs_json: BTreeMap<String, String> = serde_json::from_str(include_str!(
        "../../../../../.gemini/antigravity-cli/brain/e14cf8d2-9b11-4ff6-b7a9-e04d58b02f8b/scratch/library_documents.json"
    ))?;

    let mut session = Session::default();
    session.apply("project.new", &json!({"name": "RP2040 JST-GH Breakout Board"}))?;

    // Install the native breakout library into the session
    session.execute(pcbcraft_engine::library::sources::LOAD, &json!({
        "source": "breakout-source",
        "documents": docs_json,
    }))?;
    println!("Libraries inspect: {:#?}", session.libraries().inspect());
    println!("Looking up footprint: {:?}", session.libraries().footprint("breakout", "feather-rp2040", 1));

    let get_digest = |session: &Session, id: &str| -> String {
        for (_, item) in session.libraries().search(id, Some(Kind::Symbol)) {
            if item.inspect()["key"]["id"] == id {
                return item.inspect()["digest"].as_str().unwrap_or_default().to_owned();
            }
        }
        String::new()
    };

    let feather_digest = get_digest(&session, "feather-rp2040");
    let pololu_digest = get_digest(&session, "pololu-d24vxx");
    let xt30_digest = get_digest(&session, "xt30");
    let xt60_digest = get_digest(&session, "xt60");
    let jst4_digest = get_digest(&session, "jst-gh-4");
    let jst6_digest = get_digest(&session, "jst-gh-6");
    let jst10_digest = get_digest(&session, "jst-gh-10");
    let jst13_digest = get_digest(&session, "jst-gh-13");

    // --- 1. SCHEMATIC SYMBOL PLACEMENT ---
    let place = |session: &mut Session, def: &str, digest: &str, ref_des: &str, val: &str, x: i64, y: i64| -> Result<String, Box<dyn Error>> {
        session.apply(PLACE_SYMBOL, &json!({
            "library": "breakout",
            "definition": def,
            "definition_revision": 1,
            "definition_digest": digest,
            "x_nm": x * GRID_NM,
            "y_nm": y * GRID_NM,
            "expected_revision": session.revision(),
        }))?;
        let sym_id = session.project().and_then(|p| p.symbols().last()).ok_or("symbol missing")?.id();
        session.apply(SET_SYMBOL_FIELDS, &json!({
            "symbol": sym_id,
            "reference": ref_des,
            "value": val,
            "expected_revision": session.revision(),
        }))?;
        Ok(sym_id)
    };

    // Central Feather RP2040
    let u_feather = place(&mut session, "feather-rp2040", &feather_digest, "U_FEATHER", "Feather-RP2040", 0, 0)?;

    // Power Regulation & Direct Bench Inputs
    let u_pololu = place(&mut session, "pololu-d24vxx", &pololu_digest, "U_POLOLU", "Pololu-D24Vxx", -50, -20)?;
    let j_xt_hv1 = place(&mut session, "xt30", &xt30_digest, "J_XT_HV1", "XT30-Batt", -80, -30)?;
    let j_xt_hv2 = place(&mut session, "xt60", &xt60_digest, "J_XT_HV2", "XT60-Batt", -80, -15)?;
    let j_xt_5v1 = place(&mut session, "xt30", &xt30_digest, "J_XT_5V1", "XT30-5V", -80, 5)?;
    let j_xt_5v2 = place(&mut session, "xt60", &xt60_digest, "J_XT_5V2", "XT60-5V", -80, 20)?;

    // Dual Dedicated UART Ports
    let j_uart1 = place(&mut session, "jst-gh-4", &jst4_digest, "J_UART1", "UART0-Primary", -30, -50)?;
    let j_uart2 = place(&mut session, "jst-gh-4", &jst4_digest, "J_UART2", "UART0-Duplicate", -10, -50)?;

    // 4-Pin JST GH Pairs
    let j_a0_a1 = place(&mut session, "jst-gh-4", &jst4_digest, "J_A0_A1", "A0/A1", 10, -50)?;
    let j_a2_a3 = place(&mut session, "jst-gh-4", &jst4_digest, "J_A2_A3", "A2/A3", 30, -50)?;
    let j_d4_d5 = place(&mut session, "jst-gh-4", &jst4_digest, "J_D4_D5", "D4/D5", 50, -50)?;
    let j_d6_d9 = place(&mut session, "jst-gh-4", &jst4_digest, "J_D6_D9", "D6/D9", 70, -50)?;

    let j_d10_d11 = place(&mut session, "jst-gh-4", &jst4_digest, "J_D10_D11", "D10/D11", -50, 50)?;
    let j_d12_d13 = place(&mut session, "jst-gh-4", &jst4_digest, "J_D12_D13", "D12/D13", -30, 50)?;
    let j_d24_d25 = place(&mut session, "jst-gh-4", &jst4_digest, "J_D24_D25", "D24/D25", -10, 50)?;
    let j_sck_mosi = place(&mut session, "jst-gh-4", &jst4_digest, "J_SCK_MOSI", "SCK/MOSI", 10, 50)?;
    let j_miso_sda = place(&mut session, "jst-gh-4", &jst4_digest, "J_MISO_SDA", "MISO/SDA", 30, 50)?;
    let j_scl_rst = place(&mut session, "jst-gh-4", &jst4_digest, "J_SCL_RST", "SCL/RESET", 50, 50)?;

    // 6-Pin Multi-IO Headers
    let j_adc_quad = place(&mut session, "jst-gh-6", &jst6_digest, "J_ADC_QUAD", "A0-A3-Quad", 50, -25)?;
    let j_spi_quad = place(&mut session, "jst-gh-6", &jst6_digest, "J_SPI_QUAD", "SPI-Quad", 50, -10)?;
    let j_comms_quad = place(&mut session, "jst-gh-6", &jst6_digest, "J_COMMS_QUAD", "I2C/UART", 50, 5)?;
    let j_gpio_quad = place(&mut session, "jst-gh-6", &jst6_digest, "J_GPIO_QUAD", "D4-D9-Quad", 50, 20)?;

    // 10-Pin & 13-Pin Headers
    let j_octal_10 = place(&mut session, "jst-gh-10", &jst10_digest, "J_OCTAL_10", "GPIO-Octal", 80, -10)?;
    let j_wide_13 = place(&mut session, "jst-gh-13", &jst13_digest, "J_WIDE_13", "Main-Bus-13", 80, 15)?;

    // --- 2. SCHEMATIC WIRING ---
    let connect = |session: &mut Session, s1: &str, p1: &str, s2: &str, p2: &str| -> Result<(), Box<dyn Error>> {
        session.apply(CONNECT_PINS, &json!({
            "from": {"symbol": s1, "terminal": p1},
            "to": {"symbol": s2, "terminal": p2},
            "expected_revision": session.revision(),
        }))?;
        Ok(())
    };

    // Battery HV inputs -> Pololu VIN (Pin 1) & GND (Pin 2)
    connect(&mut session, &j_xt_hv1, "1", &u_pololu, "1")?;
    connect(&mut session, &j_xt_hv2, "1", &u_pololu, "1")?;
    connect(&mut session, &j_xt_hv1, "2", &u_pololu, "2")?;
    connect(&mut session, &j_xt_hv2, "2", &u_pololu, "2")?;

    // Direct 5V inputs -> 5V Bus (Pololu VOUT Pin 3) & GND (Pin 2)
    connect(&mut session, &j_xt_5v1, "1", &u_pololu, "3")?;
    connect(&mut session, &j_xt_5v2, "1", &u_pololu, "3")?;
    connect(&mut session, &j_xt_5v1, "2", &u_pololu, "2")?;
    connect(&mut session, &j_xt_5v2, "2", &u_pololu, "2")?;

    // 5V Bus to Feather VBUS (Pin 19)
    connect(&mut session, &u_pololu, "3", &u_feather, "19")?;
    // Common GND to Feather GND (Pin 4)
    connect(&mut session, &u_pololu, "2", &u_feather, "4")?;

    // 5V Bus to Pin 1 of all JST headers
    for hdr in [
        &j_uart1, &j_uart2, &j_a0_a1, &j_a2_a3, &j_d4_d5, &j_d6_d9,
        &j_d10_d11, &j_d12_d13, &j_d24_d25, &j_sck_mosi, &j_miso_sda, &j_scl_rst,
        &j_adc_quad, &j_spi_quad, &j_comms_quad, &j_gpio_quad, &j_octal_10, &j_wide_13,
    ] {
        connect(&mut session, &u_feather, "19", hdr, "1")?;
    }

    // Common GND to Last Pin of all JST headers
    for (hdr, last_pin) in [
        (&j_uart1, "4"), (&j_uart2, "4"), (&j_a0_a1, "4"), (&j_a2_a3, "4"),
        (&j_d4_d5, "4"), (&j_d6_d9, "4"), (&j_d10_d11, "4"), (&j_d12_d13, "4"),
        (&j_d24_d25, "4"), (&j_sck_mosi, "4"), (&j_miso_sda, "4"), (&j_scl_rst, "4"),
        (&j_adc_quad, "6"), (&j_spi_quad, "6"), (&j_comms_quad, "6"), (&j_gpio_quad, "6"),
        (&j_octal_10, "10"), (&j_wide_13, "13"),
    ] {
        connect(&mut session, &u_feather, "4", hdr, last_pin)?;
    }

    // Connect IO Signals to JST headers
    // UART TX (Feather Pin 15) & RX (Feather Pin 14)
    connect(&mut session, &u_feather, "15", &j_uart1, "2")?;
    connect(&mut session, &u_feather, "14", &j_uart1, "3")?;
    connect(&mut session, &j_uart1, "2", &j_uart2, "2")?;
    connect(&mut session, &j_uart1, "3", &j_uart2, "3")?;
    connect(&mut session, &j_uart1, "2", &j_comms_quad, "4")?;
    connect(&mut session, &j_uart1, "3", &j_comms_quad, "5")?;
    connect(&mut session, &j_uart1, "2", &j_wide_13, "2")?;
    connect(&mut session, &j_uart1, "3", &j_wide_13, "3")?;

    // Analog A0..A3 (Feather Pins 5, 6, 7, 8)
    connect(&mut session, &u_feather, "5", &j_a0_a1, "2")?;
    connect(&mut session, &u_feather, "6", &j_a0_a1, "3")?;
    connect(&mut session, &u_feather, "7", &j_a2_a3, "2")?;
    connect(&mut session, &u_feather, "8", &j_a2_a3, "3")?;

    connect(&mut session, &u_feather, "5", &j_adc_quad, "2")?;
    connect(&mut session, &u_feather, "6", &j_adc_quad, "3")?;
    connect(&mut session, &u_feather, "7", &j_adc_quad, "4")?;
    connect(&mut session, &u_feather, "8", &j_adc_quad, "5")?;

    connect(&mut session, &u_feather, "5", &j_octal_10, "8")?;
    connect(&mut session, &u_feather, "6", &j_octal_10, "9")?;
    connect(&mut session, &u_feather, "5", &j_wide_13, "6")?;
    connect(&mut session, &u_feather, "6", &j_wide_13, "7")?;
    connect(&mut session, &u_feather, "7", &j_wide_13, "8")?;
    connect(&mut session, &u_feather, "8", &j_wide_13, "9")?;

    // GPIO D4, D5, D6, D9 (Feather Pins 16, 26, 25, 24)
    connect(&mut session, &u_feather, "16", &j_d4_d5, "2")?;
    connect(&mut session, &u_feather, "26", &j_d4_d5, "3")?;
    connect(&mut session, &u_feather, "25", &j_d6_d9, "2")?;
    connect(&mut session, &u_feather, "24", &j_d6_d9, "3")?;

    connect(&mut session, &u_feather, "16", &j_gpio_quad, "2")?;
    connect(&mut session, &u_feather, "26", &j_gpio_quad, "3")?;
    connect(&mut session, &u_feather, "25", &j_gpio_quad, "4")?;
    connect(&mut session, &u_feather, "24", &j_gpio_quad, "5")?;

    connect(&mut session, &u_feather, "16", &j_wide_13, "10")?;
    connect(&mut session, &u_feather, "26", &j_wide_13, "11")?;
    connect(&mut session, &u_feather, "25", &j_wide_13, "12")?;

    // GPIO D10..D13 (Feather Pins 23, 22, 21, 20)
    connect(&mut session, &u_feather, "23", &j_d10_d11, "2")?;
    connect(&mut session, &u_feather, "22", &j_d10_d11, "3")?;
    connect(&mut session, &u_feather, "21", &j_d12_d13, "2")?;
    connect(&mut session, &u_feather, "20", &j_d12_d13, "3")?;

    connect(&mut session, &u_feather, "23", &j_octal_10, "2")?;
    connect(&mut session, &u_feather, "22", &j_octal_10, "3")?;
    connect(&mut session, &u_feather, "21", &j_octal_10, "4")?;
    connect(&mut session, &u_feather, "20", &j_octal_10, "5")?;

    // GPIO D24, D25 (Feather Pins 9, 10)
    connect(&mut session, &u_feather, "9", &j_d24_d25, "2")?;
    connect(&mut session, &u_feather, "10", &j_d24_d25, "3")?;
    connect(&mut session, &u_feather, "9", &j_spi_quad, "5")?;
    connect(&mut session, &u_feather, "9", &j_octal_10, "6")?;
    connect(&mut session, &u_feather, "10", &j_octal_10, "7")?;

    // SPI (Feather Pins 11: SCK, 12: MOSI, 13: MISO)
    connect(&mut session, &u_feather, "11", &j_sck_mosi, "2")?;
    connect(&mut session, &u_feather, "12", &j_sck_mosi, "3")?;
    connect(&mut session, &u_feather, "13", &j_miso_sda, "2")?;

    connect(&mut session, &u_feather, "11", &j_spi_quad, "2")?;
    connect(&mut session, &u_feather, "12", &j_spi_quad, "3")?;
    connect(&mut session, &u_feather, "13", &j_spi_quad, "4")?;

    // I2C (Feather Pins 28: SDA, 27: SCL)
    connect(&mut session, &u_feather, "28", &j_miso_sda, "3")?;
    connect(&mut session, &u_feather, "27", &j_scl_rst, "2")?;
    connect(&mut session, &u_feather, "28", &j_comms_quad, "2")?;
    connect(&mut session, &u_feather, "27", &j_comms_quad, "3")?;
    connect(&mut session, &u_feather, "28", &j_wide_13, "4")?;
    connect(&mut session, &u_feather, "27", &j_wide_13, "5")?;

    // RESET (Feather Pin 1)
    connect(&mut session, &u_feather, "1", &j_scl_rst, "3")?;

    // Net Labels for major rails and communication buses
    session.apply(net_labels::ADD, &json!({
        "name": "GND",
        "style": "ground",
        "x_nm": -7 * GRID_NM,
        "y_nm": -5 * GRID_NM,
        "expected_revision": session.revision(),
    }))?;
    session.apply(net_labels::ADD, &json!({
        "name": "+5V",
        "style": "power",
        "x_nm": 7 * GRID_NM,
        "y_nm": -4 * GRID_NM,
        "expected_revision": session.revision(),
    }))?;
    session.apply(net_labels::ADD, &json!({
        "name": "UART0_RX",
        "style": "global",
        "x_nm": -7 * GRID_NM,
        "y_nm": 5 * GRID_NM,
        "expected_revision": session.revision(),
    }))?;
    session.apply(net_labels::ADD, &json!({
        "name": "UART0_TX",
        "style": "global",
        "x_nm": -7 * GRID_NM,
        "y_nm": 6 * GRID_NM,
        "expected_revision": session.revision(),
    }))?;

    // --- 3. BOARD INITIALIZATION & PHYSICAL FOOTPRINT PLACEMENT ---
    // Initialize 4-layer stackup: Top (Signals), GND (Internal plane), 5V (Internal plane), Bottom (Signals)
    session.apply(INITIALIZE_BOARD_SETUP, &json!({
        "copper_layers": 4,
        "expected_revision": session.revision(),
    }))?;

    // Rename inner copper layers to designated GND and 5V power planes
    let before_setup = session
        .project()
        .and_then(|p| p.board_view(p.board_target()))
        .and_then(|b| b.setup())
        .cloned()
        .ok_or("no setup")?;
    let mut setup_records = serde_json::to_value(&before_setup)?;
    if let Some(layers) = setup_records.get_mut("layers").and_then(|l| l.as_array_mut()) {
        for layer in layers {
            if layer.get("name").and_then(|n| n.as_str()) == Some("Inner 1") {
                layer["name"] = json!("GND");
            } else if layer.get("name").and_then(|n| n.as_str()) == Some("Inner 2") {
                layer["name"] = json!("5V");
            }
        }
    }
    session.apply(SET_BOARD_SETUP, &json!({
        "setup": setup_records,
        "expected_revision": session.revision(),
    }))?;

    // Add board edge drawing (outline) bounding all components
    let outline_layer = before_setup
        .layers()
        .iter()
        .find(|l| l.purpose == LayerPurpose::Outline)
        .ok_or("no outline layer")?;
    session.apply(ADD_DRAWING, &json!({
        "layer": outline_layer.id,
        "points": [
            {"x_nm": -70_000_000, "y_nm": -45_000_000},
            {"x_nm": 70_000_000, "y_nm": -45_000_000},
            {"x_nm": 70_000_000, "y_nm": 45_000_000},
            {"x_nm": -70_000_000, "y_nm": 45_000_000},
        ],
        "closed": true,
        "expected_revision": session.revision(),
    }))?;

    let get_component = |session: &Session, sym_id: &str| -> Result<String, Box<dyn Error>> {
        let project = session.project().ok_or("no project")?;
        for sym in project.symbols() {
            if sym.id() == sym_id {
                return Ok(sym.component_id());
            }
        }
        Err(format!("cannot find component for {sym_id}").into())
    };

    let place_fp = |session: &mut Session, sym_id: &str, fp_def: &str, pin_count: usize, x_mm: i64, y_mm: i64| -> Result<(), Box<dyn Error>> {
        let comp_id = get_component(session, sym_id)?;
        println!("Assigning footprint {fp_def} for {sym_id} ({comp_id})");
        let mut mapping = serde_json::Map::new();
        for i in 1..=pin_count {
            mapping.insert(i.to_string(), json!(i.to_string()));
        }
        session.apply(ASSIGN_FOOTPRINT, &json!({
            "component": comp_id,
            "library": "breakout",
            "definition": fp_def,
            "definition_revision": 1,
            "mapping": mapping,
            "expected_revision": session.revision(),
        }))?;
        session.apply(PLACE_ASSIGNED_FOOTPRINT, &json!({
            "component": comp_id,
            "x_nm": x_mm * 1_000_000,
            "y_nm": y_mm * 1_000_000,
            "expected_revision": session.revision(),
        }))?;
        Ok(())
    };

    // Place all physical packages on the PCB
    // Central Adafruit Feather RP2040 socket landing
    place_fp(&mut session, &u_feather, "feather-rp2040", 28, 0, 0)?;

    // Power daughterboard and XT connectors
    place_fp(&mut session, &u_pololu, "pololu-d24vxx", 4, -35, -15)?;
    place_fp(&mut session, &j_xt_hv1, "xt30", 2, -55, -25)?;
    place_fp(&mut session, &j_xt_hv2, "xt60", 2, -55, -10)?;
    place_fp(&mut session, &j_xt_5v1, "xt30", 2, -55, 10)?;
    place_fp(&mut session, &j_xt_5v2, "xt60", 2, -55, 25)?;

    // Top perimeter headers
    place_fp(&mut session, &j_uart1, "jst-gh-4", 4, -25, -38)?;
    place_fp(&mut session, &j_uart2, "jst-gh-4", 4, -10, -38)?;
    place_fp(&mut session, &j_a0_a1, "jst-gh-4", 4, 5, -38)?;
    place_fp(&mut session, &j_a2_a3, "jst-gh-4", 4, 20, -38)?;
    place_fp(&mut session, &j_d4_d5, "jst-gh-4", 4, 35, -38)?;
    place_fp(&mut session, &j_d6_d9, "jst-gh-4", 4, 50, -38)?;

    // Bottom perimeter headers
    place_fp(&mut session, &j_d10_d11, "jst-gh-4", 4, -25, 38)?;
    place_fp(&mut session, &j_d12_d13, "jst-gh-4", 4, -10, 38)?;
    place_fp(&mut session, &j_d24_d25, "jst-gh-4", 4, 5, 38)?;
    place_fp(&mut session, &j_sck_mosi, "jst-gh-4", 4, 20, 38)?;
    place_fp(&mut session, &j_miso_sda, "jst-gh-4", 4, 35, 38)?;
    place_fp(&mut session, &j_scl_rst, "jst-gh-4", 4, 50, 38)?;

    // Right perimeter multi-pin headers
    place_fp(&mut session, &j_adc_quad, "jst-gh-6", 6, 35, -18)?;
    place_fp(&mut session, &j_spi_quad, "jst-gh-6", 6, 35, -6)?;
    place_fp(&mut session, &j_comms_quad, "jst-gh-6", 6, 35, 6)?;
    place_fp(&mut session, &j_gpio_quad, "jst-gh-6", 6, 35, 18)?;
    place_fp(&mut session, &j_octal_10, "jst-gh-10", 10, 55, -10)?;
    place_fp(&mut session, &j_wide_13, "jst-gh-13", 13, 55, 15)?;

    // --- 4. EXPORT AND PUBLISH ---
    let project = session.snapshot().ok_or("snapshot missing")?;
    let documents = encode_documents(project.clone())?;
    let archive = encode_packed(project)?;

    let proj_dir = destination.join("rp2040-jst-breakout");
    if proj_dir.exists() {
        fs::remove_dir_all(&proj_dir)?;
    }
    fs::create_dir_all(&proj_dir)?;

    for (relative, bytes) in &documents {
        let path = proj_dir.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)?;
    }

    fs::write(destination.join("rp2040-jst-breakout.pc.zip"), archive)?;
    println!("SUCCESS: Generated complete RP2040 breakout board at {}", proj_dir.join("project.pc").display());
    Ok(())
}
