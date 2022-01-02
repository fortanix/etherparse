use super::*;
use proptest::prelude::*;
use etherparse::de::UnexpectedEndOfSliceError;
use etherparse::de;

mod read_error {
    use super::*;

    proptest! {
        #[test]
        fn display(
            arg_u8 in any::<u8>(),
            arg_u16 in any::<u16>(),
            arg2_u16 in any::<u16>(),
            arg_usize in any::<usize>(),
            arg_usize2 in any::<usize>(),
        ) {

            use super::ReadError::*;
            use de::Ipv4Error::*;
            use de::Ipv4TotalLengthSmallerThanIhlError;

            //IoError
            {
                let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "some error");
                assert_eq!(
                    &format!("{}", custom_error),
                    &format!("{}", IoError(custom_error))
                );
            }

            //UnexpectedEndOfSlice
            assert_eq!(
                &format!(
                    "{}",
                    UnexpectedEndOfSlice(
                        UnexpectedEndOfSliceError {
                            expected_min_len: arg_usize,
                            actual_len: arg_usize2,
                        }
                    )
                ),
                &format!(
                    "{}",
                    UnexpectedEndOfSliceError {
                        expected_min_len: arg_usize,
                        actual_len: arg_usize2,
                    }
                )
            );

            //DoubleVlanOuterNonVlanEtherType
            assert_eq!(
                &format!("ReadError: Expected a double vlan header, but the ether type field value {} of the outer vlan header is a non vlan header ether type.", arg_u16),
                &format!("{}", DoubleVlanOuterNonVlanEtherType(arg_u16))
            );

            //IpUnsupportedVersion
            assert_eq!(
                &format!("ReadError: Unsupported IP version number. The IP header contained the unsupported version number {}.", arg_u8),
                &format!("{}", IpUnsupportedVersion(arg_u8))
            );

            //Ipv4(UnexpectedIpVersion)
            assert_eq!(
                &format!("de::Ipv4Error: Unexpected IP version number. Expected an IPv4 Header but the header contained the version number {}.", arg_u8),
                &format!("{}", Ipv4(IpVersionNot4(arg_u8)))
            );

            //Ipv4(IhlTooSmall)
            assert_eq!(
                &format!("de::Ipv4Error: The 'ihl' (Internet Header length) field in the IPv4 header has a value of '{}' which is smaller then minimum size of an IPv4 header (5).", arg_u8),
                &format!("{}", Ipv4(IhlTooSmall(arg_u8)))
            );

            //Ipv4(TotalLengthSmallerThanIhl)
            assert_eq!(
                &format!("de::Ipv4Error: The IPv4 'total_length' of {} octets is smaller then the length of {} octets the header itself (based on ihl).", arg2_u16, arg_u16),
                &format!("{}", Ipv4(TotalLengthSmallerThanIhl(Ipv4TotalLengthSmallerThanIhlError{ header_length: arg_u16, total_length: arg2_u16 })))
            );

            //Ipv6UnexpectedVersion
            assert_eq!(
                &format!("ReadError: Unexpected IP version number. Expected an IPv6 Header but the header contained the version number {}.", arg_u8),
                &format!("{}", Ipv6UnexpectedVersion(arg_u8))
            );

            //Ipv6HopByHopHeaderNotAtStart
            assert_eq!(
                &format!("ReadError: Encountered an IPv6 hop-by-hop header somwhere else then directly after the IPv6 header. This is not allowed according to RFC 8200."),
                &format!("{}", Ipv6HopByHopHeaderNotAtStart)
            );

            //IpAuthenticationHeaderTooSmallPayloadLength
            assert_eq!(
                &format!("ReadError: Authentication header payload size is smaller then 1 ({}) which is smaller then the minimum size of the header.", arg_u8),
                &format!("{}", IpAuthenticationHeaderTooSmallPayloadLength(arg_u8))
            );

            //TcpDataOffsetTooSmall
            assert_eq!(
                &format!("ReadError: TCP data offset too small. The data offset value {} in the tcp header is smaller then the tcp header itself.", arg_u8),
                &format!("{}", TcpDataOffsetTooSmall(arg_u8))
            );
        }
    }

    /// Check that only for std::io::Error a source is returned
    #[test]
    fn error_source() {
        use super::ReadError::*;
        use super::de::Ipv4Error::*;
        use super::de::Ipv4TotalLengthSmallerThanIhlError;
        use std::error::Error;

        let some_values = [
            IoError(std::io::Error::new(std::io::ErrorKind::Other, "some error")),
            UnexpectedEndOfSlice(
                UnexpectedEndOfSliceError {
                    expected_min_len: 0,
                    actual_len: 0,
                }
            ),
            Ipv4(IpVersionNot4(0)),
            Ipv4(IhlTooSmall(0)),
            Ipv4(
                TotalLengthSmallerThanIhl(
                    Ipv4TotalLengthSmallerThanIhlError{
                        header_length: 0,
                        total_length: 0
                    }
                )
            ),
        ];
        for value in &some_values {
            assert!(value.source().is_some());
        }

        let none_values = [
            DoubleVlanOuterNonVlanEtherType(0),
            IpUnsupportedVersion(0),
            Ipv6UnexpectedVersion(0),
            Ipv6HopByHopHeaderNotAtStart,
            IpAuthenticationHeaderTooSmallPayloadLength(0),
            TcpDataOffsetTooSmall(0),
        ];

        for value in &none_values {
            assert!(value.source().is_none());
        }
    }

    #[test]
    fn debug() {
        use super::ReadError::*;
        use super::de::Ipv4Error::*;
        use super::de::Ipv4TotalLengthSmallerThanIhlError;

        let values = [
            IoError(std::io::Error::new(std::io::ErrorKind::Other, "some error")),
            UnexpectedEndOfSlice(UnexpectedEndOfSliceError{ expected_min_len: 0, actual_len: 0 }),
            DoubleVlanOuterNonVlanEtherType(0),
            IpUnsupportedVersion(0),
            Ipv4(IpVersionNot4(0)),
            Ipv4(IhlTooSmall(0)),
            Ipv4(TotalLengthSmallerThanIhl(Ipv4TotalLengthSmallerThanIhlError{ header_length: 0, total_length: 0 })),
            Ipv6UnexpectedVersion(0),
            Ipv6HopByHopHeaderNotAtStart,
            IpAuthenticationHeaderTooSmallPayloadLength(0),
            TcpDataOffsetTooSmall(0),
        ];

        for value in &values {
            format!("{:?}", value);
        }
    }

    #[test]
    fn add_slice_offset() {
        use super::*;
        assert_matches!(
            ReadError::UnexpectedEndOfSlice(
                UnexpectedEndOfSliceError{ expected_min_len: 2, actual_len: 1 }
            ).add_slice_offset(3),
            ReadError::UnexpectedEndOfSlice(
                UnexpectedEndOfSliceError{ expected_min_len: 5, actual_len: 4 }
            )
        );
        assert_matches!(
            ReadError::DoubleVlanOuterNonVlanEtherType(2).add_slice_offset(3),
            ReadError::DoubleVlanOuterNonVlanEtherType(2)
        );
    }

    #[test]
    fn io_error() {
        use super::*;
        assert_eq!(
            std::io::ErrorKind::Other,
            ReadError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
            .io_error().unwrap().kind()
        );
        assert!(
            ReadError::UnexpectedEndOfSlice(
                UnexpectedEndOfSliceError{ expected_min_len: 0, actual_len: 0 }
            ).io_error().is_none()
        );
    }

    #[test]
    fn unexpected_end_of_slice_min_expected_size() {
        use super::*;
        assert!(
            ReadError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
            .unexpected_end_of_slice_min_expected_size().is_none()
        );
        assert_eq!(
            123,
            ReadError::UnexpectedEndOfSlice(
                UnexpectedEndOfSliceError {
                    expected_min_len: 123,
                    actual_len: 0,
                }
            ).unexpected_end_of_slice_min_expected_size().unwrap()
        );
    }

    #[test]
    fn from_io_error() {
        assert_matches!(ReadError::from(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")),
                        ReadError::IoError(_));
    }
}

/// Check the write error display fmt generate the expected strings
#[test]
fn write_error_display() {

    use WriteError::{IoError, SliceTooSmall};
    use ValueError::Ipv4OptionsLengthBad;

    //IoError
    {
        let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "some error");
        assert_eq!(
            &format!("{}", custom_error),
            &format!("{}", IoError(custom_error))
        );
    }

    //ValueError
    {
        let value_error = Ipv4OptionsLengthBad(0);
        assert_eq!(
            &format!("ValueError: {}", value_error),
            &format!("{}", WriteError::ValueError(value_error))
        );
    }

    //SliceTooSmall
    {
        let size = 1234;
        assert_eq!(
            &format!("SliceTooSmall: The slice given to write to is too small (required to be at least {} bytes large)", size),
            &format!("{}", SliceTooSmall(size))
        );
    }
}

/// Check the write error display fmt generate the expected strings
#[test]
fn write_error_source() {
    use super::WriteError::{IoError, SliceTooSmall};
    use std::error::Error;

    assert_matches!(
        IoError(std::io::Error::new(std::io::ErrorKind::Other, "some error")).source(),
        Some(_)
    );

    assert_matches!(
        WriteError::ValueError(ValueError::Ipv4OptionsLengthBad(0)).source(),
        Some(_)
    );

    assert_matches!(
        SliceTooSmall(0).source(),
        None
    );
}

/// Check that all values return None as source
#[test]
fn value_error_source() {
    use ValueError::*;
    use std::error::Error;

    let none_values = [
        Ipv4OptionsLengthBad(0),
        Ipv4PayloadLengthTooLarge(0),
        Ipv6PayloadLengthTooLarge(0),
        Ipv6ExtensionPayloadTooLarge(0),
        IpAuthenticationHeaderBadIcvLength(0),
        Ipv4ExtensionNotReferenced(IpNumber::Icmp),
        Ipv6ExtensionNotReferenced(IpNumber::Icmp),
        Ipv6ExtensionNotDefinedReference(IpNumber::Icmp),
        UdpPayloadLengthTooLarge(0),
        TcpLengthTooLarge(0),
        U8TooLarge{ value:0, max:0, field:ErrorField::Ipv4Dscp },
        U16TooLarge{ value:0, max:0, field:ErrorField::Ipv4Dscp },
        U32TooLarge{ value:0, max:0, field:ErrorField::Ipv4Dscp },
    ];

    for value in &none_values {
        assert_matches!(value.source(), None);
    }
}

#[test]
fn value_error_debug() {
    use ValueError::*;

    let values = [
        Ipv4OptionsLengthBad(0),
        Ipv4PayloadLengthTooLarge(0),
        Ipv6PayloadLengthTooLarge(0),
        Ipv6ExtensionPayloadTooLarge(0),
        IpAuthenticationHeaderBadIcvLength(0),
        Ipv4ExtensionNotReferenced(IpNumber::Icmp),
        Ipv6ExtensionNotReferenced(IpNumber::Icmp),
        Ipv6ExtensionNotDefinedReference(IpNumber::Icmp),
        UdpPayloadLengthTooLarge(0),
        TcpLengthTooLarge(0),
        U8TooLarge{ value:0, max:0, field:ErrorField::Ipv4Dscp },
        U16TooLarge{ value:0, max:0, field:ErrorField::Ipv4Dscp },
        U32TooLarge{ value:0, max:0, field:ErrorField::Ipv4Dscp },
    ];

    for value in &values {
        format!("{:?}", value);
    }
}

proptest! {
    #[test]
    fn value_error_display(
        value_u8 in any::<u8>(),
        max_u8 in any::<u8>(),
        value_u16 in any::<u16>(),
        max_u16 in any::<u16>(),
        value_u32 in any::<u32>(),
        max_u32 in any::<u32>(),
        arg_usize in any::<usize>(),
        field in error_field_any(),
        ip_protocol_number in ip_number_any(),
    ) {
        use ValueError::*;

        //Ipv4OptionsLengthBad
        assert_eq!(
            &format!("Bad IPv4 'options_len'. The IPv4 options length ({} bytes) is either not a multiple of 4 bytes or bigger then the maximum of 40 bytes.", arg_usize),
            &format!("{}", Ipv4OptionsLengthBad(arg_usize))
        );

        //Ipv4PayloadLengthTooLarge
        assert_eq!(
            &format!("IPv4 'total_legnth' too large. The IPv4 header and payload have a larger size ({} bytes) than can be be represented by the 'total_legnth' field in the IPv4 header.", arg_usize),
            &format!("{}", Ipv4PayloadLengthTooLarge(arg_usize))
        );

        //Ipv6PayloadLengthTooLarge
        assert_eq!(
            &format!("IPv6 'payload_length' too large. The IPv6 header block & payload size ({} bytes) is larger then what can be be represented by the 'payload_length' field in the IPv6 header.", arg_usize),
            &format!("{}", Ipv6PayloadLengthTooLarge(arg_usize))
        );

        //Ipv6ExtensionPayloadTooSmall
        assert_eq!(
            &format!("IPv6 extensions header payload length is too small. The payload size ({} bytes) is less then 6 octets which is the minimum IPv6 extension header payload size.", arg_usize),
            &format!("{}", Ipv6ExtensionPayloadTooSmall(arg_usize))
        );

        //Ipv6ExtensionPayloadTooLarge
        assert_eq!(
            &format!("IPv6 extensions header payload length is too large. The payload size ({} bytes) is larger then what can be be represented by the 'extended header size' field in an IPv6 extension header.", arg_usize),
            &format!("{}", Ipv6ExtensionPayloadTooLarge(arg_usize))
        );

        //Ipv6ExtensionPayloadLengthUnaligned
        assert_eq!(
            &format!("IPv6 extensions header 'payload length ({} bytes) + 2' is not multiple of 8 (+ 2 for the `next_header` and `header_length` fields). This is required as the header length field can only express lengths in multiple of 8 bytes.", arg_usize),
            &format!("{}", Ipv6ExtensionPayloadLengthUnaligned(arg_usize))
        );

        //IpAuthenticationHeaderBadIcvLength
        assert_eq!(
            &format!("IP authentication header 'raw_icv' value has a length ({} bytes) is either not a multiple of 4 bytes or bigger then the maximum of 1016 bytes.", arg_usize),
            &format!("{}", IpAuthenticationHeaderBadIcvLength(arg_usize))
        );

        //Ipv4ExtensionNotReferenced
        assert_eq!(
            &format!("IPv4 extensions '{:?}' is defined but is not referenced by any of the 'next_header' of the other extension headers or the 'protocol' field of the IPv4 header.", ip_protocol_number),
            &format!("{}", Ipv4ExtensionNotReferenced(ip_protocol_number))
        );

        //Ipv6ExtensionHopByHopNotAtStart
        assert_eq!(
            "IPv6 extensions hop-by-hop is not located directly after the IPv6 header (required by IPv6).",
            &format!("{}", Ipv6ExtensionHopByHopNotAtStart)
        );

        //Ipv6ExtensionNotReferenced
        assert_eq!(
            &format!("IPv6 extensions '{:?}' is defined but is not referenced by any of the 'next_header' of the other extension headers or the IPv6 header.", ip_protocol_number),
            &format!("{}", Ipv6ExtensionNotReferenced(ip_protocol_number))
        );

        //Ipv6ExtensionNotDefinedReference
        assert_eq!(
            &format!("IPv6 extensions '{:?}' is referenced by the 'next_header' field of an extension headers or the IPv6 header but is not defined in the 'Ipv6Extensions'.", ip_protocol_number),
            &format!("{}", Ipv6ExtensionNotDefinedReference(ip_protocol_number))
        );

        //UdpPayloadLengthTooLarge
        assert_eq!(
            &format!("UDP 'length' too large. The UDP length ({} bytes) is larger then what can be be represented by the 'length' field in the UDP header.", arg_usize),
            &format!("{}", UdpPayloadLengthTooLarge(arg_usize))
        );

        //TcpLengthTooLarge
        assert_eq!(
            &format!("TCP length too large. The TCP packet length ({} bytes) is larger then what is supported.", arg_usize),
            &format!("{}", TcpLengthTooLarge(arg_usize))
        );

        //U8TooLarge
        assert_eq!(
            &format!("The value {} of the field '{}' is larger then the allowed maximum of {}.", value_u8, field, max_u8),
            &format!("{}", U8TooLarge{
                value: value_u8,
                max: max_u8,
                field: field.clone()
            })
        );

        //U16TooLarge
        assert_eq!(
            &format!("The value {} of the field '{}' is larger then the allowed maximum of {}.", value_u16, field, max_u16),
            &format!("{}", U16TooLarge{
                value: value_u16,
                max: max_u16,
                field: field.clone()
            })
        );

        //U32TooLarge
        assert_eq!(
            &format!("The value {} of the field '{}' is larger then the allowed maximum of {}.", value_u32, field, max_u32),
            &format!("{}", U32TooLarge{
                value: value_u32,
                max: max_u32,
                field: field.clone()
            })
        );
    }
}

#[test]
fn error_field_display() {
    use ErrorField::*;

    assert_eq!("Ipv4Header.payload_len", &format!("{}", Ipv4PayloadLength));
    assert_eq!("Ipv4Header.differentiated_services_code_point", &format!("{}", Ipv4Dscp));
    assert_eq!("Ipv4Header.explicit_congestion_notification", &format!("{}", Ipv4Ecn));
    assert_eq!("Ipv4Header.fragments_offset", &format!("{}", Ipv4FragmentsOffset));
    assert_eq!("Ipv6Header.flow_label", &format!("{}", Ipv6FlowLabel));
    assert_eq!("Ipv6FragmentHeader.fragment_offset", &format!("{}", Ipv6FragmentOffset));
    assert_eq!("SingleVlanHeader.priority_code_point", &format!("{}", VlanTagPriorityCodePoint));
    assert_eq!("SingleVlanHeader.vlan_identifier", &format!("{}", VlanTagVlanId));
}
