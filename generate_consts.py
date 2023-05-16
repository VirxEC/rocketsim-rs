# read RocketSim/src/RLConst.h and generate src/consts.rs

import subprocess


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
    
    if line.startswith("const static"):
        const_type = line.split(" ")[2]
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

    if const_type == "float":
        if items[1] == "M_SQRT1_2":
            items[1] = "FRAC_1_SQRT_2"

        if items[1].startswith("(") and items[1].endswith(")"):
            items[1] = items[1].removeprefix("(").removesuffix(")")

        vals = items[1].split()
        for i, val in enumerate(vals):
            if vals[i] == "/":
                continue

            try:
                if str(int(val)) == val:
                    vals[i] += "."
            except ValueError:
                pass

        items[1] = " ".join(vals)

    if const_type == "Vec":
        vals = items[1].removeprefix("Vec(").removesuffix(")").split(", ")

        for i, val in enumerate(vals):
            if vals[i] == "/":
                continue

            try:
                if str(int(val)) == val:
                    vals[i] += "."
            except ValueError:
                pass

        items[1] = f"Vec3::new({',  '.join(vals)})"

    consts[namespace][const_type].append(items)

consts_rs = ["use crate::math::Vec3;\nuse std::f32::consts::FRAC_1_SQRT_2;\n"]

type_convert = {
    "float": "f32",
    "int": "i32",
    "Vec": "Vec3",
}

for namespace, types in consts.items():
    namespace = namespace.removeprefix("RLConst").strip().lower()

    if namespace == "":
        namespace = None
        indent = ""
    else:
        consts_rs.append(f"pub mod {namespace} {{")
        indent = "    "
    
    for raw_item_type, vars in types.items():
        item_type = type_convert.get(raw_item_type)

        if item_type is None:
            print(f"Couldn't find Rust type for {raw_item_type}")

        for var in vars:
            name = var[0]
            val = var[1]
            comment = var[2] if len(var) == 3 else None

            if comment is not None:
                consts_rs.append(f"{indent}/// {comment}")
            consts_rs.append(f"{indent}pub const {name}: {item_type} = {val};")
            consts_rs.append("")

    if namespace is not None:
        consts_rs[-1] = "}\n"

with open("src/consts.rs", "w") as file:
    file.write("\n".join(consts_rs))

subprocess.run(["cargo", "fmt"])