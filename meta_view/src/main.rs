use dicom_object::open_file; // F
use dicom_dictionary_std::tags; // Standard DICOM tags
use std::collections::HashMap;
use serde_json::{json, Value}; // Used for building JSON-like output

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the DICOM file
    let file_path = r"ID_0003_AGE_0075_CONTRAST_1_CT.dcm";

    let obj = open_file(file_path)?;

    let mut dicom_data: HashMap<String, HashMap<String, String>> = HashMap::new();

    // mapping of VR abbreviations to their full descriptions
    let vr_full_names = HashMap::from([
        ("AE", "Application Entity"),
        ("AS", "Age String"),
        ("AT", "Attribute Tag"),
        ("CS", "Code String"),
        ("DA", "Date"),
        ("DS", "Decimal String"),
        ("DT", "Date Time"),
        ("FL", "Floating Point Single"),
        ("FD", "Floating Point Double"),
        ("IS", "Integer String"),
        ("LO", "Long String"),
        ("LT", "Long Text"),
        ("OB", "Other Byte String"),
        ("OD", "Other Double"),
        ("OF", "Other Float"),
        ("OW", "Other Word String"),
        ("PN", "PatientName"),
        ("SH", "Short String"),
        ("SL", "Signed Long"),
        ("SQ", "Sequence"),
        ("SS", "Signed Short"),
        ("ST", "Short Text"),
        ("TM", "Time"),
        ("UI", "Unique Identifier"),
        ("UL", "Unsigned Long"),
        ("UN", "Unknown"),
        ("UR", "Uniform Resource Identifier"),
        ("US", "Unsigned Short"),
        ("UT", "Unlimited Text"),
    ]);

    // Iterate through all tags in the file
    for tag in obj.tags() {
        let element = obj.element(tag)?;

        // If the tag is PixelData, skip or process it separately
        if tag == tags::PIXEL_DATA {
            continue;
        }

        // Get the tag and VR (Value Representation)
        let tag_str = format!("{:?}", element.header().tag);
        let vr_str = format!("{:?}", element.vr());

        // Map the VR abbreviation to the full name
        let full_vr_name = vr_full_names.get(&vr_str.as_str())
            .unwrap_or(&"Unknown VR") 
            .to_string(); 
        let value_str = format!("{:?}", element.value());

        let tag_details = dicom_data.entry(tag_str).or_insert_with(HashMap::new);
        tag_details.insert("VR".to_string(), full_vr_name);
        tag_details.insert("Value".to_string(), value_str);
    }


    let json_output: Value = json!(dicom_data);

    // Print the resulting JSON
    println!("{}", serde_json::to_string_pretty(&json_output)?);

    Ok(())
}
