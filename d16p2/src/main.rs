use std::fs;

struct LiteralPacket {
	data: u64,
}

enum OperatorLength {
	Bits(u64),
	Subpackets(u64),
}

struct OperatorPacket {
	type_id: u64,
	length: OperatorLength,
	sub_packets: Vec<Packet>,
}

enum PacketType {
	Literal(LiteralPacket),
	Operator(OperatorPacket),
}

struct Packet {
	version: u64,
	value: PacketType,
}

struct HexadecimalParser {
	data: Vec<bool>,
}

impl HexadecimalParser {
	fn new(packet_data: String) -> Self {
		let mut raw_data: Vec<u8> = Vec::new();
		for value_index in 0..packet_data.len() / 2 {
			let index = value_index * 2;
			let value = &packet_data[index..=index + 1];
			raw_data.push(u8::from_str_radix(value, 16).unwrap());
		}

		let mut data: Vec<bool> = Vec::with_capacity(raw_data.len() * 8);
		for value in raw_data {
			data.push(value & 128 > 0);
			data.push(value & 64 > 0);
			data.push(value & 32 > 0);
			data.push(value & 16 > 0);
			data.push(value & 8 > 0);
			data.push(value & 4 > 0);
			data.push(value & 2 > 0);
			data.push(value & 1 > 0);
		}
		Self { data }
	}

	fn get_bits_in_range(&self, first: usize, last: usize) -> u64 {
		let mut result: u64 = 0;
		for index in first..=last {
			result *= 2;
			if self.data[index] {
				result += 1;
			}
		}
		result
	}

	fn get_single_bit(&self, bit: usize) -> bool {
		self.data[bit]
	}

	fn num_bits(&self) -> usize {
		self.data.len()
	}

	fn get_sub_data_from_slice(&self, first: usize, last: usize) -> Self {
		let data = self.data[first..=last].to_vec();
		Self { data }
	}
}

/// Returns the parsed packet and the location of the start of the next one
fn parse_packet_at_position(data: &HexadecimalParser, first_bit: usize) -> (Packet, usize) {
	let version = data.get_bits_in_range(first_bit, first_bit + 2);
	let type_id = data.get_bits_in_range(first_bit + 3, first_bit + 5);
	if type_id == 4 {
		let mut value = 0;
		let mut position = first_bit + 6;
		loop {
			value <<= 4;
			let continue_after = data.get_single_bit(position);
			value += data.get_bits_in_range(position + 1, position + 4);
			position += 5;
			if !continue_after {
				break;
			}
		}
		(
			Packet {
				version,
				value: PacketType::Literal(LiteralPacket { data: value }),
			},
			position,
		)
	} else if data.get_single_bit(first_bit + 6) {
		let num_sub_packets = data.get_bits_in_range(first_bit + 7, first_bit + 17);
		let mut sub_packets: Vec<Packet> = Vec::new();
		let mut start_position = first_bit + 18;
		for _ in 0..num_sub_packets {
			let (new_packet, new_start_position) = parse_packet_at_position(data, start_position);
			sub_packets.push(new_packet);
			start_position = new_start_position;
		}
		(
			Packet {
				version,
				value: PacketType::Operator(OperatorPacket {
					type_id,
					length: OperatorLength::Subpackets(num_sub_packets),
					sub_packets,
				}),
			},
			start_position,
		)
	} else {
		let total_sub_packet_length = data.get_bits_in_range(first_bit + 7, first_bit + 21);
		let sub_packet_data =
			data.get_sub_data_from_slice(first_bit + 22, first_bit + 21 + total_sub_packet_length as usize);
		let mut start_position = 0;
		let mut sub_packets: Vec<Packet> = Vec::new();
		while start_position < sub_packet_data.num_bits() {
			let (next_packet, new_start_position) = parse_packet_at_position(&sub_packet_data, start_position);
			sub_packets.push(next_packet);
			start_position = new_start_position;
		}
		(
			Packet {
				version,
				value: PacketType::Operator(OperatorPacket {
					type_id,
					length: OperatorLength::Bits(total_sub_packet_length),
					sub_packets,
				}),
			},
			first_bit + 22 + start_position,
		)
	}
}

fn get_packet_value(packet: &Packet) -> u64 {
	match &packet.value {
		PacketType::Literal(value) => value.data,
		PacketType::Operator(packet_data) => match packet_data.type_id {
			0 => packet_data.sub_packets.iter().map(get_packet_value).sum(),
			1 => packet_data.sub_packets.iter().map(get_packet_value).product(),
			2 => packet_data.sub_packets.iter().map(get_packet_value).min().unwrap(),
			3 => packet_data.sub_packets.iter().map(get_packet_value).max().unwrap(),
			5 => {
				if get_packet_value(&packet_data.sub_packets[0]) > get_packet_value(&packet_data.sub_packets[1]) {
					1
				} else {
					0
				}
			}
			6 => {
				if get_packet_value(&packet_data.sub_packets[0]) < get_packet_value(&packet_data.sub_packets[1]) {
					1
				} else {
					0
				}
			}
			7 => {
				if get_packet_value(&packet_data.sub_packets[0]) == get_packet_value(&packet_data.sub_packets[1]) {
					1
				} else {
					0
				}
			}
			_ => panic!("Invalid packet type: {}", packet_data.type_id),
		},
	}
}

fn main() {
	let raw_packet_data = fs::read_to_string("input.txt").unwrap().trim().to_owned();

	let data = HexadecimalParser::new(raw_packet_data);

	let (packet, _) = parse_packet_at_position(&data, 0);
	let value = get_packet_value(&packet);
	println!("{}", value);
}
