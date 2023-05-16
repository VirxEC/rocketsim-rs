# read RocketSim/src/RLConst.h and generate src/consts.rs

lines = []

with open("RocketSim/src/RLConst.h") as file:
    lines = file.readlines()

consts = {}

current_section = []
open_braces = 0
const_type = None

for line in lines:
    line = line.strip()

    if line == "" or line.startswith("#") or line.startswith("//"):
        continue

    if line.startswith("namespace"):
        current_section.append(line.split()[1])
        consts[" ".join(current_section)] = {}

    if line.endswith("{"):
        open_braces += 1
        continue

    if line.endswith("}"):
        open_braces -= 1

        if open_braces == 0:
            current_section.pop()

        if open_braces == 1 and len(current_section) == 2:
            current_section.pop()

        continue

    namespace = " ".join(current_section)

    if line.startswith("constexpr"):
        const_type = line.split(" ")[1]
        consts[namespace][const_type] = []

    items = line.split(" = ")
    if len(items) == 1:
        continue

    comment = items[1].split(" //")
    if len(comment) == 2:
        items[1] = comment[0]
        items.append(comment[1].strip())

    if items[1].endswith(",") or items[1].endswith(";"):
        items[1] = items[1][:-1]

    items[1] = items[1].replace("f", "")

    consts[namespace][const_type].append(items)

consts_rs = []

type_convert = {
    "float": "f32",
    "int": "i32",
}

for namespace, types in consts.items():
    namespace = namespace.removeprefix("RLConst").strip()

    if namespace == "":
        namespace = None
        indent = ""
    else:
        consts_rs.append(f"pub mod {namespace} {{")
        indent = "    "
    
    for item_type, vars in types.items():
        item_type = type_convert[item_type]

        for var in vars:
            name = var[0]
            value = var[1]
            comment = var[2] if len(var) == 3 else None

            if comment is not None:
                consts_rs.append(f"{indent}/// {comment}")
            consts_rs.append(f"{indent}pub const {name}: {item_type} = {value};")
            consts_rs.append("")

    if namespace is not None:
        consts_rs[-1] = "}\n"

with open("src/consts.rs", "w") as file:
    file.write("\n".join(consts_rs))