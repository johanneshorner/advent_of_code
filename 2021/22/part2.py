import re
import dataclasses

@dataclasses.dataclass
class Cuboid:
    x1: int; x2: int; y1: int; y2: int; z1: int; z2: int

    def volume(self):
        return abs(self.x1 - self.x2) * abs(self.y1 - self.y2) * abs(self.z1 - self.z2)

    def intersect(a, b):
        if not (a.x1 < b.x2 and a.x2 > b.x1
            and a.y1 < b.y2 and a.y2 > b.y1
            and a.z1 < b.z2 and a.z2 > b.z1):
            return [a]
        else:
            #intersection cuboid
            c = Cuboid(min(max(a.x1, b.x1), a.x2), min(max(a.x1, b.x2), a.x2),
                min(max(a.y1, b.y1), a.y2), min(max(a.y1, b.y2), a.y2),
                min(max(a.z1, b.z1), a.z2), min(max(a.z1, b.z2), a.z2))
        
            return [
                Cuboid(a.x1, c.x1, a.y1, a.y2, a.z1, a.z2),
                Cuboid(c.x2, a.x2, a.y1, a.y2, a.z1, a.z2),
                Cuboid(c.x1, c.x2, a.y1, c.y1, a.z1, a.z2),
                Cuboid(c.x1, c.x2, c.y2, a.y2, a.z1, a.z2),
                Cuboid(c.x1, c.x2, c.y1, c.y2, a.z1, c.z1),
                Cuboid(c.x1, c.x2, c.y1, c.y2, c.z2, a.z2)
            ]

def get_input():
    with open('input.in', 'r') as file:
        cuboids = [[string for string in re.findall('on|off|-?\d{1,}\.\.-?\d{1,}', line.strip())] for line in file]

        return [[cuboid[0], sum([tuple(map(lambda n: int(n), coords.split('..'))) for coords in cuboid[1:]], start=())] for cuboid in cuboids]

def solve(cuboids: list[list]):
    reactor: list[Cuboid] = []

    for cmd, (x1, x2, y1, y2, z1, z2) in cuboids:
        new_cuboid = Cuboid(x1, x2 + 1, y1, y2 + 1, z1, z2 + 1)
        reactor = [remainder for on_cuboid in reactor for remainder in on_cuboid.intersect(new_cuboid) if remainder.volume() > 0]
        if cmd == 'on':
            reactor.append(new_cuboid)

    return sum([cuboid.volume() for cuboid in reactor])

cuboids = get_input()

print(solve(cuboids))