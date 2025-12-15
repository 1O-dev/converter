use std::process;

#[derive(Debug, Clone, Copy, PartialEq)]
enum UnitCategory {
    Length,
    Temperature,
    Mass,
}

#[derive(Debug)]
struct Unit {
    name: &'static str,
    aliases: &'static [&'static str],
    category: UnitCategory,
    to_base: fn(f64) -> f64,
    from_base: fn(f64) -> f64,
}

impl Unit {
    fn matches(&self, input: &str) -> bool {
        self.name.eq_ignore_ascii_case(input) || 
        self.aliases.iter().any(|a| a.eq_ignore_ascii_case(input))
    }
}

const UNITS: &[Unit] = &[
    Unit { 
        name: "km", 
        aliases: &["kilometer", "kilometers", "kilometre", "kilometres"],
        category: UnitCategory::Length,
        to_base: |v| v * 1000.0,
        from_base: |v| v / 1000.0,
    },
    Unit { 
        name: "m", 
        aliases: &["meter", "meters", "metre", "metres"],
        category: UnitCategory::Length,
        to_base: |v| v,
        from_base: |v| v,
    },
    Unit { 
        name: "cm", 
        aliases: &["centimeter", "centimeters", "centimetre", "centimetres"],
        category: UnitCategory::Length,
        to_base: |v| v * 0.01,
        from_base: |v| v / 0.01,
    },
    Unit { 
        name: "mm", 
        aliases: &["millimeter", "millimeters", "millimetre", "millimetres"],
        category: UnitCategory::Length,
        to_base: |v| v * 0.001,
        from_base: |v| v / 0.001,
    },
    Unit { 
        name: "mi", 
        aliases: &["mile", "miles"],
        category: UnitCategory::Length,
        to_base: |v| v * 1609.344,
        from_base: |v| v / 1609.344,
    },
    Unit { 
        name: "yd", 
        aliases: &["yard", "yards"],
        category: UnitCategory::Length,
        to_base: |v| v * 0.9144,
        from_base: |v| v / 0.9144,
    },
    Unit { 
        name: "ft", 
        aliases: &["foot", "feet"],
        category: UnitCategory::Length,
        to_base: |v| v * 0.3048,
        from_base: |v| v / 0.3048,
    },
    Unit { 
        name: "in", 
        aliases: &["inch", "inches"],
        category: UnitCategory::Length,
        to_base: |v| v * 0.0254,
        from_base: |v| v / 0.0254,
    },
    Unit { 
        name: "C", 
        aliases: &["celsius", "centigrade"],
        category: UnitCategory::Temperature,
        to_base: |v| v,
        from_base: |v| v,
    },
    Unit { 
        name: "F", 
        aliases: &["fahrenheit"],
        category: UnitCategory::Temperature,
        to_base: |v| (v - 32.0) * 5.0 / 9.0,
        from_base: |v| v * 9.0 / 5.0 + 32.0,
    },
    Unit { 
        name: "K", 
        aliases: &["kelvin"],
        category: UnitCategory::Temperature,
        to_base: |v| v - 273.15,
        from_base: |v| v + 273.15,
    },
    Unit { 
        name: "kg", 
        aliases: &["kilogram", "kilograms"],
        category: UnitCategory::Mass,
        to_base: |v| v,
        from_base: |v| v,
    },
    Unit { 
        name: "g", 
        aliases: &["gram", "grams"],
        category: UnitCategory::Mass,
        to_base: |v| v * 0.001,
        from_base: |v| v / 0.001,
    },
    Unit { 
        name: "mg", 
        aliases: &["milligram", "milligrams"],
        category: UnitCategory::Mass,
        to_base: |v| v * 0.000001,
        from_base: |v| v / 0.000001,
    },
    Unit { 
        name: "lb", 
        aliases: &["pound", "pounds"],
        category: UnitCategory::Mass,
        to_base: |v| v * 0.45359237,
        from_base: |v| v / 0.45359237,
    },
    Unit { 
        name: "oz", 
        aliases: &["ounce", "ounces"],
        category: UnitCategory::Mass,
        to_base: |v| v * 0.028349523125,
        from_base: |v| v / 0.028349523125,
    },
    Unit { 
        name: "ton", 
        aliases: &["tons", "tonne", "tonnes", "metric ton"],
        category: UnitCategory::Mass,
        to_base: |v| v * 1000.0,
        from_base: |v| v / 1000.0,
    },
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() == 2 {
        match args[1].as_str() {
            "--help" | "-h" => { print_help(&args[0]); return; }
            "--version" | "-v" => { println!("Unit Converter v3.0.0"); return; }
            "--list" | "-l" => { print_units(); return; }
            _ => {}
        }
    }
    
    if args.len() != 4 {
        eprintln!("Error: Expected 3 arguments, got {}", args.len() - 1);
        eprintln!("Usage: {} <value> <from_unit> <to_unit>", args[0]);
        eprintln!("Try '{} --help' for more information", args[0]);
        process::exit(1);
    }
    
    let value: f64 = match args[1].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", args[1]);
            process::exit(1);
        }
    };
    
    let from_unit = &args[2];
    let to_unit = &args[3];
    
    let from = find_unit(from_unit);
    let to = find_unit(to_unit);
    
    match (from, to) {
        (Some(f), Some(t)) => {
            if f.category != t.category {
                eprintln!("Error: Cannot convert between different unit categories");
                eprintln!("  {} is a {:?} unit", from_unit, f.category);
                eprintln!("  {} is a {:?} unit", to_unit, t.category);
                process::exit(1);
            }
            
            if f.category == UnitCategory::Length && value < 0.0 {
                eprintln!("Warning: Negative length doesn't make physical sense");
            }
            
            if f.category == UnitCategory::Temperature && f.name == "K" && value < 0.0 {
                eprintln!("Error: Temperature below absolute zero");
                process::exit(1);
            }
            
            if f.category == UnitCategory::Temperature && f.name == "C" && value < -273.15 {
                eprintln!("Error: Temperature below absolute zero");
                process::exit(1);
            }
            
            if f.category == UnitCategory::Temperature && f.name == "F" && value < -459.67 {
                eprintln!("Error: Temperature below absolute zero");
                process::exit(1);
            }
            
            let base_value = (f.to_base)(value);
            let result = (t.from_base)(base_value);
            println!("{} {} = {} {}", value, from_unit, result, to_unit);
        }
        (None, _) => {
            eprintln!("Error: Unknown unit '{}'", from_unit);
            eprintln!("Try '{} --list' to see supported units", args[0]);
            process::exit(1);
        }
        (_, None) => {
            eprintln!("Error: Unknown unit '{}'", to_unit);
            eprintln!("Try '{} --list' to see supported units", args[0]);
            process::exit(1);
        }
    }
}

fn find_unit(input: &str) -> Option<&'static Unit> {
    UNITS.iter().find(|u| u.matches(input))
}

fn print_help(program: &str) {
    println!("Unit Converter v3.0.0");
    println!();
    println!("USAGE:");
    println!("    {} <value> <from_unit> <to_unit>", program);
    println!();
    println!("EXAMPLES:");
    println!("    {} 5 km mi", program);
    println!("    {} 100 feet meters", program);
    println!("    {} 100 C F", program);
    println!("    {} 150 kg lb", program);
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Show this help message");
    println!("    -v, --version    Show version information");
    println!("    -l, --list       List all supported units");
    println!();
    println!("Note: Unit names are case-insensitive and support common aliases");
}

fn print_units() {
    println!("Supported units:");
    println!();
    
    let categories = [
        (UnitCategory::Length, "Length"),
        (UnitCategory::Temperature, "Temperature"),
        (UnitCategory::Mass, "Mass"),
    ];
    
    for (cat, name) in categories {
        println!("{}:", name);
        for unit in UNITS.iter().filter(|u| u.category == cat) {
            print!("  {} ", unit.name);
            if !unit.aliases.is_empty() {
                print!("({})", unit.aliases.join(", "));
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
        assert!((a - b).abs() < epsilon, "{} != {} (epsilon: {})", a, b, epsilon);
    }
    
    #[test]
    fn test_km_to_miles() {
        let km = find_unit("km").unwrap();
        let mi = find_unit("mi").unwrap();
        let base = (km.to_base)(5.0);
        let result = (mi.from_base)(base);
        assert_approx_eq(result, 3.10686, 0.00001);
    }
    
    #[test]
    fn test_celsius_to_fahrenheit() {
        let c = find_unit("C").unwrap();
        let f = find_unit("F").unwrap();
        let base = (c.to_base)(100.0);
        let result = (f.from_base)(base);
        assert_approx_eq(result, 212.0, 0.00001);
    }
    
    #[test]
    fn test_celsius_to_fahrenheit_freezing() {
        let c = find_unit("C").unwrap();
        let f = find_unit("F").unwrap();
        let base = (c.to_base)(0.0);
        let result = (f.from_base)(base);
        assert_approx_eq(result, 32.0, 0.00001);
    }
    
    #[test]
    fn test_kg_to_pounds() {
        let kg = find_unit("kg").unwrap();
        let lb = find_unit("lb").unwrap();
        let base = (kg.to_base)(10.0);
        let result = (lb.from_base)(base);
        assert_approx_eq(result, 22.0462, 0.0001);
    }
    
    #[test]
    fn test_mg_to_kg() {
        let mg = find_unit("mg").unwrap();
        let kg = find_unit("kg").unwrap();
        let base = (mg.to_base)(1000000.0);
        let result = (kg.from_base)(base);
        assert_approx_eq(result, 1.0, 0.00001);
    }
    
    #[test]
    fn test_g_to_mg() {
        let g = find_unit("g").unwrap();
        let mg = find_unit("mg").unwrap();
        let base = (g.to_base)(1.0);
        let result = (mg.from_base)(base);
        assert_approx_eq(result, 1000.0, 0.00001);
    }
    
    #[test]
    fn test_unit_matching_case_insensitive() {
        assert!(find_unit("KM").is_some());
        assert!(find_unit("MeTErs").is_some());
        assert!(find_unit("FAHRENHEIT").is_some());
    }
    
    #[test]
    fn test_unit_aliases() {
        assert!(find_unit("kilometer").is_some());
        assert!(find_unit("kilometres").is_some());
        assert!(find_unit("celsius").is_some());
    }
    
    #[test]
    fn test_same_unit_conversion() {
        let m = find_unit("m").unwrap();
        let base = (m.to_base)(100.0);
        let result = (m.from_base)(base);
        assert_approx_eq(result, 100.0, 0.00001);
    }
    
    #[test]
    fn test_kelvin_to_celsius() {
        let k = find_unit("K").unwrap();
        let c = find_unit("C").unwrap();
        let base = (k.to_base)(273.15);
        let result = (c.from_base)(base);
        assert_approx_eq(result, 0.0, 0.00001);
    }
}