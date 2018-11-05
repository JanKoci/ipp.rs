//!
//! Attribute-related structs
//!
use std::collections::HashMap;
use std::io::{self, Cursor, Read, Write};

use byteorder::{BigEndian, WriteBytesExt};

use ipp::*;
use value::IppValue;

pub const ATTRIBUTES_CHARSET: &str = "attributes-charset";
pub const ATTRIBUTES_NATURAL_LANGUAGE: &str = "attributes-natural-language";
pub const CHARSET_CONFIGURED: &str = "charset-configured";
pub const CHARSET_SUPPORTED: &str = "charset-supported";
pub const COMPRESSION_SUPPORTED: &str = "compression-supported";
pub const DOCUMENT_FORMAT_DEFAULT: &str = "document-format-default";
pub const DOCUMENT_FORMAT_SUPPORTED: &str = "document-format-supported";
pub const GENERATED_NATURAL_LANGUAGE_SUPPORTED: &str = "generated-natural-language-supported";
pub const IPP_VERSIONS_SUPPORTED: &str = "ipp-versions-supported";
pub const NATURAL_LANGUAGE_CONFIGURED: &str = "natural-language-configured";
pub const OPERATIONS_SUPPORTED: &str = "operations-supported";
pub const PDL_OVERRIDE_SUPPORTED: &str = "pdl-override-supported";
pub const PRINTER_IS_ACCEPTING_JOBS: &str = "printer-is-accepting-jobs";
pub const PRINTER_MAKE_AND_MODEL: &str = "printer-make-and-model";
pub const PRINTER_NAME: &str = "printer-name";
pub const PRINTER_STATE: &str = "printer-state";
pub const PRINTER_STATE_MESSAGE: &str = "printer-state-message";
pub const PRINTER_STATE_REASONS: &str = "printer-state-reasons";
pub const PRINTER_UP_TIME: &str = "printer-up-time";
pub const PRINTER_URI: &str = "printer-uri";
pub const PRINTER_URI_SUPPORTED: &str = "printer-uri-supported";
pub const QUEUED_JOB_COUNT: &str = "queued-job-count";
pub const URI_AUTHENTICATION_SUPPORTED: &str = "uri-authentication-supported";
pub const URI_SECURITY_SUPPORTED: &str = "uri-security-supported";
pub const JOB_ID: &str = "job-id";
pub const JOB_NAME: &str = "job-name";
pub const JOB_STATE: &str = "job-state";
pub const JOB_STATE_REASONS: &str = "job-state-reasons";
pub const JOB_URI: &str = "job-uri";
pub const LAST_DOCUMENT: &str = "last-document";
pub const REQUESTING_USER_NAME: &str = "requesting-user-name";
pub const STATUS_MESSAGE: &str = "status-message";
pub const REQUESTED_ATTRIBUTES: &str = "requested-attributes";
pub const SIDES_SUPPORTED: &str = "sides-supported";
pub const OUTPUT_MODE_SUPPORTED: &str = "output-mode-supported";
pub const COLOR_SUPPORTED: &str = "color-supported";
pub const PRINTER_INFO: &str = "printer-info";
pub const PRINTER_LOCATION: &str = "printer-location";
pub const PRINTER_MORE_INFO: &str = "printer-more-info";
pub const PRINTER_RESOLUTION_DEFAULT: &str = "printer-resolution-default";
pub const PRINTER_RESOLUTION_SUPPORTED: &str = "printer-resolution-supported";
pub const COPIES_SUPPORTED: &str = "copies-supported";
pub const COPIES_DEFAULT: &str = "copies-default";
pub const SIDES_DEFAULT: &str = "sides-default";
pub const PRINT_QUALITY_DEFAULT: &str = "print-quality-default";
pub const PRINT_QUALITY_SUPPORTED: &str = "print-quality-supported";
pub const FINISHINGS_DEFAULT: &str = "finishings-default";
pub const FINISHINGS_SUPPORTED: &str = "finishings-supported";
pub const OUTPUT_BIN_DEFAULT: &str = "output-bin-default";
pub const OUTPUT_BIN_SUPPORTED: &str = "output-bin-supported";
pub const ORIENTATION_REQUESTED_DEFAULT: &str = "orientation-requested-default";
pub const ORIENTATION_REQUESTED_SUPPORTED: &str = "orientation-requested-supported";
pub const MEDIA_DEFAULT: &str = "media-default";
pub const MEDIA_SUPPORTED: &str = "media-supported";
pub const PAGES_PER_MINUTE: &str = "pages-per-minute";
pub const COLOR_MODE_SUPPORTED: &str = "color-mode-supported";
pub const PRINT_COLOR_MODE_SUPPORTED: &str = "print-color-mode-supported";

const HEADER_ATTRS: [&str; 3] = [ATTRIBUTES_CHARSET, ATTRIBUTES_NATURAL_LANGUAGE, PRINTER_URI];

fn is_header_attr(attr: &str) -> bool {
    HEADER_ATTRS.into_iter().any(|&at| at == attr)
}

/// `IppAttribute` represents an IPP attribute
#[derive(Clone, Debug)]
pub struct IppAttribute {
    /// Attribute name
    name: String,
    /// Attribute value
    value: IppValue,
}

impl IppAttribute {
    /// Create new instance of the attribute
    ///
    /// * `name` - Attribute name<br/>
    /// * `value` - Attribute value<br/>
    pub fn new(name: &str, value: IppValue) -> IppAttribute {
        IppAttribute {
            name: name.to_string(),
            value,
        }
    }

    /// Return attribute name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return attribute value
    pub fn value(&self) -> &IppValue {
        &self.value
    }

    /// Serialize attribute into binary stream
    pub fn write(&self, writer: &mut Write) -> io::Result<usize> {
        let mut retval = 0;

        writer.write_u8(self.value.to_tag() as u8)?;
        retval += 1;

        writer.write_u16::<BigEndian>(self.name.len() as u16)?;
        retval += 2;

        writer.write_all(self.name.as_bytes())?;
        retval += self.name.len();

        retval += self.value.write(writer)?;

        Ok(retval)
    }
}

/// Attribute list indexed by group and name
#[derive(Clone, Default, Debug)]
pub struct IppAttributeList {
    attributes: HashMap<DelimiterTag, HashMap<String, IppAttribute>>,
}

impl IppAttributeList {
    /// Create attribute list
    pub fn new() -> IppAttributeList {
        IppAttributeList::default()
    }

    /// Add attribute to the list
    ///
    /// * `group` - delimiter group<br/>
    /// * `attribute` - attribute to add<br/>
    pub fn add(&mut self, group: DelimiterTag, attribute: IppAttribute) {
        self.attributes.entry(group).or_insert_with(HashMap::new);
        let opt = self.attributes.get_mut(&group).unwrap();
        opt.insert(attribute.name().to_string(), attribute);
    }

    /// Get attribute from the list
    pub fn get(&self, group: DelimiterTag, name: &str) -> Option<&IppAttribute> {
        self.attributes
            .get(&group)
            .and_then(|attrs| attrs.get(name))
    }

    /// Get attribute list for a group
    pub fn get_group(&self, group: DelimiterTag) -> Option<&HashMap<String, IppAttribute>> {
        self.attributes.get(&group)
    }

    /// Get printer attributes
    pub fn get_printer_attributes(&self) -> Option<&HashMap<String, IppAttribute>> {
        self.get_group(DelimiterTag::PrinterAttributes)
    }

    /// Get job attributes
    pub fn get_job_attributes(&self) -> Option<&HashMap<String, IppAttribute>> {
        self.get_group(DelimiterTag::JobAttributes)
    }

    /// Get operation attributes
    pub fn get_operation_attributes(&self) -> Option<&HashMap<String, IppAttribute>> {
        self.get_group(DelimiterTag::OperationAttributes)
    }

    /// Serialize attribute list into binary stream
    pub fn write(&self, writer: &mut Write) -> io::Result<usize> {
        // first send the header attributes
        writer.write_u8(DelimiterTag::OperationAttributes as u8)?;

        let mut retval = 1;

        for hdr in &HEADER_ATTRS {
            if let Some(attr) = self.get(DelimiterTag::OperationAttributes, hdr) {
                retval += attr.write(writer)?
            }
        }

        // now the rest
        for hdr in &[
            DelimiterTag::OperationAttributes,
            DelimiterTag::JobAttributes,
            DelimiterTag::PrinterAttributes,
        ] {
            let group = *hdr;
            if let Some(attrs) = self.attributes.get(&group) {
                if group != DelimiterTag::OperationAttributes {
                    writer.write_u8(group as u8)?;
                    retval += 1;
                }
                for (_, attr) in attrs.iter().filter(|&(_, v)| {
                    group != DelimiterTag::OperationAttributes || !is_header_attr(v.name())
                }) {
                    retval += attr.write(writer)?;
                }
            }
        }
        writer.write_u8(DelimiterTag::EndOfAttributes as u8)?;
        retval += 1;

        Ok(retval)
    }

    pub fn into_reader(self) -> impl Read {
        let mut buf = Vec::new();
        self.write(&mut buf).unwrap();
        Cursor::new(buf)
    }
}
