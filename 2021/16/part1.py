TYPE_ID_LITERAL_VALUE = 4

with open("input.in") as file:
    bits = bin(int('1' + file.read().strip(), 16))[3:]

class Packet:
    def get_version_and_type_id(data):
        return (int(data[0:3], 2), int(data[3:6], 2), data[6:])

    def parse_literal_value(data):
        literal_value_bits = ""
        while True:
            literal_value_bits += data[1:5]
            if data[0] == "0":
                break
            data = data[5:]
        return (int(literal_value_bits, 2), data[5:])
        
    def parse_packet(data):
        sub_packets = []
        literal_value = None
        version, type_id, data = Packet.get_version_and_type_id(data)

        if type_id == TYPE_ID_LITERAL_VALUE:
            literal_value, data = Packet.parse_literal_value(data)
        else:
            if data[0] == "0":
                sub_packet_size = 16 + int(data[1:16], 2)
                current_data_len = len(data)
                data = data[16:]

                while len(data) > current_data_len - sub_packet_size:
                    packet, data = Packet.parse_packet(data)
                    sub_packets.append(packet)
            else:
                number_of_sub_packages = int(data[1:12], 2)
                data = data[12:]
        
                for _ in range(number_of_sub_packages):
                    packet, data = Packet.parse_packet(data)
                    sub_packets.append(packet)

        return (Packet(version, type_id, sub_packets, literal_value), data)

    def __init__(self, version, type_id, sub_packets, literal_value) -> None:
        self.version = version
        self.type_id = type_id
        self.sub_packets = sub_packets
        self.literal_value = literal_value

packet = Packet.parse_packet(bits)[0]

def sum_versions(packet: Packet):
    sum = packet.version

    for sub_packet in packet.sub_packets:
        sum += sum_versions(sub_packet)

    return sum

print(sum_versions(packet))
