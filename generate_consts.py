import re

# read RocketSim/src/RLConst.h and generate src/consts.rs


def ensure_rust_float(val: str):
    try:
        val = val.strip()
        if str(int(val)) == val:
            val += "."
    except ValueError:
        pass

    return val


def to_vec3_str(vals):
    return f"Vec3::new({', '.join(vals)})"


def to_car_spawn_pos_str(vals):
    return f"CarSpawnPos::new({', '.join(vals)})"


def to_linear_piece_curve_str(vals, indent):
    if len(vals) < 1:
        indents = [" ", " ", f" {indent}", "", ""]
    elif len(vals) > 4:
        indents = [
            f"\n    {indent}",
            f",\n{indent}",
            f"\n        {indent}",
            f"\n        {indent}",
            f",\n    {indent}",
        ]
    else:
        indents = [f"\n    {indent}", f",\n{indent}", f" {indent}", "", ""]
    join_val = f",{indents[2]}".join(
        [f"({ensure_rust_float(val[0])}, {ensure_rust_float(val[1])})" for val in vals]
    )
    return f"LinearPieceCurve {{{indents[0]}value_mappings: [{indents[3]}{join_val}{indents[4]}]{indents[1]}}}"


lines = []

with open("RocketSim/src/RLConst.h") as file:
    lines = file.readlines()

consts = {}

current_section = []
open_braces = 0
const_type = None

for i, line in enumerate(lines):
    line = line.strip()

    if line == "" or line.startswith("#") or line.startswith("//"):
        continue

    if line.startswith("namespace"):
        current_section.append(line.split()[1])
        consts[" ".join(current_section)] = {}
        open_braces += 1
        continue

    if "{" in line:
        open_braces += 1

    if "}" in line:
        open_braces -= 1

        if open_braces == 0:
            current_section.pop()
            continue
        elif open_braces == 1 and len(current_section) == 2:
            current_section.pop()
            continue

    namespace = " ".join(current_section)

    if line.startswith("constexpr static"):
        parts = line.split(" ")
        const_type = parts[2]

        name = None

        if len(parts) > 3 and parts[3] != "//":
            name = parts[3]
        else:
            next_line = lines[i + 1]
            pred_name = next_line.split(" = ")[0].strip()

            if "[" in pred_name:
                name = pred_name

        if name is not None and "[" in name:
            array_len = name.split("[")[1].removesuffix("]")
            if array_len.isdigit():
                const_type = f"[{const_type}; {array_len}]"
            else:
                const_type = f"[{const_type}; {array_len} as usize]"

        if consts[namespace].get(const_type) is None:
            consts[namespace][const_type] = []

        if name is not None and "[" in name:
            consts[namespace][const_type].append([name.split("[")[0].split()[-1], []])
    elif line.startswith("constexpr"):
        parts = line.split(" ")
        const_type = parts[1]

        name = None

        if len(parts) > 2 and parts[2] != "//":
            name = parts[2]
        else:
            next_line = lines[i + 1]
            pred_name = next_line.split(" = ")[0].strip()

            if "[" in pred_name:
                name = pred_name

        if name is not None and "[" in name:
            array_len = name.split("[")[1].removesuffix("]")
            const_type = f"[{const_type}; {array_len} as usize]"

        if consts[namespace].get(const_type) is None:
            consts[namespace][const_type] = []

        if name is not None:
            consts[namespace][const_type].append([name.split("[")[0].split()[-1], []])

    elif line.startswith("const static"):
        parts = line.split(" ")
        const_type = parts[2]
        name = None

        if len(parts) > 3 and parts[3] != "//":
            name = parts[3]
        else:
            next_line = lines[i + 1]
            pred_name = next_line.split(" = ")[0].strip()

            if "[" in pred_name:
                name = pred_name

        if name is not None and "[" in name:
            array_len = name.split("[")[1].removesuffix("]")
            const_type = f"[{const_type}; {array_len} as usize]"

        if consts[namespace].get(const_type) is None:
            consts[namespace][const_type] = []

        if name is not None:
            consts[namespace][const_type].append([name.split("[")[0].split()[-1], []])

    items = line.split(" = ")

    if len(items) == 1:
        if items[0][0] != "{":
            continue

        array = items[0][1:].split("}")[0].strip()
        if len(array) < 2:
            continue

        items = [
            ensure_rust_float(item.strip().replace("f", "").replace("M_", ""))
            for item in array.split(",")
        ]

        consts[namespace][const_type][-1][1].append(items)
        continue

    if "[" in items[0]:
        continue

    items[0] = (
        items[0]
        .replace("constexpr", "")
        .replace("static", "")
        .replace("const", "")
        .replace(const_type, "")  # type: ignore
        .strip()
    )
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
        else:
            items[1] = items[1].replace("M_PI", "PI")

        if items[1].startswith("(") and items[1].endswith(")"):
            items[1] = items[1].removeprefix("(").removesuffix(")")

        vals = items[1].split()
        for i, valz in enumerate(vals):
            if valz == "/":
                continue
            elif valz == "<<":
                vals[i + 1] = vals[i + 1] + " as f32"
                continue

            vals[i] = ensure_rust_float(valz)
        items[1] = " ".join(vals)

        vals = items[1].split("/")
        for i, valz in enumerate(vals):
            if valz == "/":
                continue

            vals[i] = ensure_rust_float(valz)

        items[1] = " / ".join(vals)

    elif const_type == "Vec":
        vals = items[1].removeprefix("Vec(").removesuffix(")").split(", ")

        for i, valz in enumerate(vals):
            if valz == "/":
                continue

            vals[i] = ensure_rust_float(valz)

        items[1] = to_vec3_str(vals)
    elif const_type == "LinearPieceCurve":
        continue

    consts[namespace][const_type].append(items)

consts_rs = [
    "// This file was generated by generate_consts.py",
    "",
    "use crate::{CarSpawnPos, LinearPieceCurve, math::Vec3};",
    "use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4, PI};",
    "",
]

type_convert = {
    "float": "f32",
    "int": "i32",
    "Vec": "Vec3",
    "CarSpawnPos": "CarSpawnPos",
    "LinearPieceCurve": "LinearPieceCurve",
}

for namespace, types in consts.items():
    namespace = namespace.removeprefix("RLConst").strip().lower()

    if namespace == "":
        namespace = None
        indent = ""
    else:
        consts_rs.append(f"\npub mod {namespace} {{")
        indent = "    "

        if namespace in {"boostpads", "heatseeker", "dropshot"}:
            consts_rs.append(f"{indent}use crate::math::Vec3;")

            if namespace == "heatseeker":
                consts_rs.append(f"{indent}use std::f32::consts::PI;")
            consts_rs.append("")

        if namespace == "dropshot":
            consts_rs.append(f"{indent}const BT_TO_UU: f32 = 50.0;\n")

    for raw_item_type, vars in types.items():
        if "[" in raw_item_type:
            item_type = raw_item_type
            old_type = raw_item_type.split(";")[0][1:]
            real_type = type_convert.get(old_type)

            if real_type is None:
                print(f"Couldn't find Rust type for {raw_item_type} ({vars})")
                continue

            item_type = item_type.replace(old_type, real_type)
        else:
            item_type = type_convert.get(raw_item_type)
            real_type = ""  # anything other than None

        if item_type is None or real_type is None:
            print(f"Couldn't find Rust type for {raw_item_type} ({vars})")
            continue

        for var in vars:
            name = var[0]
            val = var[1]
            comment = var[2] if len(var) == 3 else None

            if real_type != "":
                if real_type == "Vec3":
                    conv_func = to_vec3_str
                elif real_type == "CarSpawnPos":
                    for vals in val:
                        for i in range(len(vals)):
                            if "PI / 2" == vals[i]:
                                vals[i] = "FRAC_PI_2"
                                continue

                            parts = vals[i].split()

                            for i in range(len(parts)):
                                if parts[i] == "PI_4":
                                    parts[i] = "FRAC_PI_4"
                                else:
                                    parts[i] = ensure_rust_float(parts[i])

                            vals[i] = " ".join(parts)
                    conv_func = to_car_spawn_pos_str
                else:
                    continue

                vals = f",\n".join([f"    {indent}" + conv_func(vals) for vals in val])
                val = f"[\n{vals},\n{indent}]"

            if item_type.startswith("LinearPieceCurve"):
                if "<" in item_type:
                    item_type = item_type[:-3]
                item_type += f"<{len(val)}>"
                val = to_linear_piece_curve_str(val, indent)

            if comment is not None:
                consts_rs.append(f"{indent}/// {comment}")
            consts_rs.append(f"{indent}pub const {name}: {item_type} = {val};")

    if namespace is not None:
        consts_rs.append("}")

consts_rs.append("")

with open("src/consts.rs", "w") as file:
    file.write("\n".join(consts_rs))
