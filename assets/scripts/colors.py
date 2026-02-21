import json
color_consts = []
color_names = []
palette_names = []
print("use super::*;\n")
print("#[rustfmt::skip]")
print("#[allow(clippy::excessive_precision)]")
print("impl Color {")
print("    pub const WHITE: Self = Self::from_rgb(1.0, 1.0, 1.0);")
print("    pub const BLACK: Self = Self::from_rgb(0.0, 0.0, 0.0);")
print("    pub const TRANSPARENT: Self = Self::from_rgba(0.0, 0.0, 0.0, 0.0);")

with open('colors.json', 'r') as f:
    data = json.load(f)
    for color_name in data.keys():
        for brightness in data[color_name]:
            name = f"{color_name}_{brightness}".upper()
            color_consts.append(f"Self::{name}")
            color_names.append(name)
            oklch = data[color_name][brightness]
            l = oklch[0] / 100
            c = oklch[1]
            h = oklch[2]
            print(f"    pub const {name}: Self = Self::from_oklch({l:.8f}, {c:.8f}, {h:.8f});")

print(f"\n    pub const ALL: [Self; {len(color_consts)}] = [{', '.join(color_consts)}];")
print("}")

print("\n#[rustfmt::skip]")
print("impl Palette {")
with open('colors.json', 'r') as f:
    data = json.load(f)
    for color_name in data.keys():
        palette_name = color_name.upper()
        palette_names.append(f"Self::{palette_name}")
        print(f"    pub const {palette_name}: Self = Self {{")
        print(f"        v50: Color::{color_name.upper()}_50,")
        print(f"        v100: Color::{color_name.upper()}_100,")
        print(f"        v200: Color::{color_name.upper()}_200,")
        print(f"        v300: Color::{color_name.upper()}_300,")
        print(f"        v400: Color::{color_name.upper()}_400,")
        print(f"        v500: Color::{color_name.upper()}_500,")
        print(f"        v600: Color::{color_name.upper()}_600,")
        print(f"        v700: Color::{color_name.upper()}_700,")
        print(f"        v800: Color::{color_name.upper()}_800,")
        print(f"        v900: Color::{color_name.upper()}_900,")
        print(f"        v950: Color::{color_name.upper()}_950,")
        print("    };")

print(f"\n    pub const PALETTES: [Self; {len(palette_names)}] = [{', '.join(palette_names)}];")
print("}")

print("\nuse phf::phf_map;\n")
print("pub static COLOR_MAP: phf::Map<&'static str, Color> = phf_map! {")
for i, name in enumerate(color_names):
    norm = name.replace("_", "").replace("00", "").replace("950", "9.5").replace("50", "0.5").replace("-", "").upper()
    print(f'    "{name}" => Color::{name},')
    print(f'    "{norm}" => Color::{name},')
    if "." in norm:
        norm2 = norm.replace(".", "")
        print(f'    "{norm2}" => Color::{name},')
print(f'    "WHITE" => Color::WHITE,')
print(f'    "BLACK" => Color::BLACK,')
print(f'    "TRANSPARENT" => Color::TRANSPARENT,')
print("};")
