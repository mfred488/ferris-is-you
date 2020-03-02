import os
import zlib

file = "1level"
input_folder = input("Please enter the folder containing the levels:")
if len(input_folder) == 0:
    input_folder = "/home/mfred/baba_is_you_levels/levels" # Shortcut for mfred
output_folder = os.sep.join(["..", "levels", "baba_is_you"])

unparsable_levels = {}
warning_levels = {}
known_elements = {
    0x1: "🦀",
    0x2: "👽",
    0x3: "..", # Rock
    0x4: "..", # Grass text
    0x5: "..", # Tile
    0x6: "&&",
    0x7: "..", # ??? looks like an empty tile in Gallery
    0x8: "..", # Hide text
    0x9: "Fl",
    0xa: "..", # Lonely text
    0xb: "🔥",
    0x100: "🌊",
    0x101: "🧱",
    0x102: "..", # Empty text
    0x103: "..", # Tile text
    0x104: "We",
    0x105: "..", # Near text
    0x108: "Fu",
    0x10a: "ET",
    0x106: "..", # Cloud
    0x107: "..", # Pillar
    0x109: "Fe",
    0x10b: "Fg",
    0x200: "🚩",
    0x201: "❄️",
    0x202: "Sh",
    0x203: "..", # Single letter
    0x204: "..", # All text
    0x205: "..", # Right
    0x206: "..", # Single letter
    0x207: "..", # Pillar text
    0x208: "🍄",
    0x209: "..", # Rock text
    0x20a: "La",
    0x20b: "Wa",
    0x300: "Ic",
    0x301: "==",
    0x303: "..", # More text
    0x304: "..", # End
    0x305: "..", # Up text
    0x306: "⭐",
    0x307: "..", # Single letter
    0x308: "Ba",
    0x309: "Wa",
    0x30a: "Wi",
    0x30b: "Pu",
    0x400: "St",
    0x401: "Mv",
    0x402: "..", # Best text
    0x403: "..", # Tele
    0x404: "🤚",
    0x405: "..", # Left text
    0x406: "Sr",
    0x407: "..", # Single letter
    0x408: "🦇",
    0x409: "Me",
    0x40a: "Ho",
    0x40b: "U ",
    0x500: "..", # Not text
    0x501: "Si",
    0x502: "❤️", # ??? Empty in forest 12 lock the door
    0x503: "🚪",
    0x504: "Hd",
    0x505: "..", # Down text
    0x506: "..", # Dust
    0x507: "..", # Flower text
    0x508: "Ro",
    0x50a: "Df",
    0x50b: "💀",
    0x600: "..", # Grass
    0x601: "Sk",
    0x602: "Lv",
    0x603: "Do",
    0x604: "Te",
    0x605: "..", # Sleep text
    0x606: "..", # Dust text
    0x607: "..", # Single letter
    0x608: "🚀",
    0x609: "🔑",
    0x60a: "Ke",
    0x60b: "Op",
    0x700: "Cl",
    0x701: "Ha",
    0x702: "📦",
    0x703: "Bx",
    0x704: "..", # Belt
    0x705: "..", # Make text
    0x706: "..", # Fall text
    0x707: "..", # Flower
    0x708: "..", # Fence text
    0x709: "..", # Belt text
    0x70a: "..", # Me
    0x70b: "..", # Me text
    0x802: "..", # Swap text
    0x803: "Pl",
    0x804: "..", # On text
    0x806: "🌙",
    0x808: "..", # Fence
    0x900: "Cf",
    0x901: "⛰️",
    0x902: "..", # Level text
    0x903: "..", # Orb text
    0x904: "..", # Small tree ? decorative in ruins 4
    0x905: "..", # Bonus text
    0x906: "Mo",
    0x907: "..", # Group text
    0x908: "..", # Line text
    0x909: "..", # Some kind of tiles ? decorative in cavern 6
    0x90a: "..", # Single letter
    0xa09: "..", # Facing text
    0xa01: "..", # Statue text
    0xa02: "..", # Statue
    0xa05: "..", # ???
    0xa08: "..", # Some kind of line ...
    0xa0a: "👻",
    0xa0b: "Gh",
    0xb00: "..", # Word text
    0xffff: ".."
}
unknown_elements = {}

levels = [filename for filename in os.listdir(input_folder) if filename[-2:] == ".l"]

output_levels_count = 0
for level in levels:
    with open(input_folder + os.sep + level, "rb") as level_file:
        with open(input_folder + os.sep + level + "d", "r") as level_description_file:
            level_description_file.readline()
            level_description = level_description_file.readline()[5:-1]

        level_binary_content = level_file.read()
        if level_binary_content[:24] != b"ACHTUNG!\x05\x01MAP \x02\x00\x00\x00\x00\x00LAYR":
            unparsable_levels[level] = "Does not start with fixed prefix"
            continue

        width = int.from_bytes(level_binary_content[30:31], "little")
        height = int.from_bytes(level_binary_content[34:35], "little")

        index = 30
        iterations = 0
        while index < len(level_binary_content):
            index += 5 # width and height
            if level_binary_content[index:index+36] != b"\x00\x00\x00\x0c\x00\x0c\x00\x00\xff\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80?\x00\x00\x80?\x00\x00\x01\x00\x00\x80?\xff\xff\xff\x02":
                unparsable_levels[level] = "Does not contain fixed delimiter"
                continue
            index += 36

            if level_binary_content[index:index+4] != b"MAIN":
                unparsable_levels[level] = "Does not contain MAIN section"
                continue
            index += 4

            main_length = int.from_bytes(level_binary_content[index:index+4], "little")
            index += 4
            main = zlib.decompress(level_binary_content[index:index+main_length])
            index += main_length

            if level_binary_content[index:index+9] != b"DATA\x01\x00\x00\x00\x00":
                unparsable_levels[level] = "Does not contain DATA section"
                continue
            index += 9

            if len(main) != 2*width*height:
                unparsable_levels[level] = "Invalid dimensions ? main section length %d, width %d, length %d" % (len(main), width, length)
                continue

            data_length = int.from_bytes(level_binary_content[index:index+4], "little")
            index += 4
            data = zlib.decompress(level_binary_content[index:index+data_length])
            index += data_length

            if len(main) != 2*width*height:
                unparsable_levels[level] = "Invalid dimensions ? main section length %d, width %d, length %d" % (len(main), width, length)
                continue

            output = ""
            for y in range(0, height):
                for x in range(0, width):
                    local_element = int.from_bytes(main[2*(y*width + x):2*(y*width + x + 1)], "little")
                    # print((hex(local_element), x, y))
                    if local_element == 0x0000:
                        continue
                    elif local_element not in known_elements:
                        warning_levels[level] = "unknown element %s at position (%d, %d)" % (hex(local_element), x, y)
                        print(warning_levels[level])
                        if local_element not in unknown_elements:
                            unknown_elements[local_element] = [level]
                        elif level not in unknown_elements[local_element]:
                            unknown_elements[local_element] = unknown_elements[local_element] + [level]
                        output += "??"
                    else:
                        output += known_elements[local_element]
                output += "\n"

            if len(main.replace(b"\xff", b"")) > 0:
                # print("%s (%s)" % (level_description, level))
                # print(output[1:])
                if iterations == 0:
                    with open(output_folder + os.sep + level, "w") as output_file:
                        output_file.write("# " + level_description + " \n")
                        output_file.write(output[1:])
                    output_levels_count += 1
                else:
                    warning_levels[level] = "Level contains additional data"

            iterations += 1

# unknown_elements_numbers = list(unknown_elements.keys())
# unknown_elements_numbers.sort(key = lambda n: len(unknown_elements[n]), reverse=True)
# print("%d unknown elements!" % len(unknown_elements_numbers))
# print("Most common unknown elements:")
# for unknown_element_number in unknown_elements_numbers[:5]:
#     print("%s with %d occurences, in levels %s" % (hex(unknown_element_number), len(unknown_elements[unknown_element_number]), str(unknown_elements[unknown_element_number])))
print("%d levels have been found in the original folder, and %d levels have been created in %s" % (len(levels), output_levels_count, output_folder))
if len(unparsable_levels) > 0:
    print("Unparsable levels:", unparsable_levels)
if len(warning_levels) > 0:
    print("Levels parsed with warning:", warning_levels)
