import os
import zlib

file = "1level"
input_folder = "./levels"
output_folder = "./out"

unparsable_levels = {}
warning_levels = {}
known_elements = {
    0x1: "🦀",
    0x2: "..", # Keke
    0x3: "..", # Rock
    0x5: "..",
    0x6: "&&",
    0x101: "🧱",
    0x104: "We",
    0x10a: "..", # Keke text
    0x109: "Fe",
    0x10b: "Fg",
    0x200: "🚩",
    0x201: "..", # Ice
    0x202: "Sh",
    0x209: "..", # Rock text
    0x20b: "Wa",
    0x300: "..", # Ice text
    0x301: "==",
    0x304: "..", # End
    0x30a: "Wi",
    0x30b: "Pu",
    0x400: "St",
    0x401: "Mv",
    0x409: "Me",
    0x40a: "Ho",
    0x40b: "U ",
    0x600: "..",
    0x701: "Ha",
    0x704: "..", # Belt
    0x709: "..", # Belt text
    0xffff: ".."
}
unknown_elements = {}

levels = [filename for filename in os.listdir(input_folder) if filename[-2:] == ".l"]

for level in levels:
    with open(input_folder + "/" + level, "rb") as level_file:
        with open(input_folder + "/" + level + "d", "r") as level_description_file:
            level_description_file.readline()
            level_description = level_description_file.readline()[5:-1]

        level_binary_content = level_file.read()
        if level_binary_content[:24] != b"ACHTUNG!\x05\x01MAP \x02\x00\x00\x00\x00\x00LAYR":
            unparsable_levels[level] = "Does not start with fixed prefix"
            continue

        width = int.from_bytes(level_binary_content[30:31], "little")
        height = int.from_bytes(level_binary_content[34:35], "little")

        if level_binary_content[35:71] != b"\x00\x00\x00\x0c\x00\x0c\x00\x00\xff\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80?\x00\x00\x80?\x00\x00\x01\x00\x00\x80?\xff\xff\xff\x02":
            unparsable_levels[level] = "Does not contain fixed delimiter"
            continue

        if level_binary_content[71:75] != b"MAIN":
            unparsable_levels[level] = "Does not contain MAIN section"
            continue

        main_length = int.from_bytes(level_binary_content[75:79], "little")
        main = zlib.decompress(level_binary_content[79:79+main_length])

        if level_binary_content[79+main_length:79+main_length+9] != b"DATA\x01\x00\x00\x00\x00":
            unparsable_levels[level] = "Does not contain DATA section"
            continue

        if len(main) != 2*width*height:
            unparsable_levels[level] = "Invalid dimensions ? main section length %d, width %d, length %d" % (len(main), width, length)
            continue

        data_length = int.from_bytes(level_binary_content[79+main_length+9:79+main_length+13], "little")
        data = zlib.decompress(level_binary_content[79+main_length+13:79+main_length+13+data_length])

        if len(main) != 2*width*height:
            unparsable_levels[level] = "Invalid dimensions ? main section length %d, width %d, length %d" % (len(main), width, length)
            continue

        output = ""
        for y in range(0, height):
            for x in range(0, width):
                local_element = int.from_bytes(main[2*(y*width + x):2*(y*width + x + 1)], "little")
                if local_element == 0x0000:
                    continue
                elif local_element not in known_elements:
                    warning_levels[level] = "unknown element %s at position (%d, %d)" % (hex(local_element), x, y)
                    # print(warning_levels[level])
                    if local_element not in unknown_elements:
                        unknown_elements[local_element] = [level]
                    elif level not in unknown_elements[local_element]:
                        unknown_elements[local_element] = unknown_elements[local_element] + [level]
                    output += "??"
                else:
                    output += known_elements[local_element]
            output += "\n"

        if len(level_binary_content) > 79+main_length+13+data_length:
            warning_levels[level] = "Level contains additional data"

        print("%s (%s)" % (level_description, level))
        print(output[1:])

unknown_elements_numbers = list(unknown_elements.keys())
unknown_elements_numbers.sort(key = lambda n: len(unknown_elements[n]), reverse=True)
print("%d unknown elements!" % len(unknown_elements_numbers))
print("Most common unknown elements:")
for unknown_element_number in unknown_elements_numbers[:5]:
    print("%s with %d occurences, in levels %s" % (hex(unknown_element_number), len(unknown_elements[unknown_element_number]), str(unknown_elements[unknown_element_number])))
# print("Unparsable levels:", unparsable_levels)
# print("Levels parsed with warning:", warning_levels)
