//! Defines for USB parsed device descriptors; extends the `usb` module.
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

use crate::usb::*;
use crate::error::{self, Error, ErrorKind};

/// USB Descriptor Types
///
/// Can enclose struct of descriptor data
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[repr(u8)]
#[allow(missing_docs)]
// TODO structs for others
pub enum DescriptorType {
    Device(ClassDescriptor) = 0x01,
    Config(ClassDescriptor) = 0x02,
    String(String) = 0x03,
    Interface(ClassDescriptor) = 0x04,
    Endpoint(ClassDescriptor) = 0x05,
    DeviceQualifier = 0x06,
    OtherSpeedConfiguration = 0x07,
    InterfacePower = 0x08,
    // TODO do_otg
    Otg = 0x09,
    Debug = 0x0a,
    InterfaceAssociation(InterfaceAssociationDescriptor) = 0x0b,
    Security(SecurityDescriptor) = 0x0c,
    Key = 0x0d,
    Encrypted(EncryptionDescriptor) = 0x0e,
    Bos = 0x0f,
    DeviceCapability = 0x10,
    WirelessEndpointCompanion = 0x11,
    WireAdaptor = 0x21,
    Report(HidReportDescriptor) = 0x22,
    Physical = 0x23,
    Pipe = 0x24,
    // TODO do_hub
    Hub = 0x29,
    SuperSpeedHub = 0x2a,
    SsEndpointCompanion(SsEndpointCompanionDescriptor) = 0x30,
    SsIsocEndpointCompanion = 0x31,
    // these are internal
    Unknown(Vec<u8>) = 0xfe,
    Junk(Vec<u8>) = 0xff,
}

impl TryFrom<&[u8]> for DescriptorType {
    type Error = Error;

    fn try_from(v: &[u8]) -> error::Result<Self> {
        if v.len() < 2 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Descriptor type too short, must be at least 2 bytes",
            ));
        }

        // junk length
        if v[0] < 2 {
            return Ok(DescriptorType::Junk(v.to_vec()));
        }

        match v[1] {
            0x01 => Ok(DescriptorType::Device(ClassDescriptor::try_from(v)?)),
            0x02 => Ok(DescriptorType::Config(ClassDescriptor::try_from(v)?)),
            0x03 => Ok(DescriptorType::String(
                String::from_utf8_lossy(v).to_string(),
            )),
            0x04 => Ok(DescriptorType::Interface(ClassDescriptor::try_from(v)?)),
            0x05 => Ok(DescriptorType::Endpoint(ClassDescriptor::try_from(v)?)),
            0x06 => Ok(DescriptorType::DeviceQualifier),
            0x07 => Ok(DescriptorType::OtherSpeedConfiguration),
            0x08 => Ok(DescriptorType::InterfacePower),
            0x09 => Ok(DescriptorType::Otg),
            0x0a => Ok(DescriptorType::Debug),
            0x0b => Ok(DescriptorType::InterfaceAssociation(
                InterfaceAssociationDescriptor::try_from(v)?,
            )),
            0x0c => Ok(DescriptorType::Security(SecurityDescriptor::try_from(v)?)),
            0x0d => Ok(DescriptorType::Key),
            0x0e => Ok(DescriptorType::Encrypted(EncryptionDescriptor::try_from(
                v,
            )?)),
            0x0f => Ok(DescriptorType::Bos),
            0x10 => Ok(DescriptorType::DeviceCapability),
            0x11 => Ok(DescriptorType::WirelessEndpointCompanion),
            0x21 => Ok(DescriptorType::WireAdaptor),
            0x22 => Ok(DescriptorType::Report(HidReportDescriptor::try_from(v)?)),
            0x23 => Ok(DescriptorType::Physical),
            0x24 => Ok(DescriptorType::Pipe),
            0x29 => Ok(DescriptorType::Hub),
            0x2a => Ok(DescriptorType::SuperSpeedHub),
            0x30 => Ok(DescriptorType::SsEndpointCompanion(
                SsEndpointCompanionDescriptor::try_from(v)?,
            )),
            0x31 => Ok(DescriptorType::SsIsocEndpointCompanion),
            _ => Ok(DescriptorType::Unknown(v.to_vec())),
        }
    }
}

impl DescriptorType {
    /// Uses [`ClassCodeTriplet`] to update the [`ClassDescriptor`] with [`ClassCode`] for class specific descriptors
    pub fn update_with_class_context<T: Into<ClassCode> + Copy>(
        &mut self,
        class_triplet: ClassCodeTriplet<T>,
    ) -> Result<(), Error> {
        match self {
            DescriptorType::Device(d) => d.update_with_class_context(class_triplet),
            DescriptorType::Config(c) => c.update_with_class_context(class_triplet),
            DescriptorType::Interface(i) => i.update_with_class_context(class_triplet),
            DescriptorType::Endpoint(e) => e.update_with_class_context(class_triplet),
            _ => Ok(()),
        }
    }
}

/// Device Capability Type Codes (Wireless USB spec and USB 3.0 bus spec)
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(missing_docs)]
#[repr(u8)]
pub enum DeviceCapability {
    WirelessUsb = 0x01,
    Usb20Extension = 0x02,
    Superspeed = 0x03,
    ContainerId = 0x04,
    Platform = 0x05,
    SuperSpeedPlus = 0x0a,
    BillBoard = 0x0d,
    BillboardAltMode = 0x0f,
    ConfigurationSummary = 0x10,
}

/// Extra USB device data for unknown descriptors
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorData(pub Vec<u8>);

/// The Interface Association Descriptor is a specific type of USB descriptor used to associate a group of interfaces with a particular function or feature of a USB device
///
/// It helps organize and convey the relationship between different interfaces within a single device configuration.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct InterfaceAssociationDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub first_interface: u8,
    pub interface_count: u8,
    pub function_class: u8,
    pub function_sub_class: u8,
    pub function_protocol: u8,
    pub function_string_index: u8,
    pub function_string: Option<String>,
}

impl TryFrom<&[u8]> for InterfaceAssociationDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 8 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Interface Association descriptor too short",
            ));
        }

        Ok(InterfaceAssociationDescriptor {
            length: value[0],
            descriptor_type: value[1],
            first_interface: value[2],
            interface_count: value[3],
            function_class: value[4],
            function_sub_class: value[5],
            function_protocol: value[6],
            function_string_index: value[7],
            function_string: None,
        })
    }
}

/// USB SS Endpoint Companion descriptor
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SsEndpointCompanionDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub max_burst: u8,
    pub attributes: u8,
}

impl TryFrom<&[u8]> for SsEndpointCompanionDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 6 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "SS Endpoint Companion descriptor too short",
            ));
        }

        Ok(SsEndpointCompanionDescriptor {
            length: value[0],
            descriptor_type: value[1],
            max_burst: value[2],
            attributes: value[3],
        })
    }
}

/// USB security descriptor
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SecurityDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub total_length: u16,
    pub encryption_types: u8,
}

impl TryFrom<&[u8]> for SecurityDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 5 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Security descriptor too short",
            ));
        }

        Ok(SecurityDescriptor {
            length: value[0],
            descriptor_type: value[1],
            total_length: u16::from_le_bytes([value[2], value[3]]),
            encryption_types: value[4],
        })
    }
}

/// Encryption type for [`SecurityDescriptor`]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum EncryptionType {
    Unsecure,
    Wired,
    Ccm1,
    Rsa1,
    Reserved,
}

impl From<u8> for EncryptionType {
    fn from(b: u8) -> Self {
        match b {
            0x00 => EncryptionType::Unsecure,
            0x01 => EncryptionType::Wired,
            0x02 => EncryptionType::Ccm1,
            0x03 => EncryptionType::Rsa1,
            _ => EncryptionType::Reserved,
        }
    }
}

/// USB encryption descriptor
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EncryptionDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub encryption_type: EncryptionType,
    pub encryption_value: u8,
    pub auth_key_index: u8,
}

impl TryFrom<&[u8]> for EncryptionDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 5 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Encryption Type descriptor too short",
            ));
        }

        Ok(EncryptionDescriptor {
            length: value[0],
            descriptor_type: value[1],
            encryption_type: EncryptionType::from(value[2]),
            encryption_value: value[3],
            auth_key_index: value[4],
        })
    }
}

/// USB base class descriptor
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassDescriptor {
    /// USB HID extra descriptor
    Hid(HidDescriptor),
    /// USB Communication extra descriptor
    Communication(CommunicationDescriptor),
    /// USB CCID (Smart Card) extra descriptor
    Ccid(CcidDescriptor),
    /// USB Printer extra descriptor
    Printer(PrinterDescriptor),
    /// USB MIDI extra descriptor (AudioVideoAVDataAudio)
    Midi(MidiDescriptor, u8),
    /// USB Video extra descriptor
    Video(UvcDescriptor, u8),
    /// Generic descriptor with Option<ClassCode>
    ///
    /// Used for most descriptors and allows for TryFrom without knowing the [`ClassCode`]
    Generic(Option<ClassCodeTriplet<ClassCode>>, GenericDescriptor),
}

impl TryFrom<&[u8]> for ClassDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 3 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Class descriptor too short",
            ));
        }

        Ok(ClassDescriptor::Generic(
            None,
            GenericDescriptor::try_from(value)?,
        ))
    }
}

impl From<ClassDescriptor> for Vec<u8> {
    fn from(cd: ClassDescriptor) -> Self {
        match cd {
            ClassDescriptor::Generic(_, gd) => gd.into(),
            ClassDescriptor::Hid(hd) => hd.into(),
            ClassDescriptor::Ccid(cd) => cd.into(),
            ClassDescriptor::Printer(pd) => pd.into(),
            ClassDescriptor::Communication(cd) => cd.into(),
            ClassDescriptor::Midi(md, _) => md.into(),
            ClassDescriptor::Video(vd, _) => vd.into(),
        }
    }
}

impl ClassDescriptor {
    /// Uses [`ClassCodeTriplet`] to update the [`ClassDescriptor`] with [`ClassCode`] and descriptor if it is not [`GenericDescriptor`]
    pub fn update_with_class_context<T: Into<ClassCode> + Copy>(
        &mut self,
        triplet: ClassCodeTriplet<T>,
    ) -> Result<(), Error> {
        if let ClassDescriptor::Generic(_, gd) = self {
            match (triplet.0.into(), triplet.1, triplet.2) {
                (ClassCode::HID, _, _) => {
                    *self = ClassDescriptor::Hid(HidDescriptor::try_from(gd.to_owned())?)
                }
                (ClassCode::SmartCart, _, _) => {
                    *self = ClassDescriptor::Ccid(CcidDescriptor::try_from(gd.to_owned())?)
                }
                (ClassCode::Printer, _, _) => {
                    *self = ClassDescriptor::Printer(PrinterDescriptor::try_from(gd.to_owned())?)
                }
                (ClassCode::CDCCommunications, _, _) | (ClassCode::CDCData, _, _) => {
                    *self = ClassDescriptor::Communication(CommunicationDescriptor::try_from(
                        gd.to_owned(),
                    )?)
                }
                (ClassCode::Audio, 3, p) => {
                    *self = ClassDescriptor::Midi(MidiDescriptor::try_from(gd.to_owned())?, p)
                }
                (ClassCode::Video, 1, p) => {
                    *self = ClassDescriptor::Video(UvcDescriptor::try_from(gd.to_owned())?, p)
                }
                ct => *self = ClassDescriptor::Generic(Some(ct), gd.to_owned()),
            }
        }

        Ok(())
    }
}

/// USB HID report descriptor
///
/// Similar to [`GenericDescriptor`] but with a wLength rather than bLength and no sub-type
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct HidReportDescriptor {
    pub descriptor_type: u8,
    pub length: u16,
    pub data: Option<Vec<u8>>,
}

impl TryFrom<&[u8]> for HidReportDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 3 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "HID report descriptor too short",
            ));
        }

        if value[0] != 0x22 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "HID report descriptor must have descriptor type 0x22",
            ));
        }

        Ok(HidReportDescriptor {
            descriptor_type: value[0],
            length: u16::from_le_bytes([value[1], value[2]]),
            data: value.get(3..).map(|d| d.to_vec()),
        })
    }
}

impl From<HidReportDescriptor> for Vec<u8> {
    fn from(hd: HidReportDescriptor) -> Self {
        let mut ret = Vec::new();
        ret.push(hd.descriptor_type);
        ret.extend(hd.length.to_le_bytes());
        if let Some(data) = hd.data {
            ret.extend(data);
        }

        ret
    }
}

/// USB generic descriptor
///
/// Used for most [`ClassDescriptor`]s
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct GenericDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub descriptor_subtype: u8,
    pub data: Option<Vec<u8>>,
}

impl TryFrom<&[u8]> for GenericDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 3 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Generic descriptor too short",
            ));
        }

        let length = value[0];
        if length as usize > value.len() {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Generic descriptor reported length too long for data returned",
            ));
        }

        Ok(GenericDescriptor {
            length,
            descriptor_type: value[1],
            descriptor_subtype: value[2],
            data: value.get(3..).map(|d| d.to_vec()),
        })
    }
}

impl From<GenericDescriptor> for Vec<u8> {
    fn from(gd: GenericDescriptor) -> Self {
        let mut ret = Vec::new();
        ret.push(gd.length);
        ret.push(gd.descriptor_type);
        ret.push(gd.descriptor_subtype);
        if let Some(data) = gd.data {
            ret.extend(data);
        }

        ret
    }
}

impl GenericDescriptor {
    /// Returns the reported length of the data
    pub fn len(&self) -> usize {
        self.length as usize
    }

    /// Returns true if the reported length of the data is 0
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the expected length of the data based on the length field minus the bytes taken by struct fields
    pub fn expected_data_length(&self) -> usize {
        if self.len() < 3 {
            0
        } else {
            self.len() - 3
        }
    }

    /// Returns the (cloned) data as a Vec<u8>
    pub fn to_vec(&self) -> Vec<u8> {
        self.clone().into()
    }
}

/// USB HID descriptor
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct HidDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub bcd_hid: Version,
    pub country_code: u8,
    pub descriptors: Vec<HidReportDescriptor>,
}

impl TryFrom<&[u8]> for HidDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 6 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "HID descriptor too short",
            ));
        }

        let num_descriptors = value[5] as usize;
        let mut descriptors_vec = value[6..].to_vec();
        let mut descriptors = Vec::<HidReportDescriptor>::with_capacity(num_descriptors);

        for _ in 0..num_descriptors {
            if descriptors_vec.len() < 2 {
                return Err(Error::new(
                    ErrorKind::InvalidArg,
                    "HID report descriptor too short",
                ));
            }

            let len = u16::from_le_bytes([descriptors_vec[1], descriptors_vec[2]]) as usize;

            if len > descriptors_vec.len() {
                return Err(Error::new(
                    ErrorKind::InvalidArg,
                    "HID report descriptor too long for available data!",
                ));
            }

            descriptors.push(descriptors_vec.drain(..len).as_slice().try_into()?);
        }

        Ok(HidDescriptor {
            length: value[0],
            descriptor_type: value[1],
            bcd_hid: Version::from_bcd(u16::from_le_bytes([value[2], value[3]])),
            country_code: value[4],
            descriptors,
        })
    }
}

impl TryFrom<GenericDescriptor> for HidDescriptor {
    type Error = Error;

    fn try_from(gd: GenericDescriptor) -> error::Result<Self> {
        let gd_vec: Vec<u8> = gd.into();
        HidDescriptor::try_from(&gd_vec[..])
    }
}

impl From<HidDescriptor> for Vec<u8> {
    fn from(hd: HidDescriptor) -> Self {
        let mut ret = Vec::new();
        ret.push(hd.length);
        ret.push(hd.descriptor_type);
        ret.extend(u16::from(hd.bcd_hid).to_le_bytes());
        ret.push(hd.country_code);
        for desc in hd.descriptors {
            ret.extend(Vec::<u8>::from(desc));
        }

        ret
    }
}

/// USB CCID (Smart Card) descriptor
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CcidDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub version: Version,
    pub max_slot_index: u8,
    pub voltage_support: u8,
    pub protocols: u32,
    pub default_clock: u32,
    pub max_clock: u32,
    pub num_clock_supported: u8,
    pub data_rate: u32,
    pub max_data_rate: u32,
    pub num_data_rates_supp: u8,
    pub max_ifsd: u32,
    pub sync_protocols: u32,
    pub mechanical: u32,
    pub features: u32,
    pub max_ccid_msg_len: u32,
    pub class_get_response: u8,
    pub class_envelope: u8,
    pub lcd_layout: (u8, u8),
    pub pin_support: u8,
    pub max_ccid_busy_slots: u8,
}

impl TryFrom<&[u8]> for CcidDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 54 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "CCID descriptor too short",
            ));
        }

        let lcd_layout = (value[50], value[51]);

        Ok(CcidDescriptor {
            length: value[0],
            descriptor_type: value[1],
            version: Version::from_bcd(u16::from_le_bytes([value[2], value[3]])),
            max_slot_index: value[4],
            voltage_support: value[5],
            protocols: u32::from_le_bytes([value[6], value[7], value[8], value[9]]),
            default_clock: u32::from_le_bytes([value[10], value[11], value[12], value[13]]),
            max_clock: u32::from_le_bytes([value[14], value[15], value[16], value[17]]),
            num_clock_supported: value[18],
            data_rate: u32::from_le_bytes([value[19], value[20], value[21], value[22]]),
            max_data_rate: u32::from_le_bytes([value[23], value[24], value[25], value[26]]),
            num_data_rates_supp: value[27],
            max_ifsd: u32::from_le_bytes([value[28], value[29], value[30], value[31]]),
            sync_protocols: u32::from_le_bytes([value[32], value[33], value[34], value[35]]),
            mechanical: u32::from_le_bytes([value[36], value[37], value[38], value[39]]),
            features: u32::from_le_bytes([value[40], value[41], value[42], value[43]]),
            max_ccid_msg_len: u32::from_le_bytes([value[44], value[45], value[46], value[47]]),
            class_get_response: value[48],
            class_envelope: value[49],
            lcd_layout,
            pin_support: value[52],
            max_ccid_busy_slots: value[53],
        })
    }
}

impl TryFrom<GenericDescriptor> for CcidDescriptor {
    type Error = Error;

    fn try_from(gd: GenericDescriptor) -> error::Result<Self> {
        let gd_vec: Vec<u8> = gd.into();
        CcidDescriptor::try_from(&gd_vec[..])
    }
}

impl From<CcidDescriptor> for Vec<u8> {
    fn from(cd: CcidDescriptor) -> Self {
        let mut ret = Vec::new();
        ret.push(cd.length);
        ret.push(cd.descriptor_type);
        ret.extend(u16::from(cd.version).to_le_bytes());
        ret.push(cd.max_slot_index);
        ret.push(cd.voltage_support);
        ret.extend(cd.protocols.to_le_bytes());
        ret.extend(cd.default_clock.to_le_bytes());
        ret.extend(cd.max_clock.to_le_bytes());
        ret.push(cd.num_clock_supported);
        ret.extend(cd.data_rate.to_le_bytes());
        ret.extend(cd.max_data_rate.to_le_bytes());
        ret.push(cd.num_data_rates_supp);
        ret.extend(cd.max_ifsd.to_le_bytes());
        ret.extend(cd.sync_protocols.to_le_bytes());
        ret.extend(cd.mechanical.to_le_bytes());
        ret.extend(cd.features.to_le_bytes());
        ret.extend(cd.max_ccid_msg_len.to_le_bytes());
        ret.push(cd.class_get_response);
        ret.push(cd.class_envelope);
        ret.push(cd.lcd_layout.0);
        ret.push(cd.lcd_layout.1);
        ret.push(cd.pin_support);
        ret.push(cd.max_ccid_busy_slots);

        ret
    }
}

/// USB printer descriptor
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PrinterDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub release_number: u8,
    pub descriptors: Vec<PrinterReportDescriptor>,
}

impl TryFrom<&[u8]> for PrinterDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 3 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Printer descriptor too short",
            ));
        }

        let num_descriptors = value[3] as usize;
        let mut descriptors_vec = value[4..].to_vec();
        let mut descriptors = Vec::<PrinterReportDescriptor>::with_capacity(num_descriptors);

        for _ in 0..num_descriptors {
            if descriptors_vec.len() < 2 {
                return Err(Error::new(
                    ErrorKind::InvalidArg,
                    "Printer report descriptor too short",
                ));
            }

            // +2 for length and descriptor type
            let len = descriptors_vec[1] as usize + 2;

            if descriptors_vec.len() < len {
                break;
            }

            descriptors.push(descriptors_vec.drain(..len).as_slice().try_into()?);
        }

        Ok(PrinterDescriptor {
            length: value[0],
            descriptor_type: value[1],
            release_number: value[2],
            descriptors,
        })
    }
}

impl TryFrom<GenericDescriptor> for PrinterDescriptor {
    type Error = Error;

    fn try_from(gd: GenericDescriptor) -> error::Result<Self> {
        let gd_vec: Vec<u8> = gd.into();
        PrinterDescriptor::try_from(&gd_vec[..])
    }
}

impl From<PrinterDescriptor> for Vec<u8> {
    fn from(pd: PrinterDescriptor) -> Self {
        let mut ret = Vec::new();
        ret.push(pd.length);
        ret.push(pd.descriptor_type);
        ret.push(pd.release_number);
        for desc in pd.descriptors {
            ret.extend(Vec::<u8>::from(desc));
        }

        ret
    }
}

/// USB printer report descriptor
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PrinterReportDescriptor {
    pub descriptor_type: u8,
    pub length: u8,
    pub capabilities: u16,
    pub versions_supported: u8,
    pub uuid_string_index: u8,
    pub uuid_string: Option<String>,
    pub data: Option<Vec<u8>>,
}

impl TryFrom<&[u8]> for PrinterReportDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 6 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Printer report descriptor too short",
            ));
        }

        Ok(PrinterReportDescriptor {
            descriptor_type: value[0],
            length: value[1],
            capabilities: u16::from_le_bytes([value[2], value[3]]),
            versions_supported: value[4],
            uuid_string_index: value[5],
            uuid_string: None,
            data: value.get(6..).map(|d| d.to_vec()),
        })
    }
}

impl From<PrinterReportDescriptor> for Vec<u8> {
    fn from(prd: PrinterReportDescriptor) -> Self {
        let mut ret = Vec::new();
        ret.push(prd.descriptor_type);
        ret.push(prd.length);
        ret.extend(prd.capabilities.to_le_bytes());
        ret.push(prd.versions_supported);
        ret.push(prd.uuid_string_index);

        ret
    }
}

/// USB Communication Device Class (CDC) types
///
/// Used to differentiate between different CDC descriptors
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum CdcType {
    Header = 0x00,
    CallManagement = 0x01,
    AbstractControlManagement = 0x02,
    DirectLineManagement = 0x03,
    TelephoneRinger = 0x04,
    TelephoneCall = 0x05,
    Union = 0x06,
    CountrySelection = 0x07,
    TelephoneOperationalModes = 0x08,
    UsbTerminal = 0x09,
    NetworkChannel = 0x0a,
    ProtocolUnit = 0x0b,
    ExtensionUnit = 0x0c,
    MultiChannel = 0x0d,
    CapiControl = 0x0e,
    EthernetNetworking = 0x0f,
    AtmNetworking = 0x10,
    WirelessHandsetControlModel = 0x11,
    MobileDirectLineModelFunctional = 0x12,
    MobileDirectLineModelDetail = 0x13,
    DeviceManagement = 0x14,
    Obex = 0x15,
    CommandSet = 0x16,
    CommandSetDetail = 0x17,
    TelephoneControlModel = 0x18,
    ObexCommandSet = 0x19,
    Ncm = 0x1a,
    Mbim = 0x1b,
    MbimExtended = 0x1c,
    Unknown = 0xff,
}

impl From<u8> for CdcType {
    fn from(b: u8) -> Self {
        match b {
            0x00 => CdcType::Header,
            0x01 => CdcType::CallManagement,
            0x02 => CdcType::AbstractControlManagement,
            0x03 => CdcType::DirectLineManagement,
            0x04 => CdcType::TelephoneRinger,
            0x05 => CdcType::TelephoneCall,
            0x06 => CdcType::Union,
            0x07 => CdcType::CountrySelection,
            0x08 => CdcType::TelephoneOperationalModes,
            0x09 => CdcType::UsbTerminal,
            0x0a => CdcType::NetworkChannel,
            0x0b => CdcType::ProtocolUnit,
            0x0c => CdcType::ExtensionUnit,
            0x0d => CdcType::MultiChannel,
            0x0e => CdcType::CapiControl,
            0x0f => CdcType::EthernetNetworking,
            0x10 => CdcType::AtmNetworking,
            0x11 => CdcType::WirelessHandsetControlModel,
            0x12 => CdcType::MobileDirectLineModelFunctional,
            0x13 => CdcType::MobileDirectLineModelDetail,
            0x14 => CdcType::DeviceManagement,
            0x15 => CdcType::Obex,
            0x16 => CdcType::CommandSet,
            0x17 => CdcType::CommandSetDetail,
            0x18 => CdcType::TelephoneControlModel,
            0x19 => CdcType::ObexCommandSet,
            0x1a => CdcType::Ncm,
            0x1b => CdcType::Mbim,
            0x1c => CdcType::MbimExtended,
            _ => CdcType::Unknown,
        }
    }
}

/// USB Communication Device Class (CDC) descriptor
///
/// Can be used by CDCData and CDCCommunications
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CommunicationDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub communication_type: CdcType,
    pub string_index: Option<u8>,
    pub string: Option<String>,
    pub data: Vec<u8>,
}

impl TryFrom<&[u8]> for CommunicationDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 4 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Communication descriptor too short",
            ));
        }

        let length = value[0];
        if length as usize > value.len() {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Communication descriptor reported length too long for buffer",
            ));
        }

        let communication_type = CdcType::from(value[2]);
        // some CDC types have descriptor strings with index in the data
        let string_index = match communication_type {
            CdcType::EthernetNetworking | CdcType::CountrySelection => {
                value.get(3).map(|v| v.to_owned())
            }
            CdcType::NetworkChannel => value.get(4).map(|v| v.to_owned()),
            CdcType::CommandSet => value.get(5).map(|v| v.to_owned()),
            _ => None,
        };

        Ok(CommunicationDescriptor {
            length,
            descriptor_type: value[1],
            communication_type,
            string_index,
            string: None,
            data: value[3..].to_vec(),
        })
    }
}

impl From<CommunicationDescriptor> for Vec<u8> {
    fn from(cd: CommunicationDescriptor) -> Self {
        let mut ret = Vec::new();
        ret.push(cd.length);
        ret.push(cd.descriptor_type);
        ret.push(cd.communication_type as u8);
        ret.extend(cd.data);

        ret
    }
}

impl TryFrom<GenericDescriptor> for CommunicationDescriptor {
    type Error = Error;

    fn try_from(gd: GenericDescriptor) -> error::Result<Self> {
        let gd_vec: Vec<u8> = gd.into();
        CommunicationDescriptor::try_from(&gd_vec[..])
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[repr(u8)]
#[non_exhaustive]
pub enum UvcInterface {
    Undefined = 0x00,
    Header = 0x01,
    InputTerminal = 0x02,
    OutputTerminal = 0x03,
    SelectorUnit = 0x04,
    ProcessingUnit = 0x05,
    ExtensionUnit = 0x06,
    EncodingUnit = 0x07,
}

impl From<u8> for UvcInterface {
    fn from(b: u8) -> Self {
        match b {
            0x00 => UvcInterface::Undefined,
            0x01 => UvcInterface::Header,
            0x02 => UvcInterface::InputTerminal,
            0x03 => UvcInterface::OutputTerminal,
            0x04 => UvcInterface::SelectorUnit,
            0x05 => UvcInterface::ProcessingUnit,
            0x06 => UvcInterface::ExtensionUnit,
            0x07 => UvcInterface::EncodingUnit,
            _ => UvcInterface::Undefined,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct UvcDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub descriptor_subtype: u8,
    pub string_index: Option<u8>,
    pub string: Option<String>,
    pub data: Vec<u8>,
}

impl TryFrom<&[u8]> for UvcDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 4 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Video Control descriptor too short",
            ));
        }

        let length = value[0];
        if length as usize > value.len() {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Video Control descriptor reported length too long for buffer",
            ));
        }

        let video_control_subtype = UvcInterface::from(value[2]);

        let string_index = match video_control_subtype {
            UvcInterface::InputTerminal => value.get(7).copied(),
            UvcInterface::OutputTerminal => value.get(8).copied(),
            UvcInterface::SelectorUnit => {
                if let Some(p) = value.get(4) {
                    value.get(5 + *p as usize).copied()
                } else {
                    None
                }
            }
            UvcInterface::ProcessingUnit => {
                if let Some(n) = value.get(7) {
                    value.get(8 + *n as usize).copied()
                } else {
                    None
                }
            }
            UvcInterface::ExtensionUnit => {
                if let Some(p) = value.get(21) {
                    if let Some(n) = value.get(22 + *p as usize) {
                        value.get(23 + *n as usize + *p as usize).copied()
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            UvcInterface::EncodingUnit => value.get(5).copied(),
            _ => None,
        };

        Ok(UvcDescriptor {
            length,
            descriptor_type: value[1],
            descriptor_subtype: value[2],
            string_index,
            string: None,
            data: value[3..].to_vec(),
        })
    }
}

impl From<UvcDescriptor> for Vec<u8> {
    fn from(vcd: UvcDescriptor) -> Self {
        let mut ret = Vec::new();
        ret.push(vcd.length);
        ret.push(vcd.descriptor_type);
        ret.push(vcd.descriptor_subtype);
        ret.extend(vcd.data);

        ret
    }
}

impl TryFrom<GenericDescriptor> for UvcDescriptor {
    type Error = Error;

    fn try_from(gd: GenericDescriptor) -> error::Result<Self> {
        let gd_vec: Vec<u8> = gd.into();
        UvcDescriptor::try_from(&gd_vec[..])
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[repr(u8)]
#[non_exhaustive]
pub enum MidiInterface {
    Undefined = 0x00,
    Header = 0x01,
    InputJack = 0x02,
    OutputJack = 0x03,
    Element = 0x04,
}

impl From<u8> for MidiInterface {
    fn from(b: u8) -> Self {
        match b {
            0x00 => MidiInterface::Undefined,
            0x01 => MidiInterface::Header,
            0x02 => MidiInterface::InputJack,
            0x03 => MidiInterface::OutputJack,
            0x04 => MidiInterface::Element,
            _ => MidiInterface::Undefined,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MidiDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub midi_type: MidiInterface,
    pub string_index: Option<u8>,
    pub string: Option<String>,
    pub data: Vec<u8>,
}

impl TryFrom<&[u8]> for MidiDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 4 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "MidiDescriptor descriptor too short",
            ));
        }

        let length = value[0];
        if length as usize > value.len() {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "MidiDescriptor descriptor reported length too long for buffer",
            ));
        }

        let midi_type = MidiInterface::from(value[2]);

        let string_index = match midi_type {
            MidiInterface::InputJack => value.get(5).copied(),
            MidiInterface::OutputJack => value.get(5).map(|v| 6 + *v * 2),
            MidiInterface::Element => {
                // don't ask...
                if let Some(j) = value.get(4) {
                    if let Some(capsize) = value.get((5 + *j as usize * 2) + 3) {
                        value.get(9 + 2 * *j as usize + *capsize as usize).copied()
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        Ok(MidiDescriptor {
            length,
            descriptor_type: value[1],
            midi_type,
            string_index,
            string: None,
            data: value[3..].to_vec(),
        })
    }
}

impl From<MidiDescriptor> for Vec<u8> {
    fn from(md: MidiDescriptor) -> Self {
        let mut ret = Vec::new();
        ret.push(md.length);
        ret.push(md.descriptor_type);
        ret.push(md.midi_type as u8);
        ret.extend(md.data);

        ret
    }
}

impl TryFrom<GenericDescriptor> for MidiDescriptor {
    type Error = Error;

    fn try_from(gd: GenericDescriptor) -> error::Result<Self> {
        let gd_vec: Vec<u8> = gd.into();
        MidiDescriptor::try_from(&gd_vec[..])
    }
}

/// USB Audio Class (UAC) interface types based on bDescriptorSubtype
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum UacInterface {
    Undefined = 0x00,
    Header = 0x01,
    InputTerminal = 0x02,
    OutputTerminal = 0x03,
    ExtendedTerminal = 0x04,
    MixerUnit = 0x05,
    SelectorUnit = 0x06,
    FeatureUnit = 0x07,
    EffectUnit = 0x08,
    ProcessingUnit = 0x09,
    ExtensionUnit = 0x0a,
    ClockSource = 0x0b,
    ClockSelector = 0x0c,
    ClockMultiplier = 0x0d,
    SampleRateConverter = 0x0e,
    Connectors = 0x0f,
    PowerDomain = 0x10,
}

impl std::fmt::Display for UacInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // uppercase with _ instead of space for lsusb dump
            match self {
                UacInterface::Undefined => write!(f, "UNDEFINED"),
                UacInterface::Header => write!(f, "HEADER"),
                UacInterface::InputTerminal => write!(f, "INPUT_TERMINAL"),
                UacInterface::OutputTerminal => write!(f, "OUTPUT_TERMINAL"),
                UacInterface::ExtendedTerminal => write!(f, "EXTENDED_TERMINAL"),
                UacInterface::MixerUnit => write!(f, "MIXER_UNIT"),
                UacInterface::SelectorUnit => write!(f, "SELECTOR_UNIT"),
                UacInterface::FeatureUnit => write!(f, "FEATURE_UNIT"),
                UacInterface::EffectUnit => write!(f, "EFFECT_UNIT"),
                UacInterface::ProcessingUnit => write!(f, "PROCESSING_UNIT"),
                UacInterface::ExtensionUnit => write!(f, "EXTENSION_UNIT"),
                UacInterface::ClockSource => write!(f, "CLOCK_SOURCE"),
                UacInterface::ClockSelector => write!(f, "CLOCK_SELECTOR"),
                UacInterface::ClockMultiplier => write!(f, "CLOCK_MULTIPLIER"),
                UacInterface::SampleRateConverter => write!(f, "SAMPLE_RATE_CONVERTER"),
                UacInterface::Connectors => write!(f, "CONNECTORS"),
                UacInterface::PowerDomain => write!(f, "POWER_DOMAIN"),
            }
        } else {
            match self {
                UacInterface::Undefined => write!(f, "Undefined"),
                UacInterface::Header => write!(f, "Header"),
                UacInterface::InputTerminal => write!(f, "Input Terminal"),
                UacInterface::OutputTerminal => write!(f, "Output Terminal"),
                UacInterface::ExtendedTerminal => write!(f, "Extended Terminal"),
                UacInterface::MixerUnit => write!(f, "Mixer Unit"),
                UacInterface::SelectorUnit => write!(f, "Selector Unit"),
                UacInterface::FeatureUnit => write!(f, "Feature Unit"),
                UacInterface::EffectUnit => write!(f, "Effect Unit"),
                UacInterface::ProcessingUnit => write!(f, "Processing Unit"),
                UacInterface::ExtensionUnit => write!(f, "Extension Unit"),
                UacInterface::ClockSource => write!(f, "Clock Source"),
                UacInterface::ClockSelector => write!(f, "Clock Selector"),
                UacInterface::ClockMultiplier => write!(f, "Clock Multiplier"),
                UacInterface::SampleRateConverter => write!(f, "Sample Rate Converter"),
                UacInterface::Connectors => write!(f, "Connectors"),
                UacInterface::PowerDomain => write!(f, "Power Domain"),
            }
        }
    }
}

impl From<u8> for UacInterface {
    fn from(b: u8) -> Self {
        match b {
            0x00 => UacInterface::Undefined,
            0x01 => UacInterface::Header,
            0x02 => UacInterface::InputTerminal,
            0x03 => UacInterface::OutputTerminal,
            0x04 => UacInterface::ExtendedTerminal,
            0x05 => UacInterface::MixerUnit,
            0x06 => UacInterface::SelectorUnit,
            0x07 => UacInterface::FeatureUnit,
            0x08 => UacInterface::EffectUnit,
            0x09 => UacInterface::ProcessingUnit,
            0x0a => UacInterface::ExtensionUnit,
            0x0b => UacInterface::ClockSource,
            0x0c => UacInterface::ClockSelector,
            0x0d => UacInterface::ClockMultiplier,
            0x0e => UacInterface::SampleRateConverter,
            0x0f => UacInterface::Connectors,
            0x10 => UacInterface::PowerDomain,
            _ => UacInterface::Undefined,
        }
    }
}

impl UacInterface {
    /// UAC1, UAC2, and UAC3 define bDescriptorSubtype differently for the
    /// AudioControl interface, so we need to do some ugly remapping:
    pub fn get_uac_subtype(subtype: u8, protocol: u8) -> Self {
        match protocol {
            // UAC1
            0x00 => match subtype {
                0x04 => UacInterface::MixerUnit,
                0x05 => UacInterface::SelectorUnit,
                0x06 => UacInterface::FeatureUnit,
                0x07 => UacInterface::ProcessingUnit,
                0x08 => UacInterface::ExtensionUnit,
                _ => Self::from(subtype),
            },
            // UAC2
            0x20 => match subtype {
                0x04 => UacInterface::MixerUnit,
                0x05 => UacInterface::SelectorUnit,
                0x06 => UacInterface::FeatureUnit,
                0x07 => UacInterface::EffectUnit,
                0x08 => UacInterface::ProcessingUnit,
                0x09 => UacInterface::ExtensionUnit,
                0x0a => UacInterface::ClockSource,
                0x0b => UacInterface::ClockSelector,
                0x0c => UacInterface::ClockMultiplier,
                0x0d => UacInterface::SampleRateConverter,
                _ => Self::from(subtype),
            },
            // no re-map for UAC3..
            _ => Self::from(subtype),
        }
    }

    /// Get the UAC interface descriptor from the UAC interface
    pub fn get_descriptor(
        &self,
        protocol: &UacProtocol,
        data: &[u8],
    ) -> Result<UacInterfaceDescriptor, Error> {
        UacInterfaceDescriptor::from_uac_interface(self, protocol, data)
    }
}

/// USB Audio Class (UAC) interface descriptors
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum UacInterfaceDescriptor {
    AudioHeader1(AudioHeader1),
    AudioHeader2(AudioHeader2),
    AudioHeader3(AudioHeader3),
}

impl UacInterfaceDescriptor {
    /// Get the UAC interface descriptor from the UAC interface
    pub fn from_uac_interface(
        uac_interface: &UacInterface,
        protocol: &UacProtocol,
        data: &[u8],
    ) -> Result<Self, Error> {
        match uac_interface {
            UacInterface::Header => match protocol {
                UacProtocol::Uac1 => {
                    AudioHeader1::try_from(data).map(UacInterfaceDescriptor::AudioHeader1)
                }
                UacProtocol::Uac2 => {
                    AudioHeader2::try_from(data).map(UacInterfaceDescriptor::AudioHeader2)
                }
                UacProtocol::Uac3 => {
                    AudioHeader3::try_from(data).map(UacInterfaceDescriptor::AudioHeader3)
                }
                _ => Err(Error::new(
                    ErrorKind::InvalidArg,
                    "Protocol not supported for this interface",
                )),
            },
            _ => Err(Error::new(
                ErrorKind::InvalidArg,
                "Interface not supported for this descriptor",
            )),
        }
    }
}

/// USB Audio Class (UAC) protocol byte defines the version of the UAC
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum UacProtocol {
    Uac1 = 0x00,
    Uac2 = 0x20,
    Uac3 = 0x30,
    Unknown,
}

impl From<u8> for UacProtocol {
    fn from(b: u8) -> Self {
        match b {
            0x00 => UacProtocol::Uac1,
            0x20 => UacProtocol::Uac2,
            0x30 => UacProtocol::Uac3,
            _ => UacProtocol::Unknown,
        }
    }
}

impl std::fmt::Display for UacProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UacProtocol::Uac1 => write!(f, "UAC1"),
            UacProtocol::Uac2 => write!(f, "UAC2"),
            UacProtocol::Uac3 => write!(f, "UAC3"),
            UacProtocol::Unknown => write!(f, "Unknown"),
        }
    }
}

/// The control setting for a UAC bmControls byte
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ControlSetting {
    ReadOnly = 0b01,
    IllegalValue = 0b10,
    ReadWrite = 0b11,
}

impl From<u8> for ControlSetting {
    fn from(b: u8) -> Self {
        match b {
            0b01 => ControlSetting::ReadOnly,
            0b10 => ControlSetting::IllegalValue,
            0b11 => ControlSetting::ReadWrite,
            _ => ControlSetting::IllegalValue,
        }
    }
}

impl fmt::Display for ControlSetting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ControlSetting::ReadOnly => write!(f, "read-only"),
            ControlSetting::IllegalValue => write!(f, "ILLEGAL VALUE (0b10)"),
            ControlSetting::ReadWrite => write!(f, "read/write"),
        }
    }
}

/// UAC bmControl can be 1 bit for just the control type or 2 bits for control type and whether it's read-only
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ControlType {
    BmControl1,
    BmControl2,
}

/// UAC1 Header
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct AudioHeader1 {
    pub version: Version,
    pub total_length: u16,
    pub collection_bytes: u8,
    pub interfaces: Vec<u8>,
}

impl TryFrom<&[u8]> for AudioHeader1 {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 6 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Audio Header 1 descriptor too short",
            ));
        }

        let total_length = u16::from_le_bytes([value[2], value[3]]);
        let collection_bytes = value[4];
        let interfaces = value[5..].to_vec();

        Ok(AudioHeader1 {
            version: Version::from_bcd(u16::from_le_bytes([value[0], value[1]])),
            total_length,
            collection_bytes,
            interfaces,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct AudioHeader2 {
    pub version: Version,
    pub category: u8,
    pub total_length: u16,
    pub controls: u8,
}

impl TryFrom<&[u8]> for AudioHeader2 {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 6 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Audio Header 2 descriptor too short",
            ));
        }

        let total_length = u16::from_le_bytes([value[3], value[4]]);
        let controls = value[5];

        Ok(AudioHeader2 {
            version: Version::from_bcd(u16::from_le_bytes([value[0], value[1]])),
            category: value[2],
            total_length,
            controls,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct AudioHeader3 {
    pub category: u8,
    pub total_length: u16,
    pub controls: u32,
}

impl TryFrom<&[u8]> for AudioHeader3 {
    type Error = Error;

    fn try_from(value: &[u8]) -> error::Result<Self> {
        if value.len() < 7 {
            return Err(Error::new(
                ErrorKind::InvalidArg,
                "Audio Header 3 descriptor too short",
            ));
        }

        let total_length = u16::from_le_bytes([value[1], value[2]]);
        let controls = u32::from_le_bytes([value[3], value[4], value[5], value[6]]);

        Ok(AudioHeader3 {
            category: value[0],
            total_length,
            controls,
        })
    }
}