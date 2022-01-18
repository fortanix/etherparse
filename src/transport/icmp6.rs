use super::super::*;

use std::slice::from_raw_parts;

/// Module containing ICMPv6 related constants
pub mod icmpv6 {

    /// ICMPv6 type value indicating a "Destination Unreachable" message.
    pub const TYPE_DST_UNREACH: u8 = 1;

    /// ICMPv6 type value indicating a "Packet Too Big" message.
    pub const TYPE_PACKET_TOO_BIG: u8 = 2;

    /// ICMPv6 type value indicating a "Time Exceeded" message.
    pub const TYPE_TIME_EXCEEDED: u8 = 3;

    /// ICMPv6 type value indicating a "Parameter Problem" message.
    pub const TYPE_PARAM_PROB: u8 = 4;

    /// ICMPv6 type value indicating an "Echo Request" message.
    pub const TYPE_ECHO_REQUEST: u8 = 128;

    /// ICMPv6 type value indicating an "Echo Reply" message.
    pub const TYPE_ECHO_REPLY: u8 = 129;

    /// ICMPv6 type value indicating a "Multicast Listener Query" message.
    pub const TYPE_MULTICAST_LISTENER_QUERY: u8 = 130;

    /// ICMPv6 type value indicating a "Multicast Listener Report" message.
    pub const TYPE_MULTICAST_LISTENER_REPORT: u8 = 131;

    /// ICMPv6 type value indicating a "Multicast Listener Done" message.
    pub const TYPE_MULTICAST_LISTENER_REDUCTION: u8 = 132;

    /// ICMPv6 type value indicating a "Router Solicitation" message.
    pub const TYPE_ROUTER_SOLICITATION: u8 = 133;

    /// ICMPv6 type value indicating a "Router Advertisement" message.
    pub const TYPE_ROUTER_ADVERTISEMENT: u8 = 134;

    /// ICMPv6 type value indicating a "Neighbor Solicitation" message.
    pub const TYPE_NEIGHBOR_SOLICITATION: u8 = 135;

    /// ICMPv6 type value indicating a "Neighbor Advertisement" message.
    pub const TYPE_NEIGHBOR_ADVERTISEMENT: u8 = 136;

    /// ICMPv6 type value indicating a "Redirect Message" message.
    pub const TYPE_REDIRECT_MESSAGE: u8 = 137;

    /// ICMPv6 type value indicating a "Router Renumbering" message.
    pub const TYPE_ROUTER_RENUMBERING: u8 = 138;

    /// ICMPv6 type value indicating a "Inverse Neighbor Discovery Solicitation" message.
    pub const TYPE_INVERSE_NEIGHBOR_DISCOVERY_SOLICITATION: u8 = 141;

    /// ICMPv6 type value indicating a "Inverse Neighbor Discovery Advertisement" message.
    pub const TYPE_INVERSE_NEIGHBOR_DISCOVERY_ADVERTISEMENT: u8 = 142;

    /// ICMPv6 type value indicating a "Extended Echo Request" message.
    pub const TYPE_EXT_ECHO_REQUEST: u8 = 160;

    /// ICMPv6 type value indicating a "Extended Echo Reply" message.
    pub const TYPE_EXT_ECHO_REPLY: u8 = 161;

    /// ICMPv6 destination unreachable code for "no route to destination".
    pub const CODE_DST_UNREACH_NOROUTE: u8 = 0;

    /// ICMPv6 destination unreachable code for "communication with
    /// destination administratively prohibited".
    pub const CODE_DST_UNREACH_PROHIBITED: u8 = 1;

    /// ICMPv6 destination unreachable code for "beyond scope of source address".
    pub const CODE_DST_UNREACH_BEYONDSCOPE: u8 = 2;

    /// ICMPv6 destination unreachable code for "address unreachable".
    pub const CODE_DST_UNREACH_ADDR: u8 = 3;

    /// ICMPv6 destination unreachable code for "port unreachable".
    pub const CODE_DST_UNREACH_PORT: u8 = 4;

    /// ICMPv6 destination unreachable code for "source address failed ingress/egress policy".
    pub const CODE_DST_UNREACH_SOURCE_ADDRESS_FAILED_POLICY: u8 = 5;

    /// ICMPv6 destination unreachable code for "reject route to destination".
    pub const CODE_DST_UNREACH_REJECT_ROUTE_TO_DEST: u8 = 6;

    /// ICMPv6 time exceeded code for "hop limit exceeded in transit"
    pub const CODE_TIME_EXCEEDED_HOP_LIMIT_EXCEEDED: u8 = 0;

    /// ICMPv6 time exceeded code for "fragment reassembly time exceeded"
    pub const CODE_TIME_EXCEEDED_FRAGMENT_REASSEMBLY_TIME_EXCEEDED: u8 = 1;
}

use icmpv6::*;

/// "Destination Unreachable" ICMPv6 package (without the invoking packet).
///
/// # RFC 4443 Description:
///
/// A Destination Unreachable message SHOULD be generated by a router, or
/// by the IPv6 layer in the originating node, in response to a packet
/// that cannot be delivered to its destination address for reasons other
/// than congestion.  (An ICMPv6 message MUST NOT be generated if a
/// packet is dropped due to congestion.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icmp6DestUnreachable {
    /// In case of an unknown icmp code is received the header elements are stored raw.
    Raw{
        /// ICMP code (present in the 2nd byte of the ICMP packet).
        code: u8,
        /// Bytes located at th 5th, 6th, 7th and 8th position of the ICMP packet.
        four_bytes: [u8;4],
    },
    /// No route to destination
    NoRoute,
    /// Communication with destination administratively prohibited
    Prohibited,
    /// Beyond scope of source address
    BeyondScope,
    /// Address unreachable
    Address,
    /// Port unreachable
    Port,
    /// Source address failed ingress/egress policy
    SourceAddressFailedPolicy,
    /// Reject route to destination
    RejectRoute,
}

impl Icmp6DestUnreachable {
    /// Converts the raw values from an ICMPv6 "destination unreachable"
    /// packet to an `Icmp6DestUnreachable` enum.
    ///
    /// `from_bytes` expects the second byte as first argument and 5th-8th
    /// bytes as second argument.
    ///
    /// # Example Usage:
    ///
    /// ```
    /// use etherparse::{icmpv6, Icmp6DestUnreachable};
    /// let icmp_packet: [u8;8] = [
    ///     icmpv6::TYPE_DST_UNREACH, icmpv6::CODE_DST_UNREACH_PORT, 0, 0,
    ///     0, 0, 0, 0,
    /// ];
    ///
    /// if icmpv6::TYPE_DST_UNREACH == icmp_packet[0] {
    ///     let dst = Icmp6DestUnreachable::from_bytes(
    ///         icmp_packet[1],
    ///         [icmp_packet[4], icmp_packet[5], icmp_packet[6], icmp_packet[7]],
    ///     );
    ///     assert_eq!(dst, Icmp6DestUnreachable::Port);
    /// }
    /// ```
    pub fn from_bytes(code: u8, four_bytes: [u8;4]) -> Icmp6DestUnreachable {
        use Icmp6DestUnreachable::*;
        match code {
            CODE_DST_UNREACH_NOROUTE => NoRoute,
            CODE_DST_UNREACH_PROHIBITED => Prohibited,
            CODE_DST_UNREACH_BEYONDSCOPE => BeyondScope,
            CODE_DST_UNREACH_ADDR => Address,
            CODE_DST_UNREACH_PORT => Port,
            CODE_DST_UNREACH_SOURCE_ADDRESS_FAILED_POLICY => SourceAddressFailedPolicy,
            CODE_DST_UNREACH_REJECT_ROUTE_TO_DEST => RejectRoute,
            _ => Raw{code, four_bytes},
        }
    }

    /// Returns the code value of the destination unreachable packet.
    ///
    /// This is the second byte of an ICMPv6 packet.
    pub fn code(&self) -> u8 {
        use Icmp6DestUnreachable::*;
        match self {
            Raw{code, four_bytes: _} => *code,
            NoRoute => CODE_DST_UNREACH_NOROUTE,
            Prohibited => CODE_DST_UNREACH_PROHIBITED,
            BeyondScope => CODE_DST_UNREACH_BEYONDSCOPE,
            Address => CODE_DST_UNREACH_ADDR,
            Port => CODE_DST_UNREACH_PORT,
            SourceAddressFailedPolicy => CODE_DST_UNREACH_SOURCE_ADDRESS_FAILED_POLICY,
            RejectRoute => CODE_DST_UNREACH_REJECT_ROUTE_TO_DEST,
        }
    }

    /// Returns second and and 5th-8th bytes (inclusive) of
    /// the destination unreachable ICMPv6 packet.
    pub fn to_bytes(&self) -> (u8, [u8;4]) {
        use Icmp6DestUnreachable::*;
        match self {
            Raw{ code, four_bytes } => (*code, *four_bytes),
            NoRoute => (CODE_DST_UNREACH_NOROUTE, [0;4]),
            Prohibited => (CODE_DST_UNREACH_PROHIBITED, [0;4]),
            BeyondScope => (CODE_DST_UNREACH_BEYONDSCOPE, [0;4]),
            Address => (CODE_DST_UNREACH_ADDR, [0;4]),
            Port => (CODE_DST_UNREACH_PORT, [0;4]),
            SourceAddressFailedPolicy => (CODE_DST_UNREACH_SOURCE_ADDRESS_FAILED_POLICY, [0;4]),
            RejectRoute => (CODE_DST_UNREACH_REJECT_ROUTE_TO_DEST, [0;4]),
        }
    }
}

/// Code values for ICMPv6 time exceeded message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icmp6TimeExceededCode {
    /// In case of an unknown icmp code is received the header elements are stored raw.
    Raw{ code: u8 },
    /// "hop limit exceeded in transit"
    HopLimitExceeded,
    /// "fragment reassembly time exceeded"
    FragmentReassemblyTimeExceeded,
}


impl From<u8> for Icmp6TimeExceededCode {
    fn from(code: u8) -> Icmp6TimeExceededCode {
        use Icmp6TimeExceededCode::*;
        match code {
            CODE_TIME_EXCEEDED_HOP_LIMIT_EXCEEDED => HopLimitExceeded,
            CODE_TIME_EXCEEDED_FRAGMENT_REASSEMBLY_TIME_EXCEEDED => FragmentReassemblyTimeExceeded,
            code => Raw { code },
        }
    }
}

impl From<Icmp6TimeExceededCode> for u8 {
    fn from(code: Icmp6TimeExceededCode) -> u8 {
        use Icmp6TimeExceededCode::*;
        match code {
            Raw{ code } => code,
            HopLimitExceeded => CODE_TIME_EXCEEDED_HOP_LIMIT_EXCEEDED,
            FragmentReassemblyTimeExceeded => CODE_TIME_EXCEEDED_FRAGMENT_REASSEMBLY_TIME_EXCEEDED,
        }
    }
}

/// Code values for ICMPv6 parameter problem messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icmp6ParameterProblemCode {
    /// In case of an unknown icmp code is received the header elements are stored raw.
    Raw{ code: u8 },

}

impl From<u8> for Icmp6ParameterProblemCode {
    fn from(code: u8) -> Icmp6ParameterProblemCode {
        use Icmp6ParameterProblemCode::*;
        match code {
            code => Raw { code },
        }
    }
}

impl From<Icmp6ParameterProblemCode> for u8 {
    fn from(code: Icmp6ParameterProblemCode) -> u8 {
        use Icmp6ParameterProblemCode::*;
        match code {
            Raw{ code } => code,
        }
    }
}

/// Contains the starting data of an ICMPv6 packet (without the checksum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icmp6Type {
    /// In case of an unknown icmp type is received the header elements are stored raw.
    Raw{
        icmp_type: u8,
        icmp_code: u8,
        four_bytes: [u8;4]
    },
    /// Destination Unreachable Message.
    ///
    /// # RFC 4443 Description
    ///
    /// A Destination Unreachable message SHOULD be generated by a router, or
    /// by the IPv6 layer in the originating node, in response to a packet
    /// that cannot be delivered to its destination address for reasons other
    /// than congestion.  (An ICMPv6 message MUST NOT be generated if a
    /// packet is dropped due to congestion.)
    DestinationUnreachable(Icmp6DestUnreachable),
    /// Packet Too Big Message
    ///
    /// # RFC 4443 Description
    ///
    /// A Packet Too Big MUST be sent by a router in response to a packet
    /// that it cannot forward because the packet is larger than the MTU of
    /// the outgoing link.  The information in this message is used as part
    /// of the Path MTU Discovery process.
    PacketTooBig {
        /// The Maximum Transmission Unit of the next-hop link.
        mtu: u32
    },
    /// Time Exceeded Message
    ///
    /// # RFC 4443 Description
    ///
    /// If a router receives a packet with a Hop Limit of zero, or if a
    /// router decrements a packet's Hop Limit to zero, it MUST discard the
    /// packet and originate an ICMPv6 Time Exceeded message with Code 0 to
    /// the source of the packet.  This indicates either a routing loop or
    /// too small an initial Hop Limit value.
    ///
    /// An ICMPv6 Time Exceeded message with Code 1 is used to report
    /// fragment reassembly timeout, as specified in [IPv6, Section 4.5].
    TimeExceeded {
        /// Code identifying which time as exceeded.
        code: Icmp6TimeExceededCode,
    },
    /// Parameter Problem Message
    ///
    /// # RFC 4443 Description
    ///
    /// If an IPv6 node processing a packet finds a problem with a field in
    /// the IPv6 header or extension headers such that it cannot complete
    /// processing the packet, it MUST discard the packet and SHOULD
    /// originate an ICMPv6 Parameter Problem message to the packet's source,
    /// indicating the type and location of the problem.
    ParameterProblem {
        /// The code can offer additional informations about what kind of parameter
        /// problem caused the error.
        code: Icmp6ParameterProblemCode,
        /// Identifies the octet offset within the
        /// invoking packet where the error was detected.
        ///
        /// The pointer will point beyond the end of the ICMPv6
        /// packet if the field in error is beyond what can fit
        /// in the maximum size of an ICMPv6 error message.
        pointer: u32,
    },
    /// Echo Request Message
    ///
    /// # RFC 4443 Description
    ///
    /// Every node MUST implement an ICMPv6 Echo responder function that
    /// receives Echo Requests and originates corresponding Echo Replies.  A
    /// node SHOULD also implement an application-layer interface for
    /// originating Echo Requests and receiving Echo Replies, for diagnostic
    /// purposes.
    EchoRequest(IcmpEchoHeader),
    /// Echo Reply Message
    ///
    /// # RFC 4443 Description
    ///
    /// Every node MUST implement an ICMPv6 Echo responder function that
    /// receives Echo Requests and originates corresponding Echo Replies. A
    /// node SHOULD also implement an application-layer interface for
    /// originating Echo Requests and receiving Echo Replies, for diagnostic
    /// purposes.
    ///
    /// The source address of an Echo Reply sent in response to a unicast
    /// Echo Request message MUST be the same as the destination address of
    /// that Echo Request message.
    ///
    /// An Echo Reply SHOULD be sent in response to an Echo Request message
    /// sent to an IPv6 multicast or anycast address.  In this case, the
    /// source address of the reply MUST be a unicast address belonging to
    /// the interface on which the Echo Request message was received.
    ///
    /// The data received in the ICMPv6 Echo Request message MUST be returned
    /// entirely and unmodified in the ICMPv6 Echo Reply message.
    EchoReply(IcmpEchoHeader),
}

impl Icmp6Type {
    /// Decode the enum from the icmp type, code and reserved bytes (5th till and
    /// including 8th byte of the the ICMPv6 header).
    fn from_bytes(icmp_type: u8, icmp_code: u8, four_bytes: [u8;4]) -> Icmp6Type {
        use Icmp6Type::*;
        match icmp_type {
            TYPE_DST_UNREACH => 
                DestinationUnreachable(Icmp6DestUnreachable::from_bytes(icmp_code, four_bytes)),
            TYPE_PACKET_TOO_BIG => PacketTooBig {
                mtu: u32::from_be_bytes(four_bytes),
            },
            TYPE_TIME_EXCEEDED => TimeExceeded{
                code: icmp_code.into()
            },
            TYPE_PARAM_PROB => ParameterProblem{
                code: icmp_code.into(),
                pointer: u32::from_be_bytes(four_bytes),
            },
            TYPE_ECHO_REQUEST => EchoRequest(IcmpEchoHeader::from_bytes(four_bytes)),
            TYPE_ECHO_REPLY => EchoReply(IcmpEchoHeader::from_bytes(four_bytes)),
            _ => Raw{icmp_type, icmp_code, four_bytes},
        }
    }

    /// Encode the enum to the on wire format.
    ///
    /// It returns the icmp type, code and reserved bytes (5th till and
    /// including 8th byte of the the ICMPv6 header).
    fn to_bytes(&self) -> (u8, u8, [u8;4]) {
        use Icmp6Type::*;
        match self {
            Raw{icmp_type, icmp_code, four_bytes} => (*icmp_type, *icmp_code, *four_bytes),
            DestinationUnreachable(icmp_code) => 
            (TYPE_DST_UNREACH, (icmp_code.code()), [0;4]),
            PacketTooBig{ mtu } => (TYPE_PACKET_TOO_BIG, 0, mtu.to_be_bytes()),
            TimeExceeded{ code } => (TYPE_TIME_EXCEEDED, u8::from(*code), [0;4]),
            ParameterProblem{ code, pointer } => (TYPE_PARAM_PROB, u8::from(*code), pointer.to_be_bytes()),
            EchoRequest(echo) => (TYPE_ECHO_REQUEST, 0, echo.to_bytes()),
            EchoReply(echo) => (TYPE_ECHO_REPLY, 0, echo.to_bytes()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Icmp6Header {
    pub icmp_type: Icmp6Type,
    pub icmp_chksum: u16,
}

impl Icmp6Header {
    pub const SERIALIZED_SIZE: usize = 8;

    /// Serialized length of the header in bytes/octets.
    ///
    /// Note that this size is not the size of the entire
    /// ICMPv6 packet but only the header.
    pub fn header_len(&self) -> usize {
        8
    }

    pub fn new(icmp_type: Icmp6Type) -> Icmp6Header {
        Icmp6Header{
            icmp_type,
            icmp_chksum: 0, // will be filled in later
        }
    }

    ///Write the transport header to the given writer.
    pub fn write<T: io::Write + Sized>(&self, writer: &mut T) -> Result<(), WriteError> {
        let cksum_be = self.icmp_chksum.to_be_bytes();
        let (icmp_type, icmp_code, four_bytes) = self.icmp_type.to_bytes();
        writer.write_all(&[
            icmp_type,
            icmp_code,
            cksum_be[0],
            cksum_be[1],
            four_bytes[0],
            four_bytes[1],
            four_bytes[2],
            four_bytes[3],
        ]).map_err(WriteError::from)
    }

    pub fn calc_checksum_ipv6(&self, ip_header: &Ipv6Header, payload: &[u8]) -> Result<u16, ValueError> {
        //check that the total length fits into the field
        const MAX_PAYLOAD_LENGTH: usize = (std::u32::MAX as usize) - Icmp4Header::SERIALIZED_SIZE;
        if MAX_PAYLOAD_LENGTH < payload.len() {
            return Err(ValueError::Ipv6PayloadLengthTooLarge(payload.len()));
        }

        let (icmp_type, icmp_code, four_bytes) = self.icmp_type.to_bytes();
        let msg_len = payload.len() + Icmp6Header::SERIALIZED_SIZE;
        //calculate the checksum; icmp4 will always take an ip4 header
        Ok(
                // NOTE: rfc4443 section 2.3 - Icmp6 *does* use a pseudoheader, 
                // unlike Icmp4
                checksum::Sum16BitWords::new()
                .add_16bytes(ip_header.source)
                .add_16bytes(ip_header.destination)
                .add_2bytes([0, ip_number::IPV6_ICMP])
                .add_2bytes((msg_len as u16).to_be_bytes())
                .add_2bytes([icmp_type, icmp_code])
                .add_4bytes(four_bytes)
                .add_slice(payload)
                .ones_complement()
                .to_be()
        )
    }

    /// Reads an icmp6 header from a slice directly and returns a tuple containing the resulting header & unused part of the slice.
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Result<(Icmp6Header, &[u8]), ReadError> {
        Ok((
            Icmp6HeaderSlice::from_slice(slice)?.to_header(),
            &slice[Icmp6Header::SERIALIZED_SIZE..]
        ))
    }
}

/// A slice containing an icmp6 header of a network package. Struct allows the selective read of fields in the header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Icmp6HeaderSlice<'a> {
    slice: &'a [u8]
}

impl<'a> Icmp6HeaderSlice<'a> {
    /// Creates a slice containing an icmp6 header.
    #[inline]
    pub fn from_slice(slice: &'a[u8]) -> Result<Icmp6HeaderSlice<'a>, ReadError> {
        //check length
        use crate::ReadError::*;
        if slice.len() < Icmp6Header::SERIALIZED_SIZE {
            return Err(UnexpectedEndOfSlice(Icmp6Header::SERIALIZED_SIZE));
        }

        //done
        Ok(Icmp6HeaderSlice{
            // SAFETY:
            // Safe as slice length is checked to be at least
            // Icmp6Header::SERIALIZED_SIZE (8) before this.
            slice: unsafe {
                from_raw_parts(
                    slice.as_ptr(),
                    Icmp6Header::SERIALIZED_SIZE
                )
            }
        })
    }

    /// Decode all the fields and copy the results to a [`Icmp6Header`] struct
    #[inline]
    pub fn to_header(&self) -> Icmp6Header {
        Icmp6Header {
            icmp_type: self.icmp_type(),
            icmp_chksum: self.icmp_chksum(),
        }
    }

    pub fn icmp_type(&self) -> Icmp6Type {
        // SAFETY:
        // Safe as the contructor checks that the slice has
        // at least the length of Icmp6Header::SERIALIZED_SIZE (8).
        unsafe {
            Icmp6Type::from_bytes(
                *self.slice.get_unchecked(0),
                *self.slice.get_unchecked(1),
                [
                    *self.slice.get_unchecked(4),
                    *self.slice.get_unchecked(5),
                    *self.slice.get_unchecked(6),
                    *self.slice.get_unchecked(7),
                ]
            )
        }
    }

    /// Returns "code" value in the ICMPv6 header.
    #[inline]
    pub fn icmp_code(&self) -> u8 {
        // SAFETY:
        // Safe as the contructor checks that the slice has
        // at least the length of Icmp6Header::SERIALIZED_SIZE (8).
        unsafe {
            *self.slice.get_unchecked(0)
        }
    }

    /// Returns "checksum" value in the ICMPv6 header.
    #[inline]
    pub fn icmp_chksum(&self) -> u16 {
        // SAFETY:
        // Safe as the contructor checks that the slice has
        // at least the length of UdpHeader::SERIALIZED_SIZE (8).
        unsafe {
            get_unchecked_be_u16(self.slice.as_ptr().add(2))
        }
    }

    /// Returns the slice containing the icmp6 header
    #[inline]
    pub fn slice(&self) -> &'a [u8] {
        self.slice
    }
}