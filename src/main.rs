#[path = "circuit/resistor.rs"] mod resistor;
mod terminal;


fn main() {
	let colours = terminal::read_colours();
    let resistance = resistor::get_resistance_from_colours(colours);
    println!("Resistance: {}Ω", resistance);
    // let mut new_circuit = circuit::create_circuit();
    // new_circuit.add_element(
    //     (24.0, true),
    //     (0.0, false),
    //     (0.0, false),
    //     (0.0, false)
    // );
    // new_circuit.add_element(
    //     (24.0, true),
    //     (5.0, true),
    //     (0.0, false),
    //     (0.0, false)
    // );
    // new_circuit.add_element(
    //     (24.0, true),
    //     (0.0, false),
    //     (0.0, false),
    //     (5.0, true)
    // );
    // let elements = new_circuit.get_elements();

    // for item in elements.iter() {
    //     item.print()
    // }
}